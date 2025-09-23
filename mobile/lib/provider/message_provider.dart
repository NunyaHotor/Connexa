import 'dart:async';
import 'package:flutter/material.dart';
import '../screens/chat_screen.dart';
import '../services/websocket_service.dart';

class MessageProvider extends ChangeNotifier {
  final WebSocketService _webSocketService = WebSocketService();
  List<Message> _messages = [];
  StreamSubscription? _socketSubscription;

  List<Message> get messages => _messages;

  MessageProvider() {
    _socketSubscription = _webSocketService.stream.listen((message) {
      final incomingMessage = Message(
        text: message,
        isSentByUser: false, // Assume incoming messages are from the other user
        timestamp: DateTime.now(),
      );
      _messages.add(incomingMessage);
      notifyListeners();
    }, onError: (error) {
      // Handle WebSocket errors
      print('WebSocket Error: $error');
    });
  }

  void connect(String url) {
    _webSocketService.connect(url);
  }

  void sendMessage(String text) {
    final message = Message(
      text: text,
      isSentByUser: true,
      timestamp: DateTime.now(),
    );
    _messages.add(message);
    _webSocketService.sendMessage(text);
    notifyListeners();
  }

  @override
  void dispose() {
    _socketSubscription?.cancel();
    _webSocketService.disconnect();
    super.dispose();
  }
}