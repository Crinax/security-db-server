use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use super::custom_types::user_profile_roles::UserProfilesRoles;
use super::files::File;
use super::law_profiles::LawProfile;
use super::passports::Passport;

#[derive(Queryable, Associations, Selectable, Identifiable, Debug)]
#[diesel(belongs_to(Passport, foreign_key = passport_uid))]
#[diesel(belongs_to(File, foreign_key = avatar_uid))]
#[diesel(belongs_to(LawProfile, foreign_key = law_profile))]
#[diesel(table_name = crate::db::orm::schema::user_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(uid))]
pub struct UserProfile {
    pub uid: Uuid,
    pub passport_uid: Option<Uuid>,
    pub law_profile: Option<Uuid>,
    pub avatar_uid: Option<Uuid>,
    pub role: UserProfilesRoles,
    pub created_at: NaiveDateTime,
}
