-- This file should undo anything in `up.sql`
ALTER TABLE user_profiles DROP CONSTRAINT user_profiles_law_profile_key;
ALTER TABLE user_profiles DROP CONSTRAINT user_profiles_passport_uid_key;

ALTER TABLE user_profiles ADD CONSTRAINT user_profiles_law_profile_key UNIQUE NULLS NOT DISTINCT (law_profile);
ALTER TABLE user_profiles ADD CONSTRAINT user_profiles_passport_uid_key UNIQUE NULLS NOT DISTINCT (passport_uid);
