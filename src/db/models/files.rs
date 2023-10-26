use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(table_name = crate::db::orm::schema::files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(uid))]
pub struct File {
    pub uid: Uuid,
    pub file_name: String,
    pub original_name: String,
}
