use anyhow::anyhow;
use qrbill::{Address, Currency, Iban, QRBill, QRBillOptions, Reference, StructuredAddress};
use regex::Regex;

#[derive(Clone)]
pub struct QrCodeImage(pub String);

impl From<String> for QrCodeImage {
    fn from(value: String) -> Self {
        QrCodeImage(value)
    }
}

pub fn some_qr_as_image(data: QrData) -> QRBill {
    let c = data.currency.raw;
    let qr_currency = match c.as_str() {
        "CHF" => Currency::SwissFranc,
        "EUR" => Currency::Euro,
        _ => Currency::SwissFranc,
    };

    QRBill::new(QRBillOptions {
        account: data.iban.raw.parse::<Iban>().unwrap(),
        creditor: Address::Structured(StructuredAddress {
            name: data.name.to_string(),
            street: "Tellstrasse".to_string(),
            house_number: "66".to_string(),
            postal_code: "4053".to_string(),
            city: "Basel".to_string(),
            country: isocountry::CountryCode::CHE,
        }),
        amount: Some(data.amount.raw),
        currency: qr_currency,
        due_date: None,
        debtor: None,
        reference: Reference::None,
        extra_infos: None,
        alternative_processes: vec![],
        language: qrbill::Language::German,
        top_line: true,
        payment_line: true,
    })
    .unwrap()
}

pub struct QrData {
    pub iban: SimpleIban,
    pub name: String,
    pub amount: Amount,
    pub currency: SimpleCurrency,
}

impl QrData {
    pub fn test_code() -> Self {
        Self {
            iban: "CH5604835012345678009".to_owned().try_into().unwrap(),
            name: "Rust Basel".to_owned(),
            amount: 200.0.into(),
            currency: "CHF".to_owned().try_into().unwrap(),
        }
    }
}

pub struct SimpleIban {
    raw: String,
}

impl TryFrom<String> for SimpleIban {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let reg = Regex::new(r"^[A-Z]{2}[0-9]{2}[A-Z0-9]{1,30}$")?;
        if reg.is_match(&value) {
            Ok(Self { raw: value })
        } else {
            Err(anyhow!("Invalid IBAN"))
        }
    }
}

pub struct Amount {
    raw: f64,
}

impl From<f32> for Amount {
    fn from(value: f32) -> Self {
        Self { raw: value.into() }
    }
}

pub struct SimpleCurrency {
    raw: String,
}

impl TryFrom<String> for SimpleCurrency {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let reg = Regex::new(r"[A-Za-z]{3}")?;
        if reg.is_match(&value) {
            Ok(Self { raw: value })
        } else {
            Err(anyhow!("Invalid Currency, expected three letters"))
        }
    }
}
