-- Your SQL goes here
ALTER TABLE passports ALTER COLUMN "number" DROP NOT NULL;
ALTER TABLE passports ALTER COLUMN "series" DROP NOT NULL;
ALTER TABLE passports ALTER COLUMN "registration_place" DROP NOT NULL;
