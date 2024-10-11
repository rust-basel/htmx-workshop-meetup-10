mod html;
mod images;

pub use html::create;
pub use html::create_image;
pub use html::page;
pub use html::qr_table;
pub use images::qr_code_as_picture;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct QrId {
    id: Option<String>,
}
