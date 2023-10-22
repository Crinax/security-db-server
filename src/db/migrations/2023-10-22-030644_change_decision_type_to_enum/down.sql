-- This file should undo anything in `up.sql`
ALTER TABLE court_cases ALTER COLUMN decision
  TYPE SMALLINT
  USING CASE decision
    WHEN 'started'::court_cases_decisions THEN 1
    WHEN 'processing'::court_cases_decisions THEN 2
    WHEN 'complete'::court_cases_decisions THEN 3
    ELSE 1
  END;
