use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct RegistrationDto {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 3, max = 255))]
    pub username: String,

    #[validate(length(min = 8, max = 32))]
    pub password: String,

    pub first_name: String,
    pub second_name: String,
    pub patronymic: Option<String>,
    pub birth_date: chrono::NaiveDate,
}
