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

use core::convert::TryInto;

use debugless_unwrap::*;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{DrawTarget, RgbColor},
};

use st7735_lcd::ST7735;
use waveshare_rp2040_lcd_0_96::{
    entry,
    hal::{
        self,
        multicore::{Multicore, Stack},
        Sio,
    },
    pac,
};

#[allow(unused_imports)]
use panic_halt as _;

mod render;
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
        let display: *mut ST7735<
            hal::Spi<hal::spi::Enabled, pac::SPI1, 8>,
            hal::gpio::Pin<hal::gpio::bank0::Gpio8, hal::gpio::Output<hal::gpio::PushPull>>,
            hal::gpio::Pin<hal::gpio::bank0::Gpio12, hal::gpio::Output<hal::gpio::PushPull>>,
        > = &mut system.display;
        let display_ptr = display as u32;
        let _test = core1.spawn(&mut CORE1_STACK.mem, move || unsafe {
            side_loop(sys_freq, display_ptr)
        });
    }

    main_loop(&mut system)
}

static mut CORE1_STACK: Stack<47860> = Stack::new();
fn side_loop(sys_freq: u32, display_ptr: u32) -> ! {
    unsafe {
        let display: *mut ST7735<
            hal::Spi<hal::spi::Enabled, pac::SPI1, 8>,
            hal::gpio::Pin<hal::gpio::bank0::Gpio8, hal::gpio::Output<hal::gpio::PushPull>>,
            hal::gpio::Pin<hal::gpio::bank0::Gpio12, hal::gpio::Output<hal::gpio::PushPull>>,
        > = display_ptr
            as *mut ST7735<
                hal::Spi<hal::spi::Enabled, pac::SPI1, 8>,
                hal::gpio::Pin<hal::gpio::bank0::Gpio8, hal::gpio::Output<hal::gpio::PushPull>>,
                hal::gpio::Pin<hal::gpio::bank0::Gpio12, hal::gpio::Output<hal::gpio::PushPull>>,
            >;
        let pac = unsafe { pac::Peripherals::steal() };
        let core = unsafe { pac::CorePeripherals::steal() };

        let mut sio = Sio::new(pac.SIO);
        let mut delay = cortex_m::delay::Delay::new(core.SYST, sys_freq);
        let mut i = 0u32;

        let img_bytes_332 = include_bytes!("../rgb332/scaledferris.png.data").clone();
        let mut crab_x = 16;
        let mut crab_y = 16;
        let mut in_menu = false;
        loop {
            // i += 1;
            // delay.delay_ms(1000);
            // sio.fifo.write(i);
            unsafe {
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
                        render::blit(32, 32, 32, 24, &img_bytes_332);
                        render::blit(crab_x, crab_y, 32, 24, &img_bytes_332);
                        render::bottom_dialog_box(
                            "DIALOG\\b700!\\b703 so \\c700smol\\c003\\\\ so cute",
                            render::FontStyle::Normal,
                        );
                    }
                }

                // render::draw(&mut system.display);

                // match in_menu {
                //     true => {
                //         if system.key0_pressed() {
                //             in_menu = false;
                //         }
                //     }
                //     false => {
                //         if system.key0_pressed() && system.key3_pressed() {
                //             in_menu = true;
                //         } else {
                //             if system.key0_pressed() {
                //                 crab_x -= 1;
                //             }
                //             if system.key1_pressed() {
                //                 crab_y += 1;
                //             }
                //             if system.key2_pressed() {
                //                 crab_y -= 1;
                //             }
                //             if system.key3_pressed() {
                //                 crab_x += 1;
                //             }
                //         }
                //     }
                // }
                render::draw(&mut *display);
            }
        }
    }
}

fn main_loop(system: &mut System) -> ! {
    let mut frame_count = 0;

    let fifo = unsafe { &mut *system.fifo_ptr };

    loop {
        let input = fifo.read();
        frame_count = match input {
            Some(new_frame_count) => new_frame_count,
            None => frame_count,
        };
    }
}
