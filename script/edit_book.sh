curl -X PUT http://localhost:8080/book/<MongoID> -H 'Content-Type: application/json' -d '{"id":"1","name":"update name","author":"update author","number_pages":"200","tags":["utag1","utag2"]}'
