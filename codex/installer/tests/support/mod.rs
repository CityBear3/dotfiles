pub(crate) fn process_tempdir(test_name: &str) -> tempfile::TempDir {
    tempfile::Builder::new()
        .prefix(test_name)
        .tempdir_in("/private/tmp")
        .expect("create process-test directory")
}
