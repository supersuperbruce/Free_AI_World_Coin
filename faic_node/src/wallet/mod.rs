pub mod core;
pub mod transaction;

// 重导出主要类型
pub use core::{WalletService, WalletInfo, Balance};
pub use transaction::TransactionService;

// 使用统一的错误处理
use crate::error::wallet::{WalletError, WalletResult};