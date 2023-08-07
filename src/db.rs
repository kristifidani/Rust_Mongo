use crate::{errors::MongoDbErrors, handlers::BookRequest};
use futures::StreamExt;
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};

const DB_URL: &str = "mongodb://127.0.0.1:27017";
const DB_NAME: &str = "library";
const COLLECTION: &str = "books";

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub author: String,
    pub number_pages: usize,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct DB {
    client: Client,
}

impl DB {
    pub async fn init() -> Result<Self, MongoDbErrors> {
        let client = DB::create_mongodb_client().await?;
        Ok(Self { client })
    }

    async fn create_mongodb_client() -> mongodb::error::Result<Client> {
        let mut client_options = ClientOptions::parse(DB_URL.to_string()).await?;
        client_options.app_name = Some("library".to_string());
        Client::with_options(client_options)
    }

    pub async fn create_book(&self, book: &BookRequest) -> Result<Book, MongoDbErrors> {
        let number_pages = book
            .number_pages
            .parse::<i32>()
            .map_err(|e| MongoDbErrors::InvalidNumberPagesError(e))?;

        let record = doc! {
            "NAME": book.name.clone(),
            "AUTHOR": book.author.clone(),
            "NUMBER_PAGES": number_pages,
            "TAGS": book.tags.clone()
        };

        match self.get_collection().insert_one(record, None).await {
            Ok(inserted_record) => {
                if let Some(inserted_id) = inserted_record.inserted_id.as_object_id() {
                    Ok(Book {
                        id: inserted_id.to_hex(),
                        name: book.name.clone(),
                        author: book.author.clone(),
                        number_pages: number_pages as usize,
                        tags: book.tags.clone(),
                    })
                } else {
                    Err(MongoDbErrors::InvalidIdError(
                        "Failed to extract the inserted book ID".to_string(),
                    ))
                }
            }
            Err(e) => Err(MongoDbErrors::MongoQueryError(e)),
        }
    }

    pub async fn fetch_books(&self) -> Result<Vec<Book>, MongoDbErrors> {
        let mut cursor = self
            .get_collection()
            .find(None, None)
            .await
            .map_err(|e| MongoDbErrors::MongoQueryError(e))?;

        let mut result: Vec<Book> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.convert_db_document_to_book(&doc?).await?);
        }
        Ok(result)
    }

    pub async fn delete_book(&self, id: &str) -> Result<String, MongoDbErrors> {
        let book_id =
            ObjectId::parse_str(id).map_err(|_| MongoDbErrors::InvalidIdError(id.to_owned()))?;

        let filter = doc! {
            "_id": book_id,
        };

        match self.get_collection().delete_one(filter, None).await {
            Ok(result) => {
                if result.deleted_count > 0 {
                    Ok(id.to_string())
                } else {
                    Err(MongoDbErrors::InvalidIdError(format!(
                        "Failed to delete book with id {}",
                        id
                    )))
                }
            }
            Err(e) => Err(MongoDbErrors::MongoQueryError(e)),
        }
    }

    pub async fn edit_book(&self, id: &str, book: &BookRequest) -> Result<Book, MongoDbErrors> {
        let book_id =
            ObjectId::parse_str(id).map_err(|_| MongoDbErrors::InvalidIdError(id.to_owned()))?;
        let filter = doc! {
            "_id": book_id,
        };

        let number_pages = book
            .number_pages
            .parse::<i32>()
            .map_err(|e| MongoDbErrors::InvalidNumberPagesError(e))?;

        let doc = doc! {
            "$set": {
                "NAME": book.name.clone(),
                "AUTHOR": book.author.clone(),
                "NUMBER_PAGES": number_pages,
                "TAGS": book.tags.clone()
            }
        };

        match self.get_collection().update_one(filter, doc, None).await {
            Ok(result) => {
                if result.modified_count > 0 {
                    Ok(Book {
                        id: book_id.to_hex(),
                        name: book.name.clone(),
                        author: book.author.clone(),
                        number_pages: number_pages as usize,
                        tags: book.tags.clone(),
                    })
                } else {
                    Err(MongoDbErrors::InvalidIdError(format!(
                        "Failed to update book with id {}",
                        id
                    )))
                }
            }
            Err(e) => Err(MongoDbErrors::MongoQueryError(e)),
        }
    }

    fn get_collection(&self) -> Collection<Document> {
        self.client.database(DB_NAME).collection(COLLECTION)
    }

    async fn convert_db_document_to_book(&self, doc: &Document) -> Result<Book, MongoDbErrors> {
        let id = doc.get_object_id("_id")?.to_hex();
        let name = doc.get_str("NAME")?.to_owned();
        let author = doc.get_str("AUTHOR")?.to_owned();
        let number_pages = doc.get_i32("NUMBER_PAGES")? as usize;

        let tags = doc
            .get_array("TAGS")?
            .iter()
            .filter_map(|entry| {
                if let Bson::String(v) = entry {
                    Some(v.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        Ok(Book {
            id,
            name,
            author,
            number_pages,
            tags,
        })
    }
}
