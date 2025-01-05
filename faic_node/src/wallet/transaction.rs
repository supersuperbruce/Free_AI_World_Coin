use crate::network::p2p::faic_network;
use crate::security::crypto::Ed25519Crypto;
use crate::error::wallet::{WalletError, WalletResult};
use std::sync::Arc;
use tracing::{info, warn, error};

/// 交易服务实现
pub struct TransactionService {
    network: Arc<FAICNetwork>,
    crypto: Arc<Ed25519Crypto>,  // 添加加密服务
}

#[derive(Debug, Clone)]
pub struct TransactionInfo {
    pub tx_hash: String,
    pub from: String,
    pub to: String,
    pub amount: Amount,
    pub fee: Amount,
    pub timestamp: u64,
    pub status: TransactionStatus,
}

#[derive(Debug, Clone)]
pub enum TransactionStatus {
    /// 交易已创建但尚未广播
    Created,
    /// 交易已提交到网络但尚未被打包
    Pending,
    /// 交易已被打包但尚未达到确认数
    Confirming(u32),  // 包含确认数
    /// 交易已完全确认
    Confirmed,
    /// 交易被回滚
    Rollback,
    /// 交易失败
    Failed(String),   // 包含失败原因
    /// 交易超时
    Timeout,
}

impl TransactionService {
    pub fn new(network: Arc<faic_network>, crypto: Arc<Ed25519Crypto>) -> Self {
        info!("初始化交易服务");
        Self { network, crypto }
    }

    /// 发送交易
    pub async fn send_transaction(&self, from: &str, to: &str, amount: u64, private_key: &str) -> WalletResult<TransactionInfo> {
        info!("发起交易: from={}, to={}, amount={}", from, to, amount);
        
        // 验证地址格式
        if !self.crypto.verify_address(to) {
        return Err(WalletError::InvalidAddress(to.to_string()));
        }

        // 检查余额
        let balance = self.network.get_account_state(from)
            .await
            .map_err(|e| WalletError::NetworkError(e.to_string()))?;

        if balance.available < amount {
            return Err(WalletError::InsufficientBalance);
        }
        // 签名交易
        let signature = self.crypto.sign_transaction(from, to, amount, private_key)
            .map_err(|e| WalletError::SignatureError(e.to_string()))?;

        // 构建并广播交易
        let tx = self.network.broadcast_transaction(from, to, amount, signature)
            .await
            .map_err(|e| WalletError::TransactionBroadcastError(e.to_string()))?;
            
        info!("交易已广播: {}", tx.tx_hash);
        
        Ok(TransactionInfo {
            tx_hash: tx.tx_hash,
            from: from.to_string(),
            to: to.to_string(),
            amount,
            fee: tx.fee,
            timestamp: tx.timestamp,
            status: TransactionStatus::Pending,
        })
    }

    /// 查询交易状态
    pub async fn get_transaction_status(&self, tx_hash: &str) -> WalletResult<TransactionStatus> {
        info!("查询交易状态: {}", tx_hash);
        
        let status = self.network.get_transaction_status(tx_hash)
            .await
            .map_err(|e| WalletError::NetworkError(e.to_string()))?;
            
        Ok(status)
    }
}