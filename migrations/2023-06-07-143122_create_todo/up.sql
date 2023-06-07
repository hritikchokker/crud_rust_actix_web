-- Your SQL goes here
CREATE TABLE todo (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);