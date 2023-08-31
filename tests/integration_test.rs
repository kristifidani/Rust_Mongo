use rust_mongo::db::*;
use rust_mongo::handlers::*;
use rust_mongo::utils::{edit_book, get_book_id, new_book};
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::test::request;
use warp::Filter;

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

#[tokio::test]
async fn test_create_book_route() {
    let db = DB::init().await.expect("failed to initialize mongodb");
    let create_book_route = warp::path("book")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(create_book_handle);

    let response = request()
        .method("POST")
        .path("/book")
        .json(&new_book())
        .reply(&create_book_route)
        .await;

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_fetch_books_route() {
    let db = DB::init().await.expect("failed to initialize mongodb");
    let fetch_books_route = warp::path("books")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(fetch_books_handle);

    let response = request()
        .method("GET")
        .path("/books")
        .reply(&fetch_books_route)
        .await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_edit_book_route() {
    let db = DB::init().await.expect("failed to initialize mongodb");
    let book_id = get_book_id(db.clone())
        .await
        .expect("failed to edit a book");
    let edit_book_route = warp::path!("book" / String)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(edit_book_handle);

    let response = request()
        .method("PUT")
        .path(format!("/book/{}", book_id).as_str())
        .json(&edit_book())
        .reply(&edit_book_route)
        .await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_delete_book_route() {
    let db = DB::init().await.expect("failed to initialize mongodb");
    let book_id = get_book_id(db.clone())
        .await
        .expect("failed to edit a book");
    let delete_book_route = warp::path!("book" / String)
        .and(warp::delete())
        .and(with_db(db.clone()))
        .and_then(delete_book_handle);

    let response = request()
        .method("DELETE")
        .path(format!("/book/{}", book_id).as_str())
        .reply(&delete_book_route)
        .await;

    assert_eq!(response.status(), StatusCode::OK);
}
