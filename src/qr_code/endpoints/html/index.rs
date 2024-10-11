use askama::Template;
use axum::{
    extract::{Query, State},
    response::Html,
};

use crate::qr_code::{endpoints::QrId, persictence::QrCodeInMemoryDb};

#[derive(Template)]
#[template(path = "qr_code.html")]
struct QrTemplate<'a> {
    qr_code: Option<&'a str>,
}

pub async fn page(State(db): State<QrCodeInMemoryDb>, id: Query<QrId>) -> Html<String> {
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
