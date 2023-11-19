use chrono::NaiveDateTime;
use uuid::Uuid;
use diesel::prelude::*;
use serde::Serialize;

use super::custom_types::law_transaction_statuses::LawTransactionsStatues;


#[derive(Queryable, Identifiable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::db::orm::schema::law_transactions)]
#[diesel(belongs_to(super::court_cases::CourtCase, foreign_key = court_case_uid))]
#[diesel(belongs_to(super::user_profiles::UserProfiles, foreign_key = client_uid))]
#[diesel(primary_key(uid))]
pub struct LawTransactions {
    uid: Uuid,
    court_case_uid: Option<Uuid>,
    client_uid: Option<Uuid>,
    status: LawTransactionsStatues,
    created_at: NaiveDateTime
}