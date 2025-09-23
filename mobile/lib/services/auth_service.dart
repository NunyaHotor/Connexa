
import 'dart:async';

class AuthService {
  static final AuthService _instance = AuthService._internal();
  factory AuthService() => _instance;

  AuthService._internal();

  String? _token;

  String? get token => _token;

  Future<bool> login(String email, String password) async {
    // In a real app, you would make an HTTP request to your auth server
    // For now, we'll just simulate a successful login and create a dummy token
    if (email == 'test@example.com' && password == 'password') {
      _token = 'dummy_token_for_${email}';
      return true;
    } else {
      return false;
    }
  }

  void logout() {
    _token = null;
  }
}
