use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub username_or_email: String,
    pub password: String,
    pub totp_2fa_token: Option<String>,
}