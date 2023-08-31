use crate::{
    db::DB,
    handlers::{fetch_books_handle, Book},
};
use hyper::body::to_bytes;
use warp::reply::Reply;

#[allow(dead_code)]
pub async fn get_book_id(db: DB) -> String {
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

#[allow(dead_code)]
pub fn new_book() -> Book {
    let new_book = Book {
        id: "1".to_string(),
        name: "Sample Book".to_string(),
        author: "John Doe".to_string(),
        number_pages: 200.to_string(),
        tags: vec!["fiction".to_string(), "adventure".to_string()],
    };
    new_book
}

#[allow(dead_code)]
pub fn edit_book() -> Book {
    let edit_book = Book {
        id: "1".to_string(),
        name: "Edited name".to_string(),
        author: "Edited author".to_string(),
        number_pages: 200.to_string(),
        tags: vec!["fiction".to_string(), "adventure".to_string()],
    };
    edit_book
}
