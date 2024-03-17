use crate::hardware::audio::AudioFrequency as Freq;

/// these notes play at 157 bpm
/// thus, the array index advances 1256 times per minute
/// adjust array density and playback rate as necessary
pub const BPM_TEST_PLAYBACK_RATE: usize = 1;
pub const BPM_TEST: [Freq; 8] = [
    Freq::C4,
    Freq::None,
    Freq::None,
    Freq::None,
    Freq::None,
    Freq::None,
    Freq::None,
    Freq::None,
];
