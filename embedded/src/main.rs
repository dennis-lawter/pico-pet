#![no_std]
#![no_main]
#![feature(iter_advance_by)]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate debugless_unwrap;
extern crate defmt_rtt;
extern crate embedded_graphics;
extern crate embedded_hal;
extern crate embedded_time;
extern crate fixedstr;
extern crate fugit;
extern crate st7735_lcd;
extern crate waveshare_rp2040_lcd_0_96;

mod game;

use game::cores;
use game::globals::init_globals;
use waveshare_rp2040_lcd_0_96::entry;

#[entry]
fn main() -> ! {
    init_globals();

    cores::spawn_secondary_core_worker();

    cores::run_primary_main_loop()
}
