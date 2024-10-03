use qrcode::QrCode;

pub fn some_qr_code() -> String {
    let code = QrCode::new("Hello World").unwrap();
    code.render()
        .light_color(' ')
        .dark_color('#')
        .build()
        .clone()
}
