mod install;

pub(crate) use install::execute;

#[cfg(test)]
#[path = "application/application_tests.rs"]
mod tests;
