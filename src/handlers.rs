use crate::db::DB;
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reply::json, Rejection, Reply};

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

pub async fn book_list_handle(db: DB) -> Result<impl Reply, Rejection> {
    let books = db.fetch_books().await?;
    Ok(json(&books))
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

// mod tests {
//     use super::*;
//     #[allow(unused_imports)]
//     use mockall::{automock, mock, predicate::*};

//     #[cfg_attr(test, automock)]
//     pub trait MockDB {
//         fn create_book(&self, entry: &BookRequest) -> Result<(), Error>;
//     }
//     #[tokio::test]
//     async fn test_create_book_handle() {
//         let mut db_mock = MockMockDB::new();
//         db_mock
//             .expect_create_book()
//             .with(eq(BookRequest {
//                 name: "Test Book".to_owned(),
//                 author: "Test Author".to_owned(),
//                 number_pages: "100".to_owned(),
//                 tags: vec!["tag1".to_owned(), "tag2".to_owned()],
//             }))
//             .times(1)
//             .returning(|_| Ok(()));

//         let resp = create_book_handle(
//             BookRequest {
//                 name: "Test Book".to_owned(),
//                 author: "Test Author".to_owned(),
//                 number_pages: "100".to_owned(),
//                 tags: vec!["tag1".to_owned(), "tag2".to_owned()],
//             },
//             db_mock,
//         )
//         .await
//         .expect("error while attempting to create a new book");

//         assert_eq!(resp, StatusCode::CREATED);
//     }
// }
