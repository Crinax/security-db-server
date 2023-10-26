use chrono::NaiveDate;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Debug)]
#[diesel(table_name = crate::db::orm::schema::passports)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Passport {
    pub uid: Uuid,
    pub first_name: String,
    pub second_name: String,
    pub patronymic: Option<String>,
    pub number: String,
    pub series: String,
    pub registration_place: String,
    pub birthday_date: NaiveDate,
}
