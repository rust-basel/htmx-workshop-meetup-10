mod html;
mod images;

pub use html::create_a_pr_code_image;
pub use html::create_qr_code;
pub use html::qr_code_html;
pub use images::qr_code_as_picture;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QrId {
    id: Option<String>,
}
