use crate::schema::posts;

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a>{
    pub id: &'a String,
    pub title: &'a str,
    pub description: &'a str,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub description: String,
    pub ownerid: i32,
    pub private: bool
}