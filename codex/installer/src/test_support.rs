use std::path::PathBuf;

pub(crate) fn project_tempdir(test_name: &str) -> tempfile::TempDir {
    let parent = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("test-tmp")
        .join("unit");
    std::fs::create_dir_all(&parent).expect("create the project-local unit-test root");
    tempfile::Builder::new()
        .prefix(test_name)
        .tempdir_in(parent)
        .expect("create a unique project-local unit-test directory")
}
