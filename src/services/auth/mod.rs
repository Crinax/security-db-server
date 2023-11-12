use std::sync::Arc;

use crate::db::models;
use crate::db::{orm::schema::auth_data, Db, DbError, DbProvider};
use argon2::{self, Config};
use diesel::insert_into;
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header, decode, Validation, DecodingKey};
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
    AuthDataWithoutProfile,
    Unreachable,
    AlreadyExists,
    InvalidToken,
    TokenExpired,
}

#[derive(Serialize, Deserialize)]
struct JwtAccessData {
    uid: Uuid,
    sub: String,
    username: String,
    role: String,
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
                UserServiceError::NotFound => AuthServiceError::Unreachable,
            })?;

            let password = Self::hash_password(dto.password.as_bytes(), config)
                .map_err(|_| AuthServiceError::HashPassword)?;

            let uid = Self::create_auth_data(
                conn,
                &dto.username,
                &password,
                &dto.email,
                profile_uid,
            )?;

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

    pub fn refresh_tokens(&self, access_token: &str, secrets_provider: &impl SecretsProvider) -> Result<(String, String, usize, usize), DbError<AuthServiceError<()>>> {
        let user_data = AuthService::decrypt_token(access_token, secrets_provider)
            .map_err(|err| {
                match err {
                    AuthServiceError::InvalidToken => DbError::Execution(AuthServiceError::InvalidToken),
                    _ => DbError::Unreachable
                }
            })?;

        let profile_data = self.db.apply(move |conn| {
            let auth = AuthService::find_by_pk(conn, &user_data.uid)?;

            let user_data = UserService::find_user_by_pk(conn, &auth.profile_uid)
                .map_err(|err| match err {
                    UserServiceError::NotFound => AuthServiceError::UserNotFound,
                    _ => AuthServiceError::Unreachable,
                })?;

            Ok(user_data)
        })?;

        let tokens = AuthService::generate_tokens(
            user_data.uid,
            &user_data.username,
            &user_data.sub,
            profile_data.role.into(),
            secrets_provider,
        )
            .map_err(|err| match err {
                AuthServiceError::AccessTokenGeneration => DbError::Execution(AuthServiceError::AccessTokenGeneration),
                AuthServiceError::RefreshTokenGeneration => DbError::Execution(AuthServiceError::RefreshTokenGeneration),
                _ => DbError::Unreachable,
            })?;

        Ok(tokens)
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
        let result = auth_data::dsl::auth_data
            .filter(
                auth_data::dsl::email
                    .eq(email_or_username)
                    .or(auth_data::dsl::username.eq(email_or_username)),
            )
            .first(conn)
            .map_err(|_| AuthServiceError::UserNotFound)?;

        Ok(result)
    }

    fn find_by_pk(
        conn: &mut PgConnection,
        pk: &Uuid,
    ) -> Result<models::auth_data::AuthData, AuthServiceError<()>> {
        let result = auth_data::dsl::auth_data
            .find(pk)
            .first(conn)
            .map_err(|_| AuthServiceError::UserNotFound)?;

        Ok(result)
    }

    fn validate_token(access_token: &str, secrets_provider: &impl SecretsProvider) -> Result<(), AuthServiceError<()>> {
        decode::<JwtAccessData>(
            access_token,
            &DecodingKey::from_secret(secrets_provider.access_secret()),
            &Validation::default(),
        )
            .map_err(|err| {
                log::error!("{}", err);

                match err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthServiceError::TokenExpired,
                    _ => AuthServiceError::InvalidToken,
                }
            })?;

        Ok(())
    }

    fn decrypt_token(access_token: &str, secrets_provider: &impl SecretsProvider) -> Result<JwtAccessData, AuthServiceError<()>> {
        let mut validation_without_exp = Validation::default();

        validation_without_exp.validate_exp = false;

        let token = decode::<JwtAccessData>(
            access_token,
            &DecodingKey::from_secret(secrets_provider.access_secret()),
            &validation_without_exp,
        )
            .map_err(|err| {
                log::error!("{}", err);

                AuthServiceError::InvalidToken
            })?;

        Ok(token.claims)
    }

    fn generate_expiration_time() -> (usize, usize) {
        let exp = (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp() as usize;
        let refresh_exp = (chrono::Utc::now() + chrono::Duration::days(30)).timestamp() as usize;

        return (exp, refresh_exp);
    }
}
