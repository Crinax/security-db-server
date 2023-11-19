use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};
use serde::Serialize;
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow, Serialize)]
#[diesel(sql_type = crate::db::orm::schema::sql_types::CourtCasesDecisions)]
pub enum CourtCasesDecisions {
    #[serde(rename = "started")]
    Started,

    #[serde(rename = "processing")]
    Processing,

    #[serde(rename = "complete")]
    Complete
}

impl<'a> From<CourtCasesDecisions> for &'a str {
    fn from(value: CourtCasesDecisions) -> &'a str {
        match value {
            CourtCasesDecisions::Started => "started",
            CourtCasesDecisions::Processing => "processing",
            CourtCasesDecisions::Complete => "complete"
        }
    }
}

impl ToSql<crate::db::orm::schema::sql_types::CourtCasesDecisions, Pg> for CourtCasesDecisions {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            CourtCasesDecisions::Started => out.write_all(b"started")?,
            CourtCasesDecisions::Processing => out.write_all(b"processing")?,
            CourtCasesDecisions::Complete => out.write_all(b"complete")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::db::orm::schema::sql_types::CourtCasesDecisions, Pg> for CourtCasesDecisions {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"started" => Ok(CourtCasesDecisions::Started),
            b"processing" => Ok(CourtCasesDecisions::Processing),
            b"complete" => Ok(CourtCasesDecisions::Complete),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}