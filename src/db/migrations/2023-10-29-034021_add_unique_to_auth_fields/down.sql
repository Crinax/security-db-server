-- This file should undo anything in `up.sql`
ALTER TABLE auth_data DROP CONSTRAINT auth_data_email_key;
ALTER TABLE auth_data DROP CONSTRAINT auth_data_username_key;
