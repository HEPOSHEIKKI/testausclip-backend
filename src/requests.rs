use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}