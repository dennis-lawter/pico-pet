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
    image::{Image, ImageRawLE},
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::{DrawTarget, Point, RgbColor},
    text::Text,
    Drawable,
};

use fixedstr::{str8, try_format};
#[allow(unused_imports)]
use panic_halt as _;
use waveshare_rp2040_lcd_0_96::{
    entry,
    hal::{
        multicore::{Multicore, Stack},
        Sio,
    },
    pac,
};

mod system;
use system::System;

#[entry]
fn main() -> ! {
    let mut system = System::new();
    system.display.clear(Rgb565::BLACK).debugless_unwrap();

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
    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
    let mut frame_count;
    let img_bytes = include_bytes!("../assets/ferris.raw").clone();

    let fifo = unsafe { &mut *system.fifo_ptr };

    loop {
        let input = fifo.read();
        frame_count = input.unwrap_or(0);

        let img_raw: ImageRawLE<Rgb565> = ImageRawLE::new(&img_bytes, 86);
        let img = Image::with_center(&img_raw, Point { x: 64, y: 32 });
        img.draw(&mut system.display).debugless_unwrap();

        let frame_count_str = try_format!(str8, "{}", frame_count).unwrap();

        Text::new(frame_count_str.as_str(), Point::new(20, 128 - 12), style)
            .draw(&mut system.display)
            .debugless_unwrap();
        img.draw(&mut system.display).debugless_unwrap();
        system.delay.delay_ms(1000);
        system.display.clear(Rgb565::BLACK).debugless_unwrap();
    }
}
