-- Your SQL goes here
CREATE TABLE IF NOT EXISTS files (
  "uid" UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
  "file_name" CHAR(36) NOT NULL,
  "original_name" VARCHAR NOT NULL
);
