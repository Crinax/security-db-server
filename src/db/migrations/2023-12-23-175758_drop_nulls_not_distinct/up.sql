-- Your SQL goes here
ALTER TABLE user_profiles DROP CONSTRAINT user_profiles_law_profile_key;
ALTER TABLE user_profiles DROP CONSTRAINT user_profiles_passport_uid_key;

ALTER TABLE user_profiles ADD CONSTRAINT user_profiles_law_profile_key UNIQUE(law_profile);
ALTER TABLE user_profiles ADD CONSTRAINT user_profiles_passport_uid_key UNIQUE(passport_uid);
