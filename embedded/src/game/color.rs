#[derive(Clone, Copy)]
pub struct Rgb332(u8);
impl Rgb332 {
    #[allow(dead_code)]
    pub const INVISIBLE: Rgb332 = Rgb332(0b111_000_11);

    #[allow(dead_code)]
    pub const BLACK: Rgb332 = Rgb332(0b000_000_00);
    #[allow(dead_code)]
    pub const DARK_GREY: Rgb332 = Rgb332(0b010_010_01);
    #[allow(dead_code)]
    pub const LIGHT_GREY: Rgb332 = Rgb332(0b100_100_10);
    #[allow(dead_code)]
    pub const WHITE: Rgb332 = Rgb332(0b111_111_11);
    #[allow(dead_code)]
    pub const RED: Rgb332 = Rgb332(0b111_000_00);
    #[allow(dead_code)]
    pub const GREEN: Rgb332 = Rgb332(0b000_111_00);
    #[allow(dead_code)]
    pub const BLUE: Rgb332 = Rgb332(0b000_000_11);
    #[allow(dead_code)]
    pub const DARKEST_BLUE: Rgb332 = Rgb332(0b000_000_01);
    #[allow(dead_code)]
    pub const YELLOW: Rgb332 = Rgb332(0b111_111_00);

    #[allow(dead_code)]
    pub const FANCY_BORDER_CORNER_COLOR: Rgb332 = Rgb332(0b000_000_10);
    #[allow(dead_code)]
    pub const FANCY_BORDER_EDGE_COLOR: Rgb332 = Rgb332(0b000_000_11);
    #[allow(dead_code)]
    pub const FANCY_BORDER_EDGE_FILL_COLOR: Rgb332 = Rgb332(0b101_101_11);

    pub fn into_usize(self) -> usize {
        self.0 as usize
    }
    pub fn into_u8(self) -> u8 {
        self.0 as u8
    }
    pub fn from_u8(value: u8) -> Self {
        Rgb332(value)
    }
    pub fn from_components(r: u8, g: u8, b: u8) -> Self {
        Rgb332((r << 5) | (g << 2) | b)
    }
    // pub fn into_components(self) -> (u8, u8, u8) {
    //     ((self.0 >> 5) & 0b111, (self.0 >> 2) & 0b111, self.0 & 0b11)
    // }
}
