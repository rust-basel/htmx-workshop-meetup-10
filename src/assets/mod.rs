use axum::{
    body::Body,
    http::{header, Response},
    response::IntoResponse,
};

pub async fn include_htmx() -> impl IntoResponse {
    let htmx = include_str!("../../assets/htmx.min.js");
    Response::builder()
        .header(header::CONTENT_TYPE, mime::TEXT_JAVASCRIPT.as_ref())
        .body(Body::from(htmx))
        .expect("could not parse headers")
}

pub async fn include_app_css() -> impl IntoResponse {
    let app = include_str!("../../assets/app.css");
    Response::builder()
        .header(header::CONTENT_TYPE, mime::TEXT_CSS.as_ref())
        .body(Body::from(app))
        .expect("could not parse headers")
}

pub async fn include_pico_css() -> impl IntoResponse {
    let pico = include_str!("../../assets/pico.min.css");
    Response::builder()
        .header(header::CONTENT_TYPE, mime::TEXT_CSS.as_ref())
        .body(Body::from(pico))
        .expect("could not parse headers")
}
