use actix_files;
use actix_multipart::Multipart;
use actix_web::{
    delete, error, get,
    http::header::{ContentDisposition, DispositionType, CONTENT_LENGTH},
    post, put, web, HttpRequest, HttpResponse,
};
use file_format::FileFormat;
use futures_util::{StreamExt, TryStreamExt as _};
use mime::{Mime, APPLICATION_OCTET_STREAM};

use crate::database::clips::db_get_clip_meta;
use crate::database::clips::db_remove_like;
use crate::database::clips::db_update_clip_meta;
use crate::database::CreateClip;
use crate::database::RemoveClip;
use crate::storage::clips::remove_clip_file;
use crate::{database::clips::db_add_like, models::Like, storage::clips::write_clip_to_file};
use crate::{
    database::{
        self,
        clips::{db_create_clip, db_remove_clip},
    },
    models::UpdateClip,
};

//TODO Use JSON instead of headers

#[get("/v1/clip/get/{id}")]
pub async fn api_get_clip(
    id: web::Path<String>,
) -> Result<actix_files::NamedFile, actix_web::Error> {
    let file_id = database::clips::db_get_clip_file(id.clone()).await;

    match file_id {
        Some(filename) => {
            let file =
                actix_files::NamedFile::open(format!("/home/otto/Videos/Clips/{}", filename))?;
            return Ok(file
                .use_last_modified(true)
                .set_content_disposition(ContentDisposition {
                    disposition: DispositionType::Attachment,
                    parameters: vec![],
                }));
        }
        None => {
            return Err(actix_web::error::ErrorNotFound(format!(
                "No such clip with ID {}",
                id
            )));
        }
    }
}

#[get("/v1/clip/metadata/{id}")]
pub async fn api_get_metadata(id: web::Path<String>) -> HttpResponse {
    let response = db_get_clip_meta(id.to_string()).await;
    match response {
        Some(metadata) => {
            return HttpResponse::Ok().json(web::Json(metadata));
        }
        None => {
            return HttpResponse::NotFound().json("not found");
        }
    }
}

#[delete("/v1/clip/remove/{id}")]
pub async fn api_remove_clip(id: web::Path<String>) -> HttpResponse {
    let file_id = database::clips::db_get_clip_file(id.clone()).await;

    match file_id {
        Some(filename) => {
            let clip = RemoveClip { id: id.clone() };

            let removed_post = db_remove_clip(clip).await;
            if removed_post.is_ok() {
                match remove_clip_file(filename).await {
                    Ok(_) => return HttpResponse::Ok().json(format!("Removed clip {}", id)),
                    Err(_) => {
                        return HttpResponse::NotFound()
                            .json("No clip found with the associated id")
                    }
                }
            } else {
                return HttpResponse::InternalServerError().json("Something went wrong");
            }
        }
        None => {
            return HttpResponse::NotFound().json("No clip found with the associated id");
            //We are using 404 to avoid checking if a private clip exists by attempting to remove it. By keeping the responses as vague as possible, we can prevent this.
        }
    };
}

// I would use JSON here to specify the metadata, but HTTP doesn't allow for multipart data to be sent alongside JSON data. I think using headers for this is fine, we're not building rockets here!
// Alternatives I've considered: Saving the clip without any metadata and sending back the ID. The user then has to set the metadata of the clip with /clip/update before the clip can be publicly accessed.

#[post("/v1/clip/upload")]
pub async fn api_upload_clip(mut payload: Multipart, req: HttpRequest) -> HttpResponse {
    let max_file_size: usize = 300_000_000;
    let max_file_count: usize = 2;
    let legal_filetypes: [Mime; 1] = [APPLICATION_OCTET_STREAM];

    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => 0,
    };

    if content_length == 0 || content_length > max_file_size {
        return HttpResponse::BadRequest().into();
    }

    let mut current_count: usize = 0;
    loop {
        if current_count >= max_file_count {
            break;
        }

        if let Ok(Some(mut field)) = payload.try_next().await {
            let filetype: Option<&Mime> = field.content_type();

            if field.name() != "upload" {
                continue;
            }

            if filetype.is_none() {
                continue;
            }
            if !legal_filetypes.contains(&filetype.unwrap()) {
                continue;
            }

            let mut in_memory_data: Vec<u8> = Vec::new();

            while let Ok(Some(chunk)) = field.try_next().await {
                in_memory_data.extend_from_slice(&chunk);
            }

            let format: FileFormat = FileFormat::from_bytes(&in_memory_data);
            if format != FileFormat::Mpeg4Part14 {
                return HttpResponse::BadRequest().into();
            } else {
                let mut post: CreateClip = CreateClip {
                    title: String::new(),
                    description: String::new(),
                };

                if let Some(title) = req.headers().get("Title") {
                    if let Ok(title_str) = title.to_str() {
                        post.title = title_str.to_string();
                    }
                } else {
                    return HttpResponse::BadRequest()
                        .json("Missing required header: Title")
                        .into();
                }

                if let Some(description) = req.headers().get("Description") {
                    if let Ok(description_str) = description.to_str() {
                        post.description = description_str.to_string();
                    }
                } else {
                    return HttpResponse::BadRequest()
                        .json("Missing required header: Description")
                        .into();
                }

                let create_post = db_create_clip(post).await;

                write_clip_to_file(create_post.filename, &in_memory_data).await;
                return HttpResponse::Ok().json(create_post.id.clone());
            }
        }
        current_count += 1;
    }

    HttpResponse::InternalServerError().into()
}

#[put("/v1/clip/update/{id}")]
pub async fn api_update_clip(
    id: web::Path<String>,
    mut payload: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    const MAX_SIZE: usize = 262_144_000;
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            dbg!(body.len() + chunk.len());
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    if body.len() < 1 {
        return Err(error::ErrorBadRequest("missing required content"));
    }
    let update_data = serde_json::from_slice::<UpdateClip>(&body)?;
    let update = db_update_clip_meta(update_data, id.to_string()).await;
    match update {
        Ok(()) => Ok(HttpResponse::Ok().into()),
        Err(_) => Err(error::ErrorBadRequest("malformed request body")),
    }
}

#[post("/v1/clip/like/{id}")]
pub async fn api_add_like(id: web::Path<String>, req: HttpRequest) -> HttpResponse {
    let auth = req.headers().get("Authorization").cloned();
    match auth {
        Some(user_id) => {
            let like: Like = Like {
                clipid: id.to_string(),
                userid: user_id.to_str().unwrap().to_string(),
            };
            let add_like = db_add_like(like).await;
            match add_like {
                Ok(0) => return HttpResponse::Ok().into(),
                Ok(1) => return HttpResponse::Ok().json("already liked"),
                Ok(_) => return HttpResponse::InternalServerError().into(),
                Err(_) => return HttpResponse::InternalServerError().into(),
            }
        }
        None => return HttpResponse::Unauthorized().json("Missing required header: Authorization"),
    }
}

#[delete("/v1/clip/like/{id}")]
pub async fn api_remove_like(id: web::Path<String>, req: HttpRequest) -> HttpResponse {
    let auth = req.headers().get("Authorization").cloned(); // NOT VALID AUTHORIZATION, THIS IS FOR TESTING PURPOSES
    match auth {
        Some(user_id) => {
            let like: Like = Like {
                clipid: id.to_string(),
                userid: user_id.to_str().unwrap().to_string(),
            };
            let add_like = db_remove_like(like).await;
            match add_like {
                Ok(_) => return HttpResponse::Ok().into(),
                Err(_) => return HttpResponse::InternalServerError().into(),
            };
        }
        None => return HttpResponse::Unauthorized().json("Missing required header: Authorization"),
    }
}
