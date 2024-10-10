use askama::Template;
use axum::{
    extract::{Query, State},
    response::Html,
};
use uuid::Uuid;

use crate::qr_code::{
    core::{some_qr_as_image, QrCodeImage},
    persictence::QrCodeInMemoryDb,
};

use super::QrId;

#[derive(Template)]
#[template(path = "qr_code.html")]
struct QrTemplate<'a> {
    available_codes: Vec<String>,
    qr_code: Option<&'a str>,
}

pub async fn qr_code_html(State(db): State<QrCodeInMemoryDb>, id: Query<QrId>) -> Html<String> {
    //get all available
    let all_keys = db.keys().await;
    let id = id.id.clone();
    let id = id.as_deref();

    let string = QrTemplate {
        qr_code: id,
        available_codes: all_keys,
    }
    .render()
    .unwrap();
    Html(string)
}

pub async fn create_qr_code(State(db): State<QrCodeInMemoryDb>) -> Html<String> {
    // just create one for now
    let (id, code) = create_a_pr_code_image().await;
    db.set(id.clone(), code).await;

    let all_keys = db.keys().await;

    let string = QrTemplate {
        qr_code: Some(id.as_str()),
        available_codes: all_keys,
    }
    .render()
    .unwrap();
    Html(string)
}
pub async fn create_a_pr_code_image() -> (String, QrCodeImage) {
    let id = Uuid::new_v4().to_string();
    let code = some_qr_as_image();
    (id, code)
}
