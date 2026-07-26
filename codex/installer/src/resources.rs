use crate::InstallerError;

const GIBIBYTE: u64 = 1024 * 1024 * 1024;

/// Host resources used to select the agent concurrency ceiling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MachineResources {
    pub(crate) logical_cpus: usize,
    pub(crate) memory_bytes: u64,
}

/// Select an automatic thread tier or validate an explicit ceiling.
pub(crate) fn select_max_threads(
    resources: MachineResources,
    requested: &str,
) -> Result<u8, InstallerError> {
    if requested == "auto" {
        if resources.logical_cpus < 8 || resources.memory_bytes < 16 * GIBIBYTE {
            Ok(4)
        } else if resources.logical_cpus >= 12 && resources.memory_bytes >= 32 * GIBIBYTE {
            Ok(8)
        } else {
            Ok(6)
        }
    } else {
        parse_explicit_threads(requested)
    }
}

pub(crate) fn validate_agent_threads(requested: &str) -> Result<(), InstallerError> {
    if requested == "auto" {
        Ok(())
    } else {
        parse_explicit_threads(requested).map(|_| ())
    }
}

fn parse_explicit_threads(requested: &str) -> Result<u8, InstallerError> {
    let selected = requested
        .parse::<u8>()
        .map_err(|_| InstallerError::InvalidAgentThreads)?;
    if !(2..=32).contains(&selected) || selected.to_string() != requested {
        return Err(InstallerError::InvalidAgentThreads);
    }
    Ok(selected)
}

#[cfg(test)]
#[path = "resources_tests.rs"]
mod tests;
