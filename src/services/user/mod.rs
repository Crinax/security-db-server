use std::sync::Arc;

use super::dto::user::{PassportOrmData, LawProfileWithUser};
use crate::db::{
    models::{custom_types::user_profile_roles::UserProfilesRoles, user_profiles::UserProfile, law_profiles::LawProfile, passports::Passport},
    orm::schema::{user_profiles, law_profiles, passports},
    Db, DbError, DbProvider,
};
use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};
use uuid::Uuid;

#[derive(Debug)]
pub enum UserServiceError<T> {
    PassportCreation(T),
    ProfileCreation(T),
    NotFound,
    GetLaws,
}

pub struct UserService {
    db: Arc<Db>,
}

impl UserService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    fn create_user_transaction(
        &self,
        passport: &PassportOrmData,
    ) -> Result<Uuid, DbError<UserServiceError<diesel::result::Error>>> {
        self.db
            .transaction(move |conn| Self::create_user(conn, passport))
    }

    pub fn create_user(
        conn: &mut PgConnection,
        passport: &PassportOrmData,
    ) -> Result<Uuid, UserServiceError<diesel::result::Error>> {
        let uid = Self::create_passport(conn, passport)?;

        Self::create_profile(conn, &Some(uid), &UserProfilesRoles::User)
    }

    pub fn find_user_by_pk(
        conn: &mut PgConnection,
        uid: &Uuid,
    ) -> Result<UserProfile, UserServiceError<()>> {
        let result: UserProfile = user_profiles::dsl::user_profiles
            .find(uid)
            .first(conn)
            .map_err(|_| UserServiceError::NotFound)?;

        Ok(result)
    }

    pub fn get_laws(&self, page: i64) -> Result<Vec<LawProfileWithUser>, DbError<UserServiceError<()>>> {
        const LIMIT: i64 = 15;

        self.db.apply(|conn| {
            Ok(
                user_profiles::table
                    .left_join(law_profiles::table)
                    .left_join(passports::table)
                    .filter(
                        user_profiles::dsl::law_profile.is_not_null().and(
                            user_profiles::dsl::passport_uid.is_not_null()
                        )
                    )
                    .offset(page * LIMIT)
                    .limit(LIMIT)
                    .select((
                        UserProfile::as_select(),
                        LawProfile::as_select().assume_not_null(),
                        Passport::as_select().assume_not_null()
                    ))
                    .load(conn)
                    .map_err(|_| UserServiceError::GetLaws)?
                    .into_iter()
                    .map(|record: (UserProfile, LawProfile, Passport)| LawProfileWithUser {
                        uid: record.0.uid,
                        avatar_uid: record.0.avatar_uid,
                        law_uid: record.1.uid,
                        itn: record.1.itn,
                        start_activity_date: record.1.start_activity_date,
                        first_name: record.2.first_name,
                        second_name: record.2.second_name,
                        patronymic: record.2.patronymic,
                    })
                    .collect::<Vec<LawProfileWithUser>>()
            )
        })
    }

    fn create_passport(
        conn: &mut PgConnection,
        data: &PassportOrmData,
    ) -> Result<Uuid, UserServiceError<diesel::result::Error>> {
        insert_into(passports::dsl::passports)
            .values(data)
            .returning(passports::dsl::uid)
            .get_result(conn)
            .map_err(UserServiceError::PassportCreation)
    }

    fn create_profile(
        conn: &mut PgConnection,
        passport_uid: &Option<Uuid>,
        role: &UserProfilesRoles,
    ) -> Result<Uuid, UserServiceError<diesel::result::Error>> {
        insert_into(user_profiles::dsl::user_profiles)
            .values((
                user_profiles::dsl::passport_uid.eq(passport_uid),
                user_profiles::dsl::role.eq(role),
            ))
            .returning(user_profiles::dsl::uid)
            .get_result(conn)
            .map_err(UserServiceError::ProfileCreation)
    }
}
