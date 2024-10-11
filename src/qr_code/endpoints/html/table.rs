use askama::Template;
use axum::{extract::State, response::Html};

use crate::qr_code::persictence::QrCodeInMemoryDb;

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
