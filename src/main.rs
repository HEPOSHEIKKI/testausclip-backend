use std::time::Duration;

use actix_jwt_auth_middleware::use_jwt::UseJWTOnScope;
use actix_jwt_auth_middleware::{Authority, FromRequest, TokenSigner};

use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use database::Database;

use ed25519_compact::KeyPair;
use jwt_compact::alg::Ed25519;
use serde::{Deserialize, Serialize};

mod api;
use api::clip::{get_clip, upload_clip};
use api::auth::{login, register};

mod config;
mod requests;
mod models;
mod database;
mod schema;
mod error;


#[derive(Serialize, Deserialize, Debug, Clone, FromRequest)]
struct User {
    id: u32,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = config::read_config("config.toml");

    let database = Data::new(Database::new(format!("postgres://{}:{}@{}:{}/testausclip", config.database.username, config.database.password, config.database.address, config.database.port)));

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
                .service(register)
                .service(get_clip)
                .use_jwt(authority.clone(), web::scope("")
                    .service(upload_clip)
                ))
            .app_data(Data::clone(&database))
    })
    .bind((config.testausclip.ip, config.testausclip.port))?
    .run()
    .await?;

    Ok(())
}