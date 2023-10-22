-- Your SQL goes here
CREATE TABLE IF NOT EXISTS message_files (
  "uid" UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
  "message_uid" UUID NOT NULL REFERENCES messages("uid") ON DELETE CASCADE,
  "file_uid" UUID NOT NULL REFERENCES files("uid") ON DELETE CASCADE
);
