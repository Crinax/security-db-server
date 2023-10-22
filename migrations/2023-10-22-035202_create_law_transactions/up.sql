-- Your SQL goes here
CREATE TABLE IF NOT EXISTS law_transactions (
  "uid" UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
  "court_case_uid" UUID REFERENCES court_cases("uid") ON DELETE SET NULL,
  "client_uid" UUID REFERENCES user_profiles("uid") ON DELETE SET NULL,
  "status" law_transactions_statues NOT NULL,
  "created_at" TIMESTAMP NOT NULL DEFAULT NOW()
);
