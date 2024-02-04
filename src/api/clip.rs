
use std::path;

use actix_files;
use actix_web::{get, http::header::{ContentDisposition, DispositionType, CONTENT_LENGTH}, web::{self}, HttpRequest, HttpResponse};
use actix_multipart:: Multipart ;
use futures_util::TryStreamExt as _;
use mime::{Mime, APPLICATION_OCTET_STREAM};
use uuid::Uuid;
use tokio::{fs, io::AsyncWriteExt as _};
use file_format::FileFormat;

use crate::{database::posts::create_post, models::NewPost};
use crate::database::CreatePost;

#[get("/api/clip/get/{id}")]
pub async fn get_clip(path: web::Path<String>) -> Result<actix_files::NamedFile, actix_web::Error> {
    let id: String = path.to_string();
    let file = actix_files::NamedFile::open(format!("/home/otto/Videos/Clips/{}.mp4", id))?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}


pub async fn upload(mut payload: Multipart, req: HttpRequest) -> HttpResponse {
    let max_file_size: usize = 300_000_000;
    let max_file_count: usize = 1;
    let legal_filetypes: [Mime; 1] = [APPLICATION_OCTET_STREAM];
    let dir: &str = "/home/otto/Videos/Clips/";

    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => 0,
    };

    if content_length == 0 || content_length > max_file_size {
        return HttpResponse::BadRequest().into();
    }

    let mut new_uuid: String = String::new();


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
            let destination: String = format!(
                "{}{}.mp4",
                dir,
                new_uuid
            );

            let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
            let mut in_memory_data: Vec<u8> = Vec::new();

            
            while let Ok(Some(chunk)) = field.try_next().await {
                in_memory_data.extend_from_slice(&chunk);
            }
            
            let format: FileFormat = FileFormat::from_bytes(&in_memory_data);
            if format != FileFormat::Mpeg4Part14 {
                return HttpResponse::BadRequest().into();
            }
            else {
                let mut post: CreatePost = CreatePost{
                    title: String::new(),
                    description: String::new()
                };
                if let Some(title) = req.headers().get("Title") {
                    if let Ok(title_str) = title.to_str() {
                        post.title = title_str.to_string();
                    }
                }
                else {
                    return HttpResponse::BadRequest().body("Missing required header: Title\n").into();
                }

                if let Some(description) = req.headers().get("Description") {
                    if let Ok(description_str) = description.to_str() {
                        post.description = description_str.to_string();
                    }
                }
                else {
                    return HttpResponse::BadRequest().body("Missing required header: Description\n").into();
                }

                
                let create_post = create_post(post).await;
                new_uuid = create_post.clone();
                println!("{}", create_post);
                let _ = saved_file.write_all(&in_memory_data).await.unwrap();
            }
            

            

        }
        else {
            break;
        }

        current_count += 1
    }

    HttpResponse::Ok().body(new_uuid.to_string())

}
