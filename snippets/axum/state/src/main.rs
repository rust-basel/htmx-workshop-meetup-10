use state::{router, Person};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let persons: HashMap<String, Person> = HashMap::new();
    let in_memory_db = Arc::new(Mutex::new(persons));

    let router = router(in_memory_db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
