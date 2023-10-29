use diesel::prelude::*;
use serde::Deserialize;
use crate::db::{orm::schema::auth_data::{self, dsl::*}, Db, DbProvider, DbError};
use diesel::insert_into;
use argon2::{self, Config};

use super::dto::auth::RegistrationDto;

pub enum RegisterError {
    Insertion,
    Inavailable,
    Cipher,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = auth_data)]
pub struct RegisterServiceDto {
    username: String,
    password: String,
    email: String,
}

impl Into<RegisterServiceDto> for RegistrationDto {
    fn into(self) -> RegisterServiceDto {
        RegisterServiceDto {
            username: self.username,
            password: self.password,
            email: self.email
        }
    }
}

pub fn register(db: &Db, mut data: RegisterServiceDto) -> Result<(), RegisterError> {
    let config = Config::default();
    let salt = b"verysuperpuperbigsalt";

    data.password = argon2::hash_encoded(data.password.as_bytes(), salt, &config)
        .map_err(|_| RegisterError::Cipher)?;

    db.apply(|conn| {
        insert_into(auth_data)
            .values(&data)
            .execute(conn)
            .map(|_| ())
            .map_err(|err| {
                log::error!("{:?}", err);
                RegisterError::Insertion
            })
    }).map_err(|err| match err {
        DbError::Connection => RegisterError::Inavailable,
        DbError::Execution(err) => err,
        _ => unreachable!()
    })
}
