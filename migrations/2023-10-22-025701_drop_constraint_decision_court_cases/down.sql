-- This file should undo anything in `up.sql`
ALTER TABLE court_cases ADD CONSTRAINT court_cases_decision_check
  CHECK (decision > 0 AND decision < 3);
