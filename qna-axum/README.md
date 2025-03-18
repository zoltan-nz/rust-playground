# Q & A Application using Axum Crate in Rust

-[x] Setup default axum server.
-[x] Add CORS support.

Required models:
- Question
- QuestionId

Create a Store.
- We need to use async Arc and RwLock to store, read, create and update questions.
```rust
use std::sync::Arc;
use tokio::sync::RwLock;
```

Read Questions from JSON file as mock data. 

