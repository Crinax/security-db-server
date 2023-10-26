use std::io::Write;

use diesel::{
    expression::AsExpression,
    deserialize::{
        FromSqlRow,
        self,
        FromSql
    },
    pg::{PgValue, Pg},
    serialize::{
        self,
        Output,
        ToSql,
        IsNull
    }
};

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::db::orm::schema::sql_types::UserProfilesRoles)]
pub enum UserProfilesRoles {
    User,
    Employee,
    Law,
    Admin
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
