use crate::{errors::MongoDbErrors, handlers::Book};
use dotenv::dotenv;
use futures::StreamExt;
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};

#[derive(Clone, Debug)]
pub struct DB {
    client: Client,
}

impl DB {
    pub async fn init() -> Result<Self, MongoDbErrors> {
        let client = DB::create_mongodb_client().await?;
        Ok(Self { client })
    }

    async fn create_mongodb_client() -> Result<Client, MongoDbErrors> {
        dotenv().ok();
        let mongodb_url = dotenv::var("DB_URL").map_err(|_| MongoDbErrors::InvalidURL)?;
        let mut client_options = ClientOptions::parse(mongodb_url).await?;
        let mongodb_name = dotenv::var("DB_NAME").map_err(|_| MongoDbErrors::InvalidDbName)?;
        client_options.app_name = Some(mongodb_name);
        Ok(Client::with_options(client_options)
            .map_err(|client_error| MongoDbErrors::ClientError(client_error.to_string()))?)
    }

    fn get_collection(&self) -> Result<Collection<Document>, MongoDbErrors> {
        let mongodb_name = dotenv::var("DB_NAME").map_err(|_| MongoDbErrors::InvalidDbName)?;
        let mongodb_collection =
            dotenv::var("COLLECTION").map_err(|_| MongoDbErrors::InvalidCollection)?;
        Ok(self
            .client
            .database(mongodb_name.as_str())
            .collection(mongodb_collection.as_str()))
    }

    pub async fn create_book(&self, book: &Book) -> Result<Book, MongoDbErrors> {
        let number_pages = book
            .number_pages
            .parse::<i32>()
            .map_err(|int_parse_error| MongoDbErrors::InvalidNumberPages(int_parse_error))?;

        let record = doc! {
            "NAME": book.name.clone(),
            "AUTHOR": book.author.clone(),
            "NUMBER_PAGES": number_pages,
            "TAGS": book.tags.clone()
        };

        match self.get_collection()?.insert_one(record, None).await {
            Ok(inserted_record) => {
                if let Some(inserted_id) = inserted_record.inserted_id.as_object_id() {
                    Ok(Book {
                        id: inserted_id.to_hex(),
                        name: book.name.clone(),
                        author: book.author.clone(),
                        number_pages: number_pages.to_string(),
                        tags: book.tags.clone(),
                    })
                } else {
                    Err(MongoDbErrors::InvalidId(
                        "Failed to extract the inserted book ID".to_string(),
                    ))
                }
            }
            Err(mongo_error) => Err(MongoDbErrors::InvalidQuery(mongo_error)),
        }
    }

    pub async fn fetch_books(&self) -> Result<Vec<Book>, MongoDbErrors> {
        let mut cursor = self
            .get_collection()?
            .find(None, None)
            .await
            .map_err(|mongo_error| MongoDbErrors::InvalidQuery(mongo_error))?;

        let mut result: Vec<Book> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.convert_db_document_to_book(&doc?).await?);
        }
        Ok(result)
    }

    pub async fn delete_book(&self, id: &str) -> Result<String, MongoDbErrors> {
        let book_id = ObjectId::parse_str(id).map_err(|obj_id_parse_error| {
            MongoDbErrors::InvalidId(obj_id_parse_error.to_string())
        })?;

        let filter = doc! {
            "_id": book_id,
        };

        match self.get_collection()?.delete_one(filter, None).await {
            Ok(result) => {
                if result.deleted_count > 0 {
                    Ok(id.to_string())
                } else {
                    Err(MongoDbErrors::InvalidId(format!(
                        "Failed to delete book with id {}",
                        id
                    )))
                }
            }
            Err(mongo_error) => Err(MongoDbErrors::InvalidQuery(mongo_error)),
        }
    }

    pub async fn edit_book(&self, id: &str, book: &Book) -> Result<Book, MongoDbErrors> {
        let book_id = ObjectId::parse_str(id).map_err(|obj_id_parse_error| {
            MongoDbErrors::InvalidId(obj_id_parse_error.to_string())
        })?;
        let filter = doc! {
            "_id": book_id,
        };

        let number_pages = book
            .number_pages
            .parse::<i32>()
            .map_err(|int_parse_error| MongoDbErrors::InvalidNumberPages(int_parse_error))?;

        let doc = doc! {
            "$set": {
                "NAME": book.name.clone(),
                "AUTHOR": book.author.clone(),
                "NUMBER_PAGES": number_pages,
                "TAGS": book.tags.clone()
            }
        };

        match self.get_collection()?.update_one(filter, doc, None).await {
            Ok(result) => {
                if result.modified_count > 0 {
                    Ok(Book {
                        id: book_id.to_hex(),
                        name: book.name.clone(),
                        author: book.author.clone(),
                        number_pages: number_pages.to_string(),
                        tags: book.tags.clone(),
                    })
                } else {
                    Err(MongoDbErrors::InvalidId(format!(
                        "Failed to update book with id {}",
                        id
                    )))
                }
            }
            Err(mongo_error) => Err(MongoDbErrors::InvalidQuery(mongo_error)),
        }
    }

    async fn convert_db_document_to_book(&self, doc: &Document) -> Result<Book, MongoDbErrors> {
        let id = doc.get_object_id("_id")?.to_hex();
        let name = doc.get_str("NAME")?.to_owned();
        let author = doc.get_str("AUTHOR")?.to_owned();
        let number_pages = doc.get_i32("NUMBER_PAGES")?.to_string();
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
