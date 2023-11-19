-- This file should undo anything in `up.sql`
ALTER TABLE messages ALTER COLUMN sender_uid DROP NOT NULL;