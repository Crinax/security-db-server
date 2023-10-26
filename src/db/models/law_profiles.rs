use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(table_name = crate::db::orm::schema::law_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(uid))]
pub struct LawProfile {
    pub uid: Uuid,
    pub itn: String,
    pub start_activity_date: NaiveDateTime
}
