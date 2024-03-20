use super::audio_track::AudioTrack;
use super::audio_track::{self};
// use super::sources;
// use crate::hardware::audio::AudioFrequency as Freq;

pub enum AudioId {
    // BpmTest,
    BallGame,
}
impl AudioId {
    // pub fn get_sound_source(&self) -> &'static [Freq] {
    //     match self {
    //         // AudioId::BpmTest => &sources::bpm_test::BPM_TEST,
    //         AudioId::BallGame => &sources::ball_game::BALL_GAME,
    //     }
    // }
    // pub fn get_playback_rate(&self) -> usize {
    //     match self {
    //         // AudioId::BpmTest => sources::bpm_test::BPM_TEST_PLAYBACK_RATE,
    //         AudioId::BallGame => sources::ball_game::BALL_GAME_PLAYBACK_RATE,
    //     }
    // }
    pub fn get_track(&self) -> AudioTrack {
        match self {
            AudioId::BallGame => {
                audio_track::AudioTrack::new(include_bytes!("../../sound_raw/ballgame.beat"))
            }
        }
    }
}
