use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct RegistrationDto {
    #[validate(email(message = "Must be valid email address"))]
    email: String,

    #[validate(length(min = 3, max = 255, message = "Must has length between 3 and 255"))]
    login: String,

    #[validate(length(min = 8, max = 32, message = "Must has length between 8 and 32"))]
    password: String,
}
