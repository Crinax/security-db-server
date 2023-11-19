use uuid::Uuid;
use diesel::prelude::*;
use serde::Serialize;

use super::custom_types::{
    court_sides_case_statuses::CourtSidesCaseStatuses,
    court_sides_kinds::CourtSidesKinds
};

#[derive(Queryable, Identifiable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::db::orm::schema::court_sides)]
#[diesel(belongs_to(super::court_cases::CourtCase, foreign_key = court_case_uid))]
#[diesel(belongs_to(super::user_profiles::UserProfiles, foreign_key = user_uid))]
#[diesel(primary_key(uid))]
pub struct CourtSides {
    uid: Uuid,
    court_case_uid: Uuid,
    user_uid: Option<Uuid>,
    kind: CourtSidesKinds,
    case_status: CourtSidesCaseStatuses
}