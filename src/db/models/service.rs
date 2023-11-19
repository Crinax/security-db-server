use uuid::Uuid;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::db::orm::schema::services)]
#[diesel(belongs_to(super::user_profiles::UserProfile, foreign_key = law_uid))]
#[diesel(primary_key(uid))]
pub struct Service {
    pub uid: Uuid,
    pub law_uid: Uuid,
    pub name: String,
    pub cost: f64,
}