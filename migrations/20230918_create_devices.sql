CREATE TABLE devices (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    added_at TEXT NOT NULL,
    verified INTEGER NOT NULL,
    link_token TEXT
);