use crate::network::config::NetworkConfigError;
use std::fmt;

/// 定义整个 faic_core 项目的通用错误类型
#[derive(Debug)]
pub enum Error {
    /// 网络配置错误
    NetworkConfig(NetworkConfigError),
    /// 网络错误 (这里可以添加更多具体的网络错误类型)
    Network(String),
    /// IO 错误
    Io(std::io::Error),
    /// 其他错误
    Other(String),
}

// 为 Error 实现 Display trait，用于打印错误信息
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NetworkConfig(e) => write!(f, "Network config error: {}", e),
            Error::Network(e) => write!(f, "Network error: {}", e),
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

// 为 Error 实现 Error trait
impl std::error::Error for Error {}

// 实现从 NetworkConfigError 到 Error 的转换
impl From<NetworkConfigError> for Error {
    fn from(err: NetworkConfigError) -> Self {
        Error::NetworkConfig(err)
    }
}

// 实现从 std::io::Error 到 Error 的转换
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

// 可以在这里添加其他错误类型的转换，例如：
// impl From<WalletError> for Error {
//     fn from(err: WalletError) -> Self {
//         Error::Wallet(err)
//     }
// }
//
// impl From<TransactionError> for Error {
//     fn from(err: TransactionError) -> Self {
//         Error::Transaction(err)
//     }
// }

// 示例：添加一个网络错误类型的转换
impl From<libp2p::TransportError<std::io::Error>> for Error {
    fn from(err: libp2p::TransportError<std::io::Error>) -> Self {
        Error::Network(err.to_string())
    }
}