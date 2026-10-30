use super::merge_config;
use crate::InstallerError::InvalidConfiguration;

const BASE_CONFIG: &str = concat!(
    "model = \"fixture-model\"\n",
    "model_reasoning_effort = \"high\"\n",
    "plan_mode_reasoning_effort = \"high\"\n",
    "\n[agents]\nmax_threads = 6\nmax_depth = 2\n",
    "\n[tools.update_plan]\nenabled = true\n",
    "\n[features.context_management]\nexperimental_mode = true\n",
    "\n[features.multi_agent_v2]\nenabled = true\n",
    "min_wait_timeout_ms = 60000\n",
    "default_wait_timeout_ms = 120000\n",
    "max_wait_timeout_ms = 3600000\n",
);
const POLICY: &str = concat!(
    "\n[shell_environment_policy]\n",
    "inherit = \"all\"\n",
    "include_only = [\"PATH\",\"HOME\",\"HERDR_*\",\"GHOSTTY_*\",",
    "\"CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS\"]\n",
);

#[test]
fn installs_environment_policy_and_preserves_unmanaged_overrides() {
    // Arrange
    let existing = format!(
        "{BASE_CONFIG}\n[shell_environment_policy]\n\
         inherit = \"core\" # keep inheritance context\n\
         ignore_default_excludes  =  false # keep local filter\n\n\
         [shell_environment_policy.set]\n\
         CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS  =  \"1\" # keep override\n"
    );
    let managed = format!("{BASE_CONFIG}{POLICY}");
    let allowlist = POLICY
        .lines()
        .find(|line| line.starts_with("include_only"))
        .unwrap();
    let expected = existing
        .replace("inherit = \"core\"", "inherit = \"all\"")
        .replace(
            "# keep local filter\n\n",
            &format!("# keep local filter\n{allowlist}\n\n"),
        );

    // Act
    let result = merge_config(&existing, &managed, 6);

    // Assert
    assert_eq!(result, Ok(expected));
}

#[test]
fn replaces_the_complete_multiline_allowlist_without_touching_siblings() {
    // Arrange
    let existing = format!(
        "{BASE_CONFIG}\n[shell_environment_policy]\n\
         inherit = \"all\"\n\
         include_only = [\n\
           # owned old entries\n\
           \"OLD\",\n\
           \"[shell_environment_policy.set]\",\n\
         ] # keep allowlist context\n\
         exclude  =  [\"LOCAL_*\"] # keep sibling\n\n\
         [shell_environment_policy.set]\n\
         CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS = \"1\"\n"
    );
    let managed = format!("{BASE_CONFIG}{POLICY}");
    let old_assignment = concat!(
        "include_only = [\n",
        "# owned old entries\n",
        "\"OLD\",\n",
        "\"[shell_environment_policy.set]\",\n",
        "]",
    );
    let new_assignment = POLICY
        .lines()
        .find(|line| line.starts_with("include_only"))
        .unwrap();
    let expected = existing.replace(old_assignment, new_assignment);

    // Act
    let result = merge_config(&existing, &managed, 6);

    // Assert
    assert_ne!(
        expected, existing,
        "fixture replacement must cover the entire old array"
    );
    assert_eq!(result, Ok(expected));
}

#[test]
fn retains_matching_multiline_allowlist_comments_on_reinstall() {
    // Arrange
    let existing = format!(
        "{BASE_CONFIG}\n[shell_environment_policy]\n\
         inherit = \"all\"\n\
         include_only = [\n\
           # keep matching entries and formatting\n\
           \"PATH\", \"HOME\", \"HERDR_*\", \"GHOSTTY_*\",\n\
           \"CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS\",\n\
         ] # keep matching context\n"
    );
    let managed = format!("{BASE_CONFIG}{POLICY}");

    // Act
    let result = merge_config(&existing, &managed, 6);

    // Assert
    assert_eq!(result, Ok(existing));
}

#[test]
fn rejects_existing_keyed_filters_instead_of_installing_conflicting_policy() {
    // Arrange
    let existing =
        format!("{BASE_CONFIG}\n[shell_environment_policy.filters]\n\"AWS_*\" = \"exclude\"\n");
    let managed = format!("{BASE_CONFIG}{POLICY}");

    // Act
    let result = merge_config(&existing, &managed, 6);

    // Assert
    assert!(matches!(result, Err(InvalidConfiguration { message }) if message.contains("filters")));
}

#[test]
fn rejects_empty_managed_allowlist_instead_of_inheriting_everything() {
    // Arrange
    let managed = format!(
        "{BASE_CONFIG}\n[shell_environment_policy]\ninherit = \"all\"\ninclude_only = []\n"
    );

    // Act
    let result = merge_config(BASE_CONFIG, &managed, 6);

    // Assert
    assert!(
        matches!(result, Err(InvalidConfiguration { message }) if message.contains("include_only"))
    );
}

#[test]
fn array_support_does_not_expand_existing_scalar_settings() {
    // Arrange
    let managed = format!("{BASE_CONFIG}{POLICY}")
        .replace("model = \"fixture-model\"", "model = [\"fixture-model\"]");

    // Act
    let result = merge_config(BASE_CONFIG, &managed, 6);

    // Assert
    assert!(matches!(result, Err(InvalidConfiguration { message }) if message.contains("model")));
}

#[test]
fn adds_policy_when_only_an_unmanaged_set_subtable_exists() {
    // Arrange
    let existing = format!(
        "{BASE_CONFIG}\n[shell_environment_policy.set]\n\
         CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS  =  \"1\" # keep override\n"
    );
    let managed = format!("{BASE_CONFIG}{POLICY}");
    let expected = format!("{existing}{POLICY}");

    // Act
    let result = merge_config(&existing, &managed, 6);

    // Assert
    assert_eq!(result, Ok(expected));
}
