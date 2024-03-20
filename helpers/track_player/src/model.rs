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
    pub samples_played: usize,
}
impl Track {
    fn new() -> Self {
        Self {
            speed_divisor: 1,
            title: "".to_string(),
            samples: vec![],
            playing: false,
            samples_played: 0,
        }
    }
    pub fn load_text(&mut self, text: &str) {
        let lines: Vec<&str> = text.lines().collect();
        assert!(lines.len() >= 5, "Not enough lines");
        assert_eq!(lines[0], "PEAT 1", "Invalid header: PEAT version");
        assert!(lines[1].starts_with("NPMD "), "Invalid header: NPMD value");
        assert_eq!(
            lines[2], "",
            "Invalid header, expected empty line on line 3"
        );
        assert_eq!(
            lines[4], "",
            "Invalid header, expected empty line on line 5"
        );

        self.title = lines[3].to_string();
        self.speed_divisor = lines[1]
            .strip_prefix("NPMD ")
            .expect("Invalid header")
            .parse::<u8>()
            .expect("Invalid speed divisor");

        let samples_per_minute = SAMPLE_RATE * 60;
        let effective_notes_per_minute = NOTES_PER_MINUTE / self.speed_divisor as u32;
        let samples_in_note_duration = samples_per_minute / effective_notes_per_minute;

        let mut previous_frequency: Option<f32> = None;
        let mut samples: Vec<f32> = Vec::new();
        for line in lines.iter().skip(5) {
            let line = *line;
            let freq = match line {
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
