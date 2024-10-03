use image::{ImageBuffer, Luma};
use qrcode::QrCode;

pub fn some_qr_as_image() -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let code = QrCode::new("Hello World").unwrap();
    code.render::<Luma<u8>>().build()
}
