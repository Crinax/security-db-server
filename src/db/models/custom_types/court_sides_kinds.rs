use std::io::Write;

use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};
use serde::Serialize;

#[derive(Debug, AsExpression, FromSqlRow, Serialize)]
#[diesel(sql_type = crate::db::orm::schema::sql_types::CourtSidesKinds)]
pub enum CourtSidesKinds {
    #[serde(rename = "first")]
    First,

    #[serde(rename = "second")]
    Second,

    #[serde(rename = "third")]
    Third,
}

impl<'a> From<CourtSidesKinds> for &'a str {
    fn from(value: CourtSidesKinds) -> &'a str {
        match value {
            CourtSidesKinds::First => "first",
            CourtSidesKinds::Second => "second",
            CourtSidesKinds::Third => "third",
        }
    }
}

impl ToSql<crate::db::orm::schema::sql_types::CourtSidesKinds, Pg> for CourtSidesKinds {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            CourtSidesKinds::First => out.write_all(b"first")?,
            CourtSidesKinds::Second => out.write_all(b"second")?,
            CourtSidesKinds::Third => out.write_all(b"third")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::db::orm::schema::sql_types::CourtSidesKinds, Pg> for CourtSidesKinds {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"first" => Ok(CourtSidesKinds::First),
            b"second" => Ok(CourtSidesKinds::Second),
            b"third" => Ok(CourtSidesKinds::Third),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
