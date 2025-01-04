use crate::common::setup;
use tracing::{info, warn, error};
use faic_node::types::Amount;
use std::str::FromStr;

// 新增带日志的测试
test_with_logging!(test_basic_operations_with_logging, {
    info!("开始基本操作测试");
    
    info!("测试零值创建");
    let zero = Amount::zero();
    info!("零值: {}", zero);
    assert_eq!(zero.format_decimals(), "0.00000000");

    info!("测试一个FAIC创建");
    let one_faic = Amount::one_faic().unwrap();
    info!("一个FAIC: {}", one_faic);
    assert_eq!(one_faic.format_decimals(), "1.00000000");

    info!("测试加减乘除运算");
    let a = Amount::from_str("100000000").unwrap(); // 1 FAIC
    let b = Amount::from_str("50000000").unwrap();  // 0.5 FAIC
    info!("数值a: {}, 数值b: {}", a, b);
    
    let add_result = a.checked_add(&b).unwrap();
    info!("加法结果: {}", add_result);
    assert_eq!(add_result.format_decimals(), "1.50000000");

    let sub_result = a.checked_sub(&b).unwrap();
    info!("减法结果: {}", sub_result);
    assert_eq!(sub_result.format_decimals(), "0.50000000");

    let mul_result = a.checked_mul(&b).unwrap();
    info!("乘法结果: {}", mul_result);
    assert_eq!(mul_result.format_decimals(), "0.50000000");

    let div_result = a.checked_div(&b).unwrap();
    info!("除法结果: {}", div_result);
    assert_eq!(div_result.format_decimals(), "2.00000000");
});

test_with_logging!(test_fee_calculation_with_logging, {
    info!("开始手续费计算测试");
    let amount = Amount::from_str("1000000000").unwrap(); // 10 FAIC
    info!("测试金额: {}", amount);
    
    let fee = amount.calculate_fee().unwrap();
    info!("计算的手续费: {}", fee);
    assert_eq!(fee.format_decimals(), "0.03000000"); // 0.3%
});

test_with_logging!(test_minimum_amount_with_logging, {
    info!("开始最小金额测试");
    
    let min = Amount::from_str("1").unwrap(); // 0.00000001 FAIC
    info!("最小金额: {}", min);
    assert!(min.validate_minimum_amount());
    
    let zero = Amount::zero();
    info!("零值金额: {}", zero);
    assert!(!zero.validate_minimum_amount());
});

test_with_logging!(test_usdt_conversion_with_logging, {
    info!("开始USDT转换测试");
    
    let amount = Amount::from_str("1000000000").unwrap(); // 10 FAIC
    info!("FAIC金额: {}", amount);
    
    let usdt_rate = Amount::from_str("1381").unwrap(); // 0.0001381 USDT/FAIC
    info!("USDT汇率: {}", usdt_rate);
    
    let usdt_value = amount.to_usdt(&usdt_rate).unwrap();
    info!("转换后的USDT值: {}", usdt_value);
    assert_eq!(usdt_value.format_decimals(), "0.001381");
});