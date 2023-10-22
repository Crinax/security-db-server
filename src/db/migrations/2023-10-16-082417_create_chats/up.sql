-- Your SQL goes here
CREATE TABLE IF NOT EXISTS chats (
  "uid" UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
  "creator_uid" UUID NOT NULL REFERENCES user_profiles("uid") ON DELETE NO ACTION,
  "name" VARCHAR(255) NOT NULL,
  "connection_hash" VARCHAR(64) NOT NULL UNIQUE
);
