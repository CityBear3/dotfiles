use super::merge_config;
use crate::InstallerError::InvalidConfiguration;

const BASE_CONFIG: &str = concat!(
    "model = \"fixture-model\"\n",
    "model_reasoning_effort = \"high\"\n",
    "plan_mode_reasoning_effort = \"high\"\n",
    "\n",
    "[agents]\n",
    "max_threads = 6\n",
    "max_depth = 2\n",
    "\n",
    "[tools.update_plan]\n",
    "enabled = true\n",
);
const ENABLED_CONTEXT: &str = "\n[features.context_management]\nexperimental_mode = true\n";

#[test]
fn adds_context_management_when_features_are_absent() {
    // Arrange
    let managed = format!("{BASE_CONFIG}{ENABLED_CONTEXT}");

    // Act
    let result = merge_config(BASE_CONFIG, &managed, 6);

    // Assert
    assert_eq!(result, Ok(managed));
}

#[test]
fn adds_context_management_beside_unmanaged_features() {
    // Arrange
    let existing = format!("{BASE_CONFIG}\n[features]\nhooks  =  true # keep hooks\n");
    let managed = format!("{BASE_CONFIG}{ENABLED_CONTEXT}");
    let expected = format!("{existing}{ENABLED_CONTEXT}");

    // Act
    let result = merge_config(&existing, &managed, 6);

    // Assert
    assert_eq!(result, Ok(expected));
}

#[test]
fn replaces_context_management_without_changing_unmanaged_bytes() {
    // Arrange
    let existing = format!(
        "{BASE_CONFIG}\n[features]\nhooks  =  true\n\n\
         [features.context_management]\n\
         notes = '''\n[features.context_management]\nexperimental_mode = false\n'''\n\
         experimental_mode   = false # keep context-management comment\n\n\
         [features.other]\nexperimental_mode = false\n"
    );
    let managed = format!("{BASE_CONFIG}{ENABLED_CONTEXT}");
    let expected = existing.replace(
        "experimental_mode   = false # keep context-management comment",
        "experimental_mode = true # keep context-management comment",
    );

    // Act
    let result = merge_config(&existing, &managed, 6);

    // Assert
    assert_eq!(result, Ok(expected));
}

#[test]
fn adds_experimental_mode_inside_existing_context_management_table() {
    // Arrange
    let existing = format!(
        "{BASE_CONFIG}\n[features.context_management]\n\
         other_setting = \"untouched\"\n# keep comment\n\n[tui]\nanimations = false\n"
    );
    let managed = format!("{BASE_CONFIG}{ENABLED_CONTEXT}");
    let expected = existing.replace(
        "# keep comment\n",
        "# keep comment\nexperimental_mode = true\n",
    );

    // Act
    let result = merge_config(&existing, &managed, 6);

    // Assert
    assert_eq!(result, Ok(expected));
}

#[test]
fn configured_context_management_value_is_not_hard_coded_to_true() {
    // Arrange
    let existing = format!("{BASE_CONFIG}{ENABLED_CONTEXT}");
    let managed = existing.replace("experimental_mode = true", "experimental_mode = false");

    // Act
    let result = merge_config(&existing, &managed, 6);

    // Assert
    assert_eq!(result, Ok(managed));
}

#[test]
fn context_management_merge_is_idempotent() {
    // Arrange
    let managed = format!("{BASE_CONFIG}{ENABLED_CONTEXT}");
    let existing = format!("{managed}\n[features.other]\nkeep = true\n");

    // Act
    let result = merge_config(&existing, &managed, 6);

    // Assert
    assert_eq!(result, Ok(existing));
}

#[test]
fn managed_context_management_is_required() {
    // Arrange
    let managed = BASE_CONFIG;

    // Act
    let result = merge_config("", managed, 6);

    // Assert
    assert_eq!(
        result,
        Err(InvalidConfiguration {
            message: "managed configuration has unknown or missing root keys".to_owned(),
        })
    );
}

#[test]
fn managed_experimental_mode_must_be_boolean() {
    // Arrange
    let managed =
        format!("{BASE_CONFIG}\n[features.context_management]\nexperimental_mode = \"true\"\n");

    // Act
    let result = merge_config("", &managed, 6);

    // Assert
    assert_eq!(
        result,
        Err(InvalidConfiguration {
            message:
                "managed key \"features.context_management.experimental_mode\" is not a boolean"
                    .to_owned(),
        })
    );
}

#[test]
fn managed_features_must_contain_only_the_declared_key() {
    // Arrange
    let feature_fragments = [
        "[features]\n",
        "[features.context_management]\n",
        "[features.context_management]\nexperimental_mode = true\nunknown = false\n",
        "[features]\nhooks = true\n[features.context_management]\nexperimental_mode = true\n",
    ];

    for fragment in feature_fragments {
        let managed = format!("{BASE_CONFIG}\n{fragment}");

        // Act
        let result = merge_config("", &managed, 6);

        // Assert
        assert_eq!(
            result,
            Err(InvalidConfiguration {
                message:
                    "managed configuration has unknown or missing features.context_management keys"
                        .to_owned(),
            }),
            "fragment: {fragment}"
        );
    }
}

#[test]
fn non_table_context_management_is_rejected_without_replacing_it() {
    // Arrange
    let existing = format!("{BASE_CONFIG}\n[features]\ncontext_management = false\n");
    let managed = format!("{BASE_CONFIG}{ENABLED_CONTEXT}");

    // Act
    let result = merge_config(&existing, &managed, 6);

    // Assert
    assert_eq!(
        result,
        Err(InvalidConfiguration {
            message: "existing configuration does not contain an ordinary features.context_management table"
                .to_owned(),
        })
    );
}

#[test]
fn inline_or_dotted_context_management_is_rejected_as_structurally_unsupported() {
    // Arrange
    let feature_fragments = [
        "[features]\ncontext_management = { experimental_mode = false }\n",
        "[features]\ncontext_management.experimental_mode = false\n",
    ];
    let managed = format!("{BASE_CONFIG}{ENABLED_CONTEXT}");

    for fragment in feature_fragments {
        let existing = format!("{BASE_CONFIG}\n{fragment}");

        // Act
        let result = merge_config(&existing, &managed, 6);

        // Assert
        assert_eq!(
            result,
            Err(InvalidConfiguration {
                message: "existing features.context_management value is not one exact [features.context_management] table"
                    .to_owned(),
            }),
            "fragment: {fragment}"
        );
    }
}
