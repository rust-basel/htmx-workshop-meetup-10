use image::{ImageBuffer, Luma};
use qrcode::QrCode;
use rand::{distributions::Alphanumeric, Rng};

#[derive(Clone)]
pub struct QrCodeImage(pub ImageBuffer<Luma<u8>, Vec<u8>>);

impl From<ImageBuffer<Luma<u8>, Vec<u8>>> for QrCodeImage {
    fn from(value: ImageBuffer<Luma<u8>, Vec<u8>>) -> Self {
        QrCodeImage(value)
    }
}

pub fn some_qr_as_image() -> QrCodeImage {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    let code = QrCode::new(format!("Hello World: {}", random_string)).unwrap();
    code.render::<Luma<u8>>().build().into()
}
