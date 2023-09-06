#![no_std]
#![no_main]
#![feature(iter_advance_by)]
#![feature(panic_info_message)]

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
mod setting_value;
mod states;
mod system;

use crate::system::SystemComponents;

use embedded_hal::PwmPin;
use fixedstr::str_format;
use waveshare_rp2040_lcd_0_96::{
    entry,
    hal::{multicore::Multicore, rom_data},
};

// #[allow(unused_imports)]
// use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut system = SystemComponents::new();

    init_globals();

    unsafe {
        GLOBAL_SYSTEM_PTR = &mut system as *mut SystemComponents as u32;
    }

    spawn_secondary_core_worker(&mut system);

    cores::primary_main_loop(&mut system)
}

fn init_globals() {
    display::text_writer::init_singleton_fonts();
}

static mut GLOBAL_SYSTEM_PTR: u32 = 0;

fn spawn_secondary_core_worker(system: &mut SystemComponents) {
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

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    if unsafe { GLOBAL_SYSTEM_PTR } == 0 {
        loop {
            rom_data::reset_to_usb_boot(0, 0);
            // if reset fails, just sleep
            cortex_m::asm::wfi();
        }
    }
    let system_ptr = unsafe { GLOBAL_SYSTEM_PTR } as *mut SystemComponents;
    let system_mut_ref = unsafe { &mut *system_ptr as &mut SystemComponents };
    display::render::flood(0b111_000_00);
    unsafe {
        (*system_mut_ref.backlight_channel_ptr).set_duty(32767);
        system_mut_ref.end_tone();
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

    display::render::draw(unsafe { &mut (*system_ptr).display });
    loop {
        system_mut_ref.delay.delay_ms(5_000);
        rom_data::reset_to_usb_boot(0, 0);
        // if reset fails, just sleep
        cortex_m::asm::wfi();
    }
}
