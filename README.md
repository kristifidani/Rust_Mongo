# Rust REST API with MongoDB

An HTTP server build with Warp light-weight framework that provides CRUD operations with MongoDB.

#### Create book

- **HTTP Method**: POST
- **URL Path**: `http://localhost:8080/book`
- **Description**: Create a new book by providing book details in the request body.

#### Fetch books

- **HTTP Method**: GET
- **URL Path**: `http://localhost:8080/books`
- **Description**: Retrieve a list of all books.

#### Edit book

- **HTTP Method**: PUT
- **URL Path**: `http://localhost:8080/book/{id}`
- **Description**: Edit a specific book by providing its ID in the URL path.

#### Delete book

- **HTTP Method**: DELETE
- **URL Path**: `http://localhost:8080/book/{id}`
- **Description**: Delete a book by providing its ID in the URL path.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- **Rust** and **Cargo** installed. Check the official [rust website](https://www.rust-lang.org/learn/get-started).
- **Docker Compose**. You can download it from the official [docker website](https://docs.docker.com/compose/).

## Test

- Start docker containers: `docker-compose up -d`
- Run unit-tests with: `cargo test --bin rust-mongo -- --nocapture`
- Run integration tests with: `cargo test --test integration_test -- --nocapture`

## Build

- Build the project with: `cargo build`
- Run the project with: `cargo run`

## Usage

1. Start docker containers: `docker-compose up -d`
1. Run the project: `cargo run`
1. Create book: `cd script/ && ./create_book.sh`
1. Fetch books: `cd script/ && ./fetch_books.sh`
1. Edit book using its MongoDB generated ID: `cd script/ && ./edit_book.sh`
1. Delete book using its MongoDB generated ID: `cd script/ && ./delete_book.sh`

## Useful Docker and Mongo commands

- `docker-compose up -d` --> start docker containers
- `docker ps` --> shows all running containers
- `docker exec -it rust-mongo bash` --> open the Bash shell inside the running MongoDB container
- `mongod --version` --> verify mongo server
- `mongosh` --> connect to the MongoDB server
- `show dbs` --> list all existing databases
- `use library` --> switch to the library databse which we use in this application
- `db.books.find()` --> query all documents inside the books collection
