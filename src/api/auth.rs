use actix_jwt_auth_middleware::TokenSigner;
use actix_web::{get, post, web, HttpResponse, Responder};
use jwt_compact::alg::Ed25519;

use crate::database::DatabaseWrapper;
use crate::error::ClipError;
use crate::models::UserClaims;
use crate::requests::{RegisterRequest, LoginRequest};




#[get("/auth/login")]
async fn login(cookie_signer: web::Data<TokenSigner<UserClaims, Ed25519>>, db: DatabaseWrapper, data: web::Json<LoginRequest>) -> Result<impl Responder, ClipError> {
    if data.password.len() > 128 {
        return Err(ClipError::InvalidLength(
            "Password cannot be longer than 128 characters".to_string(),
        ));
    }

    let user = db.verify_user_password(&data.username_or_email.as_str(), &data.password.as_str()).await?;

    let claims = UserClaims::from(user.unwrap());

    Ok(HttpResponse::Ok()
        .cookie(cookie_signer.create_access_cookie(&claims)?)
        .cookie(cookie_signer.create_refresh_cookie(&claims)?)
        .body("You are now logged in"))
}


//TODO Check for username and/or email clashes
#[post("/auth/register")]
pub async fn register(data: web::Json<RegisterRequest>, db: DatabaseWrapper) -> Result<impl Responder, ClipError> {
    if data.password.len() < 8 || data.password.len() > 128 {
        return Ok(HttpResponse::BadRequest().json("Password must be between 8 and 128 characters long"));
    }
    if !super::VALID_USERNAME_REGEX.is_match(&data.username) {
        return Ok(HttpResponse::BadRequest().json("Bad username"));
    }

    //TODO Fucking uhhh do the like error handling and shit???

    db.user_exists(data.username.clone()).await?;
    let res = db.register_user(data.username.clone(), data.password.clone(), data.email.clone()).await;
    match res {
        Ok(_) => Ok(HttpResponse::Ok().into()),
        Err(e) => return Err(e),
    }
    
}