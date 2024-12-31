import 'package:flutter/material.dart';
import '../../services/wallet_service.dart';
import '../../models/token.dart';

class SwapPage extends StatefulWidget {
  const SwapPage({super.key});

  @override
  State<SwapPage> createState() => _SwapPageState();
}

class _SwapPageState extends State<SwapPage> {
  final _fromAmountController = TextEditingController();
  final _toAmountController = TextEditingController();
  final _walletService = WalletService();

  Token? _fromToken;
  Token? _toToken;
  bool _isLoading = false;

  Future<void> _swap() async {
    if (_fromToken == null || _toToken == null) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('请选择代币')),
      );
      return;
    }

    final amount = double.tryParse(_fromAmountController.text);
    if (amount == null || amount <= 0) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('请输入有效金额')),
      );
      return;
    }

    setState(() => _isLoading = true);

    try {
      await _walletService.swapTokens(
        fromToken: _fromToken!,
        toToken: _toToken!,
        amount: amount,
      );
      if (mounted) {
        Navigator.pop(context);
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('兑换成功')),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('兑换失败: ${e.toString()}')),
        );
      }
    } finally {
      setState(() => _isLoading = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('代币兑换')),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            // 代币选择和金额输入
            _buildTokenInput(
              label: '支付',
              amountController: _fromAmountController,
              selectedToken: _fromToken,
              onTokenSelect: (token) => setState(() => _fromToken = token),
            ),
            const Icon(Icons.arrow_downward, size: 32),
            _buildTokenInput(
              label: '接收',
              amountController: _toAmountController,
              selectedToken: _toToken,
              onTokenSelect: (token) => setState(() => _toToken = token),
            ),
            const Spacer(),
            ElevatedButton(
              onPressed: _isLoading ? null : _swap,
              child: _isLoading
                  ? const CircularProgressIndicator()
                  : const Text('确认兑换'),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildTokenInput({
    required String label,
    required TextEditingController amountController,
    required Token? selectedToken,
    required Function(Token) onTokenSelect,
  }) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(label),
            Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: amountController,
                    keyboardType: TextInputType.number,
                    decoration: const InputDecoration(
                      hintText: '0.0',
                    ),
                  ),
                ),
                TextButton(
                  onPressed: () => _showTokenSelector(onTokenSelect),
                  child: Text(selectedToken?.symbol ?? '选择代币'),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Future<void> _showTokenSelector(Function(Token) onSelect) async {
    // TODO: 实现代币选择器
  }
}
