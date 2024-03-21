use super::audio_track::AudioTrack;
use super::audio_track::{self};

pub enum AudioId {
    BallGame,
    FerrisCry,
}
impl AudioId {
    pub fn get_track(&self) -> AudioTrack {
        match self {
            AudioId::BallGame => {
                audio_track::AudioTrack::new(include_bytes!("../../sound_raw/ballgame.beat"))
            }
            AudioId::FerrisCry => {
                audio_track::AudioTrack::new(include_bytes!("../../sound_raw/ferris_cry.beat"))
            }
        }
    }
}
