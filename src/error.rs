use axum::http::StatusCode;
use axum_thiserror::ErrorStatus;
use deadpool_diesel::{InteractError, PoolError};
use thiserror::Error;

#[derive(Debug, Error, ErrorStatus)]
pub enum AppError {
    #[error("API returned an error")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    Pg(#[from] PoolError),
    #[error("API returned an error")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    DeadPool(#[from] InteractError),
    #[error("API returned an error")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    Diesel(#[from] diesel::result::Error),
}
