use crate::db::DB;
use serde::{Deserialize, Serialize};
use warp::{
    http::StatusCode,
    reply::{json, with_status},
    Rejection, Reply,
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub author: String,
    pub number_pages: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Payload<T> {
    pub data: Box<T>,
}

pub async fn create_book_handle(body: Book, db: DB) -> Result<impl Reply, Rejection> {
    let created_book = db.create_book(&body).await?;

    Ok(with_status(
        json(&Payload {
            data: Box::new(created_book),
        }),
        StatusCode::CREATED,
    ))
}

pub async fn fetch_books_handle(db: DB) -> Result<impl Reply, Rejection> {
    let books = db.fetch_books().await?;

    Ok(with_status(
        json(&Payload {
            data: Box::new(books),
        }),
        StatusCode::OK,
    ))
}

pub async fn edit_book_handle(id: String, body: Book, db: DB) -> Result<impl Reply, Rejection> {
    let updated_book = db.edit_book(&id, &body).await?;

    Ok(with_status(
        json(&Payload {
            data: Box::new(updated_book),
        }),
        StatusCode::OK,
    ))
}

pub async fn delete_book_handle(id: String, db: DB) -> Result<impl Reply, Rejection> {
    let deleted_book_id = db.delete_book(&id).await?;

    Ok(with_status(
        json(&Payload {
            data: Box::new(deleted_book_id),
        }),
        StatusCode::OK,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use hyper::body::to_bytes;

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
    async fn test_create_book_handle() {
        let db = db::DB::init().await.expect("failed to initialize mongodb");

        let new_book = Book {
            id: "1".to_string(),
            name: "Sample Book".to_string(),
            author: "John Doe".to_string(),
            number_pages: 200.to_string(),
            tags: vec!["fiction".to_string(), "adventure".to_string()],
        };

        let result = create_book_handle(new_book, db.clone())
            .await
            .expect("failed to create a book");

        assert_eq!(result.into_response().status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_delete_book_handle() {
        let db = db::DB::init().await.expect("failed to initialize mongodb");
        let book_id = get_book_id(db.clone()).await;

        let result = delete_book_handle(book_id, db.clone())
            .await
            .expect("failed to delete a book");

        assert_eq!(result.into_response().status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_edit_book_handle() {
        let edit_book = Book {
            id: "1".to_string(),
            name: "Edited name".to_string(),
            author: "Edited author".to_string(),
            number_pages: 200.to_string(),
            tags: vec!["fiction".to_string(), "adventure".to_string()],
        };

        let db = db::DB::init().await.expect("failed to initialize mongodb");
        let book_id = get_book_id(db.clone()).await;

        let result = edit_book_handle(book_id, edit_book, db.clone())
            .await
            .expect("failed to edit a book");

        assert_eq!(result.into_response().status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_fetch_books_handle() {
        let db = db::DB::init().await.expect("failed to initialize mongodb");

        let result = fetch_books_handle(db.clone())
            .await
            .expect("failed to fetch books");

        assert_eq!(result.into_response().status(), StatusCode::OK);
    }
}
