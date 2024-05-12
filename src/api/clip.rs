use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpRequest, Responder};



#[get("/clip/get/{id}")]
pub async fn get_clip(id: web::Path<String>) -> impl Responder {
    format!("{}", id)
}

#[post("/clip/upload")]
pub async fn upload_clip(mut _payload: Multipart,_req: HttpRequest) -> impl Responder {
    format!("upload")
}