use crate::{
    display::{render, text_writer},
    states::{AppState, State},
};

pub struct SelectFoodState {
    key0_down: bool,
    key1_down: bool,
    key2_down: bool,
    key3_down: bool,
    next_state: Option<AppState>,
    frame_count: usize,
    last_page_seen: u16,
}

impl State for SelectFoodState {
    fn input(&mut self) {
        let hardware = crate::globals::get_hardware();
        if !hardware.key0_pressed() && self.key0_down {
            self.next_state = Some(AppState::GamePlay);
        }
        self.key0_down = hardware.key0_pressed();
        self.key1_down = hardware.key1_pressed();
        self.key2_down = hardware.key2_pressed();
        self.key3_down = hardware.key3_pressed();
    }

    fn tick(&mut self) {
        self.frame_count += 1;
    }

    fn sound(&mut self) {
        let hardware = crate::globals::get_hardware();
        hardware.end_tone();
    }

    fn draw(&mut self) {
        render::flood(0b000_000_00);
        text_writer::full_dialog_box("NOT IMPL", "todo!()");

        // DEBUGGING
        let hardware = crate::globals::get_hardware();

        let page = ((self.frame_count / 20) % 512) as u16;
        if page != self.last_page_seen {
            // write on the page change
            // let buffer = [
            //     ((page << 3) & 0xFF) as u8,
            //     ((page << 3) & 0xFF) as u8 + 1,
            //     ((page << 3) & 0xFF) as u8 + 2,
            //     ((page << 3) & 0xFF) as u8 + 3,
            //     ((page << 3) & 0xFF) as u8 + 4,
            //     ((page << 3) & 0xFF) as u8 + 5,
            //     ((page << 3) & 0xFF) as u8 + 6,
            //     ((page << 3) & 0xFF) as u8 + 7,
            // ];
            // hardware.write_nvm_page(page, &buffer);

            self.last_page_seen = page;
        }
        let page_str = fixedstr::str_format!(fixedstr::str32, "page {:#06x}", page);
        text_writer::draw_text(
            6,
            32,
            text_writer::FontStyle::Small,
            0b000_101_00,
            page_str.as_str(),
        );

        // read
        let page = hardware.get_nvm_page(page);
        let page_str = fixedstr::str_format!(
            fixedstr::str32,
            "{:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
            page[0],
            page[1],
            page[2],
            page[3],
            page[4],
            page[5],
            page[6],
            page[7]
        );
        text_writer::draw_text(
            6,
            32 + 8,
            text_writer::FontStyle::Small,
            0b000_000_00,
            page_str.as_str(),
        );
    }

    fn next_state(&self) -> &Option<AppState> {
        &self.next_state
    }
}
impl SelectFoodState {
    pub fn new() -> Self {
        Self {
            key0_down: false,
            key1_down: false,
            key2_down: false,
            key3_down: false,
            next_state: None,
            frame_count: 0,
            last_page_seen: 511,
        }
    }
}
