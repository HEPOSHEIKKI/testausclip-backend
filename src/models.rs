use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::users;


#[derive(Insertable, Queryable, AsChangeset, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: Vec<u8>,
    pub salt: Vec<u8>,
    pub registration_date: chrono::NaiveDateTime,
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