use std::error::Error;

use axum::http::StatusCode;

pub struct APIError(StatusCode, String);

impl APIError {
    pub fn internal_error(err: impl Error) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }

    pub fn bad_request(err: impl Error) -> Self {
        Self(StatusCode::BAD_REQUEST, err.to_string())
    }

    pub fn unauthorized(err: impl Error) -> Self {
        Self(StatusCode::UNAUTHORIZED, err.to_string())
    }

    pub fn forbidden(err: impl Error) -> Self {
        Self(StatusCode::FORBIDDEN, err.to_string())
    }

    pub fn not_found(err: impl Error) -> Self {
        Self(StatusCode::NOT_FOUND, err.to_string())
    }

    pub fn conflict(err: impl Error) -> Self {
        Self(StatusCode::CONFLICT, err.to_string())
    }

    pub fn unprocessable_entity(err: impl Error) -> Self {
        Self(StatusCode::UNPROCESSABLE_ENTITY, err.to_string())
    }
}
