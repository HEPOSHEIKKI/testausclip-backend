use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}