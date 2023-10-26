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
    pub number: Option<String>,
    pub series: Option<String>,
    pub registration_place: Option<String>,
    pub birthday_date: NaiveDate,
}
