use crate::db::DB;
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BookRequest {
    pub name: String,
    pub author: String,
    pub number_pages: String,
    pub tags: Vec<String>,
}

pub async fn create_book_handle(body: BookRequest, db: DB) -> Result<impl Reply, Rejection> {
    db.create_book(&body).await?;
    Ok(StatusCode::CREATED)
}

pub async fn fetch_books_handle(db: DB) -> Result<impl Reply, Rejection> {
    let books = db.fetch_books().await?;
    println!("{:?}", books);
    Ok(StatusCode::OK)
}

pub async fn edit_book_handle(
    id: String,
    body: BookRequest,
    db: DB,
) -> Result<impl Reply, Rejection> {
    db.edit_book(&id, &body).await?;
    Ok(StatusCode::OK)
}

pub async fn delete_book_handle(id: String, db: DB) -> Result<impl Reply, Rejection> {
    db.delete_book(&id).await?;
    Ok(StatusCode::OK)
}

#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::db;

    #[tokio::test]
    async fn test_create_book_handle() {
        let mock_request = BookRequest {
            name: "Sample Book".to_string(),
            author: "John Doe".to_string(),
            number_pages: 200.to_string(),
            tags: vec!["fiction".to_string(), "adventure".to_string()],
        };

        let db = db::DB::init().await.expect("failed to initialize mongodb");

        let result = create_book_handle(mock_request, db.clone())
            .await
            .expect("failed to create a book");

        assert_eq!(result.into_response().status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_delete_book_handle() {
        let db = db::DB::init().await.expect("failed to initialize mongodb");

        let book_id = "64cfb23ed9632a4599b4f5e0".to_string();
        let result = delete_book_handle(book_id, db.clone())
            .await
            .expect("failed to delete a book");

        assert_eq!(result.into_response().status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_edit_book_handle() {
        let mock_request = BookRequest {
            name: "Eddited Book".to_string(),
            author: "John Doe".to_string(),
            number_pages: 200.to_string(),
            tags: vec!["fiction".to_string(), "adventure".to_string()],
        };

        let db = db::DB::init().await.expect("failed to initialize mongodb");
        let book_id = "64cfadbd9b3a666fb0aeac15".to_string();
        let result = edit_book_handle(book_id, mock_request, db.clone())
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
