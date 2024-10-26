use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::core::QrCodeImage;

#[derive(Clone)]
pub struct QrCodeInMemoryDb {
    pub(crate) qr_codes: Arc<Mutex<HashMap<String, QrCodeImage>>>,
    pub(crate) debug_qr_codes: Arc<Mutex<HashMap<String, String>>>,
}

impl QrCodeInMemoryDb {
    pub fn new() -> Self {
        Self {
            qr_codes: Arc::new(Mutex::new(HashMap::new())),
            debug_qr_codes: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl QrCodeInMemoryDb {
    pub async fn get(&self, id: String) -> Option<QrCodeImage> {
        let codes = self.qr_codes.lock().await;
        codes.get(&id).cloned()
    }
    
    #[allow(unused)]
    pub async fn get_debug(&self, id: String) -> Option<String> {
        let codes = self.debug_qr_codes.lock().await;
        codes.get(&id).cloned()
    }
}

impl QrCodeInMemoryDb {
    pub async fn set(&self, id: String, qr_code_image: QrCodeImage) {
        let mut codes = self.qr_codes.lock().await;
        codes.insert(id, qr_code_image);
    }

    pub async fn set_debug(&self, id: String, qr_debug: String) {
        let mut codes = self.debug_qr_codes.lock().await;
        codes.insert(id, qr_debug);
    }
}

impl QrCodeInMemoryDb {
    pub async fn keys(&self) -> Vec<String> {
        let codes = self.qr_codes.lock().await;
        codes.keys().cloned().collect()
    }
}
