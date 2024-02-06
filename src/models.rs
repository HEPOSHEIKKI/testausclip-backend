use crate::schema::clips;
use chrono::naive::NaiveDateTime;

#[derive(Insertable)]
#[table_name = "clips"]
pub struct NewClip<>{
    pub id: String,
    pub title: String,
    pub description: String,
    pub filename: String
}

#[derive(Debug, Queryable, AsChangeset, Selectable)]
#[table_name = "clips"]
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