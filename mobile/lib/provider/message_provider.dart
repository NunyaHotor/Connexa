import 'dart:async';
import 'package:flutter/material.dart';
import '../screens/chat_screen.dart';
import '../services/websocket_service.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import '../services/auth_service.dart';
import 'dart:typed_data';

class MessageProvider extends ChangeNotifier {
  final WebSocketService _webSocketService = WebSocketService();
  List<Message> _messages = [];
  StreamSubscription? _socketSubscription;
  Timer? _pruneTimer;

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

    // Periodic pruning of locally expired messages
    _pruneTimer = Timer.periodic(const Duration(seconds: 1), (_) {
      final now = DateTime.now();
      final before = _messages.length;
      _messages.removeWhere((m) => m.expiresAt != null && m.expiresAt!.isBefore(now));
      if (_messages.length != before) {
        notifyListeners();
      }
    });
  }

  void connect(String url) {
    _webSocketService.connect(url);
  }

  Future<void> sendMessage(String text, {int? ttlSeconds, String? httpBase = 'http://127.0.0.1:3000'}) async {
    final now = DateTime.now();
    final expiresAt = (ttlSeconds != null && ttlSeconds > 0)
        ? now.add(Duration(seconds: ttlSeconds))
        : null;
    final message = Message(
      text: text,
      isSentByUser: true,
      timestamp: now,
      expiresAt: expiresAt,
    );
    _messages.add(message);
    notifyListeners();

    // Prefer HTTP API to support ttl_seconds; fallback to WebSocket on failure
    try {
      final token = AuthService().token ?? 'test-user';
      final uri = Uri.parse('$httpBase/message');
      final body = jsonEncode({
        'recipient_user_id': 'test-user', // placeholder: replace with actual recipient
        'plaintext': utf8.encode(text),
        'ttl_seconds': ttlSeconds,
      });
      final resp = await http.post(
        uri,
        headers: {
          'Content-Type': 'application/json',
          'Authorization': 'Bearer $token',
        },
        body: body,
      );
      if (resp.statusCode < 200 || resp.statusCode >= 300) {
        // Fallback to WebSocket if HTTP fails
        _webSocketService.sendMessage(text);
      }
    } catch (_) {
      _webSocketService.sendMessage(text);
    }
  }

  Future<void> sendMedia({
    required Uint8List data,
    required String fileName,
    required String mimeType,
    int? ttlSeconds,
    String? httpBase = 'http://127.0.0.1:3000',
  }) async {
    final now = DateTime.now();
    final expiresAt = (ttlSeconds != null && ttlSeconds > 0)
        ? now.add(Duration(seconds: ttlSeconds))
        : null;

    // Add placeholder local message for immediate feedback
    _messages.add(Message(
      text: '[Media] ' + fileName,
      isSentByUser: true,
      timestamp: now,
      expiresAt: expiresAt,
    ));
    notifyListeners();

    try {
      // Upload media bytes
      final uploadUri = Uri.parse('$httpBase/media/upload');
      final uploadResp = await http.post(
        uploadUri,
        headers: {
          'Content-Type': mimeType,
        },
        body: data,
      );
      if (uploadResp.statusCode < 200 || uploadResp.statusCode >= 300) {
        return; // upload failed
      }
      final hash = uploadResp.body; // server returns plain hash string

      // Compose a media payload (temporary JSON format)
      final mediaPayload = jsonEncode({
        'type': 'media',
        'file_name': fileName,
        'mime_type': mimeType,
        'file_size': data.length,
        'media_url': hash,
      });

      // Send media message with TTL
      final token = AuthService().token ?? 'test-user';
      final sendUri = Uri.parse('$httpBase/message');
      final sendBody = jsonEncode({
        'recipient_user_id': 'test-user', // TODO: replace with actual recipient id
        'plaintext': utf8.encode(mediaPayload),
        'ttl_seconds': ttlSeconds,
      });
      final sendResp = await http.post(
        sendUri,
        headers: {
          'Content-Type': 'application/json',
          'Authorization': 'Bearer $token',
        },
        body: sendBody,
      );
      if (sendResp.statusCode < 200 || sendResp.statusCode >= 300) {
        // optionally handle send failure
      }
    } catch (e) {
      // optionally log
      // print('sendMedia error: $e');
    }
  }

  @override
  void dispose() {
    _socketSubscription?.cancel();
    _pruneTimer?.cancel();
    _webSocketService.disconnect();
    super.dispose();
  }
}