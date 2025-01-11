pub mod wallet;
pub mod network;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("钱包错误: {0}")]
    Wallet(#[from] wallet::WalletError),
}

pub type Result<T> = std::result::Result<T, Error>;