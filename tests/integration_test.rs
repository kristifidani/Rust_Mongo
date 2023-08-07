use rust_mongo::db::*;
use rust_mongo::handlers::*;

use hyper::body::to_bytes;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::test::request;
use warp::Filter;
use warp::Reply;

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

async fn get_book_id(db: DB) -> String {
    let books = fetch_books_handle(db.clone())
        .await
        .expect("failed to fetch books");
    let resp = books.into_response().into_body();
    // Convert the response body to bytes
    let bytes = to_bytes(resp)
        .await
        .expect("failed to convert response body to bytes format");
    // Deserialize the response body as JSON
    let parsed_response: serde_json::Value =
        serde_json::from_slice(&bytes).expect("failed to parse response as JSON");

    parsed_response
        .get("data")
        .expect("failed to get response data top level")[0]
        .get("id")
        .expect("failed to get book id")
        .to_string()
        .trim_matches('"')
        .to_string()
}

#[tokio::test]
async fn test_create_book_route() {
    let db = DB::init().await.expect("failed to initialize mongodb");
    let create_book_route = warp::path("book")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(create_book_handle);

    let book_request = Book {
        id: "123".to_string(), //this will be ignored
        name: "Test Book".to_string(),
        author: "Test Author".to_string(),
        number_pages: "100".to_string(),
        tags: vec!["tag1".to_string(), "tag2".to_string()],
    };

    let response = request()
        .method("POST")
        .path("/book")
        .json(&book_request)
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
    let book_id = get_book_id(db.clone()).await;
    let edit_book_route = warp::path!("book" / String)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(edit_book_handle);

    let book_request = Book {
        id: "123".to_string(), //this will be ignored
        name: "Updated Book".to_string(),
        author: "Updated Author".to_string(),
        number_pages: "200".to_string(),
        tags: vec!["tag1".to_string(), "tag2".to_string()],
    };

    let response = request()
        .method("PUT")
        .path(format!("/book/{}", book_id).as_str())
        .json(&book_request)
        .reply(&edit_book_route)
        .await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_delete_book_route() {
    let db = DB::init().await.expect("failed to initialize mongodb");
    let book_id = get_book_id(db.clone()).await;
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
