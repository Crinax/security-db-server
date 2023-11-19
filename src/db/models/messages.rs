use diesel::prelude::*;
use uuid::Uuid;
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, Serialize)]
#[diesel(belongs_to(super::user_profiles::UserProfile, foreign_key = sender_uid))]
#[diesel(belongs_to(super::chats::Chats, foreign_key = chat_uid))]
#[diesel(table_name = crate::db::orm::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(uid))]
pub struct Messages {
    pub uid: Uuid,
    pub chat_uid: Uuid,
    pub sender_uid: Uuid,
    pub content: String,
}