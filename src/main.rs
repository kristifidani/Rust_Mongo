mod db;
mod errors;
mod handlers;

use crate::db::DB;
use std::convert::Infallible;
use warp::{Filter, Rejection};

#[tokio::main]
async fn main() -> Result<(), Rejection> {
    let db = DB::init().await?;

    let book_routes = warp::path("book")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handlers::create_book_handle)
        .or(warp::path!("book" / String)
            .and(warp::delete())
            .and(with_db(db.clone()))
            .and_then(handlers::delete_book_handle))
        .or(warp::path!("book" / String)
            .and(warp::put())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handlers::edit_book_handle))
        .or(warp::path("books")
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handlers::fetch_books_handle));

    let routes = book_routes.recover(errors::handle_rejection);

    println!("Started on port 8080");
    Ok(warp::serve(routes).run(([127, 0, 0, 1], 8080)).await)
}

//dependency injection of DB on filters
fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
