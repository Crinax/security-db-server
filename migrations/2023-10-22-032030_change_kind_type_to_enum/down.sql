-- This file should undo anything in `up.sql`
ALTER TABLE court_cases ALTER COLUMN kind
  TYPE SMALLINT
  USING CASE kind
    WHEN 'administrative'::court_cases_kinds THEN 1
    WHEN 'arbitration'::court_cases_kinds THEN 2
    WHEN 'criminal'::court_cases_kinds THEN 3
    WHEN 'civil'::court_cases_kinds THEN 4
    WHEN 'сonstitutional'::court_cases_kinds THEN 5
    ELSE 1
  END;
