use std::collections::BTreeSet;
use std::fs;

use crate::content::CapturedContent;
use crate::test_support::project_tempdir;

use super::{
    ManifestState, OwnershipManifest, StaleAssets, manifest_content, read_manifest, stale_assets,
};

#[test]
fn absent_manifest_means_no_prior_ownership() {
    // Arrange
    let temporary = project_tempdir("ownership-absent");
    let path = temporary.path().join("state/manifest-v1.json");

    // Act
    let result = read_manifest(&path);

    // Assert
    assert_eq!(result, Ok(ManifestState::Absent));
}

#[test]
fn valid_manifest_is_loaded_with_its_immutable_content() {
    // Arrange
    let temporary = project_tempdir("ownership-valid");
    let path = temporary.path().join("manifest-v1.json");
    let bytes =
        b"{\"version\":1,\"global_agents\":true,\"skills\":[\"zeta\",\"alpha\"],\"agents\":[\"worker.toml\"]}\n";
    fs::write(&path, bytes).expect("write manifest");

    // Act
    let result = read_manifest(&path);

    // Assert
    assert_eq!(
        result,
        Ok(ManifestState::Present {
            manifest: OwnershipManifest {
                version: 1,
                global_agents: true,
                skills: BTreeSet::from(["alpha".to_owned(), "zeta".to_owned()]),
                agents: BTreeSet::from(["worker.toml".to_owned()]),
            },
            content: CapturedContent::file(bytes.to_vec()),
        })
    );
}

#[test]
fn unknown_manifest_version_and_fields_are_rejected() {
    // Arrange
    let temporary = project_tempdir("ownership-unknown");
    let version_path = temporary.path().join("unknown-version.json");
    let field_path = temporary.path().join("unknown-field.json");
    fs::write(
        &version_path,
        b"{\"version\":2,\"global_agents\":false,\"skills\":[],\"agents\":[]}",
    )
    .expect("write unknown version");
    fs::write(
        &field_path,
        b"{\"version\":1,\"global_agents\":false,\"skills\":[],\"agents\":[],\"extra\":true}",
    )
    .expect("write unknown field");

    // Act
    let version_result = read_manifest(&version_path);
    let field_result = read_manifest(&field_path);

    // Assert
    assert!(version_result.is_err());
    assert!(field_result.is_err());
}

#[test]
fn stale_assets_are_limited_to_entries_in_the_prior_manifest() {
    // Arrange
    let prior = OwnershipManifest {
        version: 1,
        global_agents: true,
        skills: BTreeSet::from(["owned-stale".to_owned(), "shared".to_owned()]),
        agents: BTreeSet::from(["owned.toml".to_owned()]),
    };
    let desired = OwnershipManifest {
        version: 1,
        global_agents: false,
        skills: BTreeSet::from(["shared".to_owned(), "external".to_owned()]),
        agents: BTreeSet::new(),
    };

    // Act
    let result = stale_assets(&prior, &desired);

    // Assert
    assert_eq!(
        result,
        StaleAssets {
            global_agents: true,
            skills: BTreeSet::from(["owned-stale".to_owned()]),
            agents: BTreeSet::from(["owned.toml".to_owned()]),
        }
    );
}

#[test]
fn desired_manifest_serialization_is_deterministic() {
    // Arrange
    let manifest = OwnershipManifest {
        version: 1,
        global_agents: true,
        skills: BTreeSet::from(["alpha".to_owned(), "zeta".to_owned()]),
        agents: BTreeSet::from(["worker.toml".to_owned()]),
    };

    // Act
    let result = manifest_content(&manifest);

    // Assert
    assert_eq!(
        result,
        Ok(CapturedContent::file(
            concat!(
                "{\n",
                "  \"version\": 1,\n",
                "  \"global_agents\": true,\n",
                "  \"skills\": [\n",
                "    \"alpha\",\n",
                "    \"zeta\"\n",
                "  ],\n",
                "  \"agents\": [\n",
                "    \"worker.toml\"\n",
                "  ]\n",
                "}\n"
            )
            .as_bytes()
            .to_vec()
        ))
    );
}

#[test]
fn manifest_rejects_an_agent_without_a_safe_name_before_toml_suffix() {
    // Arrange
    let manifest = OwnershipManifest {
        version: 1,
        global_agents: false,
        skills: BTreeSet::new(),
        agents: BTreeSet::from([".toml".to_owned()]),
    };

    // Act
    let result = manifest_content(&manifest);

    // Assert
    assert!(result.is_err());
}
