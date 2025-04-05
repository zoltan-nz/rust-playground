# Q & A Application using Axum Crate in Rust

-[x] Setup default axum server.
-[x] Add CORS support.
-[x] Models: Question, Answer
-[x] Create a Store for mock data

>We need to use async Arc and RwLock to store, read, create and update questions.
```rust
use std::sync::Arc;
use tokio::sync::RwLock;
```

-[x] Read Questions from JSON file as mock data.
-[x] Added a route and handler to get all questions. (GET /questions)
-[x] Implement POST /questions to create a new question.
-[x] Implement GET /questions/:id to get a question by id.
-[ ] Implement PUT /questions/:id to update a question by id.
-[ ] Implement DELETE /questions/:id to delete a question by id.
-[ ] Add pagination options to GET /questions.
-[ ] Add error handling. 

