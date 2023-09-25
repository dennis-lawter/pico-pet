#![no_std]
#![no_main]
#![feature(iter_advance_by)]
// TODO (RELEASE): remove dead_code
#![allow(dead_code)]

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

mod cores;
mod display;
mod exit;
mod globals;
mod hardware;
mod nvm;
mod rand;
mod setting_value;
mod states;
mod color;

use waveshare_rp2040_lcd_0_96::entry;

fn init_globals() {
    globals::init_hardware();
    globals::init_nvm();
    globals::init_rng();
    globals::init_inv();
    globals::init_input();
    globals::init_garden();
    display::text_writer::init_singleton_fonts();
}

#[entry]
fn main() -> ! {
    init_globals();

    cores::spawn_secondary_core_worker();

    cores::run_primary_main_loop()
}
