import 'package:flutter/material.dart';
import '../../services/wallet_service.dart';
import '../../models/token.dart';

class SendPage extends StatefulWidget {
  const SendPage({super.key});

  @override
  State<SendPage> createState() => _SendPageState();
}

class _SendPageState extends State<SendPage> {
  final _addressController = TextEditingController();
  final _amountController = TextEditingController();
  final _walletService = WalletService();
  Token? _selectedToken;
  bool _isLoading = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('发送')),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            Card(
              child: Padding(
                padding: const EdgeInsets.all(16),
                child: Column(
                  children: [
                    TextField(
                      controller: _addressController,
                      decoration: const InputDecoration(
                        labelText: '接收地址',
                        hintText: '输入或粘贴地址',
                      ),
                    ),
                    const SizedBox(height: 16),
                    Row(
                      children: [
                        Expanded(
                          child: TextField(
                            controller: _amountController,
                            keyboardType: TextInputType.number,
                            decoration: const InputDecoration(
                              labelText: '金额',
                              hintText: '0.0',
                            ),
                          ),
                        ),
                        TextButton(
                          onPressed: () {
                            // TODO: 实现代币选择
                          },
                          child: Text(_selectedToken?.symbol ?? '选择代币'),
                        ),
                      ],
                    ),
                  ],
                ),
              ),
            ),
            const Spacer(),
            SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                onPressed: _isLoading ? null : _send,
                child: _isLoading
                    ? const CircularProgressIndicator()
                    : const Text('确认发送'),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Future<void> _send() async {
    // TODO: 实现发送逻辑
  }
}