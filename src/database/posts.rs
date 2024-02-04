use crate::models::{NewPost, Post};
use super::{establish_connection, CreatePost};
use diesel::prelude::*;
use uuid::Uuid;


pub async fn create_post(post: CreatePost) -> String {
    use crate::schema::posts::dsl::*;
    use crate::models::Post;

    let connection = &mut establish_connection();
    let uid = Uuid::new_v4().to_string();


    let new_post = NewPost {
        id: &uid,
        title: &post.title,
        description: &post.description,
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(connection)
        .expect("Error saving new post");

    println!("{}", "insert post");
    uid

}