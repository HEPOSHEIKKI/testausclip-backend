use crate::schema::clips;
use crate::schema::likes;
use crate::schema::users;
use crate::schema::user_identities;
use chrono::naive::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};

#[derive(Insertable)]
#[diesel(table_name = clips)]
pub struct NewClip {
    pub id: String,
    pub title: String,
    pub description: String,
    pub file_name: String,
}

#[derive(Debug, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = clips)]
pub struct Clip {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub private: Option<bool>,
    pub owner_id: Option<String>,
    pub game: Option<String>,
    pub upload_date: Option<NaiveDateTime>,
    pub file_name: Option<String>,
}

#[derive(Queryable, AsChangeset, Selectable, Debug, Serialize)]
#[diesel(table_name = clips)]
pub struct ClipMeta {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub owner_id: Option<String>,
    pub private: Option<bool>,
    pub game: Option<String>,
    pub upload_date: Option<NaiveDateTime>,
}

#[derive(Queryable, AsChangeset, Selectable)]
#[diesel(table_name = clips)]
pub struct ClipFile {
    pub file_name: Option<String>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = clips)]
pub struct UpdateClip {
    pub title: String,
    pub description: String,
    pub private: Option<bool>,
    pub game: Option<String>,
}

#[derive(Insertable, Queryable, Debug)]
#[diesel(table_name = likes)]
pub struct Like {
    pub clip_id: String,
    pub user_id: String,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: String,
    pub username: String,
    pub password: Vec<u8>,
    pub salt: Vec<u8>,
    pub auth_token: String,
}

#[derive(Insertable, Queryable, AsChangeset, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: Vec<u8>,
    pub salt: Vec<u8>,
    pub auth_token: String,
    pub registration_date: Option<NaiveDateTime>,
}

#[derive(Identifiable, Queryable, Clone, Debug, Serialize, PartialEq, Eq)]
#[diesel(table_name = user_identities)]
pub struct UserIdentity {
    pub id: String,
    #[serde(skip_serializing)]
    pub auth_token: String,
    pub username: String,
    pub registration_date: Option<NaiveDateTime>,
}

#[derive(Insertable, Queryable, AsChangeset, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct UserName {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct UserId {
    pub id: String,
}