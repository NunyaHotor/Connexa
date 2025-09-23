
import 'package:sqflite/sqflite.dart';
import 'package:path/path.dart';

class DatabaseHelper {
  static final DatabaseHelper instance = DatabaseHelper._privateConstructor();
  static Database? _database;

  DatabaseHelper._privateConstructor();

  Future<Database> get database async {
    if (_database != null) return _database!;
    _database = await _initDatabase();
    return _database!;
  }

  Future<Database> _initDatabase() async {
    String path = join(await getDatabasesPath(), 'connexa.db');
    return await openDatabase(
      path,
      version: 1,
      onCreate: _onCreate,
    );
  }

  Future _onCreate(Database db, int version) async {
    await db.execute('''
      CREATE TABLE contacts(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        initial TEXT NOT NULL
      )
    ''');
  }

  // Contact operations
  Future<int> insertContact(Map<String, dynamic> contact) async {
    Database db = await instance.database;
    return await db.insert('contacts', contact);
  }

  Future<List<Map<String, dynamic>>> queryAllContacts() async {
    Database db = await instance.database;
    return await db.query('contacts');
  }

  Future<int> deleteContact(int id) async {
    Database db = await instance.database;
    return await db.delete('contacts', where: 'id = ?', whereArgs: [id]);
  }
}
