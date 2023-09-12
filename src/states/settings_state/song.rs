use crate::system::Frequency;

// let mut song_str = "C4q D4q | F4q F4q F4q F4e F4e | F4e F4e F4q C4q D4q | F4q F4q F4q F4e F4e | F4e F4e F4q C4q D4q";
pub const SONG: [Frequency; 396] = [
    Frequency::C4, // take
    Frequency::C4,
    Frequency::C4,
    Frequency::C4,
    Frequency::C4,
    Frequency::C4,
    Frequency::C4,
    Frequency::None,
    Frequency::C5, // me
    Frequency::C5,
    Frequency::C5,
    Frequency::None,
    // ===========
    Frequency::A4, // out
    Frequency::A4,
    Frequency::A4,
    Frequency::None,
    Frequency::G4, // to
    Frequency::G4,
    Frequency::G4,
    Frequency::None,
    Frequency::F4, // the
    Frequency::F4,
    Frequency::F4,
    Frequency::None,
    // ===========
    Frequency::G4, // ball
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::None,
    // ===========
    Frequency::D4, // game
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::None,
    // ===========
    Frequency::C4, // take
    Frequency::C4,
    Frequency::C4,
    Frequency::C4,
    Frequency::C4,
    Frequency::C4,
    Frequency::C4,
    Frequency::None,
    Frequency::C5, // me
    Frequency::C5,
    Frequency::C5,
    Frequency::None,
    // ===========
    Frequency::A4, // out
    Frequency::A4,
    Frequency::A4,
    Frequency::None,
    Frequency::G4, // to
    Frequency::G4,
    Frequency::G4,
    Frequency::None,
    Frequency::F4, // the
    Frequency::F4,
    Frequency::F4,
    Frequency::None,
    // ===========
    Frequency::G4, // crowd
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    // ===========
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::None,
    // ===========
    Frequency::A4, // buy
    Frequency::A4,
    Frequency::A4,
    Frequency::None,
    Frequency::A4, // me
    Frequency::A4,
    Frequency::A4,
    Frequency::None,
    Frequency::A4, // some
    Frequency::A4,
    Frequency::A4,
    Frequency::None,
    // ===========
    Frequency::E4, // hot
    Frequency::E4,
    Frequency::E4,
    Frequency::None,
    Frequency::F4, // dogs
    Frequency::F4,
    Frequency::F4,
    Frequency::None,
    Frequency::G4, // and
    Frequency::G4,
    Frequency::G4,
    Frequency::None,
    // ===========
    Frequency::A4, // crack-
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::None,
    Frequency::F4, // er
    Frequency::F4,
    Frequency::F4,
    Frequency::None,
    // ===========
    Frequency::D4, // jacks
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::None,
    // ===========
    Frequency::A4, // I
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::None,
    Frequency::A4, // don't
    Frequency::A4,
    Frequency::A4,
    Frequency::None,
    // ===========
    Frequency::A4, // care
    Frequency::A4,
    Frequency::A4,
    Frequency::None,
    Frequency::B4, // if
    Frequency::B4,
    Frequency::B4,
    Frequency::None,
    Frequency::C5, // I
    Frequency::C5,
    Frequency::C5,
    Frequency::None,
    // ===========
    Frequency::D5, // ne-
    Frequency::D5,
    Frequency::D5,
    Frequency::None,
    Frequency::B4, // ver
    Frequency::B4,
    Frequency::B4,
    Frequency::None,
    Frequency::A4, // come
    Frequency::A4,
    Frequency::A4,
    Frequency::None,
    // ===========
    Frequency::G4, // back
    Frequency::G4,
    Frequency::G4,
    Frequency::None,
    Frequency::E4, // 'cause
    Frequency::E4,
    Frequency::E4,
    Frequency::None,
    Frequency::D4, // it's
    Frequency::D4,
    Frequency::D4,
    Frequency::None,
    // ===========
    Frequency::C4, // root
    Frequency::C4,
    Frequency::C4,
    Frequency::C4,
    Frequency::C4,
    Frequency::C4,
    Frequency::C4,
    Frequency::None,
    Frequency::C5, // root
    Frequency::C5,
    Frequency::C5,
    Frequency::None,
    // ===========
    Frequency::A4, // root
    Frequency::A4,
    Frequency::A4,
    Frequency::None,
    Frequency::G4, // for
    Frequency::G4,
    Frequency::G4,
    Frequency::None,
    Frequency::F4, // the
    Frequency::F4,
    Frequency::F4,
    Frequency::None,
    // ===========
    Frequency::G4, // home
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::G4,
    Frequency::None,
    // ===========
    Frequency::D4, // team
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::D4,
    Frequency::None,
    Frequency::D4, // if
    Frequency::D4,
    Frequency::D4,
    Frequency::None,
    // ===========
    Frequency::C4, // they
    Frequency::C4,
    Frequency::C4,
    Frequency::C4,
    Frequency::C4,
    Frequency::C4,
    Frequency::C4,
    Frequency::None,
    Frequency::D4, // don't
    Frequency::D4,
    Frequency::D4,
    Frequency::None,
    // ===========
    Frequency::E4, // win
    Frequency::E4,
    Frequency::E4,
    Frequency::None,
    Frequency::F4, // it's
    Frequency::F4,
    Frequency::F4,
    Frequency::None,
    Frequency::G4, // a
    Frequency::G4,
    Frequency::G4,
    Frequency::None,
    // ===========
    Frequency::A4, // shame
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::None,
    // ===========
    Frequency::None, // {rest}
    Frequency::None,
    Frequency::None,
    Frequency::None,
    Frequency::A4, // 'cause
    Frequency::A4,
    Frequency::A4,
    Frequency::None,
    Frequency::B4, // it's
    Frequency::B4,
    Frequency::B4,
    Frequency::None,
    // ===========
    Frequency::C5, // one
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::None,
    // ===========
    Frequency::C5, // two
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::None,
    // ===========
    Frequency::C5, // three
    Frequency::C5,
    Frequency::C5,
    Frequency::None,
    Frequency::B4, // strikes
    Frequency::B4,
    Frequency::B4,
    Frequency::None,
    Frequency::A4, // you're
    Frequency::A4,
    Frequency::A4,
    Frequency::None,
    // ===========
    Frequency::G4, // out
    Frequency::G4,
    Frequency::G4,
    Frequency::None,
    Frequency::Fs4, // at
    Frequency::Fs4,
    Frequency::Fs4,
    Frequency::None,
    Frequency::G4, // the
    Frequency::G4,
    Frequency::G4,
    Frequency::None,
    // ===========
    Frequency::A4, // old
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::A4,
    Frequency::None,
    // ===========
    Frequency::B4, // ball
    Frequency::B4,
    Frequency::B4,
    Frequency::B4,
    Frequency::B4,
    Frequency::B4,
    Frequency::B4,
    Frequency::B4,
    Frequency::B4,
    Frequency::B4,
    Frequency::B4,
    Frequency::None,
    // ===========
    Frequency::C5, // ga-
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    // ===========
    Frequency::C5, // -me
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::C5,
    Frequency::None,
    // ===========
    Frequency::None,
    Frequency::None,
    Frequency::None,
    Frequency::None,
    Frequency::None,
    Frequency::None,
    Frequency::None,
    Frequency::None,
    Frequency::None,
    Frequency::None,
    Frequency::None,
    Frequency::None,
];
