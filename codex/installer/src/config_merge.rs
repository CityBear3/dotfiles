use std::collections::{BTreeMap, HashSet};

use toml::{Table, Value};

use crate::InstallerError;

const MANAGED_ROOT_KEYS: [&str; 3] = [
    "model",
    "model_reasoning_effort",
    "plan_mode_reasoning_effort",
];
const MANAGED_AGENT_KEYS: [&str; 2] = ["max_threads", "max_depth"];
const MANAGED_UPDATE_PLAN_KEYS: [&str; 1] = ["enabled"];
const UPDATE_PLAN_ENABLED_KEY: &str = "tools.update_plan.enabled";
const CONTEXT_MANAGEMENT_EXPERIMENTAL_MODE_KEY: &str =
    "features.context_management.experimental_mode";

#[derive(Clone, Debug, Eq, PartialEq)]
enum ManagedScalar {
    String(String),
    Integer(i64),
    Boolean(bool),
}

impl ManagedScalar {
    fn from_value(key: &str, value: &Value) -> Result<Self, InstallerError> {
        match value {
            Value::String(value) => Ok(Self::String(value.clone())),
            Value::Integer(value) => Ok(Self::Integer(*value)),
            Value::Boolean(value) => Ok(Self::Boolean(*value)),
            _ => Err(invalid_config(format!(
                "managed key {key:?} is not a renderable scalar"
            ))),
        }
    }

    fn render(&self) -> Result<String, InstallerError> {
        match self {
            Self::String(value) => serde_json::to_string(value).map_err(|error| {
                invalid_config(format!("could not render a managed string: {error}"))
            }),
            Self::Integer(value) => Ok(value.to_string()),
            Self::Boolean(value) => Ok(value.to_string()),
        }
    }

    fn as_toml(&self) -> Value {
        match self {
            Self::String(value) => Value::String(value.clone()),
            Self::Integer(value) => Value::Integer(*value),
            Self::Boolean(value) => Value::Boolean(*value),
        }
    }
}

#[derive(Default)]
struct LexicalState {
    multiline_delimiter: Option<&'static [u8; 3]>,
    array_depth: i32,
    inline_table_depth: i32,
}

impl LexicalState {
    fn at_statement_start(&self) -> bool {
        self.multiline_delimiter.is_none() && self.array_depth == 0 && self.inline_table_depth == 0
    }
}

struct Structure {
    statement_lines: HashSet<usize>,
    table_headers: Vec<usize>,
}

/// Merge the seven managed keys while preserving unmanaged configuration bytes.
pub(crate) fn merge_config(
    existing_text: &str,
    managed_text: &str,
    max_threads: u8,
) -> Result<String, InstallerError> {
    let existing = parse_toml(existing_text, "existing configuration")?;
    let managed = validated_managed_values(managed_text, max_threads)?;
    let mut lines = split_lines_keep_ends(existing_text);
    let structure = scan_toml_structure(&lines);
    let first_table = structure
        .table_headers
        .first()
        .copied()
        .unwrap_or(lines.len());

    let mut missing_root = Vec::new();
    for key in MANAGED_ROOT_KEYS {
        if !existing.contains_key(key) {
            missing_root.push(key);
            continue;
        }
        let assignment = unique_assignment(
            &lines,
            0,
            first_table,
            key,
            &structure.statement_lines,
            "at the document root",
        )?;
        lines[assignment] =
            replacement_line(&lines[assignment], key, managed_value(&managed, key))?;
    }
    if !missing_root.is_empty() {
        let insertion = before_trailing_blank_lines(&lines, 0, first_table);
        prepare_insertion(&mut lines, insertion);
        let assignments = missing_root
            .into_iter()
            .map(|key| new_assignment(key, managed_value(&managed, key)))
            .collect::<Result<Vec<_>, _>>()?;
        lines.splice(insertion..insertion, assignments);
    }

    match existing.get("agents") {
        Some(existing_agents) => {
            let existing_agents = existing_agents.as_table().ok_or_else(|| {
                invalid_config("existing configuration does not contain an ordinary [agents] table")
            })?;
            merge_existing_agents(&mut lines, existing_agents, &managed)?;
        }
        None => append_agents_table(&mut lines, &managed)?,
    }

    match existing.get("tools") {
        Some(existing_tools) => {
            let existing_tools = existing_tools.as_table().ok_or_else(|| {
                invalid_config("existing configuration does not contain an ordinary tools table")
            })?;
            match existing_tools.get("update_plan") {
                Some(existing_update_plan) => {
                    let existing_update_plan = existing_update_plan.as_table().ok_or_else(|| {
                        invalid_config(
                            "existing configuration does not contain an ordinary tools.update_plan table",
                        )
                    })?;
                    merge_existing_table_setting(
                        &mut lines,
                        existing_update_plan,
                        "tools.update_plan",
                        "enabled",
                        managed_value(&managed, UPDATE_PLAN_ENABLED_KEY),
                    )?;
                }
                None => append_table_setting(
                    &mut lines,
                    "tools.update_plan",
                    "enabled",
                    managed_value(&managed, UPDATE_PLAN_ENABLED_KEY),
                )?,
            }
        }
        None => append_table_setting(
            &mut lines,
            "tools.update_plan",
            "enabled",
            managed_value(&managed, UPDATE_PLAN_ENABLED_KEY),
        )?,
    }

    let context_management = match existing.get("features") {
        Some(features) => {
            let features = features.as_table().ok_or_else(|| {
                invalid_config("existing configuration does not contain an ordinary features table")
            })?;
            features
                .get("context_management")
                .map(|context| {
                    context.as_table().ok_or_else(|| {
                        invalid_config(
                            "existing configuration does not contain an ordinary features.context_management table",
                        )
                    })
                })
                .transpose()?
        }
        None => None,
    };
    let experimental_mode = managed_value(&managed, CONTEXT_MANAGEMENT_EXPERIMENTAL_MODE_KEY);
    match context_management {
        Some(context) => merge_existing_table_setting(
            &mut lines,
            context,
            "features.context_management",
            "experimental_mode",
            experimental_mode,
        )?,
        None => append_table_setting(
            &mut lines,
            "features.context_management",
            "experimental_mode",
            experimental_mode,
        )?,
    }

    let mut candidate = lines.concat();
    let trimmed_length = candidate.trim_end_matches(['\r', '\n']).len();
    candidate.truncate(trimmed_length);
    candidate.push('\n');

    let final_config = parse_toml(&candidate, "merged configuration")?;
    validate_managed_postcondition(&final_config, &managed)?;
    Ok(candidate)
}

fn merge_existing_agents(
    lines: &mut Vec<String>,
    existing_agents: &Table,
    managed: &BTreeMap<&str, ManagedScalar>,
) -> Result<(), InstallerError> {
    let structure = scan_toml_structure(lines);
    let agent_headers = structure
        .table_headers
        .iter()
        .copied()
        .filter(|index| is_exact_agents_header(line_body(&lines[*index])))
        .collect::<Vec<_>>();
    if agent_headers.len() != 1 {
        return Err(invalid_config(
            "existing agents value is not one exact [agents] table",
        ));
    }

    let mut missing_agents = Vec::new();
    for key in MANAGED_AGENT_KEYS {
        if !existing_agents.contains_key(key) {
            missing_agents.push(key);
            continue;
        }
        let assignment = unique_assignment(
            lines,
            agent_headers[0] + 1,
            structure
                .table_headers
                .iter()
                .copied()
                .find(|index| *index > agent_headers[0])
                .unwrap_or(lines.len()),
            key,
            &structure.statement_lines,
            "inside [agents]",
        )?;
        lines[assignment] = replacement_line(&lines[assignment], key, managed_value(managed, key))?;
    }
    if !missing_agents.is_empty() {
        let table_start = agent_headers[0];
        let table_end = structure
            .table_headers
            .iter()
            .copied()
            .find(|index| *index > table_start)
            .unwrap_or(lines.len());
        let insertion = before_trailing_blank_lines(lines, table_start + 1, table_end);
        prepare_insertion(lines, insertion);
        let assignments = missing_agents
            .into_iter()
            .map(|key| new_assignment(key, managed_value(managed, key)))
            .collect::<Result<Vec<_>, _>>()?;
        lines.splice(insertion..insertion, assignments);
    }
    Ok(())
}

fn append_agents_table(
    lines: &mut Vec<String>,
    managed: &BTreeMap<&str, ManagedScalar>,
) -> Result<(), InstallerError> {
    let insertion = lines.len();
    prepare_insertion(lines, insertion);
    if lines
        .last()
        .is_some_and(|line| !line_body(line).trim().is_empty())
    {
        lines.push("\n".to_owned());
    }
    lines.push("[agents]\n".to_owned());
    for key in MANAGED_AGENT_KEYS {
        lines.push(new_assignment(key, managed_value(managed, key))?);
    }
    Ok(())
}

fn merge_existing_table_setting(
    lines: &mut Vec<String>,
    existing_table: &Table,
    table_path: &str,
    key: &str,
    value: &ManagedScalar,
) -> Result<(), InstallerError> {
    let structure = scan_toml_structure(lines);
    let headers = structure
        .table_headers
        .iter()
        .copied()
        .filter(|index| is_exact_table_header(line_body(&lines[*index]), table_path))
        .collect::<Vec<_>>();
    if headers.len() != 1 {
        return Err(invalid_config(format!(
            "existing {table_path} value is not one exact [{table_path}] table"
        )));
    }

    let table_start = headers[0];
    let table_end = structure
        .table_headers
        .iter()
        .copied()
        .find(|index| *index > table_start)
        .unwrap_or(lines.len());
    if existing_table.contains_key(key) {
        let assignment = unique_assignment(
            lines,
            table_start + 1,
            table_end,
            key,
            &structure.statement_lines,
            &format!("inside [{table_path}]"),
        )?;
        lines[assignment] = replacement_line(&lines[assignment], key, value)?;
    } else {
        let insertion = before_trailing_blank_lines(lines, table_start + 1, table_end);
        prepare_insertion(lines, insertion);
        lines.insert(insertion, new_assignment(key, value)?);
    }
    Ok(())
}

fn append_table_setting(
    lines: &mut Vec<String>,
    table_path: &str,
    key: &str,
    value: &ManagedScalar,
) -> Result<(), InstallerError> {
    let insertion = lines.len();
    prepare_insertion(lines, insertion);
    if lines
        .last()
        .is_some_and(|line| !line_body(line).trim().is_empty())
    {
        lines.push("\n".to_owned());
    }
    lines.push(format!("[{table_path}]\n"));
    lines.push(new_assignment(key, value)?);
    Ok(())
}

fn parse_toml(text: &str, description: &str) -> Result<Table, InstallerError> {
    toml::from_str::<Table>(text)
        .map_err(|error| invalid_config(format!("invalid {description}: {error}")))
}

fn validated_managed_values(
    managed_text: &str,
    max_threads: u8,
) -> Result<BTreeMap<&'static str, ManagedScalar>, InstallerError> {
    let parsed = parse_toml(managed_text, "managed configuration")?;
    if parsed.len() != MANAGED_ROOT_KEYS.len() + 3
        || !MANAGED_ROOT_KEYS
            .iter()
            .all(|key| parsed.contains_key(*key))
        || !parsed.contains_key("agents")
        || !parsed.contains_key("tools")
        || !parsed.contains_key("features")
    {
        return Err(invalid_config(
            "managed configuration has unknown or missing root keys",
        ));
    }

    let agents = parsed
        .get("agents")
        .and_then(Value::as_table)
        .ok_or_else(|| {
            invalid_config("managed configuration has unknown or missing agents keys")
        })?;
    if agents.len() != MANAGED_AGENT_KEYS.len()
        || !MANAGED_AGENT_KEYS
            .iter()
            .all(|key| agents.contains_key(*key))
    {
        return Err(invalid_config(
            "managed configuration has unknown or missing agents keys",
        ));
    }

    let update_plan = parsed
        .get("tools")
        .and_then(Value::as_table)
        .filter(|tools| tools.len() == 1)
        .and_then(|tools| tools.get("update_plan"))
        .and_then(Value::as_table)
        .filter(|update_plan| {
            update_plan.len() == MANAGED_UPDATE_PLAN_KEYS.len()
                && MANAGED_UPDATE_PLAN_KEYS
                    .iter()
                    .all(|key| update_plan.contains_key(*key))
        })
        .ok_or_else(|| {
            invalid_config("managed configuration has unknown or missing tools.update_plan keys")
        })?;

    let context_management = parsed
        .get("features")
        .and_then(Value::as_table)
        .filter(|features| features.len() == 1)
        .and_then(|features| features.get("context_management"))
        .and_then(Value::as_table)
        .filter(|context| context.len() == 1 && context.contains_key("experimental_mode"))
        .ok_or_else(|| {
            invalid_config(
                "managed configuration has unknown or missing features.context_management keys",
            )
        })?;

    let mut values = BTreeMap::new();
    for key in MANAGED_ROOT_KEYS {
        values.insert(key, ManagedScalar::from_value(key, &parsed[key])?);
    }
    for key in MANAGED_AGENT_KEYS {
        values.insert(key, ManagedScalar::from_value(key, &agents[key])?);
    }
    let enabled = ManagedScalar::from_value(UPDATE_PLAN_ENABLED_KEY, &update_plan["enabled"])?;
    if !matches!(enabled, ManagedScalar::Boolean(_)) {
        return Err(invalid_config(
            "managed key \"tools.update_plan.enabled\" is not a boolean",
        ));
    }
    values.insert(UPDATE_PLAN_ENABLED_KEY, enabled);
    let experimental_mode = ManagedScalar::from_value(
        CONTEXT_MANAGEMENT_EXPERIMENTAL_MODE_KEY,
        &context_management["experimental_mode"],
    )?;
    if !matches!(experimental_mode, ManagedScalar::Boolean(_)) {
        return Err(invalid_config(
            "managed key \"features.context_management.experimental_mode\" is not a boolean",
        ));
    }
    values.insert(CONTEXT_MANAGEMENT_EXPERIMENTAL_MODE_KEY, experimental_mode);
    values.insert(
        "max_threads",
        ManagedScalar::Integer(i64::from(max_threads)),
    );
    Ok(values)
}

fn validate_managed_postcondition(
    final_config: &Table,
    managed: &BTreeMap<&str, ManagedScalar>,
) -> Result<(), InstallerError> {
    for key in MANAGED_ROOT_KEYS {
        if final_config.get(key) != Some(&managed_value(managed, key).as_toml()) {
            return Err(invalid_config(format!(
                "merged configuration does not contain managed key {key:?}"
            )));
        }
    }

    let final_agents = final_config
        .get("agents")
        .and_then(Value::as_table)
        .ok_or_else(|| invalid_config("merged configuration does not contain an agents table"))?;
    for key in MANAGED_AGENT_KEYS {
        if final_agents.get(key) != Some(&managed_value(managed, key).as_toml()) {
            return Err(invalid_config(format!(
                "merged configuration does not contain managed agents key {key:?}"
            )));
        }
    }

    let final_update_plan = final_config
        .get("tools")
        .and_then(Value::as_table)
        .and_then(|tools| tools.get("update_plan"))
        .and_then(Value::as_table)
        .ok_or_else(|| {
            invalid_config("merged configuration does not contain a tools.update_plan table")
        })?;
    if final_update_plan.get("enabled")
        != Some(&managed_value(managed, UPDATE_PLAN_ENABLED_KEY).as_toml())
    {
        return Err(invalid_config(
            "merged configuration does not contain managed tools.update_plan key \"enabled\"",
        ));
    }
    let final_experimental_mode = final_config
        .get("features")
        .and_then(Value::as_table)
        .and_then(|features| features.get("context_management"))
        .and_then(Value::as_table)
        .and_then(|context| context.get("experimental_mode"));
    if final_experimental_mode
        != Some(&managed_value(managed, CONTEXT_MANAGEMENT_EXPERIMENTAL_MODE_KEY).as_toml())
    {
        return Err(invalid_config(
            "merged configuration does not contain managed features.context_management key \"experimental_mode\"",
        ));
    }
    Ok(())
}

fn managed_value<'a>(managed: &'a BTreeMap<&str, ManagedScalar>, key: &str) -> &'a ManagedScalar {
    managed
        .get(key)
        .expect("validated managed values contain every managed key")
}

fn split_lines_keep_ends(text: &str) -> Vec<String> {
    text.split_inclusive('\n').map(ToOwned::to_owned).collect()
}

fn line_body(line: &str) -> &str {
    line.strip_suffix("\r\n")
        .or_else(|| line.strip_suffix('\n'))
        .unwrap_or(line)
}

fn line_ending(line: &str) -> &str {
    if line.ends_with("\r\n") {
        "\r\n"
    } else if line.ends_with('\n') {
        "\n"
    } else {
        ""
    }
}

fn unique_assignment(
    lines: &[String],
    start: usize,
    end: usize,
    key: &str,
    statement_lines: &HashSet<usize>,
    location: &str,
) -> Result<usize, InstallerError> {
    let matches = (start..end)
        .filter(|index| {
            statement_lines.contains(index) && is_ordinary_assignment(&lines[*index], key)
        })
        .collect::<Vec<_>>();
    if matches.len() != 1 {
        return Err(invalid_config(format!(
            "managed key {key:?} is not one ordinary single-line assignment {location}"
        )));
    }
    Ok(matches[0])
}

fn is_ordinary_assignment(line: &str, key: &str) -> bool {
    let body = line_body(line);
    let trimmed = body.trim_start_matches([' ', '\t']);
    let Some(after_key) = trimmed.strip_prefix(key) else {
        return false;
    };
    if !after_key.trim_start_matches([' ', '\t']).starts_with('=') {
        return false;
    }
    parse_toml(body, "assignment").is_ok_and(|parsed| parsed.len() == 1 && parsed.contains_key(key))
}

fn replacement_line(
    line: &str,
    key: &str,
    value: &ManagedScalar,
) -> Result<String, InstallerError> {
    let body = line_body(line);
    let comment = inline_comment(body, key);
    let ending = line_ending(line);
    let ending = if ending.is_empty() { "\n" } else { ending };
    Ok(format!("{key} = {}{comment}{ending}", value.render()?))
}

fn new_assignment(key: &str, value: &ManagedScalar) -> Result<String, InstallerError> {
    Ok(format!("{key} = {}\n", value.render()?))
}

fn inline_comment<'a>(line: &'a str, key: &str) -> &'a str {
    for (index, character) in line.char_indices() {
        if character != '#' {
            continue;
        }
        let prefix = line[..index].trim_end_matches([' ', '\t']);
        if parse_toml(prefix, "assignment")
            .is_ok_and(|parsed| parsed.len() == 1 && parsed.contains_key(key))
        {
            return &line[prefix.len()..];
        }
    }
    ""
}

fn before_trailing_blank_lines(lines: &[String], start: usize, boundary: usize) -> usize {
    let mut insertion = boundary;
    while insertion > start && line_body(&lines[insertion - 1]).trim().is_empty() {
        insertion -= 1;
    }
    insertion
}

fn prepare_insertion(lines: &mut [String], insertion: usize) {
    if insertion > 0 && line_ending(&lines[insertion - 1]).is_empty() {
        lines[insertion - 1].push('\n');
    }
}

fn scan_toml_structure(lines: &[String]) -> Structure {
    let mut statement_lines = HashSet::new();
    let mut table_headers = Vec::new();
    let mut state = LexicalState::default();

    for (index, line) in lines.iter().enumerate() {
        let body = line_body(line);
        if state.at_statement_start() {
            statement_lines.insert(index);
            if table_header_contents(body).is_some() {
                table_headers.push(index);
                continue;
            }
        }
        advance_lexical_state(body, &mut state);
    }

    Structure {
        statement_lines,
        table_headers,
    }
}

fn is_exact_agents_header(line: &str) -> bool {
    table_header_contents(line).is_some_and(|(is_array, contents)| {
        !is_array && contents.trim_matches([' ', '\t']) == "agents"
    })
}

fn is_exact_table_header(line: &str, table_path: &str) -> bool {
    table_header_contents(line).is_some_and(|(is_array, contents)| {
        !is_array && contents.trim_matches([' ', '\t']) == table_path
    })
}

fn table_header_contents(line: &str) -> Option<(bool, &str)> {
    let trimmed = line.trim_start_matches([' ', '\t']);
    let is_array = trimmed.starts_with("[[");
    let opening = if is_array {
        2
    } else if trimmed.starts_with('[') {
        1
    } else {
        return None;
    };
    let bytes = trimmed.as_bytes();
    let mut index = opening;
    let mut quote = None;
    while index < bytes.len() {
        match quote {
            Some(b'"') if bytes[index] == b'\\' => index += 2,
            Some(delimiter) if bytes[index] == delimiter => {
                quote = None;
                index += 1;
            }
            Some(_) => index += 1,
            None if matches!(bytes[index], b'"' | b'\'') => {
                quote = Some(bytes[index]);
                index += 1;
            }
            None if is_array && bytes[index..].starts_with(b"]]") => {
                let remainder = trimmed[index + 2..].trim_start_matches([' ', '\t']);
                if !(remainder.is_empty() || remainder.starts_with('#')) {
                    return None;
                }
                if toml::from_str::<Value>(line).is_err() {
                    return None;
                }
                return Some((true, &trimmed[opening..index]));
            }
            None if !is_array && bytes[index] == b']' => {
                let remainder = trimmed[index + 1..].trim_start_matches([' ', '\t']);
                if !(remainder.is_empty() || remainder.starts_with('#')) {
                    return None;
                }
                if toml::from_str::<Value>(line).is_err() {
                    return None;
                }
                return Some((false, &trimmed[opening..index]));
            }
            None => index += 1,
        }
    }
    None
}

fn advance_lexical_state(line: &str, state: &mut LexicalState) {
    let bytes = line.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if let Some(delimiter) = state.multiline_delimiter {
            let Some(closing) = find_multiline_end(bytes, delimiter, index) else {
                return;
            };
            state.multiline_delimiter = None;
            index = closing + delimiter.len();
            continue;
        }

        if bytes[index..].starts_with(b"\"\"\"") {
            state.multiline_delimiter = Some(b"\"\"\"");
            index += 3;
            continue;
        }
        if bytes[index..].starts_with(b"'''") {
            state.multiline_delimiter = Some(b"'''");
            index += 3;
            continue;
        }
        match bytes[index] {
            b'#' => return,
            b'"' => index = skip_basic_string(bytes, index + 1),
            b'\'' => {
                index = bytes[index + 1..]
                    .iter()
                    .position(|character| *character == b'\'')
                    .map_or(bytes.len(), |closing| index + closing + 2);
            }
            b'[' => {
                state.array_depth += 1;
                index += 1;
            }
            b']' => {
                state.array_depth -= 1;
                index += 1;
            }
            b'{' => {
                state.inline_table_depth += 1;
                index += 1;
            }
            b'}' => {
                state.inline_table_depth -= 1;
                index += 1;
            }
            _ => index += 1,
        }
    }
}

fn find_multiline_end(line: &[u8], delimiter: &[u8; 3], start: usize) -> Option<usize> {
    let mut index = start;
    while index + delimiter.len() <= line.len() {
        if line[index..].starts_with(delimiter) && (delimiter == b"'''" || !is_escaped(line, index))
        {
            return Some(index);
        }
        index += 1;
    }
    None
}

fn skip_basic_string(line: &[u8], start: usize) -> usize {
    let mut index = start;
    while index < line.len() {
        match line[index] {
            b'\\' => index = (index + 2).min(line.len()),
            b'"' => return index + 1,
            _ => index += 1,
        }
    }
    line.len()
}

fn is_escaped(line: &[u8], index: usize) -> bool {
    let backslashes = line[..index]
        .iter()
        .rev()
        .take_while(|character| **character == b'\\')
        .count();
    backslashes % 2 == 1
}

fn invalid_config(message: impl Into<String>) -> InstallerError {
    InstallerError::InvalidConfiguration {
        message: message.into(),
    }
}

#[cfg(test)]
#[path = "config_merge_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "config_merge_context_management_tests.rs"]
mod context_management_tests;
