//use std::{fmt::Error, io::Error};


use crate::models::Like;
use crate::models::NewClip;
use crate::models::ClipMeta;
use crate::models::ClipFile;
use crate::models::UpdateClip;
use super::{establish_connection, CreateClip, RemoveClip};
use diesel::result::DatabaseErrorKind;

use diesel::{prelude::*, query_dsl::methods::FilterDsl};
use uuid::Uuid;
use ulid::Ulid;



pub async fn db_create_clip(clip: CreateClip) -> NewClip {
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

pub async fn db_remove_clip(clip: RemoveClip) -> Result<String, ()>{
    use crate::schema::clips::dsl::*;
    let connection = &mut establish_connection();
    let uid = clip.id;
    let get_file_name = db_get_clip_file(uid.clone()).await;

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


pub async fn db_get_clip_file(clip_id: String) -> Option<String>{
    use crate::schema::clips::dsl::*;
    let connection = &mut establish_connection();

    let post = clips.find(clip_id)
        .select(ClipFile::as_select())
        .first(connection)
        .optional();

    match post {
    Ok(Some(clip_id)) => {
        let name = clip_id.filename;
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

pub async fn db_get_clip_meta(clip_id: String) -> Option<ClipMeta> {
    use crate::schema::clips::dsl::*;

    let connection = &mut establish_connection();

    let metadata = clips.find(clip_id)
        .select(ClipMeta::as_select())
        .first(connection)
        .optional();

    match metadata {
        Ok(Some(clip_id)) => {
            return Some(clip_id);
        },
        Ok(None) => {
            return None;
        },
        Err(_) => {
            return None;
        }
    }
}

pub async fn db_update_clip_meta(clip: UpdateClip, clip_id: String) -> Result<(), ()> {
    use crate::schema::clips::dsl::*;

    let connection = &mut establish_connection();
    let retrieve_clip = UpdateClip {
        title: clip.title,
        description: clip.description,
        private: clip.private,
        game: clip.game
    };
    let update = diesel::update(clips.find(clip_id))
            .set(&retrieve_clip)
            .execute(connection)
            .expect("Error updating clip");

    if update != 1 {
        return Err(());
    }

    Ok(())
}

pub async fn db_add_like(like: Like) -> Result<u32, ()> {
    
    use crate::schema::likes::dsl::*;

    let connection = &mut establish_connection();

    let add_like = diesel::insert_into(likes)
        .values(&like)
        .execute(connection)
        .optional();
    match add_like {
        Ok(Some(_)) => return Ok(0),
        Ok(None) => return Ok(0),
        Err(e) => {
            match e {
                diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    return Ok(1);
                }
                _ => return Err(()),
            }
        }
    };
}

pub async fn db_remove_like(like: Like) -> Result<(), ()> {
    
    use crate::schema::likes::dsl::*;

    let connection = &mut establish_connection();

    let remove_like = diesel::delete(FilterDsl::filter(likes, clipid.eq(like.clipid).and(userid.eq(like.userid))))
        .execute(connection)
        .expect("Could not remove like");
    if remove_like != 1 {
        return Err(());
    }
    return Ok(());
}