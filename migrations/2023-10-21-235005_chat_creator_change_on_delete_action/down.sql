-- This file should undo anything in `up.sql`
ALTER TABLE chats
DROP CONSTRAINT chats_creator_uid_fkey,
ADD CONSTRAINT chats_creator_uid_fkey
  FOREIGN KEY ("creator_uid")
  REFERENCES user_profiles("uid")
  ON DELETE NO ACTION;
