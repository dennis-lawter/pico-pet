use core::cmp::max;
use core::cmp::min;

use crate::color::Rgb332;
use crate::display::rgb_converter::RGB_332_TO_565;
use crate::hardware::hardware::Lcd;
use crate::hardware::hardware::LCD_HEIGHT;
use crate::hardware::hardware::LCD_WIDTH;

static mut BUFFER: [u16; LCD_WIDTH * LCD_HEIGHT] = [0b00000_111111_00000; 128 * 128];

pub fn draw(display: &mut Lcd) {
    unsafe {
        display.write_pixels_buffered(BUFFER).unwrap();
    }
}

pub fn blit(x0: i32, y0: i32, w: usize, h: usize, sprite_data: &[u8]) {
    for y in 0..h {
        if y as i32 + y0 >= LCD_HEIGHT as i32 {
            return;
        } else if y as i32 + y0 < 0 {
            continue;
        }
        for x in 0..w {
            if x as i32 + x0 >= LCD_WIDTH as i32 {
                break;
            } else if x as i32 + x0 < 0 {
                continue;
            }
            let src_coord = y * w + x;
            let pixel = sprite_data[src_coord];
            if pixel == Rgb332::INVISIBLE.into_u8() {
                continue;
            }
            let pixel_index: usize = pixel.into();
            let dst_coord: i32 = (y as i32 + y0) * LCD_WIDTH as i32 + (x as i32 + x0);
            unsafe {
                let dst_coord_usize: usize = dst_coord as usize;
                BUFFER[dst_coord_usize] = RGB_332_TO_565[pixel_index];
            }
        }
    }
}

pub fn blit_from_offset(x0: i32, y0: i32, offset: usize, w: usize, h: usize, sprite_data: &[u8]) {
    for y in 0..h {
        if y as i32 + y0 >= LCD_HEIGHT as i32 {
            return;
        } else if y as i32 + y0 < 0 {
            continue;
        }
        for x in 0..w {
            if x as i32 + x0 >= LCD_WIDTH as i32 {
                break;
            } else if x as i32 + x0 < 0 {
                continue;
            }
            let src_coord = y * w + x;
            let pixel = sprite_data[src_coord + offset];
            if pixel == Rgb332::INVISIBLE.into_u8() {
                continue;
            }
            let pixel_index: usize = pixel.into();
            let dst_coord: i32 = (y as i32 + y0) * LCD_WIDTH as i32 + (x as i32 + x0);
            unsafe {
                let dst_coord_usize: usize = dst_coord as usize;
                BUFFER[dst_coord_usize] = RGB_332_TO_565[pixel_index];
            }
        }
    }
}

pub fn flood(color: Rgb332) {
    let color_index = color.into_usize();
    let mapped_color = RGB_332_TO_565[color_index];
    unsafe {
        BUFFER = [mapped_color; LCD_WIDTH * LCD_HEIGHT];
    }
}

pub fn fill_rect(x0: i32, y0: i32, w: usize, h: usize, color: Rgb332) {
    let ext_color = RGB_332_TO_565[color.into_usize()];

    let effective_width = min(w, (LCD_WIDTH as i32 - x0) as usize);
    let effective_height = min(h, (LCD_HEIGHT as i32 - y0) as usize);

    let start_x = max(x0, 0);
    let end_x = start_x + effective_width as i32;

    let start_y = max(y0, 0);
    let end_y = start_y + effective_height as i32;

    for y in start_y..end_y {
        let mut idx = (y * LCD_WIDTH as i32 + start_x) as usize;
        for _ in start_x..end_x {
            unsafe {
                BUFFER[idx] = ext_color;
            }
            idx += 1;
        }
    }
}

pub fn h_solid_line(x0: i32, y0: i32, w: usize, color: Rgb332) {
    if y0 < 0 || y0 >= LCD_HEIGHT as i32 {
        return;
    }
    fill_rect(x0, y0, w, 1, color)
}

pub fn v_solid_line(x0: i32, y0: i32, h: usize, color: Rgb332) {
    if x0 < 0 || x0 >= LCD_WIDTH as i32 {
        return;
    }
    fill_rect(x0, y0, 1, h, color)
}

pub fn solid_line_rect(x0: i32, y0: i32, w: usize, h: usize, color: Rgb332) {
    h_solid_line(x0, y0, w, color);
    h_solid_line(x0, y0 + h as i32 - 1, w, color);

    v_solid_line(x0, y0, h, color);
    v_solid_line(x0 + w as i32 - 1, y0, h, color);
}

pub fn h_dithered_line(x0: i32, y0: i32, w: usize, color: Rgb332, inverted: bool) {
    if y0 < 0 || y0 >= LCD_HEIGHT as i32 {
        return;
    }
    let inverted_int = inverted as i32;
    let ext_color = RGB_332_TO_565[color.into_usize()];
    let (mut x0, w) = if (y0 + x0) % 2 == inverted_int {
        (x0 + 1, w.saturating_sub(1))
    } else {
        (x0, w)
    };

    // Adjust x0 and w to fit within screen bounds
    x0 = x0.max(0);
    let end_x = (x0 + w as i32).min(LCD_WIDTH as i32);

    for x in (x0..end_x).step_by(2) {
        unsafe {
            BUFFER[(y0 * LCD_WIDTH as i32 + x) as usize] = ext_color;
        }
    }
}

pub fn v_dithered_line(x0: i32, y0: i32, h: usize, color: Rgb332, inverted: bool) {
    if x0 < 0 || x0 >= LCD_WIDTH as i32 {
        return;
    }
    let inverted_int = inverted as i32;
    let ext_color = RGB_332_TO_565[color.into_usize()];
    let (mut y0, h) = if (y0 + x0) % 2 == inverted_int {
        (y0 + 1, h.saturating_sub(1))
    } else {
        (y0, h)
    };

    // Adjust y0 and h to fit within screen bounds
    y0 = y0.max(0);
    let end_y = (y0 + h as i32).min(LCD_HEIGHT as i32);

    for y in (y0..end_y).step_by(2) {
        unsafe {
            BUFFER[(y * LCD_WIDTH as i32 + x0) as usize] = ext_color;
        }
    }
}

pub fn dithered_line_rect(x0: i32, y0: i32, w: usize, h: usize, color: Rgb332, inverted: bool) {
    h_dithered_line(x0, y0, w, color, inverted);
    h_dithered_line(x0, y0 + h as i32 - 1, w, color, inverted);

    v_dithered_line(x0, y0, h, color, inverted);
    v_dithered_line(x0 + w as i32 - 1, y0, h, color, inverted);
}

const FANCY_BORDER_THICKNESS: usize = 4;
const FANCY_BORDER_CORNER_SIZE: usize = 7;

enum FancyBorderCornerOrientation {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn fancy_border_corner(x: i32, y: i32, orientation: FancyBorderCornerOrientation) {
    for i in 0..FANCY_BORDER_CORNER_SIZE {
        match orientation {
            FancyBorderCornerOrientation::TopLeft => {
                h_solid_line(
                    x,
                    y + i as i32,
                    FANCY_BORDER_CORNER_SIZE - i,
                    Rgb332::FANCY_BORDER_CORNER_COLOR,
                );
            }
            FancyBorderCornerOrientation::TopRight => {
                h_solid_line(
                    x - (FANCY_BORDER_CORNER_SIZE - i) as i32,
                    y + i as i32,
                    FANCY_BORDER_CORNER_SIZE - i,
                    Rgb332::FANCY_BORDER_CORNER_COLOR,
                );
            }
            FancyBorderCornerOrientation::BottomLeft => {
                h_solid_line(
                    x,
                    y - (FANCY_BORDER_CORNER_SIZE - i) as i32,
                    i,
                    Rgb332::FANCY_BORDER_CORNER_COLOR,
                );
            }
            FancyBorderCornerOrientation::BottomRight => {
                h_solid_line(
                    x - i as i32,
                    y - (FANCY_BORDER_CORNER_SIZE - i) as i32,
                    i,
                    Rgb332::FANCY_BORDER_CORNER_COLOR,
                );
            }
        }
    }
}

enum FancyBorderEdgeOrientation {
    Horizontal,
    Vertical,
}

fn fancy_border_edge(x0: i32, y0: i32, length: usize, orientation: FancyBorderEdgeOrientation) {
    match orientation {
        FancyBorderEdgeOrientation::Horizontal => {
            let y1 = y0 + FANCY_BORDER_THICKNESS as i32 - 1;
            h_solid_line(x0, y0, length, Rgb332::FANCY_BORDER_EDGE_COLOR);
            h_solid_line(x0, y1, length, Rgb332::FANCY_BORDER_EDGE_COLOR);
            for i in 1..(FANCY_BORDER_THICKNESS - 1) {
                h_dithered_line(
                    x0,
                    y0 + i as i32,
                    length,
                    Rgb332::FANCY_BORDER_EDGE_FILL_COLOR,
                    false,
                );
            }
        }
        FancyBorderEdgeOrientation::Vertical => {
            let x1 = x0 + FANCY_BORDER_THICKNESS as i32 - 1;
            v_solid_line(x0, y0, length, Rgb332::FANCY_BORDER_EDGE_COLOR);
            v_solid_line(x1, y0, length, Rgb332::FANCY_BORDER_EDGE_COLOR);
            for i in 1..(FANCY_BORDER_THICKNESS - 1) {
                v_dithered_line(
                    x0 + i as i32,
                    y0,
                    length,
                    Rgb332::FANCY_BORDER_EDGE_FILL_COLOR,
                    false,
                );
            }
        }
    }
}

pub fn fancy_border(x0: i32, y0: i32, w: usize, h: usize) {
    let y1 = y0 + (h - FANCY_BORDER_THICKNESS) as i32;
    let x1 = x0 + (w - FANCY_BORDER_THICKNESS) as i32;
    fancy_border_edge(x0, y0, w, FancyBorderEdgeOrientation::Horizontal);
    fancy_border_edge(x0, y1, w, FancyBorderEdgeOrientation::Horizontal);
    fancy_border_edge(x0, y0, h, FancyBorderEdgeOrientation::Vertical);
    fancy_border_edge(x1, y0, h, FancyBorderEdgeOrientation::Vertical);

    let x1 = x0 + w as i32;
    let y1 = y0 + h as i32;
    fancy_border_corner(x0, y0, FancyBorderCornerOrientation::TopLeft);
    fancy_border_corner(x0, y1, FancyBorderCornerOrientation::BottomLeft);
    fancy_border_corner(x1, y0, FancyBorderCornerOrientation::TopRight);
    fancy_border_corner(x1, y1, FancyBorderCornerOrientation::BottomRight);
}
