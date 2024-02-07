//use std::{fmt::Error, io::Error};


use crate::models::NewClip;
use crate::models::ClipMeta;
use crate::models::ClipFile;
use crate::models::UpdateClip;
use super::{establish_connection, CreateClip, RemoveClip};
use diesel::{prelude::*, query_dsl::methods::FilterDsl};
use uuid::Uuid;
use ulid::Ulid;



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
    let connection = &mut establish_connection();

    let post = clips.find(clip)
        .select(ClipFile::as_select())
        .first(connection)
        .optional();

    match post {
    Ok(Some(clip)) => {
        let name = clip.filename;
        return name;
    },
    Ok(None) => {
        return None
    },
    Err(_) => {
        return None
    }
}


}

pub async fn get_clip_meta(clip: String) -> Option<ClipMeta> {
    use crate::schema::clips::dsl::*;

    let connection = &mut establish_connection();

    let metadata = clips.find(clip)
        .select(ClipMeta::as_select())
        .first(connection)
        .optional();

    match metadata {
        Ok(Some(clip)) => {
            return Some(clip);
        },
        Ok(None) => {
            return None;
        },
        Err(_) => {
            return None;
        }
    }
}

pub async fn update_clip_meta(clip: UpdateClip, clip_id: String) -> Result<(), ()> {
    use crate::schema::clips::dsl::*;

    let connection = &mut establish_connection();
    let db_clip = UpdateClip {
        title: clip.title,
        description: clip.description,
        private: clip.private,
        game: clip.game
    };
    let update = diesel::update(clips.find(clip_id))
            .set(&db_clip)
            .execute(connection)
            .expect("Error updating clip");

    if update != 1 {
        return Err(());
    }

    Ok(())
}