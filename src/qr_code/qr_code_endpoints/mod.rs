use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "qr_code.html")]

struct QrTemplate<'a> {
    qr_code: &'a str,
}

pub async fn qr_code_html() -> Html<String> {
    let qr_code: String = super::core::some_qr_code();
    let string = QrTemplate {
        qr_code: qr_code.to_owned().as_str(),
    }
    .render()
    .unwrap();
    Html(string)
}
