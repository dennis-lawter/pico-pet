use crate::prelude::*;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use rodio::OutputStream;
use rodio::Sink;

use crate::freq::FREQ_TABLE;

const SAMPLE_RATE: u32 = 44_100;
const VOLUME: f32 = 0.15;

pub fn play_preview(play_state: Arc<Mutex<AtomicBool>>, speed_multiplier: Arc<Mutex<u8>>) {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let state_lock = play_state.lock().unwrap();
        state_lock.store(true, Ordering::SeqCst);
        drop(state_lock);

        for (_name, freq) in FREQ_TABLE.iter() {
            let state_lock = play_state.lock().unwrap();
            if !state_lock.load(Ordering::SeqCst) {
                break;
            }
            drop(state_lock);

            let speed_mult_lock = speed_multiplier.lock().unwrap();
            let speed_mult_value = speed_mult_lock.clone() as u32;
            let samples_per_minute = SAMPLE_RATE * 60;
            let effective_notes_per_minute = NOTES_PER_MINUTE / speed_mult_value;
            let samples_in_note_duration = (samples_per_minute / effective_notes_per_minute);

            let samples = (0..samples_in_note_duration)
                .map(move |i| {
                    let time = i as f32 / SAMPLE_RATE as f32;
                    let period = 1.0 / freq;
                    if time % period < period / 2.0 {
                        VOLUME
                    } else {
                        -VOLUME
                    }
                })
                .collect::<Vec<_>>();

            drop(speed_mult_lock);

            let source = rodio::buffer::SamplesBuffer::new(1, SAMPLE_RATE, samples);
            sink.append(source);

            while sink.len() > 0 {
                thread::sleep(Duration::from_millis(10));
                let state = play_state.lock().unwrap();
                if !state.load(Ordering::SeqCst) {
                    break;
                }
            }
        }

        // Mark playback as inactive
        let state = play_state.lock().unwrap();
        state.store(false, Ordering::SeqCst);
    });
}
