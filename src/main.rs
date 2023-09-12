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
extern crate panic_halt;
extern crate st7735_lcd;
extern crate waveshare_rp2040_lcd_0_96;

mod cores;
mod display;
mod globals;
mod hardware;
mod setting_value;
mod states;

use crate::hardware::HardwareComponents;

use waveshare_rp2040_lcd_0_96::{entry, hal::multicore::Multicore};

#[allow(unused_imports)]
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut system = HardwareComponents::new();

    init_globals();

    spawn_secondary_core_worker(&mut system);

    cores::primary_main_loop(&mut system)
}

fn init_globals() {
    display::text_writer::init_singleton_fonts();
}

fn spawn_secondary_core_worker(system: &mut HardwareComponents) {
    unsafe {
        let mut mc = Multicore::new(
            &mut *system.psm_ptr,
            &mut *system.ppb_ptr,
            &mut *system.fifo_ptr,
        );
        let cores = &mut mc.cores();
        let core1 = &mut cores[1];
        let sys_freq = system.sys_freq;
        let _test = core1.spawn(&mut cores::secondary_core::CORE1_STACK.mem, move || {
            cores::secondary_main_loop(sys_freq)
        });
    }
}
