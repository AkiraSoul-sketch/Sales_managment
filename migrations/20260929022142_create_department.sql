CREATE TABLE IF NOT EXISTS department (
    id                   BLOB PRIMARY KEY NOT NULL,
    name                 VARCHAR NOT NULL,
    Head_of_Department   VARCHAR NOT NULL,
    number               REAL NOT NULL,
    volume               REAL NOT NULL,
    created_at           TEXT NOT NULL,
    updated_at           TEXT,
    deleted_at           TEXT
);
