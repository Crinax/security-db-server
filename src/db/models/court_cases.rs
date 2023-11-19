use chrono::NaiveDateTime;
use uuid::Uuid;
use diesel::prelude::*;
use super::custom_types::{court_cases_decisions::CourtCasesDecisions, court_cases_kinds::CourtCasesKinds};
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::db::orm::schema::court_cases)]
#[diesel(primary_key(uid))]
pub struct CourtCase {
    uid: Uuid,
    number: String,
    judge_fullname: String,
    decision: CourtCasesDecisions,
    kind: CourtCasesKinds,
    created_at: NaiveDateTime
}