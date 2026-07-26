use std::path::PathBuf;

pub(crate) fn process_tempdir(test_name: &str) -> tempfile::TempDir {
    let parent = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("test-tmp")
        .join("process");
    std::fs::create_dir_all(&parent).expect("create project-local process-test root");
    tempfile::Builder::new()
        .prefix(test_name)
        .tempdir_in(parent)
        .expect("create process-test directory")
}
