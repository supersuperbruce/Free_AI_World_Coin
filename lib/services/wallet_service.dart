import 'package:bip39/bip39.dart' as bip39;
import 'package:web3dart/web3dart.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:convert/convert.dart';
import '../models/wallet.dart';
import 'package:flutter/foundation.dart';
import '../models/token.dart';

class WalletService extends ChangeNotifier {
  final FlutterSecureStorage _secureStorage = const FlutterSecureStorage();
  static const String _mnemonicKey = 'wallet_mnemonic';
  static const String _privateKeyKey = 'wallet_private_key';
  WalletModel? _wallet;

  WalletModel? get wallet => _wallet;

  // 添加 getter
  String? get currentAddress => _wallet?.address;

  // 创建新钱包
  Future<WalletModel> createWallet() async {
    // 生成随机助记词
    final mnemonic = bip39.generateMnemonic();
    // 从助记词生成私钥种子
    final seed = bip39.mnemonicToSeed(mnemonic);
    // 创建凭证
    final credentials = EthPrivateKey.fromHex(hex.encode(seed.sublist(0, 32)));
    // 获取钱包地址
    final address = credentials.address;
    // 创建钱包实例
    final wallet = WalletModel(
      address: address.hex,
      privateKey: credentials.privateKeyInt.toRadixString(16),
      mnemonic: mnemonic,
      balance: 0.0,
    );

    // 安全存储助记词和私钥
    await _secureStorage.write(key: _mnemonicKey, value: mnemonic);
    await _secureStorage.write(
        key: _privateKeyKey,
        value: credentials.privateKeyInt.toRadixString(16));

    return wallet;
  }

  // 检查是否已存在钱包
  Future<bool> hasWallet() async {
    final mnemonic = await _secureStorage.read(key: _mnemonicKey);
    return mnemonic != null;
  }

  // 获取当前钱包
  Future<WalletModel?> getCurrentWallet() async {
    final mnemonic = await _secureStorage.read(key: _mnemonicKey);
    final privateKey = await _secureStorage.read(key: _privateKeyKey);

    if (mnemonic == null || privateKey == null) {
      return null;
    }

    final credentials = EthPrivateKey.fromHex(privateKey);
    final address = credentials.address;

    return WalletModel(
      address: address.hex,
      privateKey: privateKey,
      mnemonic: mnemonic,
    );
  }

  // 添加通知方法
  void notifyWalletChanged() {
    notifyListeners();
  }

  // 检查是否存在钱包
  Future<bool> hasExistingWallet() async {
    final mnemonic = await _secureStorage.read(key: 'mnemonic');
    return mnemonic != null;
  }

  // 解锁钱包
  Future<void> unlockWallet(String password) async {
    final mnemonic = await _secureStorage.read(key: _mnemonicKey);
    final privateKey = await _secureStorage.read(key: _privateKeyKey);

    if (mnemonic == null || privateKey == null) {
      throw Exception('钱包不存在');
    }

    // TODO: 实现密码验证逻辑
    final storedPassword = await _secureStorage.read(key: 'password');
    if (password != storedPassword) {
      throw Exception('密码错误');
    }

    // 从私钥重新创建凭证
    final credentials = EthPrivateKey.fromHex(privateKey);
    final address = credentials.address;

    // 解锁成功后加载钱包
    _wallet = WalletModel(
      address: address.hex,
      privateKey: privateKey,
      mnemonic: mnemonic,
      balance: 0.0, // 初始余额设为0
    );
    notifyListeners();
  }

  // 重置钱包
  Future<void> resetWallet() async {
    await _secureStorage.deleteAll();
    _wallet = null;
    notifyListeners();
  }

  Future<WalletModel> importWalletFromMnemonic(String mnemonic) async {
    if (!bip39.validateMnemonic(mnemonic)) {
      throw Exception('无效的助记词');
    }

    // 从助记词生成私钥种子
    final seed = bip39.mnemonicToSeed(mnemonic);
    // 创建凭证
    final credentials = EthPrivateKey.fromHex(hex.encode(seed.sublist(0, 32)));
    // 获取钱包地址
    final address = credentials.address;

    // 创建钱包实例
    final wallet = WalletModel(
      address: address.hex,
      privateKey: credentials.privateKeyInt.toRadixString(16),
      mnemonic: mnemonic,
      balance: 0.0,
    );

    // 安全存储助记词和私钥
    await _secureStorage.write(key: _mnemonicKey, value: mnemonic);
    await _secureStorage.write(
      key: _privateKeyKey,
      value: credentials.privateKeyInt.toRadixString(16),
    );

    return wallet;
  }

  Future<void> lockWallet() async {
    _wallet = null;
    notifyListeners();
  }

  Future<void> swapTokens({
    required Token fromToken,
    required Token toToken,
    required double amount,
  }) async {
    // TODO: 实现代币兑换逻辑
    // 1. 检查代币余额
    // 2. 获取兑换比率
    // 3. 调用智能合约执行兑换
    // 4. 更新本地余额
    notifyListeners();
  }

  final List<Token> _supportedTokens = [
    const Token(
      symbol: 'FAIC',
      name: 'Free AI Chain',
      address: '0x...', // TODO: 添加实际合约地址
      decimals: 18,
    ),
    // 可以添加更多支持的代币
  ];

  List<Token> get supportedTokens => List.unmodifiable(_supportedTokens);

  Future<List<Token>> getTokenBalances() async {
    if (_wallet == null) return [];
    // TODO: 从区块链获取实际余额
    return _supportedTokens;
  }
}
