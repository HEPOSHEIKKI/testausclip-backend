use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use actix_jwt_auth_middleware::FromRequest;

use crate::schema::users;

//  ___   _ _____ _   ___   _   ___ ___ 
// |   \ /_\_   _/_\ | _ ) /_\ / __| __|
// | |) / _ \| |/ _ \| _ \/ _ \\__ \ _| 
// |___/_/ \_\_/_/ \_\___/_/ \_\___/___|

#[derive(Identifiable, Queryable, Clone, Debug, Serialize, PartialEq, Eq)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub password: Vec<u8>,
    #[serde(skip_serializing)]
    pub salt: Vec<u8>,
    pub registration_date: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRequest)]
pub struct UserClaims {
    pub sub: String,
    pub name: String,
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Clone, Selectable)]
#[diesel(table_name = users)]
pub struct RegisterUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: Vec<u8>,
    pub salt: Vec<u8>,
}

impl From<User> for UserClaims {
    fn from(u: User) -> UserClaims {
        UserClaims {
            sub: u.id,
            name: u.username,
        }
    }
}