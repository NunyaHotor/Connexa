import 'package:web_socket_channel/web_socket_channel.dart';
import 'dart:async';
import 'auth_service.dart';

class WebSocketService {
  static final WebSocketService _instance = WebSocketService._internal();
  factory WebSocketService() => _instance;

  final AuthService _authService = AuthService();
  WebSocketChannel? _channel;
  StreamController<String> _streamController = StreamController.broadcast();
  bool _isConnected = false;

  // Private constructor
  WebSocketService._internal();

  Stream<String> get stream => _streamController.stream;

  void connect(String url) {
    if (_isConnected) return;

    final token = _authService.token;
    if (token == null) {
      final error = '[WebSocket] Cannot connect without an authentication token.';
      _streamController.addError(error);
      throw Exception(error);
    }

    final uri = Uri.parse(url).replace(queryParameters: {'token': token});

    try {
      _channel = WebSocketChannel.connect(uri);
      _isConnected = true;
      _channel!.stream.listen(
        (data) {
          _streamController.add(data);
        },
        onDone: () {
          _isConnected = false;
          // You might want to add automatic reconnection logic here
        },
        onError: (error) {
          _isConnected = false;
          _streamController.addError(error);
        },
      );
    } catch (e) {
      _streamController.addError(e);
    }
  }

  void sendMessage(String message) {
    if (_isConnected && _channel != null) {
      _channel!.sink.add(message);
    } else {
      throw Exception('WebSocket is not connected.');
    }
  }

  void disconnect() {
    if (_isConnected && _channel != null) {
      _channel!.sink.close();
      _isConnected = false;
    }
  }
}