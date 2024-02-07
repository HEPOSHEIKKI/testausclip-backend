use crate::schema::clips;
use crate::schema::likes;
use chrono::naive::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};

#[derive(Insertable)]
#[diesel(table_name = clips)]
pub struct NewClip {
    pub id: String,
    pub title: String,
    pub description: String,
    pub filename: String,
}

#[derive(Debug, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = clips)]
pub struct Clip {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub private: Option<bool>,
    pub ownerid: Option<String>,
    pub game: Option<String>,
    pub uploaddate: Option<NaiveDateTime>,
    pub filename: Option<String>,
}

#[derive(Queryable, AsChangeset, Selectable, Debug, Serialize)]
#[diesel(table_name = clips)]
pub struct ClipMeta {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub ownerid: Option<String>,
    pub private: Option<bool>,
    pub game: Option<String>,
    pub uploaddate: Option<NaiveDateTime>,
}

#[derive(Queryable, AsChangeset, Selectable)]
#[diesel(table_name = clips)]
pub struct ClipFile {
    pub filename: Option<String>,
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
    pub clipid: String,
    pub userid: String,
}
