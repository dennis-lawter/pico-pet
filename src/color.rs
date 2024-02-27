#[derive(Clone, Copy)]
pub struct Rgb332(u8);
impl Rgb332 {
    pub fn into_usize(self) -> usize {
        self.0 as usize
    }
    pub fn into_u8(self) -> u8 {
        self.0 as u8
    }
    pub fn from_u8(value: u8) -> Self {
        Rgb332(value)
    }
}

pub const INVISIBLE: Rgb332 = Rgb332(0b111_000_11);

pub const BLACK: Rgb332 = Rgb332(0b000_000_00);
pub const WHITE: Rgb332 = Rgb332(0b111_111_11);
pub const RED: Rgb332 = Rgb332(0b111_000_00);
pub const GREEN: Rgb332 = Rgb332(0b000_111_00);
pub const BLUE: Rgb332 = Rgb332(0b000_000_11);
pub const YELLOW: Rgb332 = Rgb332(0b111_111_00);

pub const FANCY_BORDER_CORNER_COLOR: Rgb332 = Rgb332(0b000_000_10);
pub const FANCY_BORDER_EDGE_COLOR: Rgb332 = Rgb332(0b000_000_11);
pub const FANCY_BORDER_EDGE_FILL_COLOR: Rgb332 = Rgb332(0b101_101_11);
