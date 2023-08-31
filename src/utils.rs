use crate::{db::DB, errors::MongoDbErrors, handlers::Book};

#[allow(dead_code)]
pub async fn get_book_id(db: DB) -> Result<String, MongoDbErrors> {
    let books = db.fetch_books().await?;
    Ok(books[0].id.clone())
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
        tags: vec!["drama".to_string(), "real-life".to_string()],
    };
    edit_book
}
