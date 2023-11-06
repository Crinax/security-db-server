use std::sync::Arc;

use super::dto::user::PassportOrmData;
use crate::db::{
    models::custom_types::user_profile_roles::UserProfilesRoles, orm::schema::passports,
    orm::schema::user_profiles, Db, DbError, DbProvider,
};
use diesel::{insert_into, prelude::*};
use uuid::Uuid;

#[derive(Debug)]
pub enum UserServiceError<T> {
    PassportCreation(T),
    ProfileCreation(T),
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
