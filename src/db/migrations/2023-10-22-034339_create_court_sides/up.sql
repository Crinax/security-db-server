-- Your SQL goes here
CREATE TABLE IF NOT EXISTS court_sides (
  "uid" UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
  "court_case_uid" UUID NOT NULL REFERENCES court_cases("uid") ON DELETE CASCADE,
  "user_uid" UUID REFERENCES user_profiles("uid") ON DELETE SET NULL,
  "kind" court_sides_kinds NOT NULL,
  "case_status" court_sides_case_statuses NOT NULL 
);
