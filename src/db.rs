use crate::{errors::MongoDbErrors, handlers::BookRequest, Book};
use futures::TryStreamExt;
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};

const DB_URL: &str = "mongodb://127.0.0.1:27017";
const DB_NAME: &str = "library";
const COLLECTION: &str = "books";

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

    pub async fn create_book(&self, book: &BookRequest) -> Result<(), MongoDbErrors> {
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

        self.get_collection()
            .insert_one(record, None)
            .await
            .map_err(|e| MongoDbErrors::MongoQueryError(e))?;
        Ok(())
    }

    pub async fn fetch_books(&self) -> Result<Vec<Book>, MongoDbErrors> {
        let cursor = self
            .get_collection()
            .find(None, None)
            .await
            .map_err(|e| MongoDbErrors::MongoQueryError(e))?;

        let books = cursor
            .try_collect::<Vec<_>>()
            .await
            .map_err(|e| MongoDbErrors::MongoQueryError(e))?
            .into_iter()
            .map(|doc| {
                let id = doc.get_object_id("_id")?.to_hex();
                let name = doc.get_str("NAME")?.to_owned();
                let author = doc.get_str("AUTHOR")?.to_owned();
                let number_pages = doc.get_i32("NUMBER_PAGES")? as usize;
                let tags = doc
                    .get_array("TAGS")?
                    .into_iter()
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
            })
            .collect::<Result<Vec<_>, MongoDbErrors>>()?;

        Ok(books)
    }

    pub async fn delete_book(&self, id: &str) -> Result<(), MongoDbErrors> {
        let book_id =
            ObjectId::parse_str(id).map_err(|_| MongoDbErrors::InvalidIdError(id.to_owned()))?;
        let filter = doc! {
            "_id": book_id,
        };
        self.get_collection()
            .delete_one(filter, None)
            .await
            .map_err(|e| MongoDbErrors::MongoQueryError(e))?;
        Ok(())
    }

    pub async fn edit_book(&self, id: &str, book: &BookRequest) -> Result<(), MongoDbErrors> {
        let book_id =
            ObjectId::parse_str(id).map_err(|_| MongoDbErrors::InvalidIdError(id.to_owned()))?;
        let query = doc! {
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

        self.get_collection()
            .update_one(query, doc, None)
            .await
            .map_err(|e| MongoDbErrors::MongoQueryError(e))?;
        Ok(())
    }

    fn get_collection(&self) -> Collection<Document> {
        self.client.database(DB_NAME).collection(COLLECTION)
    }
}
