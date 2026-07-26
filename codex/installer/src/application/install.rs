use std::path::PathBuf;

use crate::InstallerError;
use crate::command::InstallerCommand;
use crate::plan::{InstallPlanRequest, plan_install, render_dry_run};
use crate::resources::MachineResources;

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
    let operation = command.operation_name();
    match command {
        InstallerCommand::Install(command) if command.dry_run => {
            let plan = plan_install(InstallPlanRequest {
                source_root: context.source_root,
                codex_home: command.codex_home,
                skills_home: command.skills_home,
                state_dir: command.state_dir,
                adopt_existing: command.adopt_existing,
                requested_threads: command.agent_threads,
                resources: context.resources,
            })?;
            Ok(render_dry_run(&plan))
        }
        InstallerCommand::Install(_) | InstallerCommand::Restore(_) => {
            Err(InstallerError::NotImplemented { operation })
        }
    }
}

fn resources_for(command: &InstallerCommand) -> Result<MachineResources, InstallerError> {
    let requires_detection = matches!(
        command,
        InstallerCommand::Install(command)
            if command.dry_run && command.agent_threads == "auto"
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
