use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use rodio::OutputStream;
use rodio::Sink;

use crate::freq::FREQ_TABLE;
use crate::prelude::*;

const SAMPLE_RATE: u32 = 44_100;
const VOLUME: f32 = 0.15;

pub fn play_preview(play_state: PlayState) {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let state = play_state.lock().unwrap();
        state.store(true, Ordering::SeqCst);
        drop(state); // Release the lock

        for (_name, freq) in FREQ_TABLE.iter() {
            let state = play_state.lock().unwrap();
            if !state.load(Ordering::SeqCst) {
                break;
            }
            drop(state); // Release the lock

            let samples = (0..SAMPLE_RATE)
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
