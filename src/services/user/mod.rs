use std::sync::Arc;

use super::dto::user::{PassportOrmData, LawProfileWithUser};
use crate::db::{
    models::{custom_types::user_profiles_roles::UserProfilesRoles, user_profiles::UserProfile, law_profiles::LawProfile, passports::Passport},
    orm::schema::{user_profiles, law_profiles, passports},
    Db, DbError, DbProvider,
};
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

    pub fn get_laws(&self, page: u64) -> Result<Vec<LawProfileWithUser>, DbError<UserServiceError<()>>> {
        const LIMIT: u64 = 15;

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
                    // What?
                    // Postgres: offset cannot be negative
                    // Diesel: ...can... be...
                    .offset((page * LIMIT) as i64)
                    .limit(LIMIT as i64)
                    .load(conn)
                    .map_err(|_| UserServiceError::GetLaws)?
                    .into_iter()
                    .map(|record: (UserProfile, Option<LawProfile>, Option<Passport>)| {
                        log::info!("{:?}", record);
                        let law = record.1.unwrap();
                        let passport = record.2.unwrap();
                        let user = record.0;

                        LawProfileWithUser {
                            uid: user.uid,
                            avatar_uid: user.avatar_uid,
                            law_uid: law.uid,
                            itn: law.itn,
                            start_activity_date: law.start_activity_date,
                            first_name: passport.first_name,
                            second_name: passport.second_name,
                            patronymic: passport.patronymic,
                        }
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
