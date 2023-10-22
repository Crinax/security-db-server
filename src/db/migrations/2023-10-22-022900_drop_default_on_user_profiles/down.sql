-- This file should undo anything in `up.sql`
ALTER TABLE user_profiles ALTER COLUMN role SET DEFAULT 0;
