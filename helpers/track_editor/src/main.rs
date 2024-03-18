use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use cursive::Cursive;
use cursive::CursiveExt;
use rodio::OutputStream;
use rodio::Sink;

const FREQ_TABLE: [(&str, f32); 37] = [
    ("C4", 261.63),
    ("Cs4", 277.18),
    ("D4", 293.66),
    ("Ds4", 311.13),
    ("E4", 329.63),
    ("F4", 349.23),
    ("Fs4", 369.99),
    ("G4", 392.0),
    ("Gs4", 415.3),
    ("A4", 440.0),
    ("As4", 466.16),
    ("B4", 493.88),
    ("C5", 523.25),
    ("Cs5", 554.37),
    ("D5", 587.33),
    ("Ds5", 622.25),
    ("E5", 659.25),
    ("F5", 698.46),
    ("Fs5", 739.99),
    ("G5", 783.99),
    ("Gs5", 830.61),
    ("A5", 880.0),
    ("As5", 932.33),
    ("B5", 987.77),
    ("C6", 1046.5),
    ("Cs6", 1108.73),
    ("D6", 1174.66),
    ("Ds6", 1244.51),
    ("E6", 1318.51),
    ("F6", 1396.91),
    ("Fs6", 1479.98),
    ("G6", 1567.98),
    ("Gs6", 1661.22),
    ("A6", 1760.0),
    ("As6", 1864.66),
    ("B6", 1975.53),
    ("C7", 2093.0),
];

const SAMPLE_RATE: u32 = 44_100;

type PlayState = Arc<Mutex<AtomicBool>>;

fn main() {
    let mut siv = Cursive::default();

    siv.load_toml(include_str!("../assets/style.toml")).unwrap();

    let play_state = Arc::new(Mutex::new(AtomicBool::new(false)));

    siv.add_global_callback(' ', {
        let play_state = play_state.clone();
        move |_| {
            let state = play_state.lock().unwrap();
            if state.load(Ordering::SeqCst) {
                // Stop playback
                state.store(false, Ordering::SeqCst);
            } else {
                // Start playback
                let play_state = play_state.clone();
                play_preview(play_state);
            }
        }
    });
    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}

fn play_preview(play_state: PlayState) {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Mark playback as active
        let state = play_state.lock().unwrap();
        state.store(true, Ordering::SeqCst);
        drop(state); // Release the lock

        for (_name, freq) in FREQ_TABLE.iter() {
            // Check if playback has been stopped
            let state = play_state.lock().unwrap();
            if !state.load(Ordering::SeqCst) {
                break; // Exit the loop if playback was stopped
            }
            drop(state); // Release the lock

            let samples = (0..SAMPLE_RATE)
                .map(move |i| {
                    let time = i as f32 / SAMPLE_RATE as f32;
                    let period = 1.0 / freq;
                    if time % period < period / 2.0 {
                        0.5 // High state
                    } else {
                        -0.5 // Low state
                    }
                })
                .collect::<Vec<_>>();

            let source = rodio::buffer::SamplesBuffer::new(1, SAMPLE_RATE, samples);
            sink.append(source);

            // Wait for the note to finish playing
            while sink.len() > 0 {
                thread::sleep(Duration::from_millis(10));
                // Check again inside the loop
                let state = play_state.lock().unwrap();
                if !state.load(Ordering::SeqCst) {
                    break; // Stop playback if requested
                }
            }
        }

        // Mark playback as inactive
        let state = play_state.lock().unwrap();
        state.store(false, Ordering::SeqCst);
    });
}
