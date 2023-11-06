use std::sync::Arc;

use crate::db::{orm::schema::auth_data, Db, DbError, DbProvider};
use argon2::{self, Config};
use diesel::insert_into;
use diesel::prelude::*;
use uuid::Uuid;

use super::{
    dto::{auth::RegistrationDto, user::PassportOrmData},
    user::{UserService, UserServiceError},
};

#[derive(Debug)]
pub enum AuthServiceError<T> {
    AuthDataCreation(T),
    HashPassword,
    ProfileCreation,
    PassportCreation,
}

pub trait SaltProvider {
    fn salt(&self) -> &[u8];
}

pub struct AuthService {
    db: Arc<Db>,
}

impl AuthService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub fn register_user(
        &self,
        dto: RegistrationDto,
        salt_provider: &impl SaltProvider,
    ) -> Result<(), DbError<AuthServiceError<diesel::result::Error>>> {
        self.db.transaction(move |conn| {
            let profile_uid = UserService::create_user(
                conn,
                &PassportOrmData {
                    first_name: &dto.first_name,
                    second_name: &dto.second_name,
                    patronymic: dto.patronymic.as_deref(),
                    birthday_date: dto.birth_date,
                },
            )
            .map_err(|err| match err {
                UserServiceError::ProfileCreation(_) => AuthServiceError::ProfileCreation,
                UserServiceError::PassportCreation(_) => AuthServiceError::PassportCreation,
            })?;

            let password = Self::hash_password(dto.password.as_bytes(), salt_provider)
                .map_err(|_| AuthServiceError::HashPassword)?;

            Self::create_auth_data(
                conn,
                &dto.username,
                &password,
                &dto.email,
                Some(profile_uid),
            )
        })
    }

    fn create_auth_data(
        conn: &mut PgConnection,
        username: &str,
        password: &str,
        email: &str,
        profile_uid: Option<Uuid>,
    ) -> Result<(), AuthServiceError<diesel::result::Error>> {
        insert_into(auth_data::dsl::auth_data)
            .values((
                auth_data::dsl::username.eq(username),
                auth_data::dsl::password.eq(password),
                auth_data::dsl::email.eq(email),
                auth_data::dsl::profile_uid.eq(profile_uid),
            ))
            .execute(conn)
            .map(|_| ())
            .map_err(AuthServiceError::AuthDataCreation)
    }

    fn hash_password(
        password: &[u8],
        salt_provider: &impl SaltProvider,
    ) -> Result<String, AuthServiceError<()>> {
        argon2::hash_encoded(password, salt_provider.salt(), &Config::rfc9106_low_mem())
            .map_err(|_| AuthServiceError::HashPassword)
    }
}
