use anyhow::anyhow;
use image::{ImageBuffer, Luma};
use qrcode::QrCode;
use regex::Regex;

#[derive(Clone)]
pub struct QrCodeImage(pub ImageBuffer<Luma<u8>, Vec<u8>>);

impl From<ImageBuffer<Luma<u8>, Vec<u8>>> for QrCodeImage {
    fn from(value: ImageBuffer<Luma<u8>, Vec<u8>>) -> Self {
        QrCodeImage(value)
    }
}

pub fn some_qr_as_image(data: QrData) -> QrCodeImage {
    let spc = "SPC";
    let version = "0200";
    let char_set = "1";
    let iban = data.iban.raw;
    let sct = "S";
    let name = data.name.clone();
    let street = "street";
    let street_num = "14";
    let plz = "4104";
    let city = "Basel";
    let country = "CH";
    let amount = format!("{:.1}", data.amount.raw);
    let currency = data.currency.raw;
    let all_other = r"S
Pia-Maria Rutschmann-Schnyder
Grosse Marktgasse
28
9400
Rorschach
CH
QRR
210000000003139471430009017
Instruction of 03.04.2019
EPD
//S1/10/10201409/11/190512/20/1400.000-53/30/106017086/31/180508/32/7.7/40/2:10;0:30
Name AV1: UV;UltraPay005;12345
Name AV2: XY;XYService;54321";

    let line = "\n";

    let mut s = String::new();
    s.push_str(spc);
    s.push_str(line);
    s.push_str(version);
    s.push_str(line);
    s.push_str(char_set);
    s.push_str(line);
    s.push_str(&iban);
    s.push_str(line);
    s.push_str(sct);
    s.push_str(line);
    s.push_str(&name);
    s.push_str(line);
    s.push_str(&street);
    s.push_str(line);
    s.push_str(&street_num);
    s.push_str(line);
    s.push_str(&plz);
    s.push_str(line);
    s.push_str(&city);
    s.push_str(line);
    s.push_str(&country);
    s.push_str(line);
    s.push_str(line);
    s.push_str(line);
    s.push_str(line);
    s.push_str(line);
    s.push_str(line);
    s.push_str(line);
    s.push_str(line);
    s.push_str(&amount);
    s.push_str(line);
    s.push_str(&currency);
    s.push_str(line);
    s.push_str(all_other);

    println!("{}", &s);

    let code = QrCode::new(&s).unwrap();
    code.render::<Luma<u8>>().build().into()
}

pub struct QrData {
    pub iban: Iban,
    pub name: String,
    pub amount: Amount,
    pub currency: Currency,
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

pub struct Iban {
    raw: String,
}

impl TryFrom<String> for Iban {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let reg = Regex::new(r"^[A-Z]{2}[0-9]{2}[A-Z0-9]{1,30}$").unwrap();
        if reg.is_match(&value) {
            Ok(Self { raw: value })
        } else {
            Err(anyhow!("Invalid IBAN"))
        }
    }
}

pub struct Amount {
    raw: f32,
}

impl From<f32> for Amount {
    fn from(value: f32) -> Self {
        Self { raw: value }
    }
}

pub struct Currency {
    raw: String,
}

impl TryFrom<String> for Currency {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let reg = Regex::new(r"[A-Za-z]{3}").unwrap();
        if reg.is_match(&value) {
            Ok(Self { raw: value })
        } else {
            Err(anyhow!("Invalid Currency, expected three letters"))
        }
    }
}
