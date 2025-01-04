use num_bigint::BigUint;
use num_traits::{Zero, One, CheckedAdd, CheckedSub, CheckedMul, CheckedDiv};
use std::fmt;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use crate::error::wallet::WalletError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Amount(BigUint);

impl Amount {
    pub const DECIMALS: u32 = 8;
    pub const ONE_FAIC: &'static str = "100000000"; // 10的8次方
    pub const MAX_AMOUNT: &'static str = "340282366920938463463374607431768211455"; // 2的128次方减1

    // 创建一个零金额
    pub fn zero() -> Self {
        Self(BigUint::zero())
    }

    // 创建一个表示1 FAIC的金额
    pub fn one_faic() -> Result<Self, WalletError> {
        BigUint::from_str(Self::ONE_FAIC)
            .map(Self)
            .map_err(|_| WalletError::AmountParseError(Self::ONE_FAIC.to_string()))
    }

    // 从字符串创建金额，带溢出检查
    pub fn from_str(s: &str) -> Result<Self, WalletError> {
        let value = BigUint::from_str(s)
            .map_err(|_| WalletError::AmountParseError(s.to_string()))?;

        // 检查是否超过最大值
        let max = BigUint::from_str(Self::MAX_AMOUNT)
            .map_err(|_| WalletError::AmountParseError(Self::MAX_AMOUNT.to_string()))?;
        if value > max {
            return Err(WalletError::AmountOverflow);
        }

        Ok(Self(value))
    }

    // 加法，带溢出检查
    pub fn checked_add(&self, other: &Amount) -> Result<Amount, WalletError> {
        self.0.checked_add(&other.0)
            .map(Amount)
            .ok_or(WalletError::AmountOverflow)
    }

    // 减法，带下溢检查
    pub fn checked_sub(&self, other: &Amount) -> Result<Amount, WalletError> {
        self.0.checked_sub(&other.0)
            .map(Amount)
            .ok_or(WalletError::AmountUnderflow)
    }

    // 乘法，带溢出检查
    pub fn checked_mul(&self, other: &Amount) -> Result<Amount, WalletError> {
        self.0.checked_mul(&other.0)
            .map(Amount)
            .ok_or(WalletError::AmountOverflow)
    }

    // 除法，带小数处理和四舍五入
    pub fn checked_div(&self, other: &Amount) -> Result<Amount, WalletError> {
        if other.0.is_zero() {
            return Err(WalletError::DivisionByZero);
        }

        let divisor = BigUint::from(10u32).pow(Self::DECIMALS);
        let scaled_self = self.0.clone() * divisor;
        let result = scaled_self.checked_div(&other.0)
            .ok_or(WalletError::DivisionError)?;

        // 四舍五入
        let remainder = scaled_self.clone() % &other.0;
        if !remainder.is_zero() {
            let next_digit = (remainder * BigUint::from(10u32))
                .checked_div(&other.0)
                .ok_or(WalletError::DivisionError)?;
            if next_digit >= BigUint::from(5u32) {
                let rounded_result = result.checked_add(&BigUint::one())
                    .ok_or(WalletError::AmountOverflow)?;
                Ok(Amount(rounded_result))
            } else {
                Ok(Amount(result))
            }
        } else {
            Ok(Amount(result))
        }
    }

    // 标准化为8位小数
    pub fn normalize(&self) -> Result<Amount, WalletError> {
        if self.is_normalized() {
            return Ok(self.clone());
        }
    
        let divisor = BigUint::from(10u32).pow(Self::DECIMALS);
        let normalized_value = self.0 / divisor * divisor;
        Ok(Amount(normalized_value))
    }

    // 检查是否已标准化
    pub fn is_normalized(&self) -> bool {
        let divisor = BigUint::from(10u32).pow(Self::DECIMALS);
        (&self.0 % &divisor) == BigUint::zero()
    }

    // 截断超过8位小数
    pub fn truncate_decimals(&self) -> Amount {
        let divisor = BigUint::from(10u32).pow(Self::DECIMALS);
        Amount((&self.0 / &divisor) * &divisor)
    }

    // 格式化为8位小数字符串
    pub fn format_decimals(&self) -> String {
        let s = self.0.to_string();
        let len = s.len();
        if len <= Self::DECIMALS as usize {
            format!("0.{}", &"0".repeat(Self::DECIMALS as usize - len + 1) + &s)
        } else {
            let integer_part = &s[..len - Self::DECIMALS as usize];
            let decimal_part = &s[len - Self::DECIMALS as usize..];
            format!("{}.{}", integer_part, decimal_part)
        }
    }
    // 序列化为字节
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes_be()
    }

    // 从字节反序列化
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, WalletError> {
        if bytes.is_empty() {
            return Err(WalletError::AmountParseError("empty bytes".to_string()));
        }
        Ok(Self(BigUint::from_bytes_be(bytes)))
    }
        // BIP-0021: URI方案
        pub fn to_payment_uri(&self, address: &str) -> String {
            format!("faic:{}?amount={}", address, self.format_decimals())
        }
    
    // 从URI解析金额
    pub fn from_payment_uri(uri: &str) -> Result<(String, Self), WalletError> {
        // faic:address?amount=1.23456789
        let parts: Vec<&str> = uri.split("?amount=").collect();
        if parts.len() != 2 || !parts[0].starts_with("faic:") {
            return Err(WalletError::InvalidFormat("Invalid URI format".to_string()));
        }
        
        let address = parts[0].trim_start_matches("faic:").to_string();
        let amount = Self::from_str(parts[1])?;
        
        Ok((address, amount))
    }
    // 计算手续费 (0.3%)
    pub fn calculate_fee(&self) -> Result<Amount, WalletError> {
        let fee_rate = Amount::from_str("3000")?; // 0.3% = 3000/1000000
        let fee_divisor = Amount::from_str("1000000")?;
        self.checked_mul(&fee_rate)?.checked_div(&fee_divisor)
    }

    // 验证最小交易额 (0.00000001 FAIC = 1)
    pub fn validate_minimum_amount(&self) -> bool {
        self.0 >= BigUint::from(1u32)
    }

    // 计算USDT价格 (初始价格: 0.0001381 USDT/FAIC)
    pub fn to_usdt(&self, usdt_rate: &Amount) -> Result<Amount, WalletError> {
        let rate_divisor = Amount::from_str("10000000")?;
        self.checked_mul(usdt_rate)?.checked_div(&rate_divisor)
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format_decimals())
    }
}