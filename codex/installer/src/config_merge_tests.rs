use super::merge_config;

#[test]
fn preserves_unmanaged_configuration_bytes() {
    // Arrange
    let existing = concat!(
        "# workstation-specific configuration\n",
        "model   =   \"gpt-old\"   # keep model context\n",
        "model_reasoning_effort = \"medium\"\n",
        "plan_mode_reasoning_effort = \"high\"\n",
        "model_context_window    = 262144\n",
        "model_auto_compact_token_limit = 200000\n",
        "model_auto_compact_token_limit_scope = \"remaining\"\n",
        "approval_policy    =    \"on-request\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 2 # selected per machine\n",
        "max_depth = 3\n",
        "custom_setting  =  \"untouched\"\n",
        "\n",
        "[tui]\n",
        "status_line = [\"model\", \"context\"]\n",
        "# final unmanaged comment\n",
        "\n",
    );
    let managed = concat!(
        "model = \"gpt-5.6\"\n",
        "model_reasoning_effort = \"xhigh\"\n",
        "plan_mode_reasoning_effort = \"xhigh\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 6\n",
        "max_depth = 1\n",
    );
    let expected = concat!(
        "# workstation-specific configuration\n",
        "model = \"gpt-5.6\"   # keep model context\n",
        "model_reasoning_effort = \"xhigh\"\n",
        "plan_mode_reasoning_effort = \"xhigh\"\n",
        "model_context_window    = 262144\n",
        "model_auto_compact_token_limit = 200000\n",
        "model_auto_compact_token_limit_scope = \"remaining\"\n",
        "approval_policy    =    \"on-request\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 8 # selected per machine\n",
        "max_depth = 1\n",
        "custom_setting  =  \"untouched\"\n",
        "\n",
        "[tui]\n",
        "status_line = [\"model\", \"context\"]\n",
        "# final unmanaged comment\n",
    );

    // Act
    let result = merge_config(existing, managed, 8);

    // Assert
    assert_eq!(result, Ok(expected.to_owned()));
}

#[test]
fn missing_managed_assignments_are_inserted() {
    // Arrange
    let existing = concat!(
        "# local root configuration\n",
        "approval_policy = \"on-request\"\n",
        "\n",
        "[agents]\n",
        "custom_setting = true\n",
        "\n",
        "[tui]\n",
        "status_line = [\"model\"]\n",
    );
    let managed = concat!(
        "model = \"gpt-5.6\"\n",
        "model_reasoning_effort = \"xhigh\"\n",
        "plan_mode_reasoning_effort = \"xhigh\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 6\n",
        "max_depth = 1\n",
    );
    let expected = concat!(
        "# local root configuration\n",
        "approval_policy = \"on-request\"\n",
        "model = \"gpt-5.6\"\n",
        "model_reasoning_effort = \"xhigh\"\n",
        "plan_mode_reasoning_effort = \"xhigh\"\n",
        "\n",
        "[agents]\n",
        "custom_setting = true\n",
        "max_threads = 8\n",
        "max_depth = 1\n",
        "\n",
        "[tui]\n",
        "status_line = [\"model\"]\n",
    );

    // Act
    let result = merge_config(existing, managed, 8);

    // Assert
    assert_eq!(result, Ok(expected.to_owned()));
}

#[test]
fn missing_agents_table_is_appended() {
    // Arrange
    let existing = concat!(
        "model = \"old\"\n",
        "model_reasoning_effort = \"medium\"\n",
        "plan_mode_reasoning_effort = \"high\"\n",
        "\n",
        "[unmanaged]\n",
        "keep = true\n",
        "\n",
    );
    let managed = concat!(
        "model = \"gpt-5.6\"\n",
        "model_reasoning_effort = \"xhigh\"\n",
        "plan_mode_reasoning_effort = \"xhigh\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 6\n",
        "max_depth = 1\n",
    );
    let expected = concat!(
        "model = \"gpt-5.6\"\n",
        "model_reasoning_effort = \"xhigh\"\n",
        "plan_mode_reasoning_effort = \"xhigh\"\n",
        "\n",
        "[unmanaged]\n",
        "keep = true\n",
        "\n",
        "[agents]\n",
        "max_threads = 4\n",
        "max_depth = 1\n",
    );

    // Act
    let result = merge_config(existing, managed, 4);

    // Assert
    assert_eq!(result, Ok(expected.to_owned()));
}

#[test]
fn empty_configuration_receives_only_managed_values() {
    // Arrange
    let existing = "";
    let managed = concat!(
        "model = \"gpt-5.6\"\n",
        "model_reasoning_effort = \"xhigh\"\n",
        "plan_mode_reasoning_effort = \"xhigh\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 6\n",
        "max_depth = 1\n",
    );
    let expected = concat!(
        "model = \"gpt-5.6\"\n",
        "model_reasoning_effort = \"xhigh\"\n",
        "plan_mode_reasoning_effort = \"xhigh\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 8\n",
        "max_depth = 1\n",
    );

    // Act
    let result = merge_config(existing, managed, 8);

    // Assert
    assert_eq!(result, Ok(expected.to_owned()));
}

#[test]
fn multiline_and_quoted_toml_boundaries_are_respected() {
    // Arrange
    let existing = concat!(
        "description = \"\"\"\n",
        "[agents]\n",
        "max_threads = 31\n",
        "model = \"decoy\"\n",
        "\"\"\"\n",
        "model = \"old#tag\" # real model assignment\n",
        "model_reasoning_effort = \"medium\"\n",
        "plan_mode_reasoning_effort = \"high\"\n",
        "\n",
        "[agents]\n",
        "note = '''\n",
        "[unmanaged]\n",
        "max_depth = 99\n",
        "'''\n",
        "max_threads = 2\n",
        "max_depth = 3\n",
        "label = \"keep # inside string\"\n",
    );
    let managed = concat!(
        "model = \"gpt-5.6\"\n",
        "model_reasoning_effort = \"xhigh\"\n",
        "plan_mode_reasoning_effort = \"xhigh\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 6\n",
        "max_depth = 1\n",
    );
    let expected = concat!(
        "description = \"\"\"\n",
        "[agents]\n",
        "max_threads = 31\n",
        "model = \"decoy\"\n",
        "\"\"\"\n",
        "model = \"gpt-5.6\" # real model assignment\n",
        "model_reasoning_effort = \"xhigh\"\n",
        "plan_mode_reasoning_effort = \"xhigh\"\n",
        "\n",
        "[agents]\n",
        "note = '''\n",
        "[unmanaged]\n",
        "max_depth = 99\n",
        "'''\n",
        "max_threads = 6\n",
        "max_depth = 1\n",
        "label = \"keep # inside string\"\n",
    );

    // Act
    let result = merge_config(existing, managed, 6);

    // Assert
    assert_eq!(result, Ok(expected.to_owned()));
}

#[test]
fn duplicate_managed_assignment_is_rejected() {
    // Arrange
    let existing = concat!(
        "model = \"old\"\n",
        "model = \"duplicate\"\n",
        "model_reasoning_effort = \"medium\"\n",
        "plan_mode_reasoning_effort = \"high\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 2\n",
        "max_depth = 3\n",
    );
    let managed = concat!(
        "model = \"gpt-5.6\"\n",
        "model_reasoning_effort = \"xhigh\"\n",
        "plan_mode_reasoning_effort = \"xhigh\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 6\n",
        "max_depth = 1\n",
    );

    // Act
    let result = merge_config(existing, managed, 6);

    // Assert
    let error = result.expect_err("duplicate managed assignments must be rejected");
    assert!(
        error.to_string().contains("invalid existing configuration"),
        "unexpected merge error: {error}"
    );
}

#[test]
fn quoted_managed_key_is_rejected_as_structurally_unsupported() {
    // Arrange
    let existing = concat!(
        "\"model\" = \"old\"\n",
        "model_reasoning_effort = \"medium\"\n",
        "plan_mode_reasoning_effort = \"high\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 2\n",
        "max_depth = 3\n",
    );
    let managed = concat!(
        "model = \"gpt-5.6\"\n",
        "model_reasoning_effort = \"xhigh\"\n",
        "plan_mode_reasoning_effort = \"xhigh\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 6\n",
        "max_depth = 1\n",
    );

    // Act
    let result = merge_config(existing, managed, 6);

    // Assert
    assert_eq!(
        result,
        Err(crate::InstallerError::InvalidConfiguration {
            message: String::from(
                "managed key \"model\" is not one ordinary single-line assignment at the document root"
            ),
        })
    );
}

#[test]
fn inline_agents_table_is_rejected_as_structurally_unsupported() {
    // Arrange
    let existing = concat!(
        "model = \"old\"\n",
        "model_reasoning_effort = \"medium\"\n",
        "plan_mode_reasoning_effort = \"high\"\n",
        "agents = { max_threads = 2, max_depth = 3 }\n",
    );
    let managed = concat!(
        "model = \"gpt-5.6\"\n",
        "model_reasoning_effort = \"xhigh\"\n",
        "plan_mode_reasoning_effort = \"xhigh\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 6\n",
        "max_depth = 1\n",
    );

    // Act
    let result = merge_config(existing, managed, 6);

    // Assert
    assert_eq!(
        result,
        Err(crate::InstallerError::InvalidConfiguration {
            message: String::from("existing agents value is not one exact [agents] table"),
        })
    );
}

#[test]
fn incomplete_managed_fragment_is_rejected() {
    // Arrange
    let existing = "";
    let managed = concat!(
        "model = \"gpt-5.6\"\n",
        "model_reasoning_effort = \"xhigh\"\n",
        "plan_mode_reasoning_effort = \"xhigh\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 6\n",
    );

    // Act
    let result = merge_config(existing, managed, 6);

    // Assert
    assert_eq!(
        result,
        Err(crate::InstallerError::InvalidConfiguration {
            message: String::from("managed configuration has unknown or missing agents keys"),
        })
    );
}
