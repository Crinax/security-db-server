-- This file should undo anything in `up.sql`
ALTER TABLE auth_data ALTER COLUMN password TYPE CHAR(32);
