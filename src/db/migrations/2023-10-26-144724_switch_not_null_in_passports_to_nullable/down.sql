-- This file should undo anything in `up.sql`
ALTER TABLE passports ALTER COLUMN "number" SET NOT NULL;
ALTER TABLE passports ALTER COLUMN "series" SET NOT NULL;
ALTER TABLE passports ALTER COLUMN "registration_place" SET NOT NULL;
