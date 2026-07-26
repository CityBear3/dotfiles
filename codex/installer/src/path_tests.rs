use std::fs;
use std::path::{Path, PathBuf};

use crate::InstallerError;
use crate::test_support::project_tempdir;

use super::{InstallRoots, Locator, RootId};

#[test]
fn locator_accepts_normal_relative_components() {
    // Arrange
    let relative = PathBuf::from("agents/reviewer.toml");

    // Act
    let result = Locator::new(RootId::CodexHome, &relative);

    // Assert
    assert_eq!(
        result,
        Ok(Locator {
            root: RootId::CodexHome,
            relative,
        })
    );
}

#[test]
fn locator_rejects_unsafe_relative_paths() {
    // Arrange
    let unsafe_paths = [
        "",
        "/absolute",
        "../escape",
        "skills/./review",
        "skills//review",
    ];

    // Act
    let results = unsafe_paths.map(|path| Locator::new(RootId::StateDir, path));

    // Assert
    assert!(results.into_iter().all(|result| result.is_err()));
}

#[test]
fn roots_normalize_missing_destinations_without_creating_them() {
    // Arrange
    let temporary = project_tempdir("path-normalize-missing");
    let source_root = temporary.path().join("source");
    fs::create_dir(&source_root).expect("create source root");
    let codex_home = temporary.path().join("missing/codex");
    let skills_home = temporary.path().join("missing/skills");
    let state_dir = temporary.path().join("missing/state");

    // Act
    let result = InstallRoots::normalize(&source_root, &codex_home, &skills_home, &state_dir);

    // Assert
    assert_eq!(
        result,
        Ok(InstallRoots {
            source_root: source_root.canonicalize().expect("canonicalize source"),
            codex_home: codex_home.clone(),
            skills_home: skills_home.clone(),
            state_dir: state_dir.clone(),
        })
    );
    assert_eq!(
        [
            codex_home.exists(),
            skills_home.exists(),
            state_dir.exists()
        ],
        [false, false, false]
    );
}

#[test]
fn roots_reject_normalized_overlap() {
    // Arrange
    let temporary = project_tempdir("path-overlap");
    let source_root = temporary.path().join("source");
    fs::create_dir(&source_root).expect("create source root");
    let codex_home = temporary.path().join("destinations/codex");
    let skills_home = codex_home.join("skills");
    let state_dir = temporary.path().join("state");

    // Act
    let result = InstallRoots::normalize(&source_root, &codex_home, &skills_home, &state_dir);

    // Assert
    assert_eq!(
        result,
        Err(InstallerError::UnsafePath {
            path: codex_home,
            message: format!("root overlaps {}", skills_home.display()),
        })
    );
}

#[test]
fn locator_resolves_beneath_selected_root() {
    // Arrange
    let roots = InstallRoots {
        source_root: PathBuf::from("/source"),
        codex_home: PathBuf::from("/codex"),
        skills_home: PathBuf::from("/skills"),
        state_dir: PathBuf::from("/state"),
    };
    let locator = Locator::new(RootId::SkillsHome, Path::new("review")).expect("safe locator");

    // Act
    let result = roots.resolve(&locator);

    // Assert
    assert_eq!(result, PathBuf::from("/skills/review"));
}
