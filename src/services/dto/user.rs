use chrono::{NaiveDate, NaiveDateTime};
use diesel::Insertable;
use serde::Serialize;
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = crate::db::orm::schema::passports)]
pub struct PassportOrmData<'a> {
    pub first_name: &'a str,
    pub second_name: &'a str,
    pub patronymic: Option<&'a str>,
    pub birthday_date: NaiveDate,
}

#[derive(Serialize)]
pub struct LawProfileWithUser {
    pub uid: Uuid,
    pub avatar_uid: Option<Uuid>,
    pub law_uid: Uuid,
    pub first_name: String,
    pub second_name: String,
    pub patronymic: Option<String>,
    pub itn: String,
    pub start_activity_date: NaiveDateTime,
}
