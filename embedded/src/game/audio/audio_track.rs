pub struct AudioTrack {
    pub playback_rate: u8,
    pub source: &'static [u8],
}

impl AudioTrack {
    pub fn new(raw: &'static [u8]) -> Self {
        let playback_rate = raw[0];
        let source = &raw[1..];
        Self {
            playback_rate,
            source,
        }
    }
}
