# How extraction works in axum

In the chapter before you are only able to answer to client requests, that do not contain any payload. But there must be a way to send our small little server a payload.

For example:
 A server, that has some sort of user management, you want to add a new person to your database at some point in time.

This is done by so called _extractors_. The incoming request is parsed, and if it for example contains a JSON payload, you have a JSON extractor, that parses that JSON.
After parsing you then can work on the data contained in the JSON.

There are also different extractors, which are ready to use. That would be for example:

- For queries: e.g. `/person?id=1234`
- For paths: e.g. `/person/1`, where the `1` would be extracted from the path
- For forms: You can directly parse a form in a handler - that will come handy later for our workshop.

You can also write your own extractor, if you need to. But this is out of scope for now (If you are interested, have a look at [axum's docs](https://docs.rs/axum/latest/axum/extract/index.html).

Below is a small snippet, where we explain, how for example a _JSON_ extractor in axum looks like, that extracts the JSON payload from the incoming request.

```rust
use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct PersonToAdd {
    name: String,
}

#[derive(Serialize)]
struct Person {
    id: i32,
    name: String,
}

async fn add_person_handler(Json(person_to_add): Json<PersonToAdd>) -> impl IntoResponse {
    // let's just echo the incoming json
    // usually you would insert this person now into your database
    // ...

    let imaginary_id = 42;
    let person = Person {
        id: imaginary_id,
        name: person_to_add.name,
    };

    Json(person)
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/person", post(add_person_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
```

If you look closely, only the handler function and the route changed in our axum's `main` function.
To be detailed our new route
expects a JSON payload, that is deserializable into `PersonToAdd`.

To send it fitting paylod, we can use _curl_, as CLI tool, with which we can send http request.

```sh
curl --header "Content-Type: application/json" --request POST --data '{"name":"Hans"}' http://localhost:3000/person
```

As with the _hello_world_handler_ from before, we can return anything from a handler, that implements _IntoResponse_, which axum's `Json<T>(T)` is doing.

We just echo the payload inside the handler and add an `id` to it, which a normal server would do, after creating a ressource in a database to confirm creation of the new ressource.
