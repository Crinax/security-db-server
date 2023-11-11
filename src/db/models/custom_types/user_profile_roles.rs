use std::io::Write;

use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::db::orm::schema::sql_types::UserProfilesRoles)]
pub enum UserProfilesRoles {
    User,
    Employee,
    Law,
    Admin,
}

impl<'a> Into<&'a str> for UserProfilesRoles {
    fn into(self) -> &'a str {
        match self {
            UserProfilesRoles::User => "user",
            UserProfilesRoles::Law => "law",
            UserProfilesRoles::Admin => "admin",
            UserProfilesRoles::Employee => "employee",
        }
    }
}

impl ToSql<crate::db::orm::schema::sql_types::UserProfilesRoles, Pg> for UserProfilesRoles {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            UserProfilesRoles::User => out.write_all(b"user")?,
            UserProfilesRoles::Employee => out.write_all(b"employee")?,
            UserProfilesRoles::Law => out.write_all(b"law")?,
            UserProfilesRoles::Admin => out.write_all(b"admin")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::db::orm::schema::sql_types::UserProfilesRoles, Pg> for UserProfilesRoles {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"user" => Ok(UserProfilesRoles::User),
            b"employee" => Ok(UserProfilesRoles::Employee),
            b"law" => Ok(UserProfilesRoles::Law),
            b"admin" => Ok(UserProfilesRoles::Admin),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
