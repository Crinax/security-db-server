-- This file should undo anything in `up.sql`
ALTER TABLE user_profiles ALTER COLUMN role DROP DEFAULT;
