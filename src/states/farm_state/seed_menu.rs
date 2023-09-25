use crate::{display::text_writer, hardware::input::KeyNames};

pub struct SeedMenu {
    selection: usize,
    ready_to_exit: bool,
}

impl Default for SeedMenu {
    fn default() -> Self {
        Self {
            selection: Self::NUM_OF_SELECTIONS,
            ready_to_exit: false,
        }
    }
}

impl SeedMenu {
    const NUM_OF_SELECTIONS: usize = 8;
    pub fn input(&mut self) {
        let input = crate::globals::get_input();

        if input.get_state(&KeyNames::Back).just_released {
            self.ready_to_exit = true;
            return;
        }

        if input.get_state(&KeyNames::Left).just_pressed {
            self.dec_selection();
        }
        if input.get_state(&KeyNames::Right).just_pressed {
            self.inc_selection();
        }
        if input.get_state(&KeyNames::Confirm).just_released
            && self.selection != Self::NUM_OF_SELECTIONS
        {}
    }
    pub fn draw(&mut self) {
        let inv = crate::globals::get_inv();

        text_writer::full_dialog_box("SEED BAG", "");

        inv.seed_inventory.display(16, 20);

        if self.selection != Self::NUM_OF_SELECTIONS {
            text_writer::draw_text(
                10,
                20 + 8 * self.selection as i32,
                text_writer::FontStyle::Icon,
                0b000_000_00,
                "}",
            );
        }
    }

    fn dec_selection(&mut self) {
        if self.selection == 0 {
            self.selection = Self::NUM_OF_SELECTIONS - 1;
        } else {
            self.selection -= 1;
        }
    }

    fn inc_selection(&mut self) {
        if self.selection >= Self::NUM_OF_SELECTIONS - 1 {
            self.selection = 0;
        } else {
            self.selection += 1;
        }
    }
}
