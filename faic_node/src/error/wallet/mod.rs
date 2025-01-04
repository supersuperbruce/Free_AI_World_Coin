mod create;
mod import;

pub use create::CreateWalletError;
pub use import::ImportWalletError;
pub use super::WalletError;
pub type WalletResult<T> = Result<T, WalletError>;

use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("创建钱包错误 - 私钥生成失败: {0}")]
    CreatePrivateKeyError(String),
    
    #[error("创建钱包错误 - 助记词生成失败: {0}")]
    CreateMnemonicError(String),
    
    #[error("创建钱包错误 - 地址派生失败: {0}")]
    CreateAddressError(String),
    
    #[error("导入钱包错误 - 无效助记词")]
    ImportInvalidMnemonic,
    
    #[error("导入钱包错误 - 私钥恢复失败: {0}")]
    ImportPrivateKeyError(String),
    
    #[error("加密错误: {0}")]
    CryptoError(String),
    
    #[error("网络错误: {0}")]
    NetworkError(String),
    
    #[error("余额不足")]
    InsufficientBalance,

    #[error("生成BIP84地址错误: {0}")]
    GenerateBip84AddressError(String),

    #[error("交易错误 - 无效地址: {0}")]
    InvalidAddress(String),
    
    #[error("交易错误 - 签名失败: {0}")]
    SignatureError(String),
    
    #[error("交易错误 - 广播失败: {0}")]
    TransactionBroadcastError(String),
    
    #[error("交易错误 - 状态更新失败: {0}")]
    TransactionStatusError(String),
    
    #[error("交易错误 - 超时")]
    TransactionTimeout,
    
    #[error("交易错误 - 被回滚")]
    TransactionRollback,

    #[error("金额解析错误: {0}")]
    AmountParseError(String),

    #[error("金额溢出")]
    AmountOverflow,
    
    #[error("金额下溢")]
    AmountUnderflow,
    
    #[error("除以零")]
    DivisionByZero,
    
    #[error("除法错误")]
    DivisionError,

    #[error("格式错误")]
    InvalidFormat(String),

    #[error("标准化错误")]
    NormalizationError,

    #[error("小数位错误")]
    InvalidDecimalPlaces,

    #[error("手续费率错误")]
    InvalidFeeRate,
}

pub type WalletResult<T> = Result<T, WalletError>;

// 实现日志记录
impl WalletError {
    pub fn log(&self) {
        match self {
            Self::CreatePrivateKeyError(msg) => {
                error!(error = %self, "钱包创建失败 - 私钥生成错误: {}", msg)
            },
            Self::CreateMnemonicError(msg) => {
                error!(error = %self, "钱包创建失败 - 助记词生成错误: {}", msg)
            },
            Self::DeriveAddressError(msg) => {
                error!(error = %self, "钱包创建失败 - 地址派生错误: {}", msg)
            },
            Self::ImportInvalidMnemonic => {
                error!(error = %self, "钱包导入失败 - 无效的助记词")
            },
            Self::ImportPrivateKeyError(msg) => {
                error!(error = %self, "钱包导入失败 - 私钥恢复错误: {}", msg)
            },
            Self::CryptoError(msg) => {
                error!(error = %self, "加密操作失败: {}", msg)
            },
            Self::NetworkError(msg) => {
                error!(error = %self, "网络操作失败: {}", msg)
            },
            Self::InsufficientBalance => {
                error!(error = %self, "余额不足")
            },
        }
    }
}