#![no_std]
#![no_main]

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

use embedded_hal::digital::v2::InputPin;

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
    let img_bytes_332 = include_bytes!("../rgb332/scaledferris.png.data").clone();

    let fifo = unsafe { &mut *system.fifo_ptr };

    let mut crab_x = 16;
    let mut crab_y = 16;
    loop {
        let input = fifo.read();
        frame_count = match input {
            Some(new_frame_count) => new_frame_count,
            None => frame_count,
        };

        render::flood(0b000_000_00);
        render::blit(32, 32, 32, 24, &img_bytes_332);
        render::blit(crab_x, crab_y, 32, 24, &img_bytes_332);

        // render::bottom_dialog_box("DIALOG! so smol, so cute");
        render::fs_dialog_box("When I was\nA small boy\nMy father\nTook me into the city\nTo see a marching band\nHe said \"SON WHEN\nYOU GROW UP\nWOULD YOU BE\nTHE SAVIOR OF THE BROKEN\nTHE BEATEN AND THE DAMNED?\"\nHe said \"WILL YOU\nDEFEAT THEM?\nYOUR DEMONS\nAND ALL THE NONBELIEVERS\nTHE PLANS THAT THEY HAVE MADE.\"");

        render::draw(&mut system.display);

        if system.key0.is_low().unwrap() {
            crab_x -= 1;
        }
        if system.key1.is_low().unwrap() {
            crab_y += 1;
        }
        if system.key2.is_low().unwrap() {
            crab_y -= 1;
        }
        if system.key3.is_low().unwrap() {
            crab_x += 1;
        }
    }
}
