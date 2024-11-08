use std::{collections::HashMap, sync::Arc};

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

type InMemoryDb = Arc<Mutex<HashMap<String, Person>>>;

pub fn router(db: InMemoryDb) -> Router {
    Router::new()
        .route(
            "/persons",
            post(add_person_handler).get(get_all_persons_handler),
        )
        .with_state(db)
}

#[derive(Deserialize)]
pub struct PersonToAdd {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub uuid: String,
    pub name: String,
}

pub async fn add_person_handler(
    State(db): State<InMemoryDb>,
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

// TODO: This is your task ;)
pub async fn get_all_persons_handler(State(db): State<InMemoryDb>) -> impl IntoResponse {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc, usize};

    use axum::{
        body::Body,
        http::{header::CONTENT_TYPE, Method, Request},
    };
    use serde::de::DeserializeOwned;
    use tokio::sync::Mutex;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{router, Person};

    /// Just a small test helper function to turn an axum::body into
    /// a Deserializable struct
    async fn body_to_type<T: DeserializeOwned>(body: Body) -> T {
        let body_bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .expect("Failed to read response body");
        let body_str =
            String::from_utf8(body_bytes.to_vec()).expect("Failed to convert body to string");
        serde_json::from_str(&body_str).expect("Failed to parse JSON")
    }

    #[tokio::test]
    async fn given_person_when_call_post_persons_then_returns() {
        // given
        let router = router(Arc::new(Mutex::new(HashMap::new())));
        let req = Request::builder()
            .method(Method::POST)
            .header(CONTENT_TYPE, "application/json")
            .uri("/persons")
            .body(Body::from(r#"{"name":"HansPeter"}"#))
            .unwrap();

        // when
        let response = router.oneshot(req).await.unwrap();

        // then
        let person: Person = body_to_type(response.into_body()).await;
        assert_eq!(&person.name, "HansPeter")
    }

    #[tokio::test]
    async fn given_no_people_when_get_persons_returns_empty_list() {
        // given
        let router = router(Arc::new(Mutex::new(HashMap::new())));

        let req = Request::builder()
            .method(Method::GET)
            .header(CONTENT_TYPE, "application/json")
            .uri("/persons")
            .body(Body::empty())
            .unwrap();

        // when
        let response = router.oneshot(req).await.unwrap();

        // then
        let persons: Vec<Person> = body_to_type(response.into_body()).await;
        assert!(persons.is_empty());
    }

    #[tokio::test]
    async fn given_several_people_when_get_persons_returns_persons() {
        // given
        const NAME: &str = "Frieder";
        let persons: HashMap<String, Person> = [Uuid::new_v4(), Uuid::new_v4()]
            .into_iter()
            .map(|id| {
                (
                    id.to_string(),
                    Person {
                        uuid: id.to_string(),
                        name: NAME.to_string(),
                    },
                )
            })
            .collect();
        let router = router(Arc::new(Mutex::new(persons)));

        let req = Request::builder()
            .method(Method::GET)
            .header(CONTENT_TYPE, "application/json")
            .uri("/persons")
            .body(Body::empty())
            .unwrap();

        // when
        let response = router.oneshot(req).await.unwrap();

        // then
        let persons: Vec<Person> = body_to_type(response.into_body()).await;
        assert_eq!(persons.len(), 2);
        persons.iter().for_each(|p| assert_eq!(&p.name, NAME));
    }
}
