-- Your SQL goes here
CREATE TABLE IF NOT EXISTS services (
  "uid" UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
  "law_uid" UUID NOT NULL REFERENCES user_profiles("uid"),
  "name" VARCHAR(255) NOT NULL,
  "cost" DOUBLE PRECISION NOT NULL
);
