-- Your SQL goes here
CREATE TABLE IF NOT EXISTS court_cases (
  "uid" UUID DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
  "number" VARCHAR(50) NOT NULL,
  "judge_fullname" VARCHAR(255) NOT NULL,
  "decision" SMALLINT NOT NULL CHECK (decision > 0 AND decision < 3),
  "kind" SMALLINT NOT NULL CHECK (kind > 0 AND kind < 5),
  "created_at" TIMESTAMP NOT NULL DEFAULT NOW()
);
