use crate::InstallerError;

use super::{MachineResources, select_max_threads};

const GIBIBYTE: u64 = 1024 * 1024 * 1024;

#[test]
fn constrained_machine_selects_four_threads() {
    // Arrange
    let resources = MachineResources {
        logical_cpus: 7,
        memory_bytes: 64 * GIBIBYTE,
    };

    // Act
    let result = select_max_threads(resources, "auto");

    // Assert
    assert_eq!(result, Ok(4));
}

#[test]
fn standard_machine_selects_six_threads() {
    // Arrange
    let resources = MachineResources {
        logical_cpus: 10,
        memory_bytes: 24 * GIBIBYTE,
    };

    // Act
    let result = select_max_threads(resources, "auto");

    // Assert
    assert_eq!(result, Ok(6));
}

#[test]
fn high_spec_machine_selects_eight_threads() {
    // Arrange
    let resources = MachineResources {
        logical_cpus: 12,
        memory_bytes: 32 * GIBIBYTE,
    };

    // Act
    let result = select_max_threads(resources, "auto");

    // Assert
    assert_eq!(result, Ok(8));
}

#[test]
fn explicit_threads_from_two_through_thirty_two_are_accepted() {
    // Arrange
    let resources = MachineResources {
        logical_cpus: 1,
        memory_bytes: 0,
    };

    // Act
    let results = (2_u8..=32)
        .map(|requested| {
            (
                requested,
                select_max_threads(resources, &requested.to_string()),
            )
        })
        .collect::<Vec<_>>();

    // Assert
    let expected = (2_u8..=32)
        .map(|requested| (requested, Ok(requested)))
        .collect::<Vec<_>>();
    assert_eq!(results, expected);
}

#[test]
fn invalid_explicit_thread_values_are_rejected() {
    // Arrange
    let resources = MachineResources {
        logical_cpus: 16,
        memory_bytes: 64 * GIBIBYTE,
    };
    let requested_values = ["", "1", "33", "6.0", "many", "02", "+2", " 2"];

    // Act
    let results = requested_values.map(|requested| select_max_threads(resources, requested));

    // Assert
    let expected = requested_values.map(|_| Err(InstallerError::InvalidAgentThreads));
    assert_eq!(results, expected);
}
