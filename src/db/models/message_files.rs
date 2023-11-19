use diesel::prelude::*;
use uuid::Uuid;
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, Serialize)]
#[diesel(belongs_to(super::chats::Messages, foreign_key = message_uid))]
#[diesel(belongs_to(super::chats::File, foreign_key = file_uid))]
#[diesel(table_name = crate::db::orm::schema::message_files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(uid))]
pub struct MessageFile {
    pub uid: Uuid,
    pub message_uid: Uuid,
    pub file_uid: Uuid,
}