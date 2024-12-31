import 'package:flutter/material.dart';
import '../../services/wallet_service.dart';
import 'package:qr_flutter/qr_flutter.dart';

class ReceivePage extends StatelessWidget {
  final WalletService _walletService = WalletService();

  ReceivePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('接收')),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            Card(
              child: Padding(
                padding: const EdgeInsets.all(16),
                child: Column(
                  children: [
                    const Text(
                      '钱包地址',
                      style: TextStyle(fontSize: 16),
                    ),
                    const SizedBox(height: 8),
                    Text(
                      _walletService.currentAddress ?? '未获取到地址',
                      style: const TextStyle(
                        fontSize: 14,
                        color: Colors.grey,
                      ),
                    ),
                    const SizedBox(height: 16),
                    if (_walletService.currentAddress != null)
                      QrImageView(
                        data: _walletService.currentAddress!,
                        size: 200,
                      ),
                    const SizedBox(height: 16),
                    ElevatedButton.icon(
                      onPressed: () {
                        // TODO: 实现复制地址功能
                      },
                      icon: const Icon(Icons.copy),
                      label: const Text('复制地址'),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}