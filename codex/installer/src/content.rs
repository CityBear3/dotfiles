mod capture;
#[cfg(target_os = "macos")]
mod materialize;
mod model;

pub(crate) use capture::capture_optional;
#[cfg(target_os = "macos")]
pub(crate) use materialize::materialize_durable;
pub(crate) use model::{CapturedContent, ContentPayload};

#[cfg(all(test, target_os = "macos"))]
#[path = "content/content_tests.rs"]
mod tests;
