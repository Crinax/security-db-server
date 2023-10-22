-- Your SQL goes here
CREATE TABLE IF NOT EXISTS law_profiles (
  "uid" UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
  "itn" VARCHAR(15) NOT NULL,
  "start_activity_date" TIMESTAMP NOT NULL
);
