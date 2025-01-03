pub mod core;
pub mod network;
pub mod wallet;
pub mod consensus;
pub mod validation;
pub mod utils;

// Re-export commonly used items
pub use core::{interfaces, error, logger};
pub use network::{p2p, message, discovery};
pub use wallet::{core as wallet_core, transaction};