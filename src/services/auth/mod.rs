use std::sync::Arc;

use crate::db::models;
use crate::db::{orm::schema::auth_data, Db, DbError, DbProvider};
use argon2::{self, Config};
use diesel::insert_into;
use diesel::prelude::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::dto::auth::AuthorizationDto;
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
    InvalidPassword,
    UserNotFound,
    PasswordVerify,
    Unreachable,
    AlreadyExists,
    InvalidToken,
    TokenExpired,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JwtAccessData {
    pub uid: Uuid,
    pub sub: String,
    pub username: String,
    pub role: String,
    pub exp: usize,
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

    pub fn authorize_user<T>(
        &self,
        dto: AuthorizationDto,
        config: &T,
    ) -> Result<(String, String, usize, usize), DbError<AuthServiceError<()>>>
    where
        T: SaltProvider + SecretsProvider,
    {
        self.db.apply(move |conn| {
            let data = AuthService::find_by_email_or_username(conn, &dto.email_or_username)
                .map_err(|_| AuthServiceError::UserNotFound)?;
            let hashed_password = Self::verify_password(dto.password.as_bytes(), &data.password)
                .map_err(|_| AuthServiceError::PasswordVerify)?;

            let user = UserService::find_user_by_pk(conn, &data.profile_uid)
                .map_err(|_| AuthServiceError::UserNotFound)?;

            if !hashed_password {
                return Err(AuthServiceError::InvalidPassword);
            }

            Self::generate_tokens(
                data.uid,
                &data.username,
                &data.email,
                user.role.into(),
                config,
            )
            .map_err(|err| match err {
                AuthServiceError::AccessTokenGeneration => AuthServiceError::AccessTokenGeneration,
                AuthServiceError::RefreshTokenGeneration => {
                    AuthServiceError::RefreshTokenGeneration
                }
                _ => AuthServiceError::Unreachable,
            })
        })
    }

    pub fn register_user<T>(
        &self,
        dto: RegistrationDto,
        config: &T,
    ) -> Result<(String, String, usize, usize), DbError<AuthServiceError<diesel::result::Error>>>
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
                UserServiceError::ProfileCreation(err) => match err {
                    diesel::result::Error::DatabaseError(
                        diesel::result::DatabaseErrorKind::UniqueViolation,
                        _,
                    ) => AuthServiceError::AlreadyExists,
                    _ => AuthServiceError::ProfileCreation,
                },
                UserServiceError::PassportCreation(_) => AuthServiceError::PassportCreation,
                _ => AuthServiceError::Unreachable,
            })?;

            let password = Self::hash_password(dto.password.as_bytes(), config)
                .map_err(|_| AuthServiceError::HashPassword)?;

            let uid =
                Self::create_auth_data(conn, &dto.username, &password, &dto.email, profile_uid)?;

            Self::generate_tokens(uid, &dto.username, &dto.email, "user", config).map_err(|err| {
                match err {
                    AuthServiceError::AccessTokenGeneration => {
                        AuthServiceError::AccessTokenGeneration
                    }
                    AuthServiceError::RefreshTokenGeneration => {
                        AuthServiceError::RefreshTokenGeneration
                    }
                    _ => AuthServiceError::Unreachable,
                }
            })
        })
    }

    pub fn refresh_tokens(
        &self,
        user_data: &JwtAccessData,
        secrets_provider: &impl SecretsProvider,
    ) -> Result<(String, String, usize, usize), DbError<AuthServiceError<()>>> {
        let profile_data = self.db.apply(move |conn| {
            let auth = AuthService::find_by_pk(conn, &user_data.uid)?;

            UserService::find_user_by_pk(conn, &auth.profile_uid).map_err(|err| match err {
                UserServiceError::NotFound => AuthServiceError::UserNotFound,
                _ => AuthServiceError::Unreachable,
            })
        })?;

        AuthService::generate_tokens(
            user_data.uid,
            &user_data.username,
            &user_data.sub,
            profile_data.role.into(),
            secrets_provider,
        )
        .map_err(|err| match err {
            AuthServiceError::AccessTokenGeneration => {
                DbError::Execution(AuthServiceError::AccessTokenGeneration)
            }
            AuthServiceError::RefreshTokenGeneration => {
                DbError::Execution(AuthServiceError::RefreshTokenGeneration)
            }
            _ => DbError::Unreachable,
        })
    }

    pub fn validate_token(
        access_token: &str,
        secrets_provider: &impl SecretsProvider,
    ) -> Result<JwtAccessData, AuthServiceError<()>> {
        decode::<JwtAccessData>(
            access_token,
            &DecodingKey::from_secret(secrets_provider.access_secret()),
            &Validation::default(),
        )
        .map(|jwt| jwt.claims)
        .map_err(|err| {
            log::error!("{}", err);

            match err.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthServiceError::TokenExpired,
                _ => AuthServiceError::InvalidToken,
            }
        })
    }

    fn create_auth_data(
        conn: &mut PgConnection,
        username: &str,
        password: &str,
        email: &str,
        profile_uid: Uuid,
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
    ) -> Result<(String, String, usize, usize), AuthServiceError<()>> {
        let (exp, refresh_exp) = AuthService::generate_expiration_time();
        let access_token_data = JwtAccessData {
            sub: email.to_owned(),
            uid,
            username: username.to_owned(),
            role: role.to_owned(),
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

        Ok((access_token, refresh_token, exp, refresh_exp))
    }

    fn verify_password(
        input_password: &[u8],
        record_password: &str,
    ) -> Result<bool, AuthServiceError<()>> {
        argon2::verify_encoded(record_password, input_password)
            .map_err(|_| AuthServiceError::PasswordVerify)
    }

    fn find_by_email_or_username(
        conn: &mut PgConnection,
        email_or_username: &str,
    ) -> Result<models::auth_data::AuthData, AuthServiceError<()>> {
        auth_data::dsl::auth_data
            .filter(
                auth_data::dsl::email
                    .eq(email_or_username)
                    .or(auth_data::dsl::username.eq(email_or_username)),
            )
            .first(conn)
            .map_err(|_| AuthServiceError::UserNotFound)
    }

    fn find_by_pk(
        conn: &mut PgConnection,
        pk: &Uuid,
    ) -> Result<models::auth_data::AuthData, AuthServiceError<()>> {
        auth_data::dsl::auth_data
            .find(pk)
            .first(conn)
            .map_err(|_| AuthServiceError::UserNotFound)
    }

    pub fn decrypt_token(
        access_token: &str,
        secrets_provider: &impl SecretsProvider,
    ) -> Result<JwtAccessData, AuthServiceError<()>> {
        let mut validation_without_exp = Validation::default();

        validation_without_exp.validate_exp = false;

        decode::<JwtAccessData>(
            access_token,
            &DecodingKey::from_secret(secrets_provider.access_secret()),
            &validation_without_exp,
        )
        .map(|jwt| jwt.claims)
        .map_err(|err| {
            log::error!("{}", err);

            AuthServiceError::InvalidToken
        })
    }

    fn generate_expiration_time() -> (usize, usize) {
        let exp = (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp() as usize;
        let refresh_exp = (chrono::Utc::now() + chrono::Duration::days(30)).timestamp() as usize;

        (exp, refresh_exp)
    }
}
