#[cfg(target_os = "macos")]
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::InstallerError;
#[cfg(target_os = "macos")]
use crate::backup::BackupStore;
use crate::command::InstallerCommand;
#[cfg(target_os = "macos")]
use crate::platform::macos::MacOsPlatform;
use crate::resources::MachineResources;
#[cfg(target_os = "macos")]
use crate::transaction::{RecoveryOutcome, TransactionEngine};

mod install;
mod restore;

static OPERATION_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ApplicationContext {
    pub(crate) source_root: PathBuf,
    pub(crate) resources: MachineResources,
}

pub(crate) fn execute(command: InstallerCommand) -> Result<String, InstallerError> {
    let resources = resources_for(&command)?;
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let source_root = manifest_dir
        .parent()
        .expect("installer crate must be nested under the Codex source root")
        .to_path_buf();
    execute_with_context(
        command,
        ApplicationContext {
            source_root,
            resources,
        },
    )
}

pub(crate) fn execute_with_context(
    command: InstallerCommand,
    context: ApplicationContext,
) -> Result<String, InstallerError> {
    execute_with_context_for_platform(command, context, cfg!(target_os = "macos"))
}

fn execute_with_context_for_platform(
    command: InstallerCommand,
    context: ApplicationContext,
    mutations_supported: bool,
) -> Result<String, InstallerError> {
    if !mutations_supported
        && !matches!(
            &command,
            InstallerCommand::Install(command) if command.dry_run
        )
    {
        return Err(InstallerError::UnsupportedPlatform);
    }
    let operation_id = match &command {
        InstallerCommand::Install(command) if command.dry_run => None,
        InstallerCommand::Install(_) => Some(generate_operation_id("install")),
        InstallerCommand::Restore(_) => Some(generate_operation_id("restore")),
    };
    execute_with_context_and_optional_id(command, context, operation_id.as_deref())
}

#[cfg(test)]
pub(super) fn execute_with_context_and_id(
    command: InstallerCommand,
    context: ApplicationContext,
    operation_id: &str,
) -> Result<String, InstallerError> {
    execute_with_context_and_optional_id(command, context, Some(operation_id))
}

#[cfg(test)]
pub(super) fn execute_restore_with_context_and_id(
    command: crate::command::RestoreCommand,
    context: ApplicationContext,
    operation_id: &str,
) -> Result<String, InstallerError> {
    restore::execute_mutating(command, &context.source_root, operation_id)
}

fn execute_with_context_and_optional_id(
    command: InstallerCommand,
    context: ApplicationContext,
    operation_id: Option<&str>,
) -> Result<String, InstallerError> {
    match command {
        InstallerCommand::Install(command) if command.dry_run => {
            install::execute_dry_run(command, context)
        }
        InstallerCommand::Install(command) => install::execute_mutating(
            command,
            context,
            operation_id.expect("mutating install receives an operation ID"),
        ),
        InstallerCommand::Restore(command) => restore::execute_mutating(
            command,
            &context.source_root,
            operation_id.expect("restore receives an operation ID"),
        ),
    }
}

#[cfg(target_os = "macos")]
fn recover_unfinished(
    engine: &TransactionEngine<MacOsPlatform>,
    store: &BackupStore<'_, MacOsPlatform>,
    state_dir: &Path,
) -> Result<(), InstallerError> {
    let recovery = engine.recover_with_finalization(state_dir, |transaction_id| {
        store.finalize_committed_transaction(transaction_id)
    })?;
    if let RecoveryOutcome::RolledBack { transaction_id } = recovery {
        discard_if_unselected(store, &transaction_id)?;
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn discard_if_unselected(
    store: &BackupStore<'_, MacOsPlatform>,
    backup_id: &str,
) -> Result<(), InstallerError> {
    if store
        .load_latest()?
        .is_some_and(|latest| latest.journal.backup_id == backup_id)
    {
        return Ok(());
    }
    store.discard_unselected(backup_id)
}

fn generate_operation_id(operation: &str) -> String {
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let sequence = OPERATION_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    format!("{operation}-{}-{elapsed}-{sequence}", std::process::id())
}

fn resources_for(command: &InstallerCommand) -> Result<MachineResources, InstallerError> {
    let requires_detection = matches!(
        command,
        InstallerCommand::Install(command) if command.agent_threads == "auto"
    );
    if !requires_detection {
        return Ok(MachineResources {
            logical_cpus: 1,
            memory_bytes: 0,
        });
    }
    Ok(MachineResources {
        logical_cpus: std::thread::available_parallelism()
            .map(usize::from)
            .unwrap_or(1),
        memory_bytes: physical_memory_bytes()?,
    })
}

#[cfg(target_os = "macos")]
fn physical_memory_bytes() -> Result<u64, InstallerError> {
    use std::ffi::{CString, c_void};
    use std::mem;
    use std::ptr;

    let name = CString::new("hw.memsize").expect("static sysctl name");
    let mut memory = 0_u64;
    let mut length = mem::size_of::<u64>();
    let result = unsafe {
        libc::sysctlbyname(
            name.as_ptr(),
            (&mut memory as *mut u64).cast::<c_void>(),
            &mut length,
            ptr::null_mut(),
            0,
        )
    };
    if result != 0 || length != mem::size_of::<u64>() {
        return Err(InstallerError::Filesystem {
            message: format!(
                "read physical memory with sysctl: {}",
                std::io::Error::last_os_error()
            ),
        });
    }
    Ok(memory)
}

#[cfg(not(target_os = "macos"))]
fn physical_memory_bytes() -> Result<u64, InstallerError> {
    Ok(0)
}

#[cfg(all(test, target_os = "macos"))]
#[path = "application/application_tests.rs"]
mod tests;
