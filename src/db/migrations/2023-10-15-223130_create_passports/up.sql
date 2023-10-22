-- Your SQL goes here
CREATE TABLE IF NOT EXISTS passports (
  "uid" UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
  "first_name" VARCHAR NOT NULL,
  "second_name" VARCHAR NOT NULL,
  "patronymic" VARCHAR,
  "number" CHAR(6) NOT NULL,
  "series" CHAR(4) NOT NULL,
  "registration_place" VARCHAR NOT NULL,
  "birthday_date" DATE NOT NULL
);
