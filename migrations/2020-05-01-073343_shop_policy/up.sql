-- Your SQL goes here

CREATE table shop_policy(
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  body TEXT NOT NULL,
  title VARCHAR(32) NOT NULL DEFAULT '',
  url VARCHAR(512) NOT NULL,
  )