CREATE TABLE IF NOT EXISTS users
(
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);
INSERT INTO users (name)
VALUES ('Alice');
INSERT INTO users (name)
VALUES ('Bob');
INSERT INTO users (name)
VALUES ('Charlie');