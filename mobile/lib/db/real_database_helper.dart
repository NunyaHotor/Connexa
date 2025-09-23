
import 'package:sqflite/sqflite.dart';
import 'package:path/path.dart';
import '../screens/contact_screen.dart'; // Assuming Contact class is defined here
import 'database.dart';

class RealDatabaseHelper {
  static final RealDatabaseHelper instance = RealDatabaseHelper._privateConstructor();
  RealDatabaseHelper._privateConstructor();

  final DatabaseHelper _dbHelper = DatabaseHelper.instance;

  Future<List<Contact>> queryAllContacts() async {
    final db = await _dbHelper.database;
    final List<Map<String, dynamic>> maps = await db.query('contacts');
    return List.generate(maps.length, (i) {
      return Contact(
        id: maps[i]['id'],
        name: maps[i]['name'],
        initial: maps[i]['initial'],
      );
    });
  }

  Future<int> insertContact(Contact contact) async {
    final db = await _dbHelper.database;
    return await db.insert('contacts', contact.toMap());
  }

  Future<int> deleteContact(int id) async {
    final db = await _dbHelper.database;
    return await db.delete('contacts', where: 'id = ?', whereArgs: [id]);
  }
}
