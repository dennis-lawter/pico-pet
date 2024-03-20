use lazy_static::lazy_static;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Track {
    pub speed_divisor: u8,
    pub text: String,
    pub playing: bool,
}
impl Track {
    fn new() -> Self {
        Self {
            speed_divisor: 1,
            text: "".to_string(),
            playing: false,
        }
    }
}
lazy_static! {
    pub static ref TRACK_INSTANCE: Arc<Mutex<Track>> = Arc::new(Mutex::new(Track::new()));
}
