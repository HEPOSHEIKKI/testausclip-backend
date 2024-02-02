use actix_files;
use actix_web::{get, http::header::{ContentDisposition, DispositionType}, web};

#[get("/clip/{id}")]
pub async fn get_clip(path: web::Path<u32>) -> Result<actix_files::NamedFile, actix_web::Error> {
    let id: String = path.to_string();
    let file = actix_files::NamedFile::open(format!("/home/otto/Videos/{}.mp4", id))?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}

#[get("/users/{user_id}/{friend}")] // <- define path parameters
pub async fn index(path: web::Path<(u32, String)>) -> Result<String, actix_web::Error> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}