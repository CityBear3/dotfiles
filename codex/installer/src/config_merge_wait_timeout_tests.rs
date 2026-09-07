use super::merge_config;
use crate::InstallerError::InvalidConfiguration;

const BASE_CONFIG: &str = concat!(
    "model = \"fixture-model\"\n",
    "model_reasoning_effort = \"high\"\n",
    "plan_mode_reasoning_effort = \"high\"\n",
    "\n[agents]\nmax_threads = 6\nmax_depth = 2\n",
    "\n[tools.update_plan]\nenabled = true\n",
    "\n[features.context_management]\nexperimental_mode = true\n",
);
const WAIT_CONFIG: &str = concat!(
    "\n[features.multi_agent_v2]\n",
    "enabled = true\n",
    "min_wait_timeout_ms = 60000\n",
    "default_wait_timeout_ms = 120000\n",
    "max_wait_timeout_ms = 3600000\n",
);

#[test]
fn adds_wait_timeouts_beside_unmanaged_features() {
    // Arrange
    let existing = format!("{BASE_CONFIG}\n[features.other]\nenabled  = false # keep\n");
    let managed = format!("{BASE_CONFIG}{WAIT_CONFIG}");
    let expected = format!("{existing}{WAIT_CONFIG}");

    // Act
    let result = merge_config(&existing, &managed, 6);

    // Assert
    assert_eq!(result, Ok(expected));
}

#[test]
fn replaces_wait_timeouts_without_changing_unmanaged_bytes() {
    // Arrange
    let existing = format!(
        "{BASE_CONFIG}\r\n[features.multi_agent_v2]\r\n\
         notes = '''\r\n[features.multi_agent_v2]\r\nmin_wait_timeout_ms = 1\r\n'''\r\n\
         enabled   = false # keep enabled comment\r\n\
         min_wait_timeout_ms = 10000 # keep minimum\r\n\
         default_wait_timeout_ms = 30000\r\n\
         max_wait_timeout_ms = 600000\r\n\
         other_setting  = \"unchanged\"\r\n\r\n\
         [features.multi_agent_v2.extra]\r\nmin_wait_timeout_ms = 7\r\n"
    );
    let managed = format!("{BASE_CONFIG}{WAIT_CONFIG}");
    let expected = existing
        .replace("enabled   = false #", "enabled = true #")
        .replace(
            "min_wait_timeout_ms = 10000 #",
            "min_wait_timeout_ms = 60000 #",
        )
        .replace(
            "default_wait_timeout_ms = 30000\r",
            "default_wait_timeout_ms = 120000\r",
        )
        .replace(
            "max_wait_timeout_ms = 600000\r",
            "max_wait_timeout_ms = 3600000\r",
        );
    let expected = format!("{}\n", expected.trim_end_matches(['\r', '\n']));

    // Act
    let result = merge_config(&existing, &managed, 6);

    // Assert
    assert_eq!(result, Ok(expected));
}

#[test]
fn inserts_missing_timeouts_into_an_existing_multi_agent_table() {
    // Arrange
    let existing = format!(
        "{BASE_CONFIG}\n[features.multi_agent_v2]\nenabled = false\n\
         other_setting = \"untouched\"\n# keep comment\n\n[tui]\nanimations = false\n"
    );
    let managed = format!("{BASE_CONFIG}{WAIT_CONFIG}");
    let expected = existing
        .replace("enabled = false", "enabled = true")
        .replace(
            "# keep comment\n",
            "# keep comment\nmin_wait_timeout_ms = 60000\ndefault_wait_timeout_ms = 120000\nmax_wait_timeout_ms = 3600000\n",
        );

    // Act
    let result = merge_config(&existing, &managed, 6);

    // Assert
    assert_eq!(result, Ok(expected));
}

#[test]
fn repeated_wait_timeout_merge_is_byte_identical() {
    // Arrange
    let managed = format!("{BASE_CONFIG}{WAIT_CONFIG}");
    let existing = format!("{BASE_CONFIG}\n[tui]\nanimations = false\n");

    // Act
    let first = merge_config(&existing, &managed, 6).expect("first merge succeeds");
    let second = merge_config(&first, &managed, 6);

    // Assert
    assert_eq!(second, Ok(first));
}

#[test]
fn wait_timeout_values_come_from_the_managed_fragment() {
    // Arrange
    let existing = format!("{BASE_CONFIG}{WAIT_CONFIG}");
    let managed = format!(
        "{BASE_CONFIG}\n[features.multi_agent_v2]\nenabled = false\n\
         min_wait_timeout_ms = 1000\ndefault_wait_timeout_ms = 1000\nmax_wait_timeout_ms = 1000\n"
    );

    // Act
    let result = merge_config(&existing, &managed, 6);

    // Assert
    assert_eq!(result, Ok(managed));
}

#[test]
fn managed_wait_settings_require_exactly_the_declared_keys() {
    // Arrange
    let incomplete = WAIT_CONFIG.replace("min_wait_timeout_ms = 60000\n", "");
    let unknown = format!("{WAIT_CONFIG}other_setting = true\n");
    for fragment in ["", "\n[features.multi_agent_v2]\n", &incomplete, &unknown] {
        let managed = format!("{BASE_CONFIG}{fragment}");

        // Act
        let result = merge_config("", &managed, 6);

        // Assert
        assert_eq!(
            result,
            Err(InvalidConfiguration {
                message:
                    "managed configuration has unknown or missing features.multi_agent_v2 keys"
                        .to_owned(),
            }),
            "fragment: {fragment}"
        );
    }
}

#[test]
fn managed_multi_agent_enabled_must_be_boolean() {
    // Arrange
    let wait_config = WAIT_CONFIG.replace("enabled = true", "enabled = \"true\"");
    let managed = format!("{BASE_CONFIG}{wait_config}");

    // Act
    let result = merge_config("", &managed, 6);

    // Assert
    assert_eq!(
        result,
        Err(InvalidConfiguration {
            message: "managed key \"features.multi_agent_v2.enabled\" is not a boolean".to_owned(),
        })
    );
}

#[test]
fn managed_wait_timeouts_must_be_positive_integers() {
    // Arrange
    for (key, original) in [
        ("min_wait_timeout_ms", "60000"),
        ("default_wait_timeout_ms", "120000"),
        ("max_wait_timeout_ms", "3600000"),
    ] {
        for invalid in ["0", "-1", "1.5", "true", "\"60000\""] {
            let wait_config = WAIT_CONFIG.replace(
                &format!("{key} = {original}\n"),
                &format!("{key} = {invalid}\n"),
            );
            let managed = format!("{BASE_CONFIG}{wait_config}");

            // Act
            let result = merge_config("", &managed, 6);

            // Assert
            assert_eq!(
                result,
                Err(InvalidConfiguration {
                    message: format!(
                        "managed key \"features.multi_agent_v2.{key}\" is not a positive integer"
                    ),
                }),
                "{key} = {invalid}"
            );
        }
    }
}

#[test]
fn managed_wait_timeout_default_must_be_within_bounds() {
    // Arrange
    for (before, after) in [
        (
            "min_wait_timeout_ms = 60000",
            "min_wait_timeout_ms = 180000",
        ),
        (
            "max_wait_timeout_ms = 3600000",
            "max_wait_timeout_ms = 60000",
        ),
    ] {
        let wait_config = WAIT_CONFIG.replace(before, after);
        let managed = format!("{BASE_CONFIG}{wait_config}");

        // Act
        let result = merge_config("", &managed, 6);

        // Assert
        assert_eq!(
            result,
            Err(InvalidConfiguration {
                message: "managed wait timeouts must satisfy min <= default <= max".to_owned(),
            }),
            "{after}"
        );
    }
}

#[test]
fn unsupported_existing_multi_agent_representations_are_rejected() {
    // Arrange
    let managed = format!("{BASE_CONFIG}{WAIT_CONFIG}");
    for (fragment, message) in [
        (
            "[features]\nmulti_agent_v2 = true\n",
            "existing configuration does not contain an ordinary features.multi_agent_v2 table",
        ),
        (
            "[features]\nmulti_agent_v2 = { enabled = true }\n",
            "existing features.multi_agent_v2 value is not one exact [features.multi_agent_v2] table",
        ),
        (
            "[features]\nmulti_agent_v2.enabled = true\n",
            "existing features.multi_agent_v2 value is not one exact [features.multi_agent_v2] table",
        ),
        (
            "[features.multi_agent_v2]\n\"enabled\" = true\n",
            "managed key \"enabled\" is not one ordinary single-line assignment inside [features.multi_agent_v2]",
        ),
    ] {
        let existing = format!("{BASE_CONFIG}\n{fragment}");

        // Act
        let result = merge_config(&existing, &managed, 6);

        // Assert
        assert_eq!(
            result,
            Err(InvalidConfiguration {
                message: message.to_owned(),
            }),
            "fragment: {fragment}"
        );
    }
}
