use super::audio_track::AudioTrack;
use super::audio_track::{self};

pub enum AudioId {
    BallGame,

    FerrisCry,

    PomodoroFinished,
    BreakFinished,
    Countdown321,
    CountdownGo,

    ButtonBeep,
}
impl AudioId {
    pub fn get_track(&self) -> AudioTrack {
        match self {
            AudioId::BallGame => audio_track::AudioTrack::new(include_bytes!(
                "../../assets/sound_raw/ball_game.beat"
            )),
            AudioId::FerrisCry => audio_track::AudioTrack::new(include_bytes!(
                "../../assets/sound_raw/ferris_cry.beat"
            )),

            AudioId::PomodoroFinished => audio_track::AudioTrack::new(include_bytes!(
                "../../assets/sound_raw/pomodoro_finish.beat"
            )),
            AudioId::BreakFinished => audio_track::AudioTrack::new(include_bytes!(
                "../../assets/sound_raw/break_finish.beat"
            )),
            AudioId::Countdown321 => audio_track::AudioTrack::new(include_bytes!(
                "../../assets/sound_raw/countdown_321.beat"
            )),
            AudioId::CountdownGo => audio_track::AudioTrack::new(include_bytes!(
                "../../assets/sound_raw/countdown_go.beat"
            )),

            AudioId::ButtonBeep => audio_track::AudioTrack::new(include_bytes!(
                "../../assets/sound_raw/button_beep.beat"
            )),
        }
    }
}
