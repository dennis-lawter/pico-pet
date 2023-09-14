#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum AudioFrequency {
    C4,
    Cs4,
    D4,
    Ds4,
    E4,
    F4,
    Fs4,
    G4,
    Gs4,
    A4,
    As4,
    B4,
    C5,
    Cs5,
    D5,
    Ds5,
    E5,
    F5,
    Fs5,
    G5,
    Gs5,
    A5,
    As5,
    B5,
    C6,
    Cs6,
    D6,
    Ds6,
    E6,
    F6,
    Fs6,
    G6,
    Gs6,
    A6,
    As6,
    B6,
    C7,

    None,
}
impl AudioFrequency {
    pub fn get_registers(&self) -> (u16, u8, u8) {
        match &self {
            Self::C4 => (65335, 7, 5),
            Self::Cs4 => (65003, 6, 15),
            Self::D4 => (65485, 6, 8),
            Self::Ds4 => (64930, 6, 3),
            Self::E4 => (65239, 5, 13),
            Self::F4 => (65077, 5, 8),
            Self::Fs4 => (65126, 5, 3),
            Self::G4 => (65409, 4, 14),
            Self::Gs4 => (65077, 4, 10),
            Self::A4 => (64934, 4, 6),
            Self::As4 => (65004, 4, 2),
            Self::B4 => (65314, 3, 14),
            Self::C5 => (64783, 3, 11),
            Self::Cs5 => (64422, 3, 8),
            Self::D5 => (65484, 3, 4),
            Self::Ds5 => (64281, 3, 2),
            Self::E5 => (64546, 2, 15),
            Self::F5 => (65077, 2, 12),
            Self::Fs5 => (64349, 2, 10),
            Self::G5 => (65410, 2, 7),
            Self::Gs5 => (65076, 2, 5),
            Self::A5 => (64934, 2, 3),
            Self::As5 => (65003, 2, 1),
            Self::B5 => (65313, 1, 15),
            Self::C6 => (63703, 1, 14),
            Self::Cs6 => (64422, 1, 12),
            Self::D6 => (65484, 1, 10),
            Self::Ds6 => (64281, 1, 9),
            Self::E6 => (63201, 1, 8),
            Self::F6 => (65077, 1, 6),
            Self::Fs6 => (64349, 1, 5),
            Self::G6 => (63775, 1, 4),
            Self::Gs6 => (63363, 1, 3),
            Self::A6 => (63130, 1, 2),
            Self::As6 => (63092, 1, 1),
            Self::B6 => (63273, 1, 0),
            Self::C7 => (59721, 1, 0),

            Self::None => (0, 0, 0),
        }
    }
}