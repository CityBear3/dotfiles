use std::path::PathBuf;

pub(crate) fn process_tempdir(test_name: &str) -> tempfile::TempDir {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repository_root = manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .expect("installer crate must be nested under the repository root");
    let parent = repository_root
        .join("target")
        .join("test-tmp")
        .join("process");
    std::fs::create_dir_all(&parent).expect("create project-local process-test root");
    tempfile::Builder::new()
        .prefix(test_name)
        .tempdir_in(parent)
        .expect("create process-test directory")
}
