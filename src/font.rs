use crate::render;

pub enum FontSize {
    Size5x8,
    Size8x13,
}
impl FontSize {
    const SMALL_FONT_DIMENSIONS: (usize, usize) = (5, 8);
    const BIG_FONT_DIMENSIONS: (usize, usize) = (8, 13);
    fn get_glyph_dimensions(&self) -> (usize, usize) {
        match &self {
            FontSize::Size5x8 => Self::SMALL_FONT_DIMENSIONS,
            FontSize::Size8x13 => Self::BIG_FONT_DIMENSIONS,
        }
    }
}

pub struct Font<'a> {
    size: FontSize,
    data: &'a [u8],
}

impl<'a> Font<'a> {
    pub fn new(size: FontSize, data: &'a [u8]) -> Self {
        Font { size, data }
    }

    pub fn draw_text(&self, x0: i32, y0: i32, color: u8, text: &str) {
        let mut x = x0;
        let mut y = y0;
        let (glyph_w, glyph_h) = self.size.get_glyph_dimensions();
        let mut fg_color = color;
        let mut bg_color = render::ALPHA_MASK;
        let mut char_iter = text.chars().into_iter();
        while let Some(c) = char_iter.next() {
            match c {
                '\\' => match char_iter.next() {
                    Some('c') => {
                        let r_char = char_iter.next().unwrap_or('0');
                        let g_char = char_iter.next().unwrap_or('0');
                        let b_char = char_iter.next().unwrap_or('0');
                        let r_value = r_char as u8 - 48;
                        let g_value = g_char as u8 - 48;
                        let b_value = b_char as u8 - 48;
                        fg_color = (r_value << 5) ^ (g_value << 2) ^ b_value;
                    }
                    Some('b') => {
                        let r_char = char_iter.next().unwrap_or('0');
                        let g_char = char_iter.next().unwrap_or('0');
                        let b_char = char_iter.next().unwrap_or('0');
                        let r_value = r_char as u8 - 48;
                        let g_value = g_char as u8 - 48;
                        let b_value = b_char as u8 - 48;
                        bg_color = (r_value << 5) ^ (g_value << 2) ^ b_value;
                    }
                    Some(other_char) => {
                        self.blit_glyph(x, y, other_char, bg_color, fg_color);
                        x += glyph_w as i32;
                    }
                    None => {}
                },
                '\n' => {
                    y += glyph_h as i32;
                    x = x0;
                }
                _ => {
                    self.blit_glyph(x, y, c, bg_color, fg_color);
                    x += glyph_w as i32;
                }
            }
        }
    }

    fn blit_glyph(&self, x: i32, y: i32, c: char, bg_color: u8, fg_color: u8) {
        match self.size {
            FontSize::Size5x8 => self.build_5x8_glyph(x, y, c, bg_color, fg_color),
            FontSize::Size8x13 => self.build_8x13_glyph(x, y, c, bg_color, fg_color),
        }
    }

    fn build_5x8_glyph(&self, x: i32, y: i32, c: char, bg_color: u8, fg_color: u8) {
        let mut glyph = [bg_color; 5 * 8];
        let (glyph_x0, glyph_y0) = Self::font_lookup(c);
        let (glyph_bit_x0, glyph_bit_y0) = (glyph_x0 * 5, glyph_y0 * 8);
        for glyph_y in 0..8 {
            for glyph_x in 0..5 {
                let glyph_bit_loc = (glyph_bit_y0 + glyph_y) * 80 + (glyph_bit_x0 + glyph_x);
                let glyph_byte_loc = glyph_bit_loc / 8;
                let glyph_byte_bit_offset = glyph_bit_loc % 8;
                let src = self.data[glyph_byte_loc];
                if (0b1000_0000 >> glyph_byte_bit_offset) & src != 0b0 {
                    glyph[(glyph_y * 5) + glyph_x] = fg_color.clone();
                }
            }
        }
        render::blit(x, y, 5, 8, &glyph)
    }

    fn font_lookup(c: char) -> (usize, usize) {
        let x = (c as usize - 32) % 16;
        let y = (c as usize - 32) / 16;
        (x, y)
    }

    fn build_8x13_glyph(&self, x: i32, y: i32, c: char, bg_color: u8, fg_color: u8) {
        let mut glyph = [bg_color; 8 * 13];
        let (glyph_x0, glyph_y0) = Self::font_lookup(c);
        for glyph_y in 0..13 {
            let data_row = self.data[(glyph_y + glyph_y0 * 13) * 16 + glyph_x0];
            if data_row & 0b1000_0000 == 0b1000_0000 {
                glyph[glyph_y * 8 + 0] = fg_color.clone();
            }
            if data_row & 0b0100_0000 == 0b0100_0000 {
                glyph[glyph_y * 8 + 1] = fg_color.clone();
            }
            if data_row & 0b0010_0000 == 0b0010_0000 {
                glyph[glyph_y * 8 + 2] = fg_color.clone();
            }
            if data_row & 0b0001_0000 == 0b0001_0000 {
                glyph[glyph_y * 8 + 3] = fg_color.clone();
            }
            if data_row & 0b0000_1000 == 0b0000_1000 {
                glyph[glyph_y * 8 + 4] = fg_color.clone();
            }
            if data_row & 0b0000_0100 == 0b0000_0100 {
                glyph[glyph_y * 8 + 5] = fg_color.clone();
            }
            if data_row & 0b0000_0010 == 0b0000_0010 {
                glyph[glyph_y * 8 + 6] = fg_color.clone();
            }
            if data_row & 0b0000_0001 == 0b0000_0001 {
                glyph[glyph_y * 8 + 7] = fg_color.clone();
            }
        }
        render::blit(x, y, 8, 13, &glyph)
    }
}

pub struct FontFactory;

impl FontFactory {
    pub fn new_small_font() -> Font<'static> {
        Font::new(FontSize::Size5x8, include_bytes!("../assets/font_5x8.data"))
    }

    pub fn new_big_regular_font() -> Font<'static> {
        Font::new(
            FontSize::Size8x13,
            include_bytes!("../assets/font_8x13.data"),
        )
    }

    pub fn new_big_bold_font() -> Font<'static> {
        Font::new(
            FontSize::Size8x13,
            include_bytes!("../assets/font_8x13_bold.data"),
        )
    }

    pub fn new_big_italic_font() -> Font<'static> {
        Font::new(
            FontSize::Size8x13,
            include_bytes!("../assets/font_8x13_italic.data"),
        )
    }
}
