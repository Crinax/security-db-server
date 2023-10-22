-- Your SQL goes here
ALTER TABLE court_cases ALTER COLUMN kind
  TYPE court_cases_kinds
  USING CASE kind
    WHEN 1 THEN 'administrative'::court_cases_kinds
    WHEN 2 THEN 'arbitration'::court_cases_kinds
    WHEN 3 THEN 'criminal'::court_cases_kinds
    WHEN 4 THEN 'civil'::court_cases_kinds
    WHEN 5 THEN 'сonstitutional'::court_cases_kinds
    ELSE 'administrative'::court_cases_kinds
  END;
