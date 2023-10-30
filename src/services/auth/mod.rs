use crate::db::{
    models::custom_types::user_profile_roles::UserProfilesRoles, orm::schema::auth_data,
    orm::schema::passports, orm::schema::user_profiles, Db, DbError, DbProvider,
};
use argon2::{self, Config};
use diesel::insert_into;
use diesel::prelude::*;
use uuid::Uuid;

use super::dto::auth::RegistrationDto;

pub enum RegisterError {
    Insertion,
    Inavailable,
    Cipher,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = auth_data)]
pub struct AuthOrmDto {
    username: String,
    password: String,
    email: String,
    pub profile_uid: Option<Uuid>,
}

#[derive(Insertable)]
#[diesel(table_name = user_profiles)]
pub struct ProfileOrmDto {
    passport_uid: Option<Uuid>,
    role: UserProfilesRoles,
}

#[derive(Insertable)]
#[diesel(table_name = passports)]
pub struct PassportOrmData {
    first_name: String,
    second_name: String,
    patronymic: Option<String>,
    birthday_date: chrono::NaiveDate,
}

pub fn register(db: &Db, data: RegistrationDto) -> Result<(), RegisterError> {
    let config = Config::rfc9106_low_mem();
    let salt = b"verysuperpuperbigsalt";
    let auth_info = AuthOrmDto {
        username: data.username,
        password: argon2::hash_encoded(data.password.as_bytes(), salt, &config)
            .map_err(|_| RegisterError::Cipher)?,
        email: data.email,
        profile_uid: None,
    };
    let passport_data = PassportOrmData {
        first_name: data.first_name,
        second_name: data.second_name,
        patronymic: data.patronymic,
        birthday_date: data.birth_date,
    };

    db.apply(|conn| {
        let passport_uid: Vec<Uuid> = insert_into(passports::dsl::passports)
            .values(&passport_data)
            .returning(passports::dsl::uid)
            .get_results(conn)
            .map_err(|err| {
                log::error!("{:?}", err);
                RegisterError::Insertion
            })?;

        let profile_uid: Vec<Uuid> = insert_into(user_profiles::dsl::user_profiles)
            .values(&ProfileOrmDto {
                role: UserProfilesRoles::User,
                passport_uid: passport_uid.get(0).copied(),
            })
            .returning(user_profiles::dsl::uid)
            .get_results(conn)
            .map_err(|err| {
                log::error!("{:?}", err);
                RegisterError::Insertion
            })?;

        let clonned_auth = auth_info.clone();

        insert_into(auth_data::dsl::auth_data)
            .values(&AuthOrmDto {
                username: clonned_auth.username,
                password: clonned_auth.password,
                email: clonned_auth.email,
                profile_uid: profile_uid.get(0).copied(),
            })
            .execute(conn)
            .map(|_| ())
            .map_err(|err| {
                log::error!("{:?}", err);
                RegisterError::Insertion
            })
    })
    .map_err(|err| match err {
        DbError::Connection => RegisterError::Inavailable,
        DbError::Execution(err) => err,
        _ => unreachable!(),
    })
}
