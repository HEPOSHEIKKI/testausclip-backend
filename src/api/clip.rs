use actix_files;
use actix_web::{get, http::header::{ContentDisposition, DispositionType, CONTENT_LENGTH}, web::{self}, HttpRequest, HttpResponse};
use actix_multipart::{ form, Field, Multipart };
use futures_util::TryStreamExt as _;
use mime::{Mime, Name, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG, MP4, MPEG, PNG, VIDEO};
use uuid::Uuid;
use tokio::{fs, io::AsyncWriteExt as _};

#[get("/clip/get/{id}")]
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
    let legal_filetypes: [Mime; 2] = [IMAGE_PNG, IMAGE_JPEG];
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

            let destination: String = format!(
                "{}{}-{}",
                dir,
                Uuid::new_v4(),
                field.content_disposition().get_filename().unwrap()
            );

            let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
            while let Ok(Some(chunk)) = field.try_next().await {
                let _ = saved_file.write_all(&chunk).await.unwrap();
            }
            

        }
        else {
            break;
        }

        current_count += 1
    }

    HttpResponse::Ok().into()

}
