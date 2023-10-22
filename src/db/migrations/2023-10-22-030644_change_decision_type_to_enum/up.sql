-- Your SQL goes here
ALTER TABLE court_cases ALTER COLUMN decision
  TYPE court_cases_decisions
  USING CASE decision
    WHEN 1 THEN 'started'::court_cases_decisions
    WHEN 2 THEN 'processing'::court_cases_decisions
    WHEN 3 THEN 'complete'::court_cases_decisions
    ELSE 'started'::court_cases_decisions
  END;
