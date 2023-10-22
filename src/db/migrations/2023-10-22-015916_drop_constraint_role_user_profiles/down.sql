-- This file should undo anything in `up.sql`
ALTER TABLE user_profiles ADD CONSTRAINT user_profiles_role_check CHECK (role > 0 AND role < 5);
