
CREATE TABLE accounts (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  email TEXT NOT NULL UNIQUE,
  bio TEXT,
  avtar TEXT,
  hash TEXT NOT NULL
);