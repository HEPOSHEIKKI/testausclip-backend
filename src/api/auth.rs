use actix_web::{get, post, web, HttpResponse};
use crate::requests::RegisterRequest;


#[post("/v1/auth/login")]
pub async fn api_login(data: web::Json<RegisterRequest>) -> HttpResponse {
    HttpResponse::Ok().into()
}

#[post("/v1/auth/register")]
pub async fn api_register(data: web::Json<RegisterRequest>) -> HttpResponse {
    if data.password.len() < 8 || data.password.len() > 128 {
        return HttpResponse::BadRequest().json("Password must be between 8 and 128 characters long");
    }
    if !super::VALID_USERNAME_REGEX.is_match(&data.username) {
        return HttpResponse::BadRequest().json("Bad username");
    }

    HttpResponse::Ok().into()
    
    
}