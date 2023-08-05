use mongodb::bson;
use serde::Serialize;
use thiserror::Error;
use warp::{http::StatusCode, reject::Reject, reply, Rejection, Reply};

use std::num::ParseIntError;

#[derive(Debug, Error)]
pub enum MongoDbErrors {
    #[error("mongodb error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),
    #[error("could not access field in document: {0}")]
    MongoDataError(#[from] bson::document::ValueAccessError),
    #[error("invalid id used: {0}")]
    InvalidIdError(String),
    #[error("invalid number of pages: {0}")]
    InvalidNumberPagesError(ParseIntError),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl Reject for MongoDbErrors {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    let code: StatusCode;
    let message: &str;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(e) = err.find::<MongoDbErrors>() {
        match e {
            _ => {
                println!("not handled application error: {:?}", err);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error";
            }
        }
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method not allowed";
    } else {
        println!("not handled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let error_message = reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(reply::with_status(error_message, code))
}
