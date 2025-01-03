use crate::security::crypto::BasicCrypto;
use crate::network::p2p::BasicNetwork;
use std::sync::Arc;

pub struct WalletService {
    crypto: Arc<BasicCrypto>,
    network: Arc<BasicNetwork>,
}

impl WalletService {
    pub fn new(crypto: Arc<BasicCrypto>, network: Arc<BasicNetwork>) -> Self {
        Self { crypto, network }
    }

    /// 创建新钱包
    pub async fn create_wallet(&self) -> Result<WalletInfo, CreateWalletError> {
        // 生成私钥
        let private_key = self.crypto.generate_private_key()?;
        // 生成助记词
        let mnemonic = self.crypto.generate_mnemonic()?;
        // 创建钱包地址
        let address = self.crypto.derive_address(&private_key)?;

        Ok(WalletInfo {
            address,
            mnemonic,
            private_key,
        })
    }

    /// 从助记词导入钱包
    pub async fn import_wallet(&self, mnemonic: String) -> Result<WalletInfo, ImportWalletError> {
        // 验证助记词
        if !self.crypto.verify_mnemonic(&mnemonic)? {
            return Err(ImportWalletError::InvalidMnemonic);
        }
        // 从助记词恢复私钥
        let private_key = self.crypto.recover_private_key(&mnemonic)?;
        // 重建钱包地址
        let address = self.crypto.derive_address(&private_key)?;

        Ok(WalletInfo {
            address,
            mnemonic,
            private_key,
        })
    }

    /// 查询钱包余额
    pub async fn get_balance(&self, address: &str) -> Result<Balance, GetBalanceError> {
        // 通过网络获取账户状态
        let account_state = self.network.get_account_state(address).await?;
        Ok(account_state.balance)
    }
}

#[derive(Debug, Clone)]
pub struct WalletInfo {
    pub address: String,
    pub mnemonic: String,
    pub private_key: String,
}

#[derive(Debug, Clone)]
pub struct Balance {
    pub total: u64,
    pub available: u64,
    pub locked: u64,
}