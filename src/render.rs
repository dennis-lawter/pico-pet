use st7735_lcd::ST7735;
use waveshare_rp2040_lcd_0_96::{hal, pac};

static mut BUFFER: [u16; 128 * 128] = [0b00000_111111_00000; 128 * 128];

const RGB_332_TO_RGB_565: [u16; 256] = [
    0x0000, 0x000a, 0x0015, 0x001f, 0x0120, 0x012a, 0x0135, 0x013f, 0x0240, 0x024a, 0x0255, 0x025f,
    0x0360, 0x036a, 0x0375, 0x037f, 0x0480, 0x048a, 0x0495, 0x049f, 0x05a0, 0x05aa, 0x05b5, 0x05bf,
    0x06c0, 0x06ca, 0x06d5, 0x06df, 0x07e0, 0x07ea, 0x07f5, 0x07ff, 0x2000, 0x200a, 0x2015, 0x201f,
    0x2120, 0x212a, 0x2135, 0x213f, 0x2240, 0x224a, 0x2255, 0x225f, 0x2360, 0x236a, 0x2375, 0x237f,
    0x2480, 0x248a, 0x2495, 0x249f, 0x25a0, 0x25aa, 0x25b5, 0x25bf, 0x26c0, 0x26ca, 0x26d5, 0x26df,
    0x27e0, 0x27ea, 0x27f5, 0x27ff, 0x4800, 0x480a, 0x4815, 0x481f, 0x4920, 0x492a, 0x4935, 0x493f,
    0x4a40, 0x4a4a, 0x4a55, 0x4a5f, 0x4b60, 0x4b6a, 0x4b75, 0x4b7f, 0x4c80, 0x4c8a, 0x4c95, 0x4c9f,
    0x4da0, 0x4daa, 0x4db5, 0x4dbf, 0x4ec0, 0x4eca, 0x4ed5, 0x4edf, 0x4fe0, 0x4fea, 0x4ff5, 0x4fff,
    0x6800, 0x680a, 0x6815, 0x681f, 0x6920, 0x692a, 0x6935, 0x693f, 0x6a40, 0x6a4a, 0x6a55, 0x6a5f,
    0x6b60, 0x6b6a, 0x6b75, 0x6b7f, 0x6c80, 0x6c8a, 0x6c95, 0x6c9f, 0x6da0, 0x6daa, 0x6db5, 0x6dbf,
    0x6ec0, 0x6eca, 0x6ed5, 0x6edf, 0x6fe0, 0x6fea, 0x6ff5, 0x6fff, 0x9000, 0x900a, 0x9015, 0x901f,
    0x9120, 0x912a, 0x9135, 0x913f, 0x9240, 0x924a, 0x9255, 0x925f, 0x9360, 0x936a, 0x9375, 0x937f,
    0x9480, 0x948a, 0x9495, 0x949f, 0x95a0, 0x95aa, 0x95b5, 0x95bf, 0x96c0, 0x96ca, 0x96d5, 0x96df,
    0x97e0, 0x97ea, 0x97f5, 0x97ff, 0xb000, 0xb00a, 0xb015, 0xb01f, 0xb120, 0xb12a, 0xb135, 0xb13f,
    0xb240, 0xb24a, 0xb255, 0xb25f, 0xb360, 0xb36a, 0xb375, 0xb37f, 0xb480, 0xb48a, 0xb495, 0xb49f,
    0xb5a0, 0xb5aa, 0xb5b5, 0xb5bf, 0xb6c0, 0xb6ca, 0xb6d5, 0xb6df, 0xb7e0, 0xb7ea, 0xb7f5, 0xb7ff,
    0xd800, 0xd80a, 0xd815, 0xd81f, 0xd920, 0xd92a, 0xd935, 0xd93f, 0xda40, 0xda4a, 0xda55, 0xda5f,
    0xdb60, 0xdb6a, 0xdb75, 0xdb7f, 0xdc80, 0xdc8a, 0xdc95, 0xdc9f, 0xdda0, 0xddaa, 0xddb5, 0xddbf,
    0xdec0, 0xdeca, 0xded5, 0xdedf, 0xdfe0, 0xdfea, 0xdff5, 0xdfff, 0xf800, 0xf80a, 0xf815, 0xf81f,
    0xf920, 0xf92a, 0xf935, 0xf93f, 0xfa40, 0xfa4a, 0xfa55, 0xfa5f, 0xfb60, 0xfb6a, 0xfb75, 0xfb7f,
    0xfc80, 0xfc8a, 0xfc95, 0xfc9f, 0xfda0, 0xfdaa, 0xfdb5, 0xfdbf, 0xfec0, 0xfeca, 0xfed5, 0xfedf,
    0xffe0, 0xffea, 0xfff5, 0xffff,
];

const ALPHA_MASK: u8 = 0b11100011;

const BIG_FONT: &[u8; 1248] = include_bytes!("../assets/font_8x13.data");
const BIG_FONT_BOLD: &[u8; 1248] = include_bytes!("../assets/font_8x13_bold.data");
const BIG_FONT_ITALIC: &[u8; 1248] = include_bytes!("../assets/font_8x13_italic.data");

static mut FONT: [[u8; 5]; 96] = [[0; 5]; 96];

pub enum FontStyle {
    Normal,
    Big,
    BigBold,
    BigItalic,
}

pub fn init_font() {
    let raw = include_bytes!("../assets/font_5x8.data");
    for char_row_num in 0..6 {
        for char_pix_num in 0..8 {
            let raw_y = (char_row_num * 8 + char_pix_num) * 10;
            let row: [u8; 10] = [
                raw[raw_y + 0],
                raw[raw_y + 1],
                raw[raw_y + 2],
                raw[raw_y + 3],
                raw[raw_y + 4],
                raw[raw_y + 5],
                raw[raw_y + 6],
                raw[raw_y + 7],
                raw[raw_y + 8],
                raw[raw_y + 9],
            ];

            // each row of the source image contains 80 pixels
            // store every 5 pixels into a "nyckle" to be assembled later

            let first_nyckle = (row[0] & 0b1111_1000) >> 3;
            let second_nyckle = ((row[0] & 0b0000_0111) << 2) ^ ((row[1] & 0b1100_0000) >> 6);
            let third_nyckle = (row[1] & 0b0011_1110) >> 1;
            let fourth_nyckle = ((row[1] & 0b0000_0001) << 4) ^ ((row[2] & 0b1111_0000) >> 4);
            let fifth_nyckle = ((row[2] & 0b0000_1111) << 1) ^ ((row[3] & 0b1000_0000) >> 7);
            let sixth_nyckle = (row[3] & 0b0111_1100) >> 2;
            let seventh_nyckle = ((row[3] & 0b0000_0011) << 3) ^ ((row[4] & 0b1110_0000) >> 5);
            let eigth_nyckle = row[4] & 0b001_1111;
            let ninth_nyckle = (row[5] & 0b1111_1000) >> 3;
            let tenth_nyckle = ((row[5] & 0b0000_0111) << 2) ^ ((row[6] & 0b1100_0000) >> 6);
            let eleventh_nyckle = (row[6] & 0b0011_1110) >> 1;
            let twelfth_nyckle = ((row[6] & 0b0000_0001) << 4) ^ ((row[7] & 0b1111_0000) >> 4);
            let thirteenth_nyckle = ((row[7] & 0b0000_1111) << 1) ^ ((row[8] & 0b1000_0000) >> 7);
            let fourteenth_nyckle = (row[8] & 0b0111_1100) >> 2;
            let fifteenth_nyckle = ((row[8] & 0b0000_0011) << 3) ^ ((row[9] & 0b1110_0000) >> 5);
            let sixteenth_nyckle = row[9] & 0b001_1111;

            let nyckles: [u8; 16] = [
                first_nyckle,
                second_nyckle,
                third_nyckle,
                fourth_nyckle,
                fifth_nyckle,
                sixth_nyckle,
                seventh_nyckle,
                eigth_nyckle,
                ninth_nyckle,
                tenth_nyckle,
                eleventh_nyckle,
                twelfth_nyckle,
                thirteenth_nyckle,
                fourteenth_nyckle,
                fifteenth_nyckle,
                sixteenth_nyckle,
            ];

            for i in 0..16 {
                let nyckle = nyckles[i];
                let char_to_mutate = char_row_num * 16 + i;
                unsafe {
                    // if nyckle & 0b0001_0000 == 0b0001_0000 {
                    //     FONT[char_to_mutate][0] ^= 0b1000_0000 >> char_pix_num;
                    // }
                    // if nyckle & 0b0000_1000 == 0b0000_1000 {
                    //     FONT[char_to_mutate][1] ^= 0b1000_0000 >> char_pix_num;
                    // }
                    // if nyckle & 0b0000_0100 == 0b0000_0100 {
                    //     FONT[char_to_mutate][2] ^= 0b1000_0000 >> char_pix_num;
                    // }
                    // if nyckle & 0b0000_0010 == 0b0000_0010 {
                    //     FONT[char_to_mutate][3] ^= 0b1000_0000 >> char_pix_num;
                    // }
                    // if nyckle & 0b0000_0001 == 0b0000_0001 {
                    //     FONT[char_to_mutate][4] ^= 0b1000_0000 >> char_pix_num;
                    // }
                    FONT[char_to_mutate][0] ^= ((nyckle & 0b0001_0000) << 3) >> char_pix_num;
                    FONT[char_to_mutate][1] ^= ((nyckle & 0b0000_1000) << 4) >> char_pix_num;
                    FONT[char_to_mutate][2] ^= ((nyckle & 0b0000_0100) << 5) >> char_pix_num;
                    FONT[char_to_mutate][3] ^= ((nyckle & 0b0000_0010) << 6) >> char_pix_num;
                    FONT[char_to_mutate][4] ^= ((nyckle & 0b0000_0001) << 7) >> char_pix_num;
                }
            }
        }
    }
}

pub fn draw(
    display: &mut ST7735<
        hal::Spi<hal::spi::Enabled, pac::SPI1, 8>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio8, hal::gpio::Output<hal::gpio::PushPull>>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio12, hal::gpio::Output<hal::gpio::PushPull>>,
    >,
) {
    unsafe {
        display.write_pixels_buffered(BUFFER).unwrap();
    }
}

pub fn blit(x0: i32, y0: i32, w: usize, h: usize, sprite_data: &[u8]) {
    for y in 0..h {
        if y as i32 + y0 >= 128 {
            return;
        } else if y as i32 + y0 < 0 {
            continue;
        }
        for x in 0..w {
            if x as i32 + x0 >= 128 {
                break;
            } else if x as i32 + x0 < 0 {
                continue;
            }
            let src_coord = y * w + x;
            let pixel = sprite_data[src_coord];
            if pixel == ALPHA_MASK {
                continue;
            }
            let pixel_index: usize = pixel.into();
            let dst_coord: i32 = (y as i32 + y0) * 128 + (x as i32 + x0);
            unsafe {
                let dst_coord_usize: usize = dst_coord as usize;
                BUFFER[dst_coord_usize] = RGB_332_TO_RGB_565[pixel_index];
            }
        }
    }
}

pub fn flood(color: u8) {
    let color_index = color as usize;
    let mapped_color = RGB_332_TO_RGB_565[color_index];
    unsafe {
        BUFFER = [mapped_color; 128 * 128];
    }
}

pub fn blit_str(x0: i32, y0: i32, color: u8, text: &str, style: FontStyle) {
    match style {
        FontStyle::Normal => blit_normal_str(x0, y0, color, text),
        FontStyle::Big => blit_big_str(x0, y0, color, text, BIG_FONT),
        FontStyle::BigBold => blit_big_str(x0, y0, color, text, BIG_FONT_BOLD),
        FontStyle::BigItalic => blit_big_str(x0, y0, color, text, BIG_FONT_ITALIC),
    }
}

fn blit_normal_str(x0: i32, y0: i32, color: u8, text: &str) {
    let mut x = x0;
    let mut y = y0;
    for c in text.chars() {
        match c {
            '\n' => {
                y += 8;
                x = x0;
            }
            _ => {
                blit_normal_char(x, y, color, c);
                x += 5;
            }
        }
    }
}

fn blit_big_str(x0: i32, y0: i32, color: u8, text: &str, font_src: &[u8]) {
    let mut x = x0;
    let mut y = y0;
    for c in text.chars() {
        match c {
            '\n' => {
                y += 13;
                x = x0;
            }
            _ => {
                blit_big_char(x, y, color, c, font_src);
                x += 8;
            }
        }
    }
}

fn char_to_offset(c: char) -> (usize, usize) {
    let x = (c as usize - 32) % 16;
    let y = (c as usize - 32) / 16;
    (x, y * 13)
}

fn blit_big_char(x0: i32, y0: i32, color: u8, c: char, font_src: &[u8]) {
    let mut glyph = [0b111_000_11u8; 8 * 13];
    let (glyph_x, glyph_y) = char_to_offset(c);
    for y in 0..13 {
        let data_row = font_src[(y + glyph_y) * 16 + glyph_x];
        if data_row & 0b1000_0000 == 0b1000_0000 {
            glyph[y * 8 + 0] = color.clone();
        }
        if data_row & 0b0100_0000 == 0b0100_0000 {
            glyph[y * 8 + 1] = color.clone();
        }
        if data_row & 0b0010_0000 == 0b0010_0000 {
            glyph[y * 8 + 2] = color.clone();
        }
        if data_row & 0b0001_0000 == 0b0001_0000 {
            glyph[y * 8 + 3] = color.clone();
        }
        if data_row & 0b0000_1000 == 0b0000_1000 {
            glyph[y * 8 + 4] = color.clone();
        }
        if data_row & 0b0000_0100 == 0b0000_0100 {
            glyph[y * 8 + 5] = color.clone();
        }
        if data_row & 0b0000_0010 == 0b0000_0010 {
            glyph[y * 8 + 6] = color.clone();
        }
        if data_row & 0b0000_0001 == 0b0000_0001 {
            glyph[y * 8 + 7] = color.clone();
        }
    }
    blit(x0, y0, 8, 13, &glyph);
}

pub fn blit_normal_char(x0: i32, y0: i32, color: u8, c: char) {
    let mut glyph = [0b111_000_11u8; 5 * 8];
    unsafe {
        let glyph_raw = FONT[c as usize - 32];
        for x in 0..5 {
            let vert_slice = glyph_raw[x];
            if vert_slice & 0b1000_0000 == 0b1000_0000 {
                glyph[(0 * 5) + x] = color.clone();
            }
            if vert_slice & 0b0100_0000 == 0b0100_0000 {
                glyph[(1 * 5) + x] = color.clone();
            }
            if vert_slice & 0b0010_0000 == 0b0010_0000 {
                glyph[(2 * 5) + x] = color.clone();
            }
            if vert_slice & 0b0001_0000 == 0b0001_0000 {
                glyph[(3 * 5) + x] = color.clone();
            }
            if vert_slice & 0b0000_1000 == 0b0000_1000 {
                glyph[(4 * 5) + x] = color.clone();
            }
            if vert_slice & 0b0000_0100 == 0b0000_0100 {
                glyph[(5 * 5) + x] = color.clone();
            }
            if vert_slice & 0b0000_0010 == 0b0000_0010 {
                glyph[(6 * 5) + x] = color.clone();
            }
            if vert_slice & 0b0000_0001 == 0b0000_0001 {
                glyph[(7 * 5) + x] = color.clone();
            }
        }
    }
    blit(x0, y0, 5, 8, &glyph);
}

pub fn fill_rect(x0: i32, y0: i32, w: usize, h: usize, color: u8) {
    let ext_color = RGB_332_TO_RGB_565[color as usize];
    for y in y0..y0 + (h as i32) {
        if y >= 128 {
            return;
        } else if y < 0 {
            continue;
        }
        for x in x0..x0 + (w as i32) {
            if x >= 128 {
                break;
            } else if x < 0 {
                continue;
            }
            let dst_coord: i32 = (y as i32) * 128 + (x as i32);
            unsafe {
                let dst_coord_usize: usize = dst_coord as usize;
                BUFFER[dst_coord_usize] = ext_color;
            }
        }
    }
}

pub fn h_solid_line(x0: i32, y0: i32, w: usize, color: u8) {
    let ext_color = RGB_332_TO_RGB_565[color as usize];
    for x in x0..x0 + w as i32 {
        if (y0 * 128 + x) > 128 * 128 {
            return;
        }
        unsafe {
            BUFFER[(y0 * 128 + x) as usize] = ext_color;
        }
    }
}

pub fn v_solid_line(x0: i32, y0: i32, h: usize, color: u8) {
    let ext_color = RGB_332_TO_RGB_565[color as usize];
    for y in y0..y0 + h as i32 {
        if (y * 128 + x0) > 128 * 128 {
            return;
        }
        unsafe {
            BUFFER[(y * 128 + x0) as usize] = ext_color;
        }
    }
}

pub fn h_dithered_line(x0: i32, y0: i32, w: usize, color: u8) {
    let ext_color = RGB_332_TO_RGB_565[color as usize];
    let (x0, w) = if (y0 + x0) % 2 == 0 {
        (x0 + 1, w - 1)
    } else {
        (x0, w)
    };
    for x in (x0..x0 + w as i32).step_by(2) {
        if (y0 * 128 + x) > 128 * 128 {
            return;
        }
        unsafe {
            BUFFER[(y0 * 128 + x) as usize] = ext_color;
        }
    }
}

pub fn v_dithered_line(x0: i32, y0: i32, h: usize, color: u8) {
    let ext_color = RGB_332_TO_RGB_565[color as usize];
    let (y0, h) = if (y0 + x0) % 2 == 0 {
        (y0 + 1, h - 1)
    } else {
        (y0, h)
    };
    for y in (y0..y0 + h as i32).step_by(2) {
        if (y * 128 + x0) > 128 * 128 {
            return;
        }
        unsafe {
            BUFFER[(y * 128 + x0) as usize] = ext_color;
        }
    }
}

pub fn bottom_dialog_box(text: &str, style: FontStyle) {
    let (x, y) = match style {
        FontStyle::Normal => (5, 128 - 19 + 4),
        _ => (4, 128 - 19 + 4 - 2),
    };
    fill_rect(0, 128 - (4 + 4 + 13), 128, 4 + 4 + 13, 0b111_111_11);
    fancy_border(0, 128 + 1 - (4 + 4 + 13), 128, 4 + 4 + 13 - 2);
    blit_str(x, y, 0b000_000_11, text, style)
}

pub fn fs_dialog_box(text: &str) {
    fill_rect(0, 0, 128, 128, 0b111_111_11);
    fancy_border(0, 0, 128, 128);
    blit_str(5, 5, 0b000_000_11, text, FontStyle::Normal)
}

fn fancy_border(x0: i32, y0: i32, w: usize, h: usize) {
    let hard_color = 0b000_000_10;
    let color = 0b000_000_11;
    let soft_color = 0b010_010_11;

    // pipes
    h_solid_line(x0, y0, w, color);
    h_solid_line(x0, y0 + 3, w, color);
    h_solid_line(x0, y0 + h as i32 - 4, w, color);
    h_solid_line(x0, y0 + h as i32 - 1, w, color);

    v_solid_line(x0, y0, h, color);
    v_solid_line(x0 + 3, y0, h, color);
    v_solid_line(x0 + w as i32 - 4, y0, h, color);
    v_solid_line(x0 + w as i32 - 1, y0, h, color);

    // ditherfill
    h_dithered_line(x0, y0 + 1, w, soft_color);
    h_dithered_line(x0, y0 + 2, w, soft_color);
    h_dithered_line(x0, y0 + h as i32 - 3, w, soft_color);
    h_dithered_line(x0, y0 + h as i32 - 2, w, soft_color);

    v_dithered_line(x0 + 1, y0, h, soft_color);
    v_dithered_line(x0 + 2, y0, h, soft_color);
    v_dithered_line(x0 + w as i32 - 3, y0, h, soft_color);
    v_dithered_line(x0 + w as i32 - 2, y0, h, soft_color);

    // corners
    // top left
    h_solid_line(x0, y0, 7, hard_color);
    h_solid_line(x0, y0 + 1, 6, hard_color);
    h_solid_line(x0, y0 + 2, 5, hard_color);
    h_solid_line(x0, y0 + 3, 4, hard_color);
    h_solid_line(x0, y0 + 4, 3, hard_color);
    h_solid_line(x0, y0 + 5, 2, hard_color);
    h_solid_line(x0, y0 + 6, 1, hard_color);

    // bot left
    h_solid_line(x0, y0 + h as i32 - 7, 1, hard_color);
    h_solid_line(x0, y0 + h as i32 - 6, 2, hard_color);
    h_solid_line(x0, y0 + h as i32 - 5, 3, hard_color);
    h_solid_line(x0, y0 + h as i32 - 4, 4, hard_color);
    h_solid_line(x0, y0 + h as i32 - 3, 5, hard_color);
    h_solid_line(x0, y0 + h as i32 - 2, 6, hard_color);
    h_solid_line(x0, y0 + h as i32 - 1, 7, hard_color);

    // top right
    h_solid_line(x0 + w as i32 - 7, y0, 7, hard_color);
    h_solid_line(x0 + w as i32 - 6, y0 + 1, 6, hard_color);
    h_solid_line(x0 + w as i32 - 5, y0 + 2, 5, hard_color);
    h_solid_line(x0 + w as i32 - 4, y0 + 3, 4, hard_color);
    h_solid_line(x0 + w as i32 - 3, y0 + 4, 3, hard_color);
    h_solid_line(x0 + w as i32 - 2, y0 + 5, 2, hard_color);
    h_solid_line(x0 + w as i32 - 1, y0 + 6, 1, hard_color);

    // bot right
    h_solid_line(x0 + w as i32 - 1, y0 + h as i32 - 7, 1, hard_color);
    h_solid_line(x0 + w as i32 - 2, y0 + h as i32 - 6, 2, hard_color);
    h_solid_line(x0 + w as i32 - 3, y0 + h as i32 - 5, 3, hard_color);
    h_solid_line(x0 + w as i32 - 4, y0 + h as i32 - 4, 4, hard_color);
    h_solid_line(x0 + w as i32 - 5, y0 + h as i32 - 3, 5, hard_color);
    h_solid_line(x0 + w as i32 - 6, y0 + h as i32 - 2, 6, hard_color);
    h_solid_line(x0 + w as i32 - 7, y0 + h as i32 - 1, 7, hard_color);
}
