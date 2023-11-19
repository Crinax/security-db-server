use diesel::prelude::*;
use uuid::Uuid;
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, Serialize)]
#[diesel(belongs_to(super::user_profiles::UserProfile, foreign_key = creator_uid))]
#[diesel(table_name = crate::db::orm::schema::chats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(uid))]
pub struct Chats {
    pub uid: Uuid,
    pub creator_uid: Uuid,
    pub name: String,
    pub connection_hash: String,
}