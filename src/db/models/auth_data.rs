use diesel::prelude::*;
use uuid::Uuid;

use super::user_profiles::UserProfile;

#[derive(Queryable, Associations, Selectable, Identifiable, Debug)]
#[diesel(belongs_to(UserProfile, foreign_key = profile_uid))]
#[diesel(table_name = crate::db::orm::schema::auth_data)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(uid))]
pub struct AuthData {
    pub uid: Uuid,
    pub profile_uid: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
}
