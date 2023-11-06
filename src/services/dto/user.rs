use chrono::NaiveDate;
use diesel::Insertable;

#[derive(Insertable)]
#[diesel(table_name = crate::db::orm::schema::passports)]
pub struct PassportOrmData<'a> {
    pub first_name: &'a str,
    pub second_name: &'a str,
    pub patronymic: Option<&'a str>,
    pub birthday_date: NaiveDate,
}
