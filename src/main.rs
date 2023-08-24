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

use debugless_unwrap::*;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{DrawTarget, RgbColor},
};

use sprite::SpriteFactory;
use waveshare_rp2040_lcd_0_96::{
    entry,
    hal::{
        multicore::{Multicore, Stack},
        Sio,
    },
    pac,
};

#[allow(unused_imports)]
use panic_halt as _;

mod render;
mod sprite;
mod system;
use system::System;

#[entry]
fn main() -> ! {
    let mut system = System::new();
    system.display.clear(Rgb565::BLACK).debugless_unwrap();
    render::init_font();

    // spawn thread
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

    main_loop(&mut system)
}

static mut CORE1_STACK: Stack<4096> = Stack::new();
fn side_loop(sys_freq: u32) -> ! {
    let pac = unsafe { pac::Peripherals::steal() };
    let core = unsafe { pac::CorePeripherals::steal() };

    let mut sio = Sio::new(pac.SIO);
    let mut delay = cortex_m::delay::Delay::new(core.SYST, sys_freq);
    let mut i = 0u32;
    loop {
        i += 1;
        delay.delay_ms(1000);
        sio.fifo.write(i);
    }
}

fn main_loop(system: &mut System) -> ! {
    let mut frame_count = 0;

    let mut ferris = SpriteFactory::new_ferris_sprite();
    ferris.x = 32;
    ferris.y = 32;
    let mut urchin = SpriteFactory::new_urchin_sprite();
    urchin.x = 64;
    urchin.y = 64;

    let fifo = unsafe { &mut *system.fifo_ptr };

    let mut in_menu = false;
    loop {
        let input = fifo.read();
        frame_count = match input {
            Some(new_frame_count) => new_frame_count,
            None => frame_count,
        };

        render::flood(0b000_000_00);

        match in_menu {
            true => {
                render::fs_dialog_box(
                    "MENU",
                    r#"[#] brightness
[ ] sound
[ ] clock
[ ] sleep time
[ ] RESET !!!"#,
                );
            }
            false => {
                urchin.draw();
                ferris.draw();
                render::bottom_dialog_box(
                    "DIALOG\\b700!\\b703 so \\c700smol\\c003\\\\ so cute",
                    render::FontStyle::Normal,
                );
            }
        }

        render::draw(&mut system.display);

        match in_menu {
            true => {
                if system.key0_pressed() {
                    in_menu = false;
                }
            }
            false => {
                if system.key0_pressed() && system.key3_pressed() {
                    in_menu = true;
                } else {
                    if system.key0_pressed() {
                        ferris.x -= 1;
                    }
                    if system.key1_pressed() {
                        ferris.y += 1;
                    }
                    if system.key2_pressed() {
                        ferris.y -= 1;
                    }
                    if system.key3_pressed() {
                        ferris.x += 1;
                    }
                }
            }
        }
    }
}
