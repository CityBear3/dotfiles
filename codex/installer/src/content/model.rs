use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CapturedContent {
    pub(crate) payload: ContentPayload,
    pub(crate) sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ContentPayload {
    File(Vec<u8>),
    Directory {
        directories: BTreeSet<PathBuf>,
        files: BTreeMap<PathBuf, Vec<u8>>,
    },
}

impl CapturedContent {
    pub(crate) fn file(bytes: Vec<u8>) -> Self {
        Self::from_payload(ContentPayload::File(bytes))
    }

    pub(crate) fn directory(
        directories: BTreeSet<PathBuf>,
        files: BTreeMap<PathBuf, Vec<u8>>,
    ) -> Self {
        Self::from_payload(ContentPayload::Directory { directories, files })
    }

    pub(crate) fn file_bytes(&self) -> Option<&[u8]> {
        match &self.payload {
            ContentPayload::File(bytes) => Some(bytes),
            ContentPayload::Directory { .. } => None,
        }
    }

    fn from_payload(payload: ContentPayload) -> Self {
        let sha256 = fingerprint(&payload);
        Self { payload, sha256 }
    }
}

fn fingerprint(payload: &ContentPayload) -> String {
    let mut hasher = Sha256::new();
    match payload {
        ContentPayload::File(bytes) => {
            hasher.update(b"file\0");
            update_bytes(&mut hasher, bytes);
        }
        ContentPayload::Directory { directories, files } => {
            hasher.update(b"directory\0");
            for directory in directories {
                hasher.update(b"directory-entry\0");
                update_bytes(&mut hasher, directory.as_os_str().as_encoded_bytes());
            }
            for (path, bytes) in files {
                hasher.update(b"file-entry\0");
                update_bytes(&mut hasher, path.as_os_str().as_encoded_bytes());
                update_bytes(&mut hasher, bytes);
            }
        }
    }
    format!("{:x}", hasher.finalize())
}

fn update_bytes(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update(bytes.len().to_le_bytes());
    hasher.update(bytes);
}
