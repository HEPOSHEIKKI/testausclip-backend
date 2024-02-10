use actix_web::{post, web, HttpResponse};
use crate::requests::RegisterRequest;

use crate::database::auth::user_exists;


#[post("/v1/auth/login")]
pub async fn api_login(_data: web::Json<RegisterRequest>) -> HttpResponse {
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
    else if user_exists(data.username.clone()).await.unwrap() {
        return HttpResponse::BadRequest().json("Username is taken");
    }

    

    HttpResponse::Ok().into()
    
    
}