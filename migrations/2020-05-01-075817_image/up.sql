-- Your SQL goes here
CREATE TABLE image(

  id VARCHAR(36) NOT NULL PRIMARY KEY,
  alt_text VARCHAR(32) NOT NULL DEFAULT '',
  url VARCHAR(512) NOT NULL DEFAULT ''
)