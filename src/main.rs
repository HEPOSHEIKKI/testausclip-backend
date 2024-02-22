#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;

use actix_web::web::Data;
use actix_web::{get, App, HttpServer, Responder};
use database::Database;
use serde_derive::{Deserialize, Serialize};

mod api;
// use api::clip::api_add_like;
// use api::clip::api_get_clip;
// use api::clip::api_get_metadata;
// use api::clip::api_remove_clip;
// use api::clip::api_remove_like;
// use api::clip::api_update_clip;
// use api::clip::api_upload_clip;

use api::auth::api_login;
use api::auth::api_register;

mod database;
mod models;
mod schema;
mod storage;
mod requests;
mod error;


#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    address: String,
    port: i16,
}

#[get("/ping")]
async fn ping() -> impl Responder {
    "Pong!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();
    let database = Data::new(Database::new(env::var("DATABASE_URL").expect("DATABASE_URL must be set")));

    HttpServer::new(move || {
        App::new()
            .service(api_login)
            .service(api_register)
            .service(ping)
            .app_data(Data::clone(&database))
            // .service(api_get_clip)
            // .service(api_get_metadata)
            // .service(api_upload_clip)
            // .service(api_remove_clip)
            // .service(api_update_clip)
            // .service(api_add_like)
            // .service(api_remove_like)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
