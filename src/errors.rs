use mongodb::bson;
use serde::Serialize;
use std::num::ParseIntError;
use thiserror::Error;
use warp::{http::StatusCode, reject::Reject, reply, Rejection, Reply};

#[derive(Debug, Error)]
pub enum MongoDbErrors {
    #[error("mongodb error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("mongodb query error: {0}")]
    InvalidQuery(mongodb::error::Error),
    #[error("mongodb data access error: {0}")]
    InvalidData(#[from] bson::document::ValueAccessError),
    #[error("invalid record id: {0}")]
    InvalidId(String),
    #[error("invalid number of pages: {0}")]
    InvalidNumberPages(ParseIntError),
    #[error("mongodb url not found")]
    InvalidURL,
    #[error("mongodb name not found")]
    InvalidDbName,
    #[error("mongodb collection not found")]
    InvalidCollection,
    #[error("failed to create mongo client: {0}")]
    ClientError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl Reject for MongoDbErrors {}

#[allow(dead_code)]
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    let (code, message) = match () {
        _ if err.is_not_found() => (StatusCode::NOT_FOUND, "Error not found"),
        _ if err
            .find::<warp::filters::body::BodyDeserializeError>()
            .is_some() =>
        {
            (StatusCode::BAD_REQUEST, "Invalid request")
        }
        _ if err.find::<MongoDbErrors>().is_some() => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        }
        _ if err.find::<warp::reject::MethodNotAllowed>().is_some() => {
            (StatusCode::METHOD_NOT_ALLOWED, "Method not allowed")
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
    };

    let error_message = reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(reply::with_status(error_message, code))
}
