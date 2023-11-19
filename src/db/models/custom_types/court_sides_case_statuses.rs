use std::io::Write;

use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};
use serde::Serialize;

#[derive(Debug, AsExpression, FromSqlRow, Serialize)]
#[diesel(sql_type = crate::db::orm::schema::sql_types::CourtSidesCaseStatuses)]
pub enum CourtSidesCaseStatuses {
    #[serde(rename = "winning")]
    Winning,

    #[serde(rename = "loss")]
    Loss,

    #[serde(rename = "unknown")]
    Unknown,
}

impl<'a> From<CourtSidesCaseStatuses> for &'a str {
    fn from(value: CourtSidesCaseStatuses) -> &'a str {
        match value {
            CourtSidesCaseStatuses::Winning => "winning",
            CourtSidesCaseStatuses::Loss => "loss",
            CourtSidesCaseStatuses::Unknown => "unknown",
        }
    }
}

impl ToSql<crate::db::orm::schema::sql_types::CourtSidesCaseStatuses, Pg> for CourtSidesCaseStatuses {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            CourtSidesCaseStatuses::Winning => out.write_all(b"winning")?,
            CourtSidesCaseStatuses::Loss => out.write_all(b"loss")?,
            CourtSidesCaseStatuses::Unknown => out.write_all(b"unknown")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::db::orm::schema::sql_types::CourtSidesCaseStatuses, Pg> for CourtSidesCaseStatuses {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"winning" => Ok(CourtSidesCaseStatuses::Winning),
            b"loss" => Ok(CourtSidesCaseStatuses::Loss),
            b"unknown" => Ok(CourtSidesCaseStatuses::Unknown),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
