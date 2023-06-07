-- Your SQL goes here
CREATE TABLE user (
  id INTEGER PRIMARY KEY,
  email TEXT NOT NULL,
  password TEXT NOT NULL,
  name TEXT NOT NULL,
  age INTEGER NOT NULL
);