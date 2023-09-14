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
// extern crate panic_halt;
extern crate st7735_lcd;
extern crate waveshare_rp2040_lcd_0_96;

mod cores;
mod display;
mod globals;
mod hardware;
mod setting_value;
mod states;

use waveshare_rp2040_lcd_0_96::{
    entry,
    hal::{multicore::Multicore, rom_data},
};

// #[allow(unused_imports)]
// use panic_halt as _;

#[entry]
fn main() -> ! {
    init_globals();

    spawn_secondary_core_worker();

    cores::primary_main_loop()
}

fn init_globals() {
    globals::init_hardware();
    globals::init_input();
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

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    if unsafe { globals::HARDWARE.is_none() } {
        loop {
            rom_data::reset_to_usb_boot(0, 0);
            // if reset fails, just sleep
            cortex_m::asm::wfi();
        }
    }
    let hardware = globals::get_hardware();
    display::render::flood(0b111_000_00);
    unsafe {
        embedded_hal::PwmPin::set_duty(&mut (*hardware.backlight_channel_ptr), 32767);
        hardware.end_tone();
    }
    display::text_writer::draw_text_centered(
        64,
        64 - 7,
        display::text_writer::FontStyle::BigBold,
        0b111_111_11,
        "PANIC!",
    );
    display::text_writer::draw_text_centered(
        64,
        128 - 15,
        display::text_writer::FontStyle::Small,
        0b111_111_11,
        "Resetting to USB...",
    );

    display::render::draw(&mut hardware.display);
    loop {
        hardware.delay.delay_ms(5_000);
        rom_data::reset_to_usb_boot(0, 0);
        // if reset fails, just sleep
        cortex_m::asm::wfi();
    }
}
