use crate::security::crypto::Ed25519Crypto;
use libp2p::{
    core::PeerId,
    kad::record::Key,
    swarm::{NetworkBehaviour,Swarm},
};
use crate::network::p2p::faic_network; 
use crate::error::wallet::{WalletError, WalletResult};
use std::sync::Arc;
use std::str::FromStr;
use tracing::{info, warn, error};

/// BIP相关
use bip39::{Mnemonic, Language};
use bip32::{ExtendedPrivKey, DerivationPath};
use ed25519_dalek::{Keypair, SecretKey};
use sha2::{Sha256, Digest};
use bitcoin::bip84;
use bitcoin::network::constants::Network;

/// HD钱包服务实现
pub struct WalletService {
    crypto: Arc<Ed25519Crypto>,
    network: Arc<faic_network>,
}

impl WalletService {
    pub fn new(crypto: Arc<Ed25519Crypto>, network: Arc<faic_network>) -> Self {
        info!("初始化HD钱包服务");
        Self { crypto, network }
    }

    /// 创建新钱包
    /// 实现 BIP32/39/44/47/84 标准的HD钱包
    pub async fn create_wallet(&self) -> WalletResult<WalletInfo> {
        info!("开始创建新HD钱包");

        // 生成私钥
        let private_key = self.crypto.generate_private_key()?;
        // 生成助记词
        let mnemonic = self.crypto.generate_mnemonic()?;
        // 创建钱包地址
        let address = self.crypto.derive_address(&private_key)?;
        // 生成 BIP84 原生隔离见证地址
        let segwit_address = self.generate_bip84_address(0)?;
        
        info!("HD钱包创建成功: {}", address);
        info!("原生隔离见证地址: {}", segwit_address);


        Ok(WalletInfo {
            address,
            mnemonic,
            private_key,
            segwit_address, // 添加 BIP84 地址
        })
    }

    /// 从助记词导入钱包
    pub async fn import_wallet(&self, mnemonic: String) -> WalletResult<WalletInfo> {
        info!("开始从助记词导入钱包");

        // 验证助记词
        if !self.crypto.verify_mnemonic(&mnemonic)? {
            return Err(WalletError::ImportInvalidMnemonic);
        }
        // 从助记词恢复私钥
        let private_key = self.crypto.recover_private_key(&mnemonic)?;
        // 重建钱包地址
        let address = self.crypto.derive_address(&private_key)?;
        // 重建 BIP84 原生隔离见证地址
        let segwit_address = self.generate_bip84_address(0)?;

        info!("钱包导入成功: {}", address);
        info!("原生隔离见证地址: {}", segwit_address);

        Ok(WalletInfo {
            address,
            mnemonic,
            private_key,
            segwit_address, // 添加 BIP84 地址
        })
    }

    /// 生成 BIP84 原生隔离见证地址
    pub fn generate_bip84_address(&self, index: u32) -> WalletResult<String> {
        info!("生成 BIP84 原生隔离见证地址, index: {}", index);
        
        // BIP84 使用路径 m/84'/0'/0'/0/*
        let path = format!("m/84'/0'/0'/0/{}", index);
        let derivation_path = DerivationPath::from_str(&path)
            .map_err(|e| WalletError::CryptoError(e.to_string()))?;
            
        // 使用 Ed25519Crypto 中的方法生成地址
        let address = self.crypto.derive_bip84_address(&derivation_path)?;
        
        info!("BIP84 地址生成成功: {}", address);
        Ok(address)
    }

    pub async fn get_balance(&self, address: &str) -> WalletResult<Balance> {
        info!("查询地址余额: {}", address);
        
        // 通过网络获取账户状态
        let account_state = self.network.get_account_state(address)
            .await
            .map_err(|e| WalletError::NetworkError(e.to_string()))?;
            
        Ok(Balance {
            total: account_state.balance,
            available: account_state.available,
            locked: account_state.locked,
        })
    }
}

#[derive(Debug, Clone)]
pub struct WalletInfo {
    pub address: String,
    pub mnemonic: String,
    pub private_key: String,
    pub segwit_address: String, // 添加 BIP84 隔离见证地址
}

#[derive(Debug, Clone)]
pub struct Balance {
    /// 总余额
    pub total: Amount,
    /// 可用余额
    pub available: Amount,
    /// 锁定余额
    pub locked: Amount,
}