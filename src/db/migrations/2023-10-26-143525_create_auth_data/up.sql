-- Your SQL goes here
CREATE TABLE IF NOT EXISTS auth_data (
  "uid" UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
  "profile_uid" UUID REFERENCES user_profiles("uid") ON DELETE CASCADE,
  "email" VARCHAR(255) NOT NULL,
  "username" VARCHAR(255) NOT NULL,
  "password" VARCHAR(32) NOT NULL
);
