pub(crate) fn project_tempdir(test_name: &str) -> tempfile::TempDir {
    let parent = std::fs::canonicalize(std::env::temp_dir())
        .expect("resolve the operating system temporary directory");
    tempfile::Builder::new()
        .prefix(test_name)
        .tempdir_in(parent)
        .expect("create a unique unit-test directory")
}
