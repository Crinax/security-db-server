use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct RegistrationDto {
    #[validate(email)]
    email: String,

    #[validate(length(min = 3, max = 255))]
    login: String,

    #[validate(length(min = 8, max = 32))]
    password: String,
}
