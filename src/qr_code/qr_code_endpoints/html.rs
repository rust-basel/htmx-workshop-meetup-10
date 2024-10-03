use askama::Template;
use axum::{extract::Query, response::Html};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "qr_code.html")]
struct QrTemplate<'a> {
    qr_code: &'a str,
}

#[derive(Deserialize)]
pub struct QrId {
    id: String,
}

pub async fn qr_code_html(qr_code_id: Query<QrId>) -> Html<String> {
    let string = QrTemplate {
        qr_code: &qr_code_id.0.id,
    }
    .render()
    .unwrap();
    Html(string)
}
