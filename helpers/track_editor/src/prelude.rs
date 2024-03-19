use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::Mutex;

pub type PlayState = Arc<Mutex<AtomicBool>>;
