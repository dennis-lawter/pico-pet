use lazy_static::lazy_static;
use std::sync::Arc;
use std::sync::Mutex;

use crate::freq::get_freq;
use crate::prelude::*;

pub struct Track {
    pub speed_divisor: u8,
    pub title: String,
    pub samples: Vec<f32>,
    pub playing: bool,
    // pub samples_played: usize,
}
impl Track {
    fn new() -> Self {
        Self {
            speed_divisor: 1,
            title: "".to_string(),
            samples: vec![],
            playing: false,
            // samples_played: 0,
        }
    }
    pub fn load_text(&mut self, text: &str) {
        let (header, notes_string) = text
            .split_once("\n\n")
            .expect("Invalid track, no header divider");
        let header_lines: Vec<&str> = header.lines().collect();
        assert!(header_lines.len() == 3, "Not enough lines");
        assert_eq!(header_lines[0], "PEAT 1", "Invalid header: PEAT version");
        assert!(
            header_lines[1].starts_with("NPMD "),
            "Invalid header: NPMD value"
        );

        self.title = header_lines[2].to_string();
        self.speed_divisor = header_lines[1]
            .strip_prefix("NPMD ")
            .expect("Invalid header")
            .parse::<u8>()
            .expect("Invalid speed divisor");

        let samples_per_minute = SAMPLE_RATE * 60;
        let effective_notes_per_minute = NOTES_PER_MINUTE / self.speed_divisor as u32;
        let samples_in_note_duration = samples_per_minute / effective_notes_per_minute;

        let mut previous_frequency: Option<f32> = None;
        let mut samples: Vec<f32> = Vec::new();

        let notes: Vec<_> = notes_string.split_whitespace().collect();
        for note in notes {
            let freq = match note {
                "." => match previous_frequency {
                    Some(freq) => freq,
                    None => panic!("Sustain without prior note"),
                },
                "" => continue, // TODO handle follower offset
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

        self.samples = samples;
    }
}
lazy_static! {
    pub static ref TRACK_INSTANCE: Arc<Mutex<Track>> = Arc::new(Mutex::new(Track::new()));
}
