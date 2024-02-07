#[macro_use]
extern crate diesel;
extern crate dotenv;

use serde_derive::{Serialize, Deserialize};
use actix_web::{get, App, HttpServer, Responder};

mod api;
use api::clip::get_clip;
use api::clip::upload_clip;
use api::clip::remove_clip_file;
use api::clip::get_metadata;
use api::clip::update_clip;

mod database;

mod schema;
mod models;

#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    address: String,
    port: i16
}



#[get("/ping")]
async fn ping() -> impl Responder {
    "Pong!"
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {   
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(get_clip)
            .service(get_metadata)
            .service(upload_clip)
            .service(remove_clip_file)
            .service(update_clip)

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}