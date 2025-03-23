# Q & A Application using Axum Crate in Rust

-[x] Setup default axum server.
-[x] Add CORS support.
-[x] Models: Question, Answer
-[x] Create a Store for mock data

- We need to use async Arc and RwLock to store, read, create and update questions.
```rust
use std::sync::Arc;
use tokio::sync::RwLock;
```

-[x] Read Questions from JSON file as mock data.
-[x] Added a route and handler to get all questions.
- 

