use cursive::views::ProgressBar;
use cursive::Cursive;
use rodio::OutputStream;
use rodio::Sink;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::prelude::SAMPLE_RATE;

pub fn play_preview(siv: &mut Cursive) {
    let cb_sink = siv.cb_sink().clone();

    let model = Arc::clone(&crate::model::TRACK_INSTANCE);

    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let mut samples = {
            let model = model.lock().unwrap();
            model.samples.clone()
        };

        let split_size = SAMPLE_RATE as usize * 60 / crate::prelude::NOTES_PER_MINUTE as usize;
        while samples.len() > 0 {
            let (one_note, remaining_samples) = if samples.len() > split_size {
                let splits = samples.split_at(split_size);
                (splits.0.to_vec(), splits.1.to_vec())
            } else {
                (samples, vec![])
            };

            let source = rodio::buffer::SamplesBuffer::new(1, SAMPLE_RATE, one_note);
            sink.append(source);
            samples = remaining_samples;
        }

        let initial_len = sink.len();

        while !sink.empty() {
            {
                let model = model.lock().unwrap();
                if !model.playing {
                    break;
                }
            }

            let sink_len = sink.len();
            let progress = initial_len - sink_len;

            cb_sink
                .send(Box::new(move |s: &mut Cursive| {
                    s.call_on_name("progress_bar", |p: &mut ProgressBar| {
                        p.set_value(progress);
                    });
                }))
                .expect("Failed to send update to cursive");

            thread::sleep(Duration::from_millis(10));
        }
    });
}
