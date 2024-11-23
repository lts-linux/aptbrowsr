-- Your SQL goes here
CREATE TABLE distros (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  repo_url TEXT NOT NULL,
  name_or_path TEXT NOT NULL,
  repo_key TEXT,
  armored_key BOOLEAN NOT NULL DEFAULT 1,
  flat_repo BOOLEAN NOT NULL DEFAULT 0
)
