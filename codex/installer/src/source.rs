mod inventory;

pub(crate) use inventory::{SourceInventory, inventory, validate_agent_name, validate_asset_name};

#[cfg(test)]
#[path = "source/source_tests.rs"]
mod tests;
