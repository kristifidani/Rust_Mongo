use crate::{errors::MongoDbErrors, handlers::BookRequest, Book};
use futures::StreamExt;
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};

const DB_NAME: &str = "booky";
const COLL: &str = "books";

const ID: &str = "_id";
const NAME: &str = "name";
const AUTHOR: &str = "author";
const NUMBER_PAGES: &str = "number_pages";
const TAGS: &str = "tags";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self, MongoDbErrors> {
        let client = DB::create_mongodb_client().await?;
        Ok(Self { client })
    }

    async fn create_mongodb_client() -> mongodb::error::Result<Client> {
        let mut client_options = ClientOptions::parse("mongodb://mongodb:27017").await?;
        client_options.app_name = Some("booky".to_string());
        Client::with_options(client_options)
    }

    pub async fn fetch_books(&self) -> Result<Vec<Book>, MongoDbErrors> {
        let mut cursor = self
            .get_collection()
            .find(None, None)
            .await
            .map_err(|e| MongoDbErrors::MongoQueryError(e))?;
        let mut result: Vec<Book> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_book(&doc?)?);
        }
        Ok(result)
    }

    pub async fn create_book(&self, entry: &BookRequest) -> Result<(), MongoDbErrors> {
        let number_pages = entry
            .number_pages
            .parse::<i32>()
            .map_err(|e| MongoDbErrors::InvalidNumberPagesError(e))?;

        let doc = doc! {
            NAME: entry.name.clone(),
            AUTHOR: entry.author.clone(),
            NUMBER_PAGES: number_pages,
            TAGS: entry.tags.clone()
        };
        self.get_collection()
            .insert_one(doc, None)
            .await
            .map_err(|e| MongoDbErrors::MongoQueryError(e))?;
        Ok(())
    }

    pub async fn delete_book(&self, id: &str) -> Result<(), MongoDbErrors> {
        let oid =
            ObjectId::parse_str(id).map_err(|_| MongoDbErrors::InvalidIdError(id.to_owned()))?;
        let filter = doc! {
            "_id": oid,
        };
        self.get_collection()
            .delete_one(filter, None)
            .await
            .map_err(|e| MongoDbErrors::MongoQueryError(e))?;
        Ok(())
    }

    pub async fn edit_book(&self, id: &str, entry: &BookRequest) -> Result<(), MongoDbErrors> {
        let oid =
            ObjectId::parse_str(id).map_err(|_| MongoDbErrors::InvalidIdError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        let number_pages = entry
            .number_pages
            .parse::<i32>()
            .map_err(|e| MongoDbErrors::InvalidNumberPagesError(e))?;

        let doc = doc! {
            NAME: entry.name.clone(),
            AUTHOR: entry.author.clone(),
            NUMBER_PAGES: number_pages,
            TAGS: entry.tags.clone()
        };
        self.get_collection()
            .update_one(query, doc, None)
            .await
            .map_err(|e| MongoDbErrors::MongoQueryError(e))?;
        Ok(())
    }

    fn get_collection(&self) -> Collection<Document> {
        self.client.database(DB_NAME).collection(COLL)
    }

    fn doc_to_book(&self, doc: &Document) -> Result<Book, MongoDbErrors> {
        let id = doc.get_object_id(ID)?;
        let name = doc.get_str(NAME)?;
        let author = doc.get_str(AUTHOR)?;
        let number_pages = doc.get_i32(NUMBER_PAGES)?;
        let tags = doc.get_array(TAGS)?;

        let tags: Vec<String> = tags
            .iter()
            .filter_map(|entry| match entry {
                Bson::String(v) => Some(v.to_owned()),
                _ => None,
            })
            .collect();

        let book = Book {
            id: id.to_hex(),
            name: name.to_owned(),
            author: author.to_owned(),
            number_pages: number_pages as usize,
            tags,
        };
        Ok(book)
    }
}
