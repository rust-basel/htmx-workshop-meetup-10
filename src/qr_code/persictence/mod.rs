use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::core::QrCodeImage;

#[derive(Clone)]
pub struct QrCodeInMemoryDb {
    pub(crate) qr_codes: Arc<Mutex<HashMap<String, QrCodeImage>>>,
}

impl QrCodeInMemoryDb {
    pub fn new() -> Self {
        Self {
            qr_codes: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl QrCodeInMemoryDb {
    pub async fn get(&self, id: String) -> Option<QrCodeImage> {
        let codes = self.qr_codes.lock().await;
        codes.get(&id).cloned()
    }
}

impl QrCodeInMemoryDb {
    pub async fn set(&self, id: String, qr_code_image: QrCodeImage) {
        let mut codes = self.qr_codes.lock().await;
        codes.insert(id, qr_code_image);
    }
}

impl QrCodeInMemoryDb {
    pub async fn keys(&self) -> Vec<String> {
        let codes = self.qr_codes.lock().await;
        codes.keys().cloned().collect()
    }
}
