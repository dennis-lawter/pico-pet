use super::{
    font::{Font, FontFactory},
    render,
};

static mut SMALL_FONT: Option<Font> = None;
static mut ICON_FONT: Option<Font> = None;
static mut BIG_REGULAR_FONT: Option<Font> = None;
static mut BIG_BOLD_FONT: Option<Font> = None;
static mut BIG_ITALIC_FONT: Option<Font> = None;

// TODO: Remove unused fonts
#[allow(dead_code)]
pub enum FontStyle {
    Small,
    Big,
    BigBold,
    BigItalic,
    Icon,
}

pub fn init_singleton_fonts() {
    unsafe {
        SMALL_FONT = Some(FontFactory::new_small_font());
        ICON_FONT = Some(FontFactory::new_small_icon_font());
        BIG_REGULAR_FONT = Some(FontFactory::new_big_regular_font());
        BIG_BOLD_FONT = Some(FontFactory::new_big_bold_font());
        BIG_ITALIC_FONT = Some(FontFactory::new_big_italic_font());
    }
}

// TODO: center each line
pub fn draw_text_centered(x: i32, y: i32, style: FontStyle, color: u8, text: &str) {
    let font = get_font(style);
    let (width, _) = font.size.get_glyph_dimensions();
    let x = x - ((width * text.len()) as i32 / 2);
    font.draw_text(x, y, color, text, false);
}

pub fn draw_text(x: i32, y: i32, style: FontStyle, color: u8, text: &str) {
    let font = get_font(style);

    font.draw_text(x, y, color, text, false)
}

pub fn draw_text_wrapped(x: i32, y: i32, style: FontStyle, color: u8, text: &str) {
    let font = get_font(style);

    font.draw_text(x, y, color, text, true)
}

fn get_font(style: FontStyle) -> &'static Font<'static> {
    unsafe {
        let font_opt = match style {
            FontStyle::Small => &SMALL_FONT,
            FontStyle::Big => &BIG_REGULAR_FONT,
            FontStyle::BigBold => &BIG_BOLD_FONT,
            FontStyle::BigItalic => &BIG_ITALIC_FONT,
            FontStyle::Icon => &ICON_FONT,
        };
        &font_opt.as_ref().unwrap()
    }
}

#[allow(dead_code)]
pub fn bottom_dialog_box(text: &str) {
    let height = 18usize;
    let box_x: i32 = 0;
    let box_y = 128 - 24 - height as i32;
    let text_x = 5;
    let text_y = 128 - 24 + 4 - height as i32;

    render::fill_rect(box_x, box_y, 128, height, 0b111_111_11);
    render::fancy_border(0, box_y, 128, height);

    draw_text(text_x, text_y, FontStyle::Small, 0b000_000_11, text)
}

pub fn full_dialog_box(title: &str, text: &str) {
    let title_width = 8 * title.len() as i32;
    let title_x = 64 - (title_width / 2);

    render::flood(0b111_111_11);
    render::fancy_border(0, 0, 128, 128);

    draw_text(title_x, 5, FontStyle::BigBold, 0b000_000_00, title);
    draw_text(5, 18, FontStyle::Small, 0b000_000_11, text);
}
