Amount 数据类型：
    基础数据类型：biguint
    DECIMALS: u32 = 8; //最小单位: 1 (0.00000001 FAIC),实际精度:8位小数。采用doge的精度
    ONE_FAIC: &'static str = "100000000"; // 10^8
    MAX_AMOUNT: &'static str = "340282366920938463463374607431768211455"; // 2^128 - 1