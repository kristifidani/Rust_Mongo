mod db;
mod errors;
mod handlers;

use crate::db::DB;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{Filter, Rejection};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub author: String,
    pub number_pages: usize,
    pub tags: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Rejection> {
    let db = DB::init().await?;
    let book = warp::path("book");

    let book_routes = book
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handlers::create_book_handle)
        .or(book
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handlers::edit_book_handle))
        .or(book
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handlers::delete_book_handle))
        .or(book
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handlers::book_list_handle));

    let routes = book_routes.recover(errors::handle_rejection);
    println!("Started on port 8080");
    Ok(warp::serve(routes).run(([127, 0, 0, 1], 8080)).await)
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
