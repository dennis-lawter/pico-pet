use crate::hardware::audio::AudioFrequency as Freq;

use super::audio_library::AudioId;

pub struct AudioPlayer<'a> {
    repeat: bool,
    tracker: Option<usize>,
    current_freq: Freq,
    data: &'a [Freq],
    playback_rate: usize,
}
impl AudioPlayer<'_> {
    pub fn new(audio_id: AudioId, repeat: bool, autoplay: bool) -> Self {
        Self {
            repeat,
            tracker: if autoplay { Some(0) } else { None },
            current_freq: Freq::None,
            data: audio_id.get_sound_source(),
            playback_rate: audio_id.get_playback_rate(),
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
            let note = self.data[tracker / self.playback_rate];

            let hardware = crate::globals::get_hardware();
            if note != self.current_freq {
                self.current_freq = note;
                hardware.start_tone(&note);
            }

            if (tracker + 1) / self.playback_rate < self.data.len() {
                self.tracker = Some(tracker + 1);
            } else if self.repeat {
                self.tracker = Some(0);
            }
        }
    }
}
