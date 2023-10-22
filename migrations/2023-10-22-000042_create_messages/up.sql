-- Your SQL goes here
CREATE TABLE IF NOT EXISTS messages (
  "uid" UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
  "chat_uid" UUID NOT NULL REFERENCES chats("uid") ON DELETE CASCADE,
  "sender_uid" UUID REFERENCES user_profiles("uid") ON DELETE SET NULL,
  "content" TEXT NOT NULL
);
