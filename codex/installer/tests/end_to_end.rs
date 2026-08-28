mod support;

use std::fs;
use std::path::Path;
use std::process::Command;

use support::process_tempdir;

#[test]
fn managed_bundle_declares_depth_two_and_bounded_orchestration_profiles() {
    // Arrange
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer crate must be nested under the Codex source root");
    let config_text = fs::read_to_string(source_root.join("config.toml"))
        .expect("read managed Codex configuration");
    let config =
        toml::from_str::<toml::Table>(&config_text).expect("parse managed Codex configuration");
    let agents_config = config
        .get("agents")
        .and_then(toml::Value::as_table)
        .expect("managed configuration has agents table");
    let mut profiles = fs::read_dir(source_root.join("agents"))
        .expect("read managed agent directory")
        .filter_map(|entry| {
            let entry = entry.expect("read managed agent entry");
            if !entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "toml")
            {
                return None;
            }
            let name = entry
                .file_name()
                .into_string()
                .expect("managed agent name is UTF-8");
            let text = fs::read_to_string(entry.path()).expect("read managed agent profile");
            let profile = toml::from_str::<toml::Table>(&text)
                .unwrap_or_else(|error| panic!("parse managed agent {name}: {error}"));
            Some((name, profile))
        })
        .collect::<Vec<_>>();
    profiles.sort_by(|left, right| left.0.cmp(&right.0));

    // Act
    let task_orchestrator = profiles
        .iter()
        .find(|(name, _)| name == "task-orchestrator.toml")
        .map(|(_, profile)| profile)
        .expect("managed Task orchestrator profile");
    let instructions = task_orchestrator
        .get("developer_instructions")
        .and_then(toml::Value::as_str)
        .expect("Task orchestrator developer instructions");
    let review_integrator = profiles
        .iter()
        .find(|(name, _)| name == "review-integrator.toml")
        .map(|(_, profile)| profile)
        .expect("managed review integrator profile");
    let review_integrator_instructions = review_integrator
        .get("developer_instructions")
        .and_then(toml::Value::as_str)
        .expect("review integrator developer instructions");

    // Assert
    assert_eq!(
        (
            agents_config
                .get("max_threads")
                .and_then(toml::Value::as_integer),
            agents_config
                .get("max_depth")
                .and_then(toml::Value::as_integer),
            task_orchestrator.get("name").and_then(toml::Value::as_str),
            task_orchestrator
                .get("sandbox_mode")
                .and_then(toml::Value::as_str),
        ),
        (
            Some(6),
            Some(2),
            Some("task-orchestrator"),
            Some("read-only")
        )
    );
    for required in [
        "exactly one planned Task Contract",
        "Own only that Task's execute-task loop",
        "Do not edit or commit Task source",
        "root-granted baseline leaf",
        "spawn only the implementer, verifier, reviewer, adversarial-integrator, or review-integrator leaves",
        "never grant, expand, or infer your own lease",
        "require findings-only integration followed by receiving-code-review",
        "confirmed Design Escalation",
        "Candidate",
        "Accepted",
        "BLOCKED",
        "Escalate",
        "Each of these statuses ends the current turn",
        "Do not choose another Task, alter Review policy, release dependencies, decide Feature acceptance",
        "publish or merge work",
        "clean or remove a workspace",
    ] {
        assert!(
            instructions.contains(required),
            "Task orchestrator instructions omit {required:?}"
        );
    }
    assert_eq!(
        (
            review_integrator.get("name").and_then(toml::Value::as_str),
            review_integrator.get("model").and_then(toml::Value::as_str),
            review_integrator
                .get("model_reasoning_effort")
                .and_then(toml::Value::as_str),
            review_integrator
                .get("sandbox_mode")
                .and_then(toml::Value::as_str),
        ),
        (
            Some("review-integrator"),
            Some("gpt-5.6-sol"),
            Some("xhigh"),
            Some("read-only")
        )
    );
    for required in [
        "do not spawn descendants",
        "Do not rely on conversation memory",
        "introduced, worsened, merely exposed, or did not cause it",
        "proposed remedy is necessary, proportionate, and inside current authority",
        "Design Doc is missing, contradictory, or materially ambiguous",
        "non-blocking concern",
        "Do not classify final workflow action as Fix, Push back, or Escalate",
    ] {
        assert!(
            review_integrator_instructions.contains(required),
            "review integrator instructions omit {required:?}"
        );
    }
    for (name, profile) in profiles
        .iter()
        .filter(|(name, _)| name != "task-orchestrator.toml")
    {
        let leaf_instructions = profile
            .get("developer_instructions")
            .and_then(toml::Value::as_str)
            .unwrap_or_else(|| panic!("managed leaf {name} has developer instructions"));
        assert!(
            leaf_instructions.contains("do not spawn descendants"),
            "managed leaf {name} must prohibit descendant spawning"
        );
    }
}

#[test]
fn managed_task_loop_verifier_uses_medium_effort_in_the_bounded_sandbox() {
    // Arrange
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer crate must be nested under the Codex source root");
    let profile_text = fs::read_to_string(source_root.join("agents/implementation-verifier.toml"))
        .expect("read managed implementation verifier profile");

    // Act
    let profile =
        toml::from_str::<toml::Table>(&profile_text).expect("parse implementation verifier TOML");

    // Assert
    assert_eq!(
        (
            profile.get("name").and_then(toml::Value::as_str),
            profile.get("model").and_then(toml::Value::as_str),
            profile
                .get("model_reasoning_effort")
                .and_then(toml::Value::as_str),
            profile.get("sandbox_mode").and_then(toml::Value::as_str),
        ),
        (
            Some("implementation-verifier"),
            Some("gpt-5.6-sol"),
            Some("medium"),
            Some("workspace-write"),
        )
    );
}

#[test]
fn managed_task_loop_implementer_receives_only_writer_role_input() {
    // Arrange
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer crate must be nested under the Codex source root");
    let execute_task = fs::read_to_string(source_root.join("skills/execute-task/SKILL.md"))
        .expect("read execute-task skill");
    let profile_text = fs::read_to_string(source_root.join("agents/implementer.toml"))
        .expect("read managed implementer profile");
    let fallback = fs::read_to_string(
        source_root.join("skills/agent-teams-driven-development/implementer-prompt.md"),
    )
    .expect("read implementer fallback prompt");

    // Act
    let profile = toml::from_str::<toml::Table>(&profile_text).expect("parse implementer TOML");
    let instructions = profile
        .get("developer_instructions")
        .and_then(toml::Value::as_str)
        .expect("implementer developer instructions");
    let task_message = fallback
        .split("## Task message")
        .nth(1)
        .and_then(|section| section.split("```text").nth(1))
        .and_then(|block| block.split("```").next())
        .expect("implementer fallback has a bounded Task message block")
        .to_lowercase();
    let correction_message = fallback
        .split("## Correction message")
        .nth(1)
        .and_then(|section| section.split("```text").nth(1))
        .and_then(|block| block.split("```").next())
        .expect("implementer fallback has a bounded Correction message block")
        .to_lowercase();
    let execute_task = execute_task
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let instructions = instructions
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let fallback = fallback.split_whitespace().collect::<Vec<_>>().join(" ");
    let writer_handoff_contract = (
        execute_task.contains(
            "Construct one compact writer role message from the complete Task-loop handoff",
        ),
        execute_task.contains("Pass only the selected role and writer role message"),
        instructions.contains("Require a compact writer role message containing only"),
        fallback.contains("Require a compact writer role message containing only"),
        [&instructions, &fallback].iter().all(|prompt| {
            prompt.contains(
                "Review context, Review policy, completed gate evidence, review scheduling, capacity, and queue state remain with the Task-loop owner and are not required implementer inputs",
            )
        }),
        [&task_message, &correction_message]
            .iter()
            .all(|message| {
                [
                    "review context",
                    "review policy",
                    "completed gate evidence",
                    "review scheduling",
                    "capacity",
                    "queue",
                ]
                .iter()
                .all(|owner_field| !message.contains(owner_field))
            }),
    );

    // Assert
    assert_eq!(
        writer_handoff_contract,
        (true, true, true, true, true, true)
    );
}

#[test]
fn managed_task_loop_matrix_has_one_owner_and_a_mechanical_executor() {
    // Arrange
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer crate must be nested under the Codex source root");
    let execute_task = fs::read_to_string(source_root.join("skills/execute-task/SKILL.md"))
        .expect("read execute-task skill");
    let verify =
        fs::read_to_string(source_root.join("skills/verify/SKILL.md")).expect("read verify skill");
    let orchestrator = fs::read_to_string(source_root.join("agents/task-orchestrator.toml"))
        .expect("read Task orchestrator profile");
    let verifier_profile =
        fs::read_to_string(source_root.join("agents/implementation-verifier.toml"))
            .expect("read managed implementation verifier profile");

    // Act
    let verifier_profile = toml::from_str::<toml::Table>(&verifier_profile)
        .expect("parse implementation verifier TOML");
    let verifier_instructions = verifier_profile
        .get("developer_instructions")
        .and_then(toml::Value::as_str)
        .expect("implementation verifier developer instructions")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let execute_task = execute_task
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let verify = verify.split_whitespace().collect::<Vec<_>>().join(" ");
    let orchestrator = orchestrator
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let phase_contract = (
        execute_task.contains("one in-memory current-head Verification Matrix"),
        execute_task.contains("head, range, controlling authority, or material verification route"),
        verify.contains("Execute the supplied Verification Matrix"),
        verify.contains("Do not perform semantic review"),
        orchestrator.contains("derive and invalidate the current-head Verification Matrix"),
        [
            "do not perform semantic review",
            "contract-quality judgment",
            "architecture review",
            "scope review",
            "maintainability review",
            "test-adequacy review",
        ]
        .iter()
        .all(|excluded_review| verifier_instructions.contains(excluded_review)),
    );

    // Assert
    assert_eq!(phase_contract, (true, true, true, true, true, true));
}

#[test]
fn managed_task_loop_verifier_reports_mechanical_fail_fast_evidence() {
    // Arrange
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer crate must be nested under the Codex source root");
    let profile_text = fs::read_to_string(source_root.join("agents/implementation-verifier.toml"))
        .expect("read managed implementation verifier profile");
    let verify =
        fs::read_to_string(source_root.join("skills/verify/SKILL.md")).expect("read verify skill");

    // Act
    let profile =
        toml::from_str::<toml::Table>(&profile_text).expect("parse implementation verifier TOML");
    let instructions = profile
        .get("developer_instructions")
        .and_then(toml::Value::as_str)
        .expect("implementation verifier developer instructions");
    let profile_order = instructions
        .split("Execute applicable rows mechanically and fail fast in this order:")
        .nth(1)
        .expect("implementation verifier declares its fail-fast order");
    let skill_order = verify
        .split("Run applicable matrix rows fresh in exactly this order:")
        .nth(1)
        .expect("verify skill declares its fail-fast order");
    let profile_order = profile_order
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let skill_order = skill_order.split_whitespace().collect::<Vec<_>>().join(" ");
    let profile_positions = [
        "target identity and clean-state precondition",
        "range, changed-file, and whitespace/diff checks",
        "documented non-mutating format check",
        "focused behavior tests",
        "build or type check",
        "lint",
        "owning package, workspace, or full tests",
        "integration, smoke, browser, API, or snapshot checks",
        "final head and mutation-invariant comparison",
    ]
    .map(|stage| {
        profile_order
            .find(stage)
            .unwrap_or_else(|| panic!("implementation verifier order omits {stage:?}"))
    });
    let skill_positions = [
        "target identity and required clean-state precondition",
        "exact range, changed-file inventory, `git diff --check`, and bounded diff consistency",
        "format check using only the documented non-mutating mode",
        "focused behavior tests",
        "build or type check",
        "lint",
        "owning package, workspace, or full tests",
        "integration, smoke, browser, API, or snapshot checks",
        "final head and mutation-invariant comparison",
    ]
    .map(|stage| {
        skill_order
            .find(stage)
            .unwrap_or_else(|| panic!("verify skill order omits {stage:?}"))
    });
    let fail_fast_contract = (
        profile_positions.windows(2).all(|pair| pair[0] < pair[1]),
        skill_positions.windows(2).all(|pair| pair[0] < pair[1]),
        profile_order.contains("A conclusive failure stops later dependent or more expensive rows")
            && profile_order.contains("Record every unrun row and why"),
        skill_order.contains("A conclusive failure stops later dependent or more expensive rows")
            && skill_order.contains("Record each unrun matrix row and the failure or blocked prerequisite that prevented it"),
        profile_order.contains("always run the final mutation check")
            && skill_order.contains("After the final command or an earlier conclusive stop, run the final head and mutation-invariant comparison"),
        profile_order.contains("A required mechanical mismatch is `FAIL`")
            && skill_order.contains("An observed mechanical mismatch is `FAIL`"),
        profile_order.contains("Return the completed Verification Matrix plus exactly `PASS`, `FAIL`, or `BLOCKED`")
            && skill_order.contains("Return the completed Verification Matrix and exactly one verdict"),
    );

    // Assert
    assert_eq!(
        fail_fast_contract,
        (true, true, true, true, true, true, true)
    );
}

#[test]
fn managed_task_loop_lease_expands_only_for_the_source_reviewer_wave() {
    // Arrange
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer crate must be nested under the Codex source root");
    let execute_plan = fs::read_to_string(source_root.join("skills/execute-plan/SKILL.md"))
        .expect("read execute-plan skill");
    let scheduling =
        fs::read_to_string(source_root.join("skills/agent-teams-driven-development/SKILL.md"))
            .expect("read Task leaf scheduling skill");
    let review =
        fs::read_to_string(source_root.join("skills/review/SKILL.md")).expect("read review skill");
    let orchestrator = fs::read_to_string(source_root.join("agents/task-orchestrator.toml"))
        .expect("read Task orchestrator profile");

    // Act
    let execute_plan = execute_plan
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let scheduling = scheduling.split_whitespace().collect::<Vec<_>>().join(" ");
    let review = review.split_whitespace().collect::<Vec<_>>().join(" ");
    let orchestrator = orchestrator
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let lease_contract = (
        execute_plan.contains("one root-granted baseline leaf"),
        scheduling.contains("only for a policy-selected source-reviewer wave"),
        review.contains("temporary reviewer-wave expansion"),
        review.contains("revoked before findings integration, triage, or correction"),
        orchestrator.contains("Free capacity is not authority"),
        scheduling.contains("at most three total Task leaves or the smaller current capacity"),
        review.contains(
            "Only the root may grant it, up to three total Task leaves or the smaller current capacity",
        ),
        orchestrator.contains(
            "bounded by three total Task leaves, the root grant, and effective capacity",
        ),
        scheduling.contains(
            "after fresh verification `PASS` and only when at least two independent source reviewers were selected",
        ),
        review.contains(
            "When fresh verification passes and the policy selected at least two independent source reviewers",
        ),
        orchestrator.contains(
            "Only after verifier `PASS` and selection of at least two independent source reviewers",
        ),
    );

    // Assert
    assert_eq!(
        lease_contract,
        (
            true, true, true, true, true, true, true, true, true, true, true
        )
    );
}

#[test]
fn managed_task_loop_correction_review_is_delta_first_with_a_fresh_full_verdict() {
    // Arrange
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer crate must be nested under the Codex source root");
    let execute_task = fs::read_to_string(source_root.join("skills/execute-task/SKILL.md"))
        .expect("read execute-task skill");
    let review =
        fs::read_to_string(source_root.join("skills/review/SKILL.md")).expect("read review skill");
    let triage = fs::read_to_string(source_root.join("skills/receiving-code-review/SKILL.md"))
        .expect("read receiving-code-review skill");
    let fallback_prompts = [
        "focused-reviewer-prompt.md",
        "spec-reviewer-prompt.md",
        "code-quality-reviewer-prompt.md",
    ]
    .map(|name| {
        fs::read_to_string(
            source_root
                .join("skills/agent-teams-driven-development")
                .join(name),
        )
        .unwrap_or_else(|error| panic!("read reviewer fallback {name}: {error}"))
    });

    // Act
    let execute_task = execute_task
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let review = review.split_whitespace().collect::<Vec<_>>().join(" ");
    let triage = triage.split_whitespace().collect::<Vec<_>>().join(" ");
    let fallback_prompts =
        fallback_prompts.map(|prompt| prompt.split_whitespace().collect::<Vec<_>>().join(" "));
    let rebuilt_matrix = execute_task
        .find("rebuild the Verification Matrix for `H2`")
        .expect("correction sequence rebuilds the H2 matrix");
    let fresh_verification = execute_task
        .find("invoke fresh authoritative `verify`")
        .expect("correction sequence freshly verifies H2");
    let same_set_review = execute_task
        .find("only after `PASS`, rerun the same complete policy-selected reviewer set")
        .expect("correction sequence gates same-set review on fresh PASS");
    let correction_contract = (
        execute_task.contains("`H1..H2` correction delta"),
        rebuilt_matrix < fresh_verification && fresh_verification < same_set_review,
        review.contains("delta-first"),
        review.contains("fresh verdict for the full `base..H2` target"),
        review.contains("Use ordinary full traversal when")
            && review
                .contains("A missing or stale prior report disables the delta-first optimization"),
        triage.contains("same complete policy-selected reviewer set"),
        triage.contains("lacks complete prior evidence"),
        fallback_prompts
            .iter()
            .all(|prompt| prompt.contains("Prior review evidence is navigation evidence only")),
    );

    // Assert
    assert_eq!(
        correction_contract,
        (true, true, true, true, true, true, true, true)
    );
}

#[test]
fn managed_task_loop_dispatches_new_roles_without_parent_history_and_with_complete_handoffs() {
    // Arrange
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer crate must be nested under the Codex source root");
    let execute_plan = fs::read_to_string(source_root.join("skills/execute-plan/SKILL.md"))
        .expect("read execute-plan skill");
    let task_dispatch =
        fs::read_to_string(source_root.join("skills/dispatching-parallel-agents/SKILL.md"))
            .expect("read Task orchestrator dispatch skill");
    let leaf_dispatch =
        fs::read_to_string(source_root.join("skills/agent-teams-driven-development/SKILL.md"))
            .expect("read Task leaf dispatch skill");
    let orchestrator_text =
        fs::read_to_string(source_root.join("agents/task-orchestrator.toml"))
            .expect("read Task orchestrator profile");

    // Act
    let orchestrator =
        toml::from_str::<toml::Table>(&orchestrator_text).expect("parse Task orchestrator TOML");
    let orchestrator_instructions = orchestrator
        .get("developer_instructions")
        .and_then(toml::Value::as_str)
        .expect("Task orchestrator developer instructions")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let execute_plan = execute_plan
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let task_dispatch = task_dispatch
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let leaf_dispatch = leaf_dispatch
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    // Assert
    for required in [
        "explicit `fork_turns=\"none\"`",
        "newly spawned Task orchestrator",
        "complete Task-orchestrator handoff",
        "exact authority sources directly readable",
    ] {
        assert!(
            task_dispatch.contains(required),
            "Task dispatch contract omits {required:?}"
        );
    }
    for required in [
        "explicit `fork_turns=\"none\"`",
        "newly spawned implementer, verifier, reviewer, adversarial-integrator, or review-integrator",
        "complete role-specific message unchanged",
        "directly re-resolve current Git and authority",
    ] {
        assert!(
            leaf_dispatch.contains(required),
            "leaf dispatch contract omits {required:?}"
        );
    }
    assert!(
        execute_plan.contains("new Task orchestrator with explicit `fork_turns=\"none\"`")
            && execute_plan.contains("complete Task-local authority")
            && execute_plan.contains("directly revalidate Git and authority")
    );
    assert_eq!(
        orchestrator.get("name").and_then(toml::Value::as_str),
        Some("task-orchestrator")
    );
    for required in [
        "explicit `fork_turns=\"none\"`",
        "complete role-specific handoff",
        "directly revalidate current Git and authority",
        "Parent conversation, identity, and liveness are never correctness evidence",
    ] {
        assert!(
            orchestrator_instructions.contains(required),
            "Task orchestrator profile omits {required:?}"
        );
    }
}

#[test]
fn managed_task_loop_waits_for_events_in_normal_five_to_ten_minute_bounds() {
    // Arrange
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer crate must be nested under the Codex source root");
    let task_dispatch =
        fs::read_to_string(source_root.join("skills/dispatching-parallel-agents/SKILL.md"))
            .expect("read Task orchestrator dispatch skill");
    let leaf_dispatch =
        fs::read_to_string(source_root.join("skills/agent-teams-driven-development/SKILL.md"))
            .expect("read Task leaf dispatch skill");
    let execute_plan = fs::read_to_string(source_root.join("skills/execute-plan/SKILL.md"))
        .expect("read execute-plan skill");
    let orchestrator_text =
        fs::read_to_string(source_root.join("agents/task-orchestrator.toml"))
            .expect("read Task orchestrator profile");

    // Act
    let orchestrator =
        toml::from_str::<toml::Table>(&orchestrator_text).expect("parse Task orchestrator TOML");
    let orchestrator_instructions = orchestrator
        .get("developer_instructions")
        .and_then(toml::Value::as_str)
        .expect("Task orchestrator developer instructions")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let owner_contracts = [&task_dispatch, &leaf_dispatch, &execute_plan].map(|contract| {
        contract
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    });

    // Assert
    for contract in owner_contracts.iter().chain([&orchestrator_instructions]) {
        for required in [
            "normally 300,000 to 600,000 milliseconds",
            "returns early on mailbox, completion, or steered user input",
            "deadline, teardown, or interruption boundary",
            "do not replace it with repeated short polls",
        ] {
            assert!(
                contract.contains(required),
                "event-responsive wait contract omits {required:?}"
            );
        }
    }
    assert!(
        task_dispatch.contains("A terminal Task-orchestrator result ends the turn without another wait")
            && orchestrator_instructions
                .contains("A terminal result ends the turn without another wait or poll")
    );
}

#[test]
fn managed_task_loop_implementer_batching_preserves_decision_and_tdd_boundaries() {
    // Arrange
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer crate must be nested under the Codex source root");
    let execute_task = fs::read_to_string(source_root.join("skills/execute-task/SKILL.md"))
        .expect("read execute-task skill");
    let tdd = fs::read_to_string(source_root.join("skills/test-driven-development/SKILL.md"))
        .expect("read test-driven-development skill");
    let implementer_text = fs::read_to_string(source_root.join("agents/implementer.toml"))
        .expect("read implementer profile");
    let fallback = fs::read_to_string(
        source_root.join("skills/agent-teams-driven-development/implementer-prompt.md"),
    )
    .expect("read implementer fallback prompt");

    // Act
    let implementer =
        toml::from_str::<toml::Table>(&implementer_text).expect("parse implementer TOML");
    let implementer_instructions = implementer
        .get("developer_instructions")
        .and_then(toml::Value::as_str)
        .expect("implementer developer instructions")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let contracts = [execute_task, tdd, fallback].map(|contract| {
        contract
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    });

    // Assert
    for contract in contracts.iter().chain([&implementer_instructions]) {
        for required in [
            "Independent initial authority reads, repository searches, relevant file reads, and Git inspection",
            "one bounded programmatic batch",
            "separately attributable",
            "stop before a result-dependent judgment",
            "focused RED -> production edit -> focused GREEN -> refactor while green",
            "mechanical post-edit checks only after focused GREEN",
        ] {
            assert!(
                contract.contains(required),
                "implementer execution contract omits {required:?}"
            );
        }
    }
}

#[test]
fn managed_task_loop_search_cache_has_one_writer_and_plan_matched_lifecycle() {
    // Arrange
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer crate must be nested under the Codex source root");
    let workflow = fs::read_to_string(source_root.join("skills/agentic-engineering-workflow/SKILL.md"))
        .expect("read agentic workflow skill");
    let create_plan = fs::read_to_string(source_root.join("skills/create-plan/SKILL.md"))
        .expect("read create-plan skill");
    let execute_plan = fs::read_to_string(source_root.join("skills/execute-plan/SKILL.md"))
        .expect("read execute-plan skill");
    let execute_task = fs::read_to_string(source_root.join("skills/execute-task/SKILL.md"))
        .expect("read execute-task skill");
    let verify =
        fs::read_to_string(source_root.join("skills/verify/SKILL.md")).expect("read verify skill");
    let review =
        fs::read_to_string(source_root.join("skills/review/SKILL.md")).expect("read review skill");
    let finish = fs::read_to_string(source_root.join("skills/finish-branch/SKILL.md"))
        .expect("read finish-branch skill");
    let orchestrator_text =
        fs::read_to_string(source_root.join("agents/task-orchestrator.toml"))
            .expect("read Task orchestrator profile");
    let implementer_text = fs::read_to_string(source_root.join("agents/implementer.toml"))
        .expect("read implementer profile");
    let readme = fs::read_to_string(source_root.join("README.md")).expect("read Codex README");

    // Act
    let orchestrator =
        toml::from_str::<toml::Table>(&orchestrator_text).expect("parse Task orchestrator TOML");
    let implementer =
        toml::from_str::<toml::Table>(&implementer_text).expect("parse implementer TOML");
    let orchestrator_instructions = orchestrator
        .get("developer_instructions")
        .and_then(toml::Value::as_str)
        .expect("Task orchestrator developer instructions");
    let implementer_instructions = implementer
        .get("developer_instructions")
        .and_then(toml::Value::as_str)
        .expect("implementer developer instructions")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    // Assert
    for producer in [&workflow, &create_plan] {
        assert!(producer.contains("docs/plans/YYYY-MM-DD-<feature>/search-cache.md"));
        assert!(producer.contains("Feature lead is the only writer"));
        assert!(producer.contains("ignored, workspace-only, and non-authoritative"));
    }
    for consumer in [&execute_plan, &execute_task, &verify, &review] {
        assert!(consumer.contains("look up a current matching cache entry before new discovery"));
        assert!(consumer.contains("never replaces fresh Git, authority, verification, or review evidence"));
        assert!(consumer.contains("return attributable cache candidates to the Feature lead"));
    }
    for profile in [orchestrator_instructions, implementer_instructions.as_str()] {
        assert!(profile.contains("look up a current matching `search-cache.md` entry before new discovery"));
        assert!(profile.contains("Feature lead is the only writer"));
        assert!(profile.contains("return attributable cache candidates"));
    }
    assert!(finish.contains("Retain `search-cache.md` with `implementation-plan.md`"));
    assert!(finish.contains("retire all three only with authorized removal of that coordination worktree"));
    assert!(readme.contains("`docs/plans/YYYY-MM-DD-<feature>/search-cache.md`"));
    assert!(readme.contains("same lifecycle as the ignored Implementation Plan"));
}

#[test]
fn managed_task_loop_separates_tdd_history_from_current_acceptance() {
    // Arrange
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer crate must be nested under the Codex source root");
    let tdd = fs::read_to_string(source_root.join("skills/test-driven-development/SKILL.md"))
        .expect("read test-driven-development skill");
    let execute_task = fs::read_to_string(source_root.join("skills/execute-task/SKILL.md"))
        .expect("read execute-task skill");
    let review =
        fs::read_to_string(source_root.join("skills/review/SKILL.md")).expect("read review skill");
    let triage = fs::read_to_string(source_root.join("skills/receiving-code-review/SKILL.md"))
        .expect("read receiving-code-review skill");
    let implementer_text = fs::read_to_string(source_root.join("agents/implementer.toml"))
        .expect("read implementer profile");
    let fallback = fs::read_to_string(
        source_root.join("skills/agent-teams-driven-development/implementer-prompt.md"),
    )
    .expect("read implementer fallback prompt");

    // Act
    let implementer =
        toml::from_str::<toml::Table>(&implementer_text).expect("parse implementer TOML");
    let implementer_instructions = implementer
        .get("developer_instructions")
        .and_then(toml::Value::as_str)
        .expect("implementer developer instructions")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let history_contracts = [tdd, execute_task, review, triage, fallback]
        .map(|contract| contract.split_whitespace().collect::<Vec<_>>().join(" "));

    // Assert
    for contract in history_contracts
        .iter()
        .chain([&implementer_instructions])
    {
        for required in [
            "actual pre-production RED and its reason",
            "never recreate or repair historical RED evidence after the production edit",
            "Disclose an unrepairable historical discipline gap",
            "not an Acceptance blocker by itself",
            "reachable current defect, material current evidence gap, material contract deviation, or controlling authority",
        ] {
            assert!(
                contract.contains(required),
                "TDD history/current Acceptance contract omits {required:?}"
            );
        }
    }
}

#[test]
fn managed_task_loop_asset_inventory_remains_unchanged() {
    // Arrange
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer crate must be nested under the Codex source root");
    let mut agent_names = fs::read_dir(source_root.join("agents"))
        .expect("read managed agent directory")
        .map(|entry| {
            entry
                .expect("read managed agent entry")
                .file_name()
                .into_string()
                .expect("managed agent name is UTF-8")
        })
        .collect::<Vec<_>>();
    let mut skill_names = fs::read_dir(source_root.join("skills"))
        .expect("read managed skill directory")
        .map(|entry| {
            entry
                .expect("read managed skill entry")
                .file_name()
                .into_string()
                .expect("managed skill name is UTF-8")
        })
        .collect::<Vec<_>>();

    // Act
    agent_names.sort();
    skill_names.sort();

    // Assert
    assert_eq!(
        agent_names,
        [
            "adversarial-api-reviewer.toml",
            "adversarial-integrator.toml",
            "adversarial-performance-reviewer.toml",
            "adversarial-robustness-reviewer.toml",
            "adversarial-tests-reviewer.toml",
            "code-architect.toml",
            "code-quality-reviewer.toml",
            "code-reviewer.toml",
            "design-alignment-reviewer.toml",
            "implementation-verifier.toml",
            "implementer.toml",
            "review-integrator.toml",
            "scope-reviewer.toml",
            "spec-reviewer.toml",
            "task-orchestrator.toml",
            "test-coverage-reviewer.toml",
        ]
    );
    assert_eq!(
        skill_names,
        [
            "agent-teams-driven-development",
            "agentic-engineering-workflow",
            "commit",
            "create-plan",
            "create-pr",
            "create-workspace",
            "design-discussion",
            "design-doc",
            "dispatching-parallel-agents",
            "execute-plan",
            "execute-task",
            "finish-branch",
            "receiving-code-review",
            "review",
            "session-teardown",
            "systematic-debugging",
            "test-driven-development",
            "verify",
            "walkthrough-plan",
        ]
    );
}

#[test]
fn install_and_restore_round_trip_with_normal_binary() {
    // Arrange
    let temporary = process_tempdir("install-restore-round-trip");
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer crate must be nested under the Codex source root");
    let source_guidance =
        fs::read(source_root.join("AGENTS.global.md")).expect("read source guidance");
    let source_skill = fs::read(source_root.join("skills/agent-teams-driven-development/SKILL.md"))
        .expect("read representative source skill");
    let source_agent = fs::read(source_root.join("agents/code-reviewer.toml"))
        .expect("read representative source agent");
    let source_task_orchestrator = fs::read(source_root.join("agents/task-orchestrator.toml"))
        .expect("read source Task orchestrator");
    let source_review_integrator = fs::read(source_root.join("agents/review-integrator.toml"))
        .expect("read source review integrator");
    let codex_home = temporary.path().join("codex-home");
    let skills_home = temporary.path().join("skills-home");
    let state_dir = temporary.path().join("state");
    for directory in [&codex_home, &skills_home, &state_dir] {
        fs::create_dir(directory).expect("create end-to-end root");
    }
    let unmanaged_config = concat!(
        "# preserve this unmanaged block byte-for-byte\n",
        "model_context_window = 123456\n",
        "statusline = [\"model\", \"context\"]\n",
    );
    let prior_config = concat!(
        "model = \"old-model\"\n",
        "model_reasoning_effort = \"medium\"\n",
        "plan_mode_reasoning_effort = \"high\"\n",
        "# preserve this unmanaged block byte-for-byte\n",
        "model_context_window = 123456\n",
        "statusline = [\"model\", \"context\"]\n",
        "\n",
        "[agents]\n",
        "max_threads = 2\n",
        "max_depth = 3\n",
    );
    let prior_manifest = concat!(
        "{\n",
        "  \"version\": 1,\n",
        "  \"global_agents\": true,\n",
        "  \"skills\": [\n",
        "    \"stale-skill\"\n",
        "  ],\n",
        "  \"agents\": []\n",
        "}\n",
    );
    let prior_guidance = b"prior global guidance\n";
    let prior_stale_skill = b"prior stale skill\n";
    let unrelated_skill = skills_home.join("unrelated-skill/SKILL.md");
    let unrelated_agent = codex_home.join("agents/unrelated-agent.toml");
    let system_sentinel = codex_home.join("skills/.system/sentinel");
    fs::write(codex_home.join("config.toml"), prior_config).expect("write prior config");
    fs::write(codex_home.join("AGENTS.md"), prior_guidance).expect("write prior guidance");
    fs::create_dir(skills_home.join("stale-skill")).expect("create stale owned skill");
    fs::write(skills_home.join("stale-skill/SKILL.md"), prior_stale_skill)
        .expect("write stale owned skill");
    fs::create_dir(skills_home.join("unrelated-skill")).expect("create unrelated skill");
    fs::write(&unrelated_skill, b"unrelated skill\n").expect("write unrelated skill");
    fs::create_dir(codex_home.join("agents")).expect("create Codex agents directory");
    fs::write(&unrelated_agent, b"unrelated agent\n").expect("write unrelated agent");
    fs::create_dir_all(codex_home.join("skills/.system"))
        .expect("create Codex system skills directory");
    fs::write(&system_sentinel, b"system sentinel\n").expect("write system sentinel");
    fs::write(state_dir.join("manifest-v1.json"), prior_manifest)
        .expect("write prior ownership manifest");

    // Act
    let install = Command::new(env!("CARGO_BIN_EXE_dotfiles-codex-installer"))
        .env_clear()
        .env("HOME", temporary.path().join("home"))
        .env("TMPDIR", temporary.path())
        .env("PATH", "/usr/bin:/bin")
        .args(["install", "--agent-threads", "4", "--codex-home"])
        .arg(&codex_home)
        .arg("--skills-home")
        .arg(&skills_home)
        .arg("--state-dir")
        .arg(&state_dir)
        .output()
        .expect("run normal binary install");
    let installed_config = fs::read(codex_home.join("config.toml")).expect("read installed config");
    let installed_config_text =
        String::from_utf8(installed_config.clone()).expect("installed config is UTF-8");
    let installed_config_table =
        toml::from_str::<toml::Table>(&installed_config_text).expect("parse installed config");
    let installed_agents = installed_config_table
        .get("agents")
        .and_then(toml::Value::as_table)
        .expect("installed config has agents table");
    let installed_manifest =
        fs::read(state_dir.join("manifest-v1.json")).expect("read installed manifest");
    let stale_exists_after_install = skills_home.join("stale-skill").exists();
    let installed_guidance =
        fs::read(codex_home.join("AGENTS.md")).expect("read installed guidance");
    let installed_skill = fs::read(skills_home.join("agent-teams-driven-development/SKILL.md"))
        .expect("read installed representative skill");
    let installed_agent = fs::read(codex_home.join("agents/code-reviewer.toml"))
        .expect("read installed representative agent");
    let installed_task_orchestrator = fs::read(codex_home.join("agents/task-orchestrator.toml"))
        .expect("read installed Task orchestrator");
    let installed_review_integrator = fs::read(codex_home.join("agents/review-integrator.toml"))
        .expect("read installed review integrator");
    let unrelated_after_install = (
        fs::read(&unrelated_skill).expect("read unrelated skill after install"),
        fs::read(&unrelated_agent).expect("read unrelated agent after install"),
        fs::read(&system_sentinel).expect("read system sentinel after install"),
    );
    let backups_dir = state_dir.join("backups");
    let latest_after_install =
        fs::read(backups_dir.join("latest")).expect("read latest marker after install");
    let selected_backup_id = std::str::from_utf8(&latest_after_install)
        .expect("latest marker is UTF-8")
        .strip_suffix('\n')
        .expect("latest marker ends with newline")
        .to_owned();
    let selected_backup = backups_dir.join(&selected_backup_id);
    let mut backup_directories_after_install = fs::read_dir(&backups_dir)
        .expect("read backups after install")
        .filter_map(|entry| {
            let entry = entry.expect("read backup entry after install");
            entry
                .file_type()
                .expect("read backup entry type after install")
                .is_dir()
                .then(|| entry.path())
        })
        .collect::<Vec<_>>();
    backup_directories_after_install.sort();
    let journal_after_install =
        fs::read(selected_backup.join("journal-v1.json")).expect("read selected backup journal");
    let payload_root = selected_backup.join("payload");
    let payload_after_install = (
        fs::read(payload_root.join("codex-home/config.toml")).expect("read backed up config"),
        fs::read(payload_root.join("codex-home/AGENTS.md")).expect("read backed up guidance"),
        fs::read(payload_root.join("skills-home/stale-skill/SKILL.md"))
            .expect("read backed up stale skill"),
        fs::read(payload_root.join("state-dir/manifest-v1.json")).expect("read backed up manifest"),
    );
    let wal_exists_after_install = state_dir.join("transaction/wal-v1.json").exists();
    let work_is_empty_after_install = fs::read_dir(state_dir.join("transaction/work"))
        .expect("read transaction work after install")
        .next()
        .is_none();
    let restore = Command::new(env!("CARGO_BIN_EXE_dotfiles-codex-installer"))
        .env_clear()
        .env("HOME", temporary.path().join("home"))
        .env("TMPDIR", temporary.path())
        .env("PATH", "/usr/bin:/bin")
        .args(["restore", "--state-dir"])
        .arg(&state_dir)
        .output()
        .expect("run normal binary restore");
    let restored_state = (
        fs::read(codex_home.join("config.toml")).expect("read restored config"),
        fs::read(codex_home.join("AGENTS.md")).expect("read restored guidance"),
        fs::read(skills_home.join("stale-skill/SKILL.md")).expect("read restored stale skill"),
        fs::read(state_dir.join("manifest-v1.json")).expect("read restored manifest"),
    );
    let unrelated_after_restore = (
        fs::read(&unrelated_skill).expect("read unrelated skill after restore"),
        fs::read(&unrelated_agent).expect("read unrelated agent after restore"),
        fs::read(&system_sentinel).expect("read system sentinel after restore"),
    );
    let latest_after_restore =
        fs::read(backups_dir.join("latest")).expect("read latest marker after restore");
    let journal_after_restore =
        fs::read(selected_backup.join("journal-v1.json")).expect("read journal after restore");
    let payload_after_restore = (
        fs::read(payload_root.join("codex-home/config.toml")).expect("reread backed up config"),
        fs::read(payload_root.join("codex-home/AGENTS.md")).expect("reread backed up guidance"),
        fs::read(payload_root.join("skills-home/stale-skill/SKILL.md"))
            .expect("reread backed up stale skill"),
        fs::read(payload_root.join("state-dir/manifest-v1.json"))
            .expect("reread backed up manifest"),
    );
    let mut backup_directories_after_restore = fs::read_dir(&backups_dir)
        .expect("read backups after restore")
        .filter_map(|entry| {
            let entry = entry.expect("read backup entry after restore");
            entry
                .file_type()
                .expect("read backup entry type after restore")
                .is_dir()
                .then(|| entry.path())
        })
        .collect::<Vec<_>>();
    backup_directories_after_restore.sort();
    let wal_exists_after_restore = state_dir.join("transaction/wal-v1.json").exists();
    let work_is_empty_after_restore = fs::read_dir(state_dir.join("transaction/work"))
        .expect("read transaction work after restore")
        .next()
        .is_none();

    // Assert
    assert_eq!(
        (
            install.status.success(),
            String::from_utf8(install.stdout).expect("UTF-8 install stdout"),
            String::from_utf8(install.stderr).expect("UTF-8 install stderr"),
        ),
        (true, "install complete\n".to_owned(), String::new())
    );
    assert_eq!(
        (
            installed_config_table
                .get("model")
                .and_then(toml::Value::as_str),
            installed_config_table
                .get("model_reasoning_effort")
                .and_then(toml::Value::as_str),
            installed_config_table
                .get("plan_mode_reasoning_effort")
                .and_then(toml::Value::as_str),
            installed_agents
                .get("max_threads")
                .and_then(toml::Value::as_integer),
            installed_agents
                .get("max_depth")
                .and_then(toml::Value::as_integer),
        ),
        (
            Some("gpt-5.6-sol"),
            Some("xhigh"),
            Some("xhigh"),
            Some(4),
            Some(2),
        )
    );
    assert!(
        installed_config_text.contains(unmanaged_config),
        "unmanaged bytes were not preserved:\n{installed_config_text}"
    );
    assert_ne!(installed_manifest, prior_manifest.as_bytes());
    assert!(!stale_exists_after_install);
    assert_eq!(
        (
            installed_guidance,
            installed_skill,
            installed_agent,
            installed_task_orchestrator,
            installed_review_integrator,
        ),
        (
            source_guidance,
            source_skill,
            source_agent,
            source_task_orchestrator,
            source_review_integrator,
        )
    );
    assert_eq!(
        unrelated_after_install,
        (
            b"unrelated skill\n".to_vec(),
            b"unrelated agent\n".to_vec(),
            b"system sentinel\n".to_vec(),
        )
    );
    assert_eq!(
        backup_directories_after_install,
        vec![selected_backup.clone()]
    );
    assert_eq!(
        payload_after_install,
        (
            prior_config.as_bytes().to_vec(),
            prior_guidance.to_vec(),
            prior_stale_skill.to_vec(),
            prior_manifest.as_bytes().to_vec(),
        )
    );
    assert!(!journal_after_install.is_empty());
    assert!(!wal_exists_after_install);
    assert!(work_is_empty_after_install);
    assert_eq!(
        (
            restore.status.success(),
            String::from_utf8(restore.stdout).expect("UTF-8 restore stdout"),
            String::from_utf8(restore.stderr).expect("UTF-8 restore stderr"),
        ),
        (true, "restore complete\n".to_owned(), String::new())
    );
    assert_eq!(
        restored_state,
        (
            prior_config.as_bytes().to_vec(),
            prior_guidance.to_vec(),
            prior_stale_skill.to_vec(),
            prior_manifest.as_bytes().to_vec(),
        )
    );
    assert!(!skills_home.join("agent-teams-driven-development").exists());
    assert!(!codex_home.join("agents/code-reviewer.toml").exists());
    assert!(!codex_home.join("agents/task-orchestrator.toml").exists());
    assert!(!codex_home.join("agents/review-integrator.toml").exists());
    assert_eq!(unrelated_after_restore, unrelated_after_install);
    assert_eq!(latest_after_restore, latest_after_install);
    assert_eq!(journal_after_restore, journal_after_install);
    assert_eq!(payload_after_restore, payload_after_install);
    assert_eq!(
        backup_directories_after_restore,
        backup_directories_after_install
    );
    assert!(!wal_exists_after_restore);
    assert!(work_is_empty_after_restore);
}
