use std::sync::Arc;

use crate::db::{orm::schema::auth_data, Db, DbError, DbProvider};
use argon2::{self, Config};
use diesel::insert_into;
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
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
    AccessTokenGeneration,
    RefreshTokenGeneration,
}

#[derive(Serialize, Deserialize)]
struct JwtAccessData<'a> {
    uid: Uuid,
    sub: &'a str,
    username: &'a str,
    role: &'a str,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
struct JwtRefreshData {
    uid: Uuid,
    exp: usize,
}

pub trait SaltProvider {
    fn salt(&self) -> &[u8];
}

pub trait SecretsProvider {
    fn access_secret(&self) -> &[u8];
    fn refresh_secret(&self) -> &[u8];
}

pub struct AuthService {
    db: Arc<Db>,
}

impl AuthService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub fn register_user<T>(
        &self,
        dto: RegistrationDto,
        config: &T,
    ) -> Result<(String, String), DbError<AuthServiceError<diesel::result::Error>>>
    where
        T: SaltProvider + SecretsProvider,
    {
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

            let password = Self::hash_password(dto.password.as_bytes(), config)
                .map_err(|_| AuthServiceError::HashPassword)?;

            let uid = Self::create_auth_data(
                conn,
                &dto.username,
                &password,
                &dto.email,
                Some(profile_uid),
            )?;

            Self::generate_tokens(uid, &dto.username, &dto.email, "user", config).map_err(|err| {
                match err {
                    AuthServiceError::AccessTokenGeneration => {
                        AuthServiceError::AccessTokenGeneration
                    }
                    AuthServiceError::RefreshTokenGeneration => {
                        AuthServiceError::RefreshTokenGeneration
                    }
                    _ => unreachable!(),
                }
            })
        })
    }

    fn create_auth_data(
        conn: &mut PgConnection,
        username: &str,
        password: &str,
        email: &str,
        profile_uid: Option<Uuid>,
    ) -> Result<Uuid, AuthServiceError<diesel::result::Error>> {
        insert_into(auth_data::dsl::auth_data)
            .values((
                auth_data::dsl::username.eq(username),
                auth_data::dsl::password.eq(password),
                auth_data::dsl::email.eq(email),
                auth_data::dsl::profile_uid.eq(profile_uid),
            ))
            .returning(auth_data::dsl::uid)
            .get_result(conn)
            .map_err(AuthServiceError::AuthDataCreation)
    }

    fn hash_password(
        password: &[u8],
        salt_provider: &impl SaltProvider,
    ) -> Result<String, AuthServiceError<()>> {
        argon2::hash_encoded(password, salt_provider.salt(), &Config::rfc9106_low_mem())
            .map_err(|_| AuthServiceError::HashPassword)
    }

    fn generate_tokens(
        uid: Uuid,
        username: &str,
        email: &str,
        role: &str,
        secrets_provider: &impl SecretsProvider,
    ) -> Result<(String, String), AuthServiceError<()>> {
        let exp = (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp() as usize;
        let refresh_exp = (chrono::Utc::now() + chrono::Duration::days(30)).timestamp() as usize;

        let access_token_data = JwtAccessData {
            sub: email,
            uid,
            username,
            role,
            exp,
        };
        let refresh_token_data = JwtRefreshData {
            uid: Uuid::new_v4(),
            exp: refresh_exp,
        };

        let access_token = encode(
            &Header::default(),
            &access_token_data,
            &EncodingKey::from_secret(secrets_provider.access_secret()),
        )
        .map_err(|_| AuthServiceError::AccessTokenGeneration)?;

        let refresh_token = encode(
            &Header::default(),
            &refresh_token_data,
            &EncodingKey::from_secret(secrets_provider.refresh_secret()),
        )
        .map_err(|_| AuthServiceError::RefreshTokenGeneration)?;

        Ok((access_token, refresh_token))
    }
}
