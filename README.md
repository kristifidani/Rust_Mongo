# Rust with MongoDB

An HTTP server build with Warp light-weight framework that provides CRUD operations with MongoDB.

## Test

Give the instructions to test the project.

- Start docker containers: `docker-compose up -d`
- Run unit-tests with: `cargo test --bin rust-mongo -- --nocapture`
- Run integration tests with: `cargo test --test integration_test -- --nocapture`

## Build

Give the instructions to build the project.

- Build the project with: `cargo build`
- Run the project with: `cargo run`

## Useful Docker and Mongo commands

- `docker-compose up -d` --> start docker containers
- `docker ps` --> shows all running containers
- `docker exec -it rust-mongo bash` --> open the Bash shell inside the running MongoDB container
- `mongod --version` --> verify mongo server
- `mongosh` --> connect to the MongoDB server
- `show dbs` --> list all existing databases
- `use library` --> switch to the library databse which we use in this application
- `db.books.find()` --> query all documents inside the books collection
