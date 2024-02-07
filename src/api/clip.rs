

use actix_files;
use actix_web::{delete, error, get, http::header::{ContentDisposition, DispositionType, CONTENT_LENGTH}, post, web, put, HttpRequest, HttpResponse};
use actix_multipart:: Multipart ;
use futures_util::{StreamExt, TryStreamExt as _};
use mime::{Mime, APPLICATION_OCTET_STREAM};
use tokio::{fs, io::AsyncWriteExt as _};
use file_format::FileFormat;

use crate::{database::{self, clips::{create_clip, remove_clip}}, models::UpdateClip};
use crate::database::CreateClip;
use crate::database::RemoveClip;
use crate::database::clips::update_clip_meta;
use crate::database::clips::get_clip_meta;

//TODO Use JSON instead of headers


#[get("/v1/clip/get/{id}")]
pub async fn get_clip(path: web::Path<String>) -> Result<actix_files::NamedFile, actix_web::Error> {
    let id: String = path.to_string();
    let file_id = database::clips::get_clip_file(id.clone()).await;

    match file_id {
        Some(filename) => {
            let file = actix_files::NamedFile::open(format!("/home/otto/Videos/Clips/{}", filename))?;
            return Ok(file
                .use_last_modified(true)
                .set_content_disposition(ContentDisposition {
                    disposition: DispositionType::Attachment,
                    parameters: vec![],
                }));
        },
        None => {
            return Err(actix_web::error::ErrorNotFound(format!("No such clip with ID {}", id)));
        }
    }
}

#[get("/v1/clip/metadata/{id}")]
pub async fn get_metadata(clip: web::Path<String>) -> HttpResponse {
    let response = get_clip_meta(clip.to_string()).await;
    match response {
        Some(metadata) => {
            return HttpResponse::Ok().json(web::Json(metadata));
        },
        None => {
            return HttpResponse::NotFound().json("not found");
        }
    }
}

#[delete("/v1/clip/remove/{id}")]
    pub async fn remove_clip_file(path: web::Path<String>) -> HttpResponse {
        let id: String = path.to_string();

        let file_id = database::clips::get_clip_file(id.clone()).await;

        match file_id {
            Some(filename) => {
                let path = format!("/home/otto/Videos/Clips/{}", filename);

                let clip = RemoveClip{
                    id: id.clone()
                };

                let removed_post = remove_clip(clip).await;
                if removed_post.is_ok() {
                    match fs::remove_file(path).await {
                        Ok(_) => {
                            return HttpResponse::Ok().json(format!("Removed clip {}", id))
                        }
                        Err(_) => {
                            return HttpResponse::NotFound().json("No clip found with the associated id")
                        }
                    }
                }
                else {
                    return HttpResponse::InternalServerError().json("Something went wrong")
                }
            },
            None => {
                return HttpResponse::NotFound().json("No clip found with the associated id") //We are using 404 to avoid checking if a private clip exists by attempting to remove it. By keeping the responses as vague as possible, we can prevent this.
            }
        };


        
    }


#[post("/v1/clip/upload")]
pub async fn upload_clip(mut payload: Multipart, req: HttpRequest) -> HttpResponse {
    let max_file_size: usize = 300_000_000;
    let max_file_count: usize = 2;
    let legal_filetypes: [Mime; 1] = [APPLICATION_OCTET_STREAM];
    let dir: &str = "/home/otto/Videos/Clips/";

    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => 0,
    };

    if content_length == 0 || content_length > max_file_size {
        return HttpResponse::BadRequest().into();
    }

    let mut current_count: usize = 0;
    loop {
        if current_count >= max_file_count { break; }

        if let Ok(Some(mut field)) = payload.try_next().await {
            let filetype: Option<&Mime> = field.content_type();

            if field.name() != "upload"{
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
            }
            else {
                let mut post: CreateClip = CreateClip{
                    title: String::new(),
                    description: String::new()
                };
                if let Some(title) = req.headers().get("Title") {
                    if let Ok(title_str) = title.to_str() {
                        post.title = title_str.to_string();
                    }
                }
                else {
                    return HttpResponse::BadRequest().json("Missing required header: Title").into();
                }

                if let Some(description) = req.headers().get("Description") {
                    if let Ok(description_str) = description.to_str() {
                        post.description = description_str.to_string();
                    }
                }
                else {
                    return HttpResponse::BadRequest().json("Missing required header: Description").into();
                }

                
                let create_post = create_clip(post).await;

                let destination: String = format!(
                    "{}{}",
                    dir,
                    create_post.filename
                );

                let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();

                println!("{}", create_post.filename);
                let _ = saved_file.write_all(&in_memory_data).await.unwrap();
                return HttpResponse::Ok().json(create_post.id.clone());
            } 
        }
        current_count += 1;
    }

    HttpResponse::InternalServerError().into()

}


#[put("/v1/clip/update/{id}")]
pub async fn update_clip(path: web::Path<String>, mut payload: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    const MAX_SIZE: usize = 262_144_000;
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            dbg!( body.len() + chunk.len());
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    if body.len() < 1 {
        return Err(error::ErrorBadRequest("missing required content"));
    }
    let obj = serde_json::from_slice::<UpdateClip>(&body)?;
    let update = update_clip_meta(obj, path.to_string()).await;
    match update {
        Ok(()) => Ok(HttpResponse::Ok().json("success")),
        Err(_) => Err(error::ErrorBadRequest("malformed request body")),
    }
}