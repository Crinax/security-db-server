use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Debug)]
#[diesel(table_name = crate::db::orm::schema::law_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LawProfile {
    pub uid: Uuid,
    pub itn: String,
    pub start_activity_date: DateTime<Utc>
}
