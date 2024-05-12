use std::time::Duration;

use actix_jwt_auth_middleware::use_jwt::UseJWTOnScope;
use actix_jwt_auth_middleware::{Authority, FromRequest, TokenSigner};

use actix_web::{web, App, HttpServer};
use ed25519_compact::KeyPair;
use jwt_compact::alg::Ed25519;
use serde::{Deserialize, Serialize};

mod api;
use api::clip::{get_clip, upload_clip};
use api::auth::login;


#[derive(Serialize, Deserialize, Debug, Clone, FromRequest)]
struct User {
    id: u32,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let KeyPair {
        pk: public_key,
        sk: secret_key,
    } = KeyPair::generate();

    HttpServer::new(move || {
        let authority = Authority::<User, Ed25519, _, _>::new()
            .refresh_authorizer(|| async move { Ok(()) })
            .token_signer(Some(
                TokenSigner::new()
                    .signing_key(secret_key.clone())
                    .algorithm(Ed25519)
                    .access_token_lifetime(Duration::from_secs(1200))
                    .refresh_token_lifetime(Duration::from_secs(129600 * 60))
                    .build()
                    .expect(""),
            ))
            .verifying_key(public_key)
            .build()
            .expect("");

        App::new()
            .service(web::scope("/v1")
                .service(login)
                .service(get_clip)
                .use_jwt(authority.clone(), web::scope("")
                    .service(upload_clip)
                ))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}