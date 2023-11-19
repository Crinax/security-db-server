use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};
use serde::Serialize;
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow, Serialize)]
#[diesel(sql_type = crate::db::orm::schema::sql_types::CourtCasesKinds)]
pub enum CourtCasesKinds {
    #[serde(rename = "administrative")]
    Administrative,

    #[serde(rename = "arbitration")]
    Arbitration,

    #[serde(rename = "criminal")]
    Criminal,

    #[serde(rename = "civil")]
    Civil,

    #[serde(rename = "constitutional")]
    Constitutional
}

impl<'a> From<CourtCasesKinds> for &'a str {
    fn from(value: CourtCasesKinds) -> &'a str {
        match value {
            CourtCasesKinds::Administrative => "administrative",
            CourtCasesKinds::Arbitration => "arbitration",
            CourtCasesKinds::Criminal => "criminal",
            CourtCasesKinds::Civil => "civil",
            CourtCasesKinds::Constitutional => "constitutional",
        }
    }
}

impl ToSql<crate::db::orm::schema::sql_types::CourtCasesDecisions, Pg> for CourtCasesKinds {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            CourtCasesKinds::Administrative => out.write_all(b"administrative")?,
            CourtCasesKinds::Arbitration => out.write_all(b"arbitration")?,
            CourtCasesKinds::Criminal => out.write_all(b"criminal")?,
            CourtCasesKinds::Civil => out.write_all(b"civil")?,
            CourtCasesKinds::Constitutional => out.write_all(b"constitutional")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::db::orm::schema::sql_types::CourtCasesDecisions, Pg> for CourtCasesKinds {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"administrative" => Ok(CourtCasesKinds::Administrative),
            b"arbitration" => Ok(CourtCasesKinds::Arbitration),
            b"criminal" => Ok(CourtCasesKinds::Criminal),
            b"constitutional" => Ok(CourtCasesKinds::Constitutional),
            b"civil" => Ok(CourtCasesKinds::Civil),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}