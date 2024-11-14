use axum::extract::State;
use axum::http::StatusCode;
use axum::{body::Body, extract::Query};
use axum::{
    http::{header, Response},
    response::IntoResponse,
};

use crate::qr_code::endpoints::QrId;
use crate::qr_code::persictence::QrCodeInMemoryDb;

pub async fn image(
    State(db): State<QrCodeInMemoryDb>,
    qr_code_id: Query<QrId>,
) -> impl IntoResponse {
    let id = qr_code_id.id.to_owned();

    let Some(image) = db.get(id).await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let response = Response::builder()
        .header(header::CONTENT_TYPE, mime::IMAGE_SVG.as_ref())
        .header("HX-Trigger", "update_debug")
        .body(Body::from(image.0))
        .expect("could not parse headers");

    response
}

pub async fn debug(
    State(db): State<QrCodeInMemoryDb>,
    qr_code_id: Query<QrId>,
) -> impl IntoResponse {
    let id = qr_code_id.id.to_owned();

    let Some(debug) = db.get_debug(id).await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let response = Response::builder()
        .header(header::CONTENT_TYPE, mime::TEXT.as_ref())
        .body(Body::from(debug))
        .expect("could not parse headers");

    response
}
