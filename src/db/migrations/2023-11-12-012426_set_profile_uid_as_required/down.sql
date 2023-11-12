-- This file should undo anything in `up.sql`
ALTER TABLE auth_data ALTER COLUMN profile_uid DROP NOT NULL;
