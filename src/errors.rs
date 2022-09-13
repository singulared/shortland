use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::{backend::BackendError, shortener::ShortnerError};

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    Backend(#[from] BackendError),
    #[error(transparent)]
    Sortner(#[from] ShortnerError),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        match self {
            ServiceError::Backend(BackendError::NotFound) => StatusCode::NOT_FOUND.into_response(),
            ServiceError::Sortner(ShortnerError::Decode(_))
            | ServiceError::Backend(BackendError::DateTimeOverflow) => {
                StatusCode::BAD_REQUEST.into_response()
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response(),
        }
    }
}
