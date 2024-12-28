import 'package:flutter/material.dart';
import 'package:faic_wallet/constants/app_constants.dart';

class AboutPage extends StatelessWidget {
  const AboutPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('关于'),
      ),
      body: ListView(
        padding: const EdgeInsets.all(16.0),
        children: [
          const Text(
            'FAIC Wallet',
            style: TextStyle(
              fontSize: 24,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 16),
          Text('版本: ${AppConstants.appVersion}'),
          const SizedBox(height: 24),
          const Text(
            '关于 FAIC',
            style: TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 8),
          const Text(AppConstants.aboutText),
          // ... 其他信息部分
        ],
      ),
    );
  }
}
