use actix_web::{
    error::{BlockingError, ResponseError},
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClipError {
    #[error("Failed to connect to database connection pool")]
    DeadpoolError(#[from] diesel_async::pooled_connection::deadpool::PoolError),
    #[error("Diesel transaction failed `{0}`")]
    DieselError(#[from] diesel::result::Error),
    #[error("Internal server error")]
    DieselConnectionError(#[from] diesel::result::ConnectionError),
    #[error(transparent)]
    ActixError(#[from] actix_web::error::Error),
    #[error("User exists")]
    UserExists,
    #[error("User not found")]
    UserNotFound,
    #[error("You are not authorized")]
    Unauthorized,
    #[error("Missing secured access token")]
    UnauthroizedSecuredAccess,
    #[error("Invalid username or password")]
    InvalidCredentials,
    #[error("{0}")]
    InvalidLength(String),
    #[error("Username has to contain characters from [a-zA-Z0-9_] and has to be between 2 and 32 characters")]
    BadUsername,
    #[error("Leaderboard name has to contain characters from [a-zA-Z0-9_] and has to be between 2 and 32 characters")]
    BadLeaderboardName,
    #[error("Bad id")]
    BadId,
    #[error("Bad code")]
    BadCode,
    #[error("Something broke big time :( oops")]
    UnknownError,
    #[error("You are trying to register again after a short time")]
    TooManyRegisters,
    #[error("The user has no active session")]
    NotActive,
}

impl ResponseError for ClipError {
    fn status_code(&self) -> StatusCode {
        match self {
            ClipError::UserNotFound | ClipError::NotActive => {
                StatusCode::NOT_FOUND
            }
            ClipError::BadUsername
            | ClipError::InvalidLength(_)
            | ClipError::BadId
            | ClipError::UserExists => StatusCode::CONFLICT,
            ClipError::Unauthorized
            | ClipError::InvalidCredentials
            | ClipError::UnauthroizedSecuredAccess => StatusCode::UNAUTHORIZED,
            ClipError::TooManyRegisters => StatusCode::TOO_MANY_REQUESTS,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(json!({ "error": self.to_string() }).to_string())
    }
}