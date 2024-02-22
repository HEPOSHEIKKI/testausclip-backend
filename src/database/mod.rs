// pub mod clips;
pub mod auth;

use std::{future::Future, pin::Pin, sync::Arc};

use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use diesel_async::{
    pooled_connection::{
        deadpool::{Object, Pool},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};

use crate::error::ClipError;

pub struct CreateClip {
    pub title: String,
    pub description: String,
}

pub struct RemoveClip {
    pub id: String,
}

type DatabaseConnection = Object<AsyncPgConnection>;

pub struct Database {
    backend: Pool<AsyncPgConnection>,
}

pub struct DatabaseWrapper {
    db: Arc<Database>,
}

impl FromRequest for DatabaseWrapper {
    type Error = ClipError;
    type Future = Pin<Box<dyn Future<Output = actix_web::Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let wrapper = DatabaseWrapper {
            db: req
                .app_data::<Data<Database>>()
                .unwrap()
                .clone()
                .into_inner(),
        };

        Box::pin(async move { Ok(wrapper) })
    }
}

impl Database {
    async fn get(&self) -> Result<DatabaseConnection, ClipError> {
        Ok(self.backend.get().await?)
    }

    pub fn new(url: String) -> Self {
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);

        let pool = Pool::builder(manager)
            .build()
            .expect("Failed to create connection pool");

        Self { backend: pool }
    }
}