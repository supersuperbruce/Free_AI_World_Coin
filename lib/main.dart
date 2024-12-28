import 'package:flutter/material.dart';
import 'package:faic_wallet/screens/home_page.dart';

void main() {
  runApp(const FAICWallet());
}

class FAICWallet extends StatelessWidget {
  const FAICWallet({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'FAIC Wallet',
      theme: ThemeData(
        primarySwatch: Colors.blue,
        useMaterial3: true,
      ),
      home: const HomePage(),
    );
  }
}
