use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "qr_code.html")]
struct QrTemplate {}

pub async fn page() -> Html<String> {
    let rendered = QrTemplate {}.render().unwrap();
    Html(rendered)
}
