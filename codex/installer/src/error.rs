use clap::error::ErrorKind;
use std::path::PathBuf;
use thiserror::Error;

/// Errors emitted by the installer command shell.
#[derive(Debug, Eq, Error, PartialEq)]
pub enum InstallerError {
    #[error("{message}")]
    Cli {
        message: String,
        exit_code: u8,
        use_stderr: bool,
    },

    #[error("HOME must be set to resolve installer defaults")]
    MissingHome,

    #[error("--agent-threads must be auto or an integer from 2 to 32")]
    InvalidAgentThreads,

    #[error("mutating installer commands are supported only on macOS")]
    UnsupportedPlatform,

    #[error("{message}")]
    InvalidConfiguration { message: String },

    #[error("{message}")]
    InvalidInventory { message: String },

    #[error("{message}")]
    InvalidManifest { message: String },

    #[error("unsafe path {path}: {message}")]
    UnsafePath { path: PathBuf, message: String },

    #[error("{message}")]
    Filesystem { message: String },

    #[error("{message}")]
    Lock { message: String },

    #[error("{message}")]
    Transaction { message: String },

    #[error("transaction {transaction_id} failed and was rolled back: {cause}")]
    TransactionRolledBack {
        transaction_id: String,
        cause: Box<InstallerError>,
    },

    #[error(
        "transaction {transaction_id} rollback failed at {wal}: {rollback_cause}; original cause: {cause:?}; paths: {paths:?}"
    )]
    TransactionRollbackFailed {
        transaction_id: String,
        wal: PathBuf,
        paths: Vec<PathBuf>,
        cause: Option<Box<InstallerError>>,
        rollback_cause: Box<InstallerError>,
    },

    #[error(
        "transaction {transaction_id} committed live state, but finalization/cleanup is incomplete at {wal}; retry a mutating command: {cleanup_cause}; original cause: {cause:?}; paths: {paths:?}"
    )]
    CommittedCleanupIncomplete {
        transaction_id: String,
        wal: PathBuf,
        paths: Vec<PathBuf>,
        cause: Option<Box<InstallerError>>,
        cleanup_cause: Box<InstallerError>,
    },

    #[error("invalid transaction WAL: {message}")]
    InvalidWal { message: String },

    #[error("invalid backup: {message}")]
    InvalidBackup { message: String },

    #[error(
        "transaction {transaction_id} state cannot be classified at {wal}: {message}; paths: {paths:?}"
    )]
    UnclassifiableTransaction {
        transaction_id: String,
        wal: PathBuf,
        paths: Vec<PathBuf>,
        message: String,
    },

    #[error(
        "transaction {transaction_id} WAL authority is unresolved at {wal}: {message}; paths: {paths:?}"
    )]
    UnresolvedWalAuthority {
        transaction_id: String,
        wal: PathBuf,
        paths: Vec<PathBuf>,
        message: String,
    },

    #[error("injected transaction fault: {point}")]
    InjectedTransactionFault { point: &'static str },

    #[error("unmanaged destination conflicts: {paths:?}")]
    UnmanagedConflict { paths: Vec<PathBuf> },

    #[error("{operation} is not implemented yet")]
    NotImplemented { operation: &'static str },
}

impl InstallerError {
    pub(crate) fn from_clap(error: clap::Error) -> Self {
        let is_display = matches!(
            error.kind(),
            ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
        );
        Self::Cli {
            message: error.to_string(),
            exit_code: if is_display { 0 } else { 1 },
            use_stderr: error.use_stderr(),
        }
    }

    /// Installer exit status associated with this error.
    pub fn exit_code(&self) -> u8 {
        match self {
            Self::Cli { exit_code, .. } => *exit_code,
            Self::MissingHome
            | Self::InvalidAgentThreads
            | Self::UnsupportedPlatform
            | Self::InvalidConfiguration { .. }
            | Self::InvalidInventory { .. }
            | Self::InvalidManifest { .. }
            | Self::UnsafePath { .. }
            | Self::Filesystem { .. }
            | Self::Lock { .. }
            | Self::Transaction { .. }
            | Self::TransactionRolledBack { .. }
            | Self::TransactionRollbackFailed { .. }
            | Self::CommittedCleanupIncomplete { .. }
            | Self::InvalidWal { .. }
            | Self::InvalidBackup { .. }
            | Self::UnclassifiableTransaction { .. }
            | Self::UnresolvedWalAuthority { .. }
            | Self::InjectedTransactionFault { .. }
            | Self::NotImplemented { .. } => 1,
            Self::UnmanagedConflict { .. } => 2,
        }
    }

    /// Whether the error should be written to standard error.
    pub fn use_stderr(&self) -> bool {
        match self {
            Self::Cli { use_stderr, .. } => *use_stderr,
            Self::MissingHome
            | Self::InvalidAgentThreads
            | Self::UnsupportedPlatform
            | Self::InvalidConfiguration { .. }
            | Self::InvalidInventory { .. }
            | Self::InvalidManifest { .. }
            | Self::UnsafePath { .. }
            | Self::Filesystem { .. }
            | Self::Lock { .. }
            | Self::Transaction { .. }
            | Self::TransactionRolledBack { .. }
            | Self::TransactionRollbackFailed { .. }
            | Self::CommittedCleanupIncomplete { .. }
            | Self::InvalidWal { .. }
            | Self::InvalidBackup { .. }
            | Self::UnclassifiableTransaction { .. }
            | Self::UnresolvedWalAuthority { .. }
            | Self::InjectedTransactionFault { .. }
            | Self::UnmanagedConflict { .. }
            | Self::NotImplemented { .. } => true,
        }
    }
}
