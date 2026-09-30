CREATE TABLE IF NOT EXISTS product (
    item_number     TEXT PRIMARY KEY NOT NULL,
    name            TEXT NOT NULL,
    unit            INTEGER NOT NULL,
    price           REAL NOT NULL,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT,
    deleted_at      TEXT
);
