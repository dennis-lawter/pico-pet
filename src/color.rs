pub struct Rgb332(u8);

pub const INVISIBLE: Rgb332 = Rgb332(0b111_000_11);

pub const BLACK: Rgb332 = Rgb332(0b000_000_00);
pub const WHITE: Rgb332 = Rgb332(0b111_111_11);
pub const RED: Rgb332 = Rgb332(0b111_000_00);
pub const GREEN: Rgb332 = Rgb332(0b000_111_00);
pub const BLUE: Rgb332 = Rgb332(0b000_000_11);
pub const YELLOW: Rgb332 = Rgb332(0b111_111_00);
