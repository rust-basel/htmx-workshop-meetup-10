use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::core::QrCodeImage;

#[derive(Clone)]
pub struct QrCodeInMemoryDb {
    pub(crate) qr_codes: Arc<Mutex<HashMap<String, QrCodeImage>>>,
    pub(crate) debug_qr_codes: Arc<Mutex<HashMap<String, String>>>,
    pub(crate) game_state: Arc<AtomicI64>,
}

impl QrCodeInMemoryDb {
    pub fn new() -> Self {
        Self {
            qr_codes: Arc::new(Mutex::new(HashMap::new())),
            debug_qr_codes: Arc::new(Mutex::new(HashMap::new())),
            game_state: Arc::new(AtomicI64::new(0)),
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
    pub async fn increment(&self) -> i64 {
        self.game_state.fetch_add(1, Ordering::Relaxed)
    }

    pub async fn decrement(&self) -> i64 {
        self.game_state.fetch_sub(1, Ordering::Relaxed)
    }

    pub async fn get_game_state(&self) -> i64 {
        self.game_state.load(Ordering::Relaxed)
    }
}
