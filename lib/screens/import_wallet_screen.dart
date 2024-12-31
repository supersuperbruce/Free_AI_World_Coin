import 'package:flutter/material.dart';
import '../services/wallet_service.dart';
import 'home_screen.dart';

class ImportWalletScreen extends StatefulWidget {
  const ImportWalletScreen({super.key});

  @override
  State<ImportWalletScreen> createState() => _ImportWalletScreenState();
}

class _ImportWalletScreenState extends State<ImportWalletScreen> {
  final _mnemonicController = TextEditingController();
  bool _isLoading = false;
  bool _agreedToTerms = false;

  Future<void> _importWallet() async {
    if (!_agreedToTerms) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('请先同意用户协议')),
      );
      return;
    }

    final mnemonic = _mnemonicController.text.trim();
    if (mnemonic.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('请输入助记词')),
      );
      return;
    }

    setState(() {
      _isLoading = true;
    });

    try {
      // TODO: 实现导入钱包逻辑
      // final wallet = await _walletService.importWallet(mnemonic);

      if (mounted) {
        // 导入成功后直接进入主页
        Navigator.of(context).pushAndRemoveUntil(
          MaterialPageRoute(
            builder: (context) => const HomeScreen(),
          ),
          (route) => false,
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('导入钱包失败: ${e.toString()}')),
        );
      }
    } finally {
      setState(() {
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('导入钱包')),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const Text(
              '通过助记词导入',
              style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 32),
            TextField(
              controller: _mnemonicController,
              maxLines: 3,
              decoration: const InputDecoration(
                labelText: '助记词',
                helperText: '请输入12个助记词，用空格分隔',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 24),
            CheckboxListTile(
              value: _agreedToTerms,
              onChanged: (value) => setState(() => _agreedToTerms = value!),
              title: const Text('我已阅读并同意用户协议'),
            ),
            const SizedBox(height: 32),
            ElevatedButton(
              onPressed: _isLoading ? null : _importWallet,
              child: _isLoading
                  ? const CircularProgressIndicator()
                  : const Text('导入钱包'),
            ),
          ],
        ),
      ),
    );
  }

  @override
  void dispose() {
    _mnemonicController.dispose();
    super.dispose();
  }
}
