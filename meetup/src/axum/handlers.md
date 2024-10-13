# How to add Handlers

Below is a small boilerplate in axum, which we examine, on how to write a handler in axum:

```rust
use axum::{response::IntoResponse, routing::get, Router};

async fn hello_world_handler() -> impl IntoResponse {
    "Hello World"
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(hello_world_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
```

As everything is asynchronous, we will need an asynchronous runtime, that executes our async functions. We will use [tokio](https://tokio.rs/) for this (as axum is also from the same guys).

A handler in axum is just an ordinary asynchronous free function (`hello_world_handler`), that returns something, that implements `IntoResponse`. `IntoResponse` is a trait (Rust's interfaces) that tell the compiler, that this thing is convertible into a response.

To look what this means, you would have to look into the definition of `IntoResponse`. Fow now, it's sufficient to know, that it returns a payload and a status code.

For this example our handler returns a string literal, which will be converted in a http response of with payload of type `text/plain` with status code 200.
