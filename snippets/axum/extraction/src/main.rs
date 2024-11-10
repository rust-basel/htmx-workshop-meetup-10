use axum::{response::IntoResponse, routing::post, Form, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct MyForm {
    name: String,
    age: u8,
}

#[derive(Serialize, Deserialize)]
struct EchoedForm {
    echo_name: String,
    echo_age: u8,
}

// TODO: You have to implement this one :)
async fn my_form(Form(form): Form<MyForm>) -> impl IntoResponse {
    todo!()
}

fn router() -> Router {
    Router::new().route("/submit", post(my_form))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router()).await.unwrap();
}

#[cfg(test)]
mod tests {
    use crate::{my_form, router, EchoedForm};

    use axum::{
        body::Body,
        http::{header::CONTENT_TYPE, Method, Request},
        routing::post,
        Router,
    };
    use serde::de::DeserializeOwned;
    use tower::ServiceExt;

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
    async fn given_form_when_posting_form_then_echoes_form_input() {
        // given
        let router = router();
        let form_data = "name=HansPeter&age=47";
        let req = Request::builder()
            .method(Method::POST)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .uri("/submit")
            .body(Body::from(form_data))
            .unwrap();

        // when
        let response = router.oneshot(req).await.unwrap();

        // then
        let echoed_form: EchoedForm = body_to_type(response.into_body()).await;
        assert_eq!(&echoed_form.echo_name, "HansPeter");
        assert_eq!(echoed_form.echo_age, 47);
    }
}
