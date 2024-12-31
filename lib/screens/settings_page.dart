import 'package:flutter/material.dart';

class SettingsPage extends StatelessWidget {
  const SettingsPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('设置'),
      ),
      body: ListView(
        children: const [
          ListTile(
            leading: Icon(Icons.security),
            title: Text('安全设置'),
            subtitle: Text('密码、备份等'),
          ),
          ListTile(
            leading: Icon(Icons.language),
            title: Text('语言设置'),
          ),
          ListTile(
            leading: Icon(Icons.info),
            title: Text('关于'),
            subtitle: Text('版本信息'),
          ),
        ],
      ),
    );
  }
}