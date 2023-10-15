-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user_profiles (
  "uid" UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
  "passport_uid" UUID REFERENCES passports("uid") ON DELETE CASCADE UNIQUE NULLS NOT DISTINCT,
  "law_profile" UUID REFERENCES law_profiles("uid") ON DELETE SET NULL UNIQUE NULLS NOT DISTINCT,
  "avatar_uid" UUID REFERENCES files("uid") ON DELETE SET NULL,
  "role" SMALLINT NOT NULL CHECK (role > 0 AND role < 5) DEFAULT 0,
  "created_at" TIMESTAMP NOT NULL DEFAULT NOW()
);
