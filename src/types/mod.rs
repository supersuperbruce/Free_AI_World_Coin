
use lazy_static::lazy_static;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

lazy_static! {
    /// 1 FAIC = 10^8
    pub static ref ONE_FAIC: BigUint = BigUint::from(100_000_000u64);
    /// 最大数量: 2^128 - 1
    pub static ref MAX_AMOUNT: BigUint = BigUint::parse_bytes(b"340282366920938463463374607431768211455", 10).unwrap();
}

/// Amount 数据类型

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Amount {
    value: BigUint,
}

impl Amount {
    /// 最小单位: 1 (0.00000001 FAIC), 实际精度: 8位小数。参考来源doge
    pub const DECIMALS: u64 = 8;

    /// 从 BigUint 创建 Amount
    pub fn from_biguint(value: BigUint) -> Result<Self, &'static str> {
        if value > *MAX_AMOUNT {
            return Err("Amount exceeds maximum value");
        }
        Ok(Amount { value })
    }

    /// 从字符串创建 Amount
    pub fn from_str(value: &str) -> Result<Self, &'static str> {
        let parsed_value = match BigUint::from_str(value) {
            Ok(v) => v,
            Err(_) => return Err("Invalid amount string"),
        };
        Self::from_biguint(parsed_value)
    }

    /// 获取 Amount 的值
    pub fn value(&self) -> &BigUint {
        &self.value
    }

    /// 将 Amount 转换为字符串，包含八位小数
    pub fn to_string(&self) -> String {
        let value_str = self.value.to_string();
        let len = value_str.len();

        if len <= Self::DECIMALS as usize {
            // 如果数值小于 1 FAIC，需要在前面补零
            format!("0.{:0>8}", value_str)
        } else {
            // 插入小数点
            let (integer_part, decimal_part) = value_str.split_at(len - Self::DECIMALS as usize);
            format!("{}.{}", integer_part, decimal_part)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_from_biguint() {
        let amount = Amount::from_biguint(BigUint::from(100u32)).unwrap();
        assert_eq!(amount.value(), &BigUint::from(100u32));

        let max_amount = Amount::from_biguint(MAX_AMOUNT.clone()).unwrap();
        assert_eq!(max_amount.value(), &*MAX_AMOUNT);

        let invalid_amount = Amount::from_biguint(MAX_AMOUNT.clone() + BigUint::from(1u32));
        assert!(invalid_amount.is_err());

    }

    #[test]
    fn test_amount_from_str() {
        let amount = Amount::from_str("100").unwrap();
        assert_eq!(amount.value(), &BigUint::from(100u32));

        let invalid_amount = Amount::from_str("abc");
        assert!(invalid_amount.is_err());

    }

    #[test]
    fn test_amount_to_string() {
        // 测试数值小于 1 FAIC 的情况
        let amount = Amount::from_biguint(BigUint::from(1234567u64)).unwrap();
        assert_eq!(amount.to_string(), "0.01234567");

        // 测试数值大于 1 FAIC 的情况
        let amount = Amount::from_biguint(BigUint::from(123456789u64)).unwrap();
        assert_eq!(amount.to_string(), "1.23456789");

        // 测试数值为整数 FAIC 的情况
        let amount = Amount::from_biguint(BigUint::from(100_000_000u64)).unwrap();
        assert_eq!(amount.to_string(), "1.00000000");
    }    
}