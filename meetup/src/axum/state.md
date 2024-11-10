# State

In this chapter we look how you add state to your webserver. Then we'll look how you
can access this state in your handlers.

## How to add state

What's a webserver worth, if it cannot handle state?
In this example we will fill a `HashMap` of people (from the previous chapter)
and add a query to fetch all - and fetch one. That will be your task.

To add state in axum we have to options:

- `State`: it's type safe - will fail at compile time
- `Extension`: Not typesafe - i.e will fail at runtime

We will use State here. To add a state, you can call the `.with_state()` at the
end of your `Router Builder`. We will just put a HashMap for learning reason (It's wrapped in an `Arc<Mutex<_>>`, so we can share it between threads). Usually you would
give a handle to a database pool - which the handler then can use to
make queries and insertions to a database.

```rust
#[tokio::main]
async fn main() {
    let persons: HashMap<String, Person> = HashMap::new();
    let in_memory_db = Arc::new(Mutex::new(persons));

    let router = Router::new()
        .route("/persons", post(add_person_handler))
        .with_state(in_memory_db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
```

## How to access (and mutate) state

With axum's extraction logic, you can also extract `State` in the same way in the handler like a `Json` extractor.
Now you can really add persons in the `add_person_handler`, like the following:

```rust
async fn add_person_handler(
    State(db): State<InMemoryDb>, // <-- State extractor - here we get the state
    Json(person_to_add): Json<PersonToAdd>,
) -> impl IntoResponse {
    let mut db_lock = db.lock().await; // <-- Get the mutex lock

    let uuid = Uuid::new_v4();
    let person = Person {
        uuid: uuid.to_string(),
        name: person_to_add.name,
    };
    db_lock.insert(uuid.to_string(), person.clone()); // <-- Insert the person

    Json(person)
}
```

Let's write a small handler to receive all of our current person - but this time as a small challenge for you ;).

## Your Task

- Go into the project `snippets/axum/state` and open your IDE of choice there.
- Write a handler, that returns a list of `Person` as Json, when you call `GET /persons`. Of course the list
  will only be filled up, if you posted some persons
- We already gave you a small starter here:

```rust
// TODO: This is your task ;)
pub async fn get_all_persons_handler(State(db): State<InMemoryDb>) -> impl IntoResponse {
    todo!()
}
```

- run `cargo test` to verify you implemented the handler correctly

_Hint_: You can also test your webserver afterwards with `cargo run`. To add persons with this curl command:

```sh
curl --header "Content-Type: application/json" --request POST --data '{"name":"Hans"}' http://localhost:3000/persons
```
