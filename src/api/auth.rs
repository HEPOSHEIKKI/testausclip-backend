use actix_jwt_auth_middleware::{AuthResult, TokenSigner};
use actix_web::{get, post, web, HttpResponse, Responder};
use jwt_compact::alg::Ed25519;

use crate::database::DatabaseWrapper;
use crate::error::ClipError;
use crate::User;
use crate::requests::RegisterRequest;



#[get("/auth/login")]
async fn login(cookie_signer: web::Data<TokenSigner<User, Ed25519>>) -> AuthResult<HttpResponse> {
    let user = User { id: 1 };
    Ok(HttpResponse::Ok()
        .cookie(cookie_signer.create_access_cookie(&user)?)
        .cookie(cookie_signer.create_refresh_cookie(&user)?)
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