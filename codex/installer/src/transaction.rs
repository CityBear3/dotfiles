mod engine;
mod model;
mod move_protocol;
mod recovery;
mod wal;

pub(crate) use engine::TransactionEngine;
#[cfg(test)]
pub(crate) use model::TransactionOutcome;
pub(crate) use model::{FaultPoint, RecoveryOutcome};

#[cfg(test)]
#[path = "transaction/transaction_tests.rs"]
mod tests;
