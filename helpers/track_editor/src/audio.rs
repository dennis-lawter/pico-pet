use crate::freq::get_freq;
use crate::prelude::*;

use std::thread;
// use std::time::Duration;

use rodio::OutputStream;
use rodio::Sink;

const SAMPLE_RATE: u32 = 44_100;
const VOLUME: f32 = 0.05;

fn play_samples(text: &str, speed_divisor: u8) {
    // panic!("{}", text);
    // let text = "C4\nC4\nC4\nC4\n";
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let speed_divisor = speed_divisor as u32;
    let samples_per_minute = SAMPLE_RATE * 60;
    let effective_notes_per_minute = NOTES_PER_MINUTE / speed_divisor;
    let samples_in_note_duration = samples_per_minute / effective_notes_per_minute;
    let mut previous_frequency: Option<f32> = None;
    let mut samples: Vec<f32> = Vec::new();
    for line in text.lines() {
        let freq = match line {
            "." => match previous_frequency {
                Some(freq) => freq,
                None => panic!("Sustain without prior note"),
            },
            "" => continue,
            other_text => match get_freq(other_text) {
                Some(freq) => freq,
                None => {
                    panic!("Invalid note: {}", other_text);
                }
            },
        };
        previous_frequency = Some(freq);
        let mut note_samples = (0..samples_in_note_duration)
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
        samples.append(&mut note_samples);
    }

    let source = rodio::buffer::SamplesBuffer::new(1, SAMPLE_RATE, samples);
    sink.append(source);

    while sink.len() > 0 {
        // thread::sleep(Duration::from_millis(10));
        let model = crate::model::TRACK_INSTANCE.lock().unwrap();
        if !model.playing {
            return;
        }
        drop(model);
    }
}

pub fn play_preview() {
    thread::spawn(move || {
        let model = crate::model::TRACK_INSTANCE.lock().unwrap();
        let local_speed_divisor = model.speed_divisor.clone();
        let local_text = model.text.clone();
        drop(model);

        play_samples(&local_text, local_speed_divisor);

        // for (_name, freq) in FREQ_TABLE.iter() {
        //     let speed_mult_value = local_speed_divisor as u32;
        //     let samples_per_minute = SAMPLE_RATE * 60;
        //     let effective_notes_per_minute = NOTES_PER_MINUTE / speed_mult_value;
        //     let samples_in_note_duration = samples_per_minute / effective_notes_per_minute;

        //     let samples = (0..samples_in_note_duration)
        //         .map(move |i| {
        //             let time = i as f32 / SAMPLE_RATE as f32;
        //             let period = 1.0 / freq;
        //             if time % period < period / 2.0 {
        //                 VOLUME
        //             } else {
        //                 -VOLUME
        //             }
        //         })
        //         .collect::<Vec<_>>();

        //     let source = rodio::buffer::SamplesBuffer::new(1, SAMPLE_RATE, samples);
        //     sink.append(source);

        //     while sink.len() > 0 {
        //         thread::sleep(Duration::from_millis(10));
        //         let model = crate::model::TRACK_INSTANCE.lock().unwrap();
        //         if !model.playing {
        //             return;
        //         }
        //         drop(model);
        //     }
        // }
        let mut model = crate::model::TRACK_INSTANCE.lock().unwrap();
        // TODO: use an arc mutex around cursive to update the button?
        model.playing = false;
        drop(model);
    });
}
