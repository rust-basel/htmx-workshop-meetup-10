use askama::Template;
use axum::{extract::State, response::Html};
use uuid::Uuid;

use crate::qr_code::{
    core::{some_qr_as_image, QrCodeImage},
    persictence::QrCodeInMemoryDb,
};

#[derive(Template)]
#[template(path = "created_qr.html")]
struct CreatedQrTemplate<'a> {
    qr_code: Option<&'a str>,
}

pub async fn create(State(db): State<QrCodeInMemoryDb>) -> Html<String> {
    // just create one for now
    let (id, code) = create_image().await;
    db.set(id.clone(), code).await;

    let string = CreatedQrTemplate {
        qr_code: Some(id.as_str()),
    }
    .render()
    .unwrap();
    Html(string)
}

pub async fn create_image() -> (String, QrCodeImage) {
    let id = Uuid::new_v4().to_string();
    let code = some_qr_as_image();
    (id, code)
}
