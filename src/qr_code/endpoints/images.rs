use axum::extract::State;
use axum::http::StatusCode;
use axum::{body::Body, extract::Query};
use axum::{
    http::{header, Response},
    response::IntoResponse,
};

use crate::qr_code::persictence::QrCodeInMemoryDb;

pub async fn qr_code_as_picture(
    State(db): State<QrCodeInMemoryDb>,
    qr_code_id: Query<super::QrId>,
) -> impl IntoResponse {
    // TODO handle unwrap
    let id = qr_code_id.id.clone().unwrap();
    let Some(image) = db.get(id).await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let response = Response::builder()
        .header(header::CONTENT_TYPE, mime::IMAGE_SVG.as_ref())
        .header("HX-Trigger", "update_debug")
        .body(Body::from(image.0))
        .unwrap();

    response
}

pub async fn qr_code_debug(
    State(db): State<QrCodeInMemoryDb>,
    qr_code_id: Query<super::QrId>,
) -> impl IntoResponse {
    // TODO handle unwrap
    let id = qr_code_id.id.clone().unwrap();

    let Some(debug) = db.get_debug(id).await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let response = Response::builder()
        .header(header::CONTENT_TYPE, mime::TEXT.as_ref())
        .body(Body::from(debug))
        .unwrap();

    response
}
