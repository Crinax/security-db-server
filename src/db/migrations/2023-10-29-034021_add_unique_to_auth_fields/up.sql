-- Your SQL goes here
ALTER TABLE auth_data ADD UNIQUE("email");
ALTER TABLE auth_data ADD UNIQUE("username");
