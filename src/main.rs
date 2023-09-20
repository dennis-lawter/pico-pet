#![no_std]
#![no_main]
#![feature(iter_advance_by)]
// TODO: remove dead_code
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
// extern crate panic_halt;
extern crate st7735_lcd;
extern crate waveshare_rp2040_lcd_0_96;

mod cores;
mod display;
mod exit;
mod globals;
mod hardware;
mod nvm;
mod setting_value;
mod states;

use waveshare_rp2040_lcd_0_96::{entry, hal::multicore::Multicore};

#[entry]
fn main() -> ! {
    init_globals();

    spawn_secondary_core_worker();

    cores::primary_main_loop()
}

fn init_globals() {
    globals::init_hardware();
    globals::init_input();
    globals::init_nvm();
    display::text_writer::init_singleton_fonts();
}

fn spawn_secondary_core_worker() {
    unsafe {
        let hardware = globals::get_hardware();
        let mut mc = Multicore::new(
            &mut *hardware.psm_ptr,
            &mut *hardware.ppb_ptr,
            &mut *hardware.fifo_ptr,
        );
        let cores = &mut mc.cores();
        let core1 = &mut cores[1];
        let sys_freq = hardware.sys_freq;
        let _test = core1.spawn(&mut cores::secondary_core::CORE1_STACK.mem, move || {
            cores::secondary_main_loop(sys_freq)
        });
    }
}
