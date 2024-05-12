use actix_jwt_auth_middleware::{AuthResult, TokenSigner};
use actix_web::{get, web, HttpResponse};
use jwt_compact::alg::Ed25519;

use crate::User;




#[get("/auth/login")]
async fn login(cookie_signer: web::Data<TokenSigner<User, Ed25519>>) -> AuthResult<HttpResponse> {
    let user = User { id: 1 };
    Ok(HttpResponse::Ok()
        .cookie(cookie_signer.create_access_cookie(&user)?)
        .cookie(cookie_signer.create_refresh_cookie(&user)?)
        .body("You are now logged in"))
}