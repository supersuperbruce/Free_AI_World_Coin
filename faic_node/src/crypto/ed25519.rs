use crate::error::wallet::{WalletError, WalletResult};
use bip39::{Mnemonic, Language, MnemonicType};
use ed25519_dalek::{Keypair, SecretKey, PublicKey, Signer, Verifier};
use sha2::{Sha256, Digest};
use rand::rngs::OsRng;
use bitcoin::{
    network::constants::Network,
    bip32::{ExtendedPrivKey, DerivationPath},
    util::bip32::ChildNumber,
};
use tracing::{info, warn, error};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FAICNetwork {
    Mainnet,
    Testnet,
    Devnet,
}

pub struct Ed25519Crypto {
    // 可以添加必要的字段
    network: FAICNetwork, 
}

impl Ed25519Crypto {
    pub fn new() -> Self {
        info!("初始化 Ed25519 加密服务");
        Self {
            network: FAICNetwork::Mainnet  // 默认使用 FAIC 主网
        }
    }

    pub fn with_network(network: FAICNetwork) -> Self {
        info!("初始化 Ed25519 加密服务，网络类型: {:?}", network);
        Self { network }
    }

    /// 生成新的私钥
    pub fn generate_private_key(&self) -> WalletResult<String> {
        info!("生成新的私钥");
        let mut rng = OsRng{};
        let keypair = Keypair::generate(&mut rng);
        let private_key = hex::encode(keypair.secret.as_bytes());
        Ok(private_key)
    }

    /// 生成新的助记词 (BIP39)
    pub fn generate_mnemonic(&self) -> WalletResult<String> {
        info!("生成新的助记词");
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        Ok(mnemonic.phrase().to_string())
    }


    /// 验证助记词
    pub fn verify_mnemonic(&self, mnemonic: &str) -> WalletResult<bool> {
        info!("验证助记词");
        match Mnemonic::from_phrase(mnemonic, Language::English) {
            Ok(_) => Ok(true),
            Err(e) => {
                warn!("助记词验证失败: {}", e);
                Ok(false)
            }
        }
    }

    /// 从助记词恢复私钥
    pub fn recover_private_key(&self, mnemonic: &str) -> WalletResult<String> {
        info!("从助记词恢复私钥");
        let mnemonic = Mnemonic::from_phrase(mnemonic, Language::English)
            .map_err(|e| WalletError::ImportPrivateKeyError(e.to_string()))?;
        
        let seed = mnemonic.to_seed("");
        let master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)
            .map_err(|e| WalletError::ImportPrivateKeyError(e.to_string()))?;
            
        Ok(hex::encode(master_key.private_key.secret_bytes()))
    }

    /// 从私钥派生地址
    pub fn derive_address(&self, private_key: &str) -> WalletResult<String> {
        info!("从私钥派生地址");
        let private_bytes = hex::decode(private_key)
            .map_err(|e| WalletError::CreateAddressError(e.to_string()))?;
            
        let secret_key = SecretKey::from_bytes(&private_bytes)
            .map_err(|e| WalletError::CreateAddressError(e.to_string()))?;
            
        let public_key = PublicKey::from(&secret_key);
        
        // 使用 FAIC 的地址前缀
        let prefix = match self.network {
            FAICNetwork::Mainnet => "FAIC",
            FAICNetwork::Testnet => "tFAIC",
            FAICNetwork::Devnet => "dFAIC",
        };
        
        let address = format!("{}{}", prefix, hex::encode(public_key.as_bytes()));
        Ok(address)
    }

    /// 使用私钥对消息进行签名
    pub fn sign_message(&self, private_key: &str, message: &[u8]) -> WalletResult<Vec<u8>> {
        info!("使用私钥签名消息");
        
        let private_bytes = hex::decode(private_key)
            .map_err(|e| WalletError::SignatureError(e.to_string()))?;
        
        let secret_key = SecretKey::from_bytes(&private_bytes)
            .map_err(|e| WalletError::SignatureError(e.to_string()))?;
        
    let keypair = Keypair {
            secret: secret_key,
            public: PublicKey::from(&secret_key),
        };
        
        Ok(keypair.sign(message).to_bytes().to_vec())
    }
        /// 验证签名
    pub fn verify_signature(&self, public_key: &str, message: &[u8], signature: &[u8]) -> WalletResult<bool> {
        let public_bytes = hex::decode(public_key)
            .map_err(|e| WalletError::SignatureError(e.to_string()))?;
            
        let public_key = PublicKey::from_bytes(&public_bytes)
            .map_err(|e| WalletError::SignatureError(e.to_string()))?;
            
        let signature = ed25519_dalek::Signature::from_bytes(signature)
            .map_err(|e| WalletError::SignatureError(e.to_string()))?;
            
        Ok(public_key.verify(message, &signature).is_ok())
    }

    /// 派生 BIP84 原生隔离见证地址
    /// 实现 BIP84 标准的原生隔离见证地址派生
    /// path 格式: m/84'/0'/0'/0/* (参考 BIP84 规范)
    pub fn derive_bip84_address(&self, path: &DerivationPath) -> WalletResult<String> {
        info!("派生 BIP84 原生隔离见证地址, path: {}", path);
        
        // 1. 获取主私钥种子
        let mnemonic = self.generate_mnemonic()?;
        let seed = Mnemonic::from_phrase(&mnemonic, Language::English)
            .map_err(|e| WalletError::GenerateBip84AddressError(e.to_string()))?
        .to_seed("");

        // 2. 创建主密钥
        let master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)
            .map_err(|e| WalletError::GenerateBip84AddressError(e.to_string()))?;

        // 3. 派生子密钥
        let child_key = master_key
            .derive_priv(&bitcoin::secp256k1::Secp256k1::new(), path)
            .map_err(|e| WalletError::GenerateBip84AddressError(e.to_string()))?;

        // 4. 生成公钥
        let public_key = child_key.public_key();

        // 5. 根据网络类型生成地址前缀
        let prefix = match self.network {
            FAICNetwork::Mainnet => "faic1",  // FAIC 原生隔离见证地址前缀
            FAICNetwork::Testnet => "tfaic1",
            FAICNetwork::Devnet => "dfaic1",
        };

        // 6. 生成 bech32 地址
        let address = bech32::encode(
               prefix,
            public_key.to_bytes().to_base32(),
            bitcoin::bech32::Variant::Bech32
        ).map_err(|e| WalletError::GenerateBip84AddressError(e.to_string()))?;

        info!("BIP84 地址派生成功: {}", address);
        Ok(address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_recover() -> WalletResult<()> {
        let crypto = Ed25519Crypto::new();
        
        // 生成助记词
        let mnemonic = crypto.generate_mnemonic()?;
        assert!(crypto.verify_mnemonic(&mnemonic)?);
        
        // 恢复私钥
        let private_key = crypto.recover_private_key(&mnemonic)?;
        assert!(!private_key.is_empty());
        
        // 派生地址
        let address = crypto.derive_address(&private_key)?;
        assert!(address.starts_with("FAIC"));
        
        Ok(())
    }

    #[test]
    fn test_sign_and_verify() -> WalletResult<()> {
        let crypto = Ed25519Crypto::new();
        let private_key = crypto.generate_private_key()?;
        let message = b"test message";
        
        // 签名
        let signature = crypto.sign_message(&private_key, message)?;
        
        // 从私钥派生地址(公钥)
        let address = crypto.derive_address(&private_key)?;
        
        // 验证签名
        assert!(crypto.verify_signature(&address, message, &signature)?);
        
        Ok(())
    }

    #[test]
    fn test_bip84_address() -> WalletResult<()> {
        let crypto = Ed25519Crypto::new();
        
        // 测试主网地址
        let path = DerivationPath::from_str("m/84'/0'/0'/0/0").unwrap();
        let address = crypto.derive_bip84_address(&path)?;
        assert!(address.starts_with("faic1"));
        
        // 测试测试网地址
        let crypto = Ed25519Crypto::with_network(FAICNetwork::Testnet);
        let address = crypto.derive_bip84_address(&path)?;
        assert!(address.starts_with("tfaic1"));
        
        Ok(())
    }
}
