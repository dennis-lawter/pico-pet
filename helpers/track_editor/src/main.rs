mod audio;
mod freq;
mod tui;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::Mutex;

use tui::tui;

type PlayState = Arc<Mutex<AtomicBool>>;

fn main() {
    tui();
}
