-- Your SQL goes here
CREATE TABLE distros (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  url TEXT NOT NULL,
  name TEXT,
  path TEXT,
  key TEXT,
  armored_key BOOLEAN NOT NULL DEFAULT 1
)
