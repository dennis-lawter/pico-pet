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

use waveshare_rp2040_lcd_0_96::{
    entry,
    hal::multicore::{Multicore, Stack},
    pac,
};

#[allow(unused_imports)]
use panic_halt as _;

mod core_loops;
mod font;
mod render;
mod rgb_converter;
mod setting_value;
mod sprite;
mod state;
mod system;
mod text_writer;
use system::System;

#[entry]
fn main() -> ! {
    let mut system = System::new();

    text_writer::init_singleton_fonts();

    unsafe {
        let mut mc = Multicore::new(
            &mut *system.psm_ptr,
            &mut *system.ppb_ptr,
            &mut *system.fifo_ptr,
        );
        let cores = &mut mc.cores();
        let core1 = &mut cores[1];
        let sys_freq = system.sys_freq;
        let _test = core1.spawn(&mut CORE1_STACK.mem, move || side_loop(sys_freq));
    }

    core_loops::main_loop(&mut system)
}

static mut CORE1_STACK: Stack<4096> = Stack::new();
fn side_loop(sys_freq: u32) -> ! {
    let core = unsafe { pac::CorePeripherals::steal() };

    let mut delay = cortex_m::delay::Delay::new(core.SYST, sys_freq);
    loop {
        delay.delay_ms(1000);
    }
}
