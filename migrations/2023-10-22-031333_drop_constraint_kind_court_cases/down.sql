-- This file should undo anything in `up.sql`
ALTER TABLE court_cases ADD CONSTRAINT court_cases_kind_check
  CHECK (kind > 0 AND kind < 5);
