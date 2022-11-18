use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use serde_with::DisplayFromStr;
use sqlx::error::DatabaseError;
use validator::ValidationErrors;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// 401
    #[error("authentication required")]
    Unauthorized,

    /// 403
    #[error("user may not perform that action")]
    Forbidden,

    /// 404
    #[error("entity not found")]
    NotFound,

    /// 409
    #[error("{0}")]
    Conflict(String),

    /// 422
    #[error("validation error in request body")]
    InvalidEntity(#[from] ValidationErrors),

    /// 422
    #[error("{0}")]
    UnprocessableEntity(String),

    /// 500
    #[error("an internal database error occurred")]
    Sqlx(#[from] sqlx::Error),

    /// 500
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[serde_with::serde_as]
        #[serde_with::skip_serializing_none]
        #[derive(serde::Serialize)]
        struct ErrorResponse<'a> {
            // Serialize the `Display` output as the error message
            #[serde_as(as = "DisplayFromStr")]
            message: &'a Error,

            errors: Option<&'a ValidationErrors>,
        }

        let errors = match &self {
            Error::InvalidEntity(errors) => Some(errors),
            _ => None,
        };

        (
            self.status_code(),
            Json(ErrorResponse {
                message: &self,
                errors,
            }),
        )
            .into_response()
    }
}

pub trait DbErrorResultExt<T> {
    /// If `self` contains a SQLx database constraint error with the given name,
    /// transform the error.
    ///
    /// Otherwise, the result is passed through unchanged.
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T>;
}

impl<T, E> DbErrorResultExt<T> for std::result::Result<T, E>
where
    E: Into<Error>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T> {
        self.map_err(|e| match e.into() {
            Error::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}

impl Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;

        match self {
            Unauthorized => StatusCode::UNAUTHORIZED,
            Forbidden => StatusCode::FORBIDDEN,
            Sqlx(_) | Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
            InvalidEntity(_) | UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Conflict(_) => StatusCode::CONFLICT,
            NotFound => StatusCode::NOT_FOUND,
        }
    }
}
