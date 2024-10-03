use axum::{body::Body, extract::Query};
use axum::{
    http::{header, Response},
    response::IntoResponse,
};
use image::ImageFormat;

use std::io::{BufWriter, Cursor};

use crate::qr_code::core::some_qr_as_image;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QrId {
    _id: String,
}

pub async fn qr_code_as_picture(_qr_code_id: Query<QrId>) -> impl IntoResponse {
    let image = some_qr_as_image();

    let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
    image.write_to(&mut buffer, ImageFormat::Png).unwrap();

    let bytes: Vec<u8> = buffer.into_inner().unwrap().into_inner();

    let response = Response::builder()
        .header(header::CONTENT_TYPE, mime::IMAGE_PNG.as_ref())
        .body(Body::from(bytes))
        .unwrap();

    response
}
