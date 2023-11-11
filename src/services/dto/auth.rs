use serde::Deserialize;
use validator::{validate_email, validate_length, Validate, ValidationError};

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

fn email_or_username(value: &str) -> Result<(), ValidationError> {
    if validate_email(value) {
        return Ok(());
    }

    if validate_length(value, Some(3), Some(255), None) {
        return Ok(());
    }

    Err(ValidationError::new("email_or_username"))
}

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct AuthorizationDto {
    #[validate(custom = "email_or_username")]
    pub email_or_username: String,

    #[validate(length(min = 8, max = 32))]
    pub password: String,
}
