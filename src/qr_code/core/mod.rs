use image::{ImageBuffer, Luma};
use qrcode::QrCode;

#[derive(Clone)]
pub struct QrCodeImage(pub ImageBuffer<Luma<u8>, Vec<u8>>);

impl From<ImageBuffer<Luma<u8>, Vec<u8>>> for QrCodeImage {
    fn from(value: ImageBuffer<Luma<u8>, Vec<u8>>) -> Self {
        QrCodeImage(value)
    }
}

pub fn some_qr_as_image() -> QrCodeImage {
    let code = QrCode::new("Hello World").unwrap();
    code.render::<Luma<u8>>().build().into()
}
