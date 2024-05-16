use actix_jwt_auth_middleware::AuthError;
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

use thiserror::Error;

impl From<AuthError> for ClipError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::NoToken => return ClipError::Unauthorized,
            AuthError::NoTokenSigner => return ClipError::SignerError,
            AuthError::TokenCreation(err) => return ClipError::TokenCreationError(err.to_string()),
            _ => return ClipError::JWTMiddleWareError,
        }
    }
}

#[allow(dead_code)]
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
    #[error("Missing access token")]
    UnauthroizedSecuredAccess,
    #[error("Invalid username or password")]
    InvalidCredentials,
    #[error("Username has to contain characters from [a-zA-Z0-9_] and has to be between 2 and 32 characters")]
    BadUsername,
    #[error("Bad id")]
    BadId,
    #[error("Something broke big time :( oops")]
    UnknownError,
    #[error("You are trying to register again after a short time")]
    TooManyRegisters,
    #[error("{0}")]
    InvalidLength(String),
    #[error("Token signer malfunctioned or invalid")]
    SignerError,
    #[error("{0}")]
    TokenCreationError(String),
    #[error("Generic JWT middleware error")]
    JWTMiddleWareError,
}

impl ResponseError for ClipError {
    fn status_code(&self) -> StatusCode {
        match self {
            ClipError::UserNotFound => {
                StatusCode::NOT_FOUND
            }
            ClipError::BadUsername
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

        match self.status_code() {

            // Match error 500 and strip any backend error messages

            StatusCode::INTERNAL_SERVER_ERROR => {
                HttpResponse::build(self.status_code())
                    .insert_header(ContentType::json())
                    .body("Internal Error".to_string())
            },
            _ => HttpResponse::build(self.status_code())
                    .insert_header(ContentType::json())
                    .body(self.to_string())
        }
    }
}