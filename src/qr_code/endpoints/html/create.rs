use askama::Template;
use axum::{extract::State, http::StatusCode, response::Html, Form};
use serde::Deserialize;
use uuid::Uuid;

use crate::qr_code::{
    core::{some_qr_as_image, Amount, Currency, Iban, QrCodeImage, QrData},
    persictence::QrCodeInMemoryDb,
};

#[derive(Template)]
#[template(path = "created_qr.html")]
struct CreatedQrTemplate<'a> {
    qr_code: Option<&'a str>,
}

#[derive(Deserialize)]
pub struct CreateQrForm {
    iban: String,
    qr_code_name: String,
    qrcode_amount: f32,
    currency: String,
}

pub async fn create(
    State(db): State<QrCodeInMemoryDb>,
    Form(create_data): Form<CreateQrForm>,
) -> Result<Html<String>, StatusCode> {
    // just create one for now
    let data = match create_data.try_into() {
        Ok(d) => d,
        Err(e) => {
            println!("QrCode error: {e}");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let (id, code) = create_image(data).await;
    db.set(id.clone(), code).await;

    let string = CreatedQrTemplate {
        qr_code: Some(id.as_str()),
    }
    .render()
    .unwrap();
    Ok(Html(string))
}

impl TryFrom<CreateQrForm> for QrData {
    type Error = anyhow::Error;

    fn try_from(value: CreateQrForm) -> Result<Self, Self::Error> {
        let iban: Iban = value.iban.try_into()?;
        let name: String = value.qr_code_name;
        let amount: Amount = value.qrcode_amount.into();
        let currency: Currency = value.currency.try_into()?;

        Ok(Self {
            iban,
            name,
            amount,
            currency,
        })
    }
}

pub async fn create_image(data: QrData) -> (String, QrCodeImage) {
    let id = Uuid::new_v4().to_string();
    let code = some_qr_as_image(data);
    (id, code)
}
