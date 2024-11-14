mod html;

pub use html::create;
pub use html::current;
pub use html::debug;
pub use html::dec;
pub use html::game_view;
pub use html::image;
pub use html::inc;
pub use html::page;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct QrId {
    id: String,
}
