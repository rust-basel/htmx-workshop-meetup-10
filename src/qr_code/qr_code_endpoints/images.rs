use axum::extract::State;
use axum::http::StatusCode;
use axum::{body::Body, extract::Query};
use axum::{
    http::{header, Response},
    response::IntoResponse,
};
use image::ImageFormat;

use std::io::{BufWriter, Cursor};

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

    let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
    image.0.write_to(&mut buffer, ImageFormat::Png).unwrap();

    let bytes: Vec<u8> = buffer.into_inner().unwrap().into_inner();

    let response = Response::builder()
        .header(header::CONTENT_TYPE, mime::IMAGE_PNG.as_ref())
        .body(Body::from(bytes))
        .unwrap();

    response
}
