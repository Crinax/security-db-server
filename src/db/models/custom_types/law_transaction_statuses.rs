use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};
use serde::Serialize;
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow, Serialize)]
#[diesel(sql_type = crate::db::orm::schema::sql_types::LawTransactionsStatues)]
pub enum LawTransactionsStatues {
    #[serde(rename = "started")]
    Started,

    #[serde(rename = "processing")]
    Processing,

    #[serde(rename = "complete")]
    Complete
}

impl<'a> From<LawTransactionsStatues> for &'a str {
    fn from(value: LawTransactionsStatues) -> &'a str {
        match value {
            LawTransactionsStatues::Started => "started",
            LawTransactionsStatues::Processing => "processing",
            LawTransactionsStatues::Complete => "complete"
        }
    }
}

impl ToSql<crate::db::orm::schema::sql_types::LawTransactionsStatues, Pg> for LawTransactionsStatues {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            LawTransactionsStatues::Started => out.write_all(b"started")?,
            LawTransactionsStatues::Processing => out.write_all(b"processing")?,
            LawTransactionsStatues::Complete => out.write_all(b"complete")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::db::orm::schema::sql_types::LawTransactionsStatues, Pg> for LawTransactionsStatues {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"started" => Ok(LawTransactionsStatues::Started),
            b"processing" => Ok(LawTransactionsStatues::Processing),
            b"complete" => Ok(LawTransactionsStatues::Complete),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}