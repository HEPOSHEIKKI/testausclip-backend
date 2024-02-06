//use std::{fmt::Error, io::Error};


use crate::models::NewClip;
use super::{establish_connection, CreateClip, RemoveClip};
use diesel::{prelude::*, query_dsl::methods::FilterDsl};
use uuid::Uuid;
use ulid::Ulid;

use crate::models::Clip;


pub async fn create_clip(clip: CreateClip) -> NewClip {
    use crate::schema::clips::dsl::*;

    let connection = &mut establish_connection();
    let uid = Uuid::new_v4().to_string();
    let fileulid = Ulid::new();



    let new_clip = NewClip {
        id: uid.clone(),
        title: clip.title.clone(),
        description: clip.description.clone(),
        filename: format!("{}.mp4", fileulid)
    };

    diesel::insert_into(clips)
        .values(&new_clip)
        .execute(connection)
        .expect("Error saving new clip");

    println!("{}", "insert clip");
    new_clip
}

pub async fn remove_clip(clip: RemoveClip) -> Result<String, ()>{
    use crate::schema::clips::dsl::*;
    let connection = &mut establish_connection();
    let uid = clip.id;
    let get_file_name = get_clip_file(uid.clone()).await;

    match get_file_name {
        Some(file_name) => {
            let deletion_result: Result<usize, diesel::result::Error> = Ok(diesel::delete(FilterDsl::filter(clips, id.like(uid)))
                .execute(connection)
                .expect("Error removing clip from database"));
            if deletion_result == Ok(1) {
                return Ok(file_name);
            }
            else {
                return Err(());
            }
        },
        None => {
            return Err(());
        }
    }
}

pub async fn get_clip_file(clip: String) -> Option<String>{
    use crate::schema::clips::dsl::*;
    // HOW THE FUCK DO I GET SHIT
    let connection = &mut establish_connection();

    let post = clips.find(clip)
        .select(Clip::as_select())
        .first(connection)
        .optional();

    match post {
    Ok(Some(clip)) => {
        let kys = clip.filename;
        return kys;
    },
    Ok(None) => {
        return None
    },
    Err(_) => {
        return None
    }
}


}