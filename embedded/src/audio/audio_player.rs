use crate::hardware::audio::AudioFrequency as Freq;

use super::audio_library::AudioId;
use super::audio_track::AudioTrack;

pub struct AudioPlayer {
    repeat: bool,
    tracker: Option<usize>,
    current_freq: Freq,
    audio_track: AudioTrack,
}
impl AudioPlayer {
    pub fn new(audio_id: AudioId, repeat: bool, autoplay: bool) -> Self {
        Self {
            repeat,
            tracker: if autoplay { Some(0) } else { None },
            current_freq: Freq::None,
            audio_track: audio_id.get_track(),
        }
    }

    #[allow(dead_code)]
    pub fn play(&mut self) {
        self.tracker = Some(0);
    }

    #[allow(dead_code)]
    pub fn stop(&mut self) {
        self.tracker = None;
    }

    #[allow(dead_code)]
    pub fn is_playing(&self) -> bool {
        self.tracker.is_some()
    }

    pub fn tick(&mut self) {
        if let Some(tracker) = self.tracker {
            let note = Freq::from_byte(
                self.audio_track.source[tracker / self.audio_track.playback_rate as usize],
            );

            let hardware = crate::globals::get_hardware();
            if note != self.current_freq {
                self.current_freq = note;
                hardware.start_tone(&note);
            }

            if (tracker + 1) / (self.audio_track.playback_rate as usize)
                < self.audio_track.source.len()
            {
                self.tracker = Some(tracker + 1);
            } else if self.repeat {
                self.tracker = Some(0);
            }
        }
    }
}
