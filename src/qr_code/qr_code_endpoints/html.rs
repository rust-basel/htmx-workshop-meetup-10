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
    qr_code: Option<&'a str>,
}

pub async fn qr_code_html(State(db): State<QrCodeInMemoryDb>, id: Query<QrId>) -> Html<String> {
    let keys = db.keys().await;
    let query_id = id.id.clone();
    let query_id = query_id.as_deref();

    let actual_id = match query_id {
        Some(id) => keys
            .iter()
            .find(|k| **k == id)
            .or(keys.first())
            .map(|s| s.as_str()),
        None => None,
    };

    let string = QrTemplate { qr_code: actual_id }.render().unwrap();
    Html(string)
}

#[derive(Template)]
#[template(path = "created_qr.html")]
struct CreatedQrTemplate<'a> {
    qr_code: Option<&'a str>,
}

pub async fn create_qr_code(State(db): State<QrCodeInMemoryDb>) -> Html<String> {
    // just create one for now
    let (id, code) = create_a_pr_code_image().await;
    db.set(id.clone(), code).await;

    let string = CreatedQrTemplate {
        qr_code: Some(id.as_str()),
    }
    .render()
    .unwrap();
    Html(string)
}

#[derive(Template)]
#[template(path = "qr_table.html")]
struct QrTableTemplate {
    available_codes: Vec<String>,
}

pub async fn qr_table(State(db): State<QrCodeInMemoryDb>) -> Html<String> {
    //get all available
    let all_keys = db.keys().await;

    let string = QrTableTemplate {
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
