CREATE TABLE IF NOT EXISTS department (
    id                   TEXT PRIMARY KEY NOT NULL,
    name                 VARCHAR NOT NULL,
    Head_of_Department   VARCHAR NOT NULL,
    number               INTEGER NOT NULL,
    volume               INTEGER NOT NULL,
    created_at           TEXT NOT NULL,
    updated_at           TEXT,
    deleted_at           TEXT
);
