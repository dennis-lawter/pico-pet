use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Track {
    pub speed_multiplier: Arc<Mutex<u8>>,
    pub text: Arc<Mutex<String>>,
    pub playing: Arc<Mutex<AtomicBool>>,
}
impl Track {
    pub fn new() -> Self {
        Self {
            speed_multiplier: Arc::new(Mutex::new(1)),
            text: Arc::new(Mutex::new("".to_string())),
            playing: Arc::new(Mutex::new(AtomicBool::new(false))),
        }
    }
}
