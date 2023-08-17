#![no_std]
#![no_main]
#![feature(slice_flatten)]

// extern crate alloc;
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

// use alloc::{format, string::String};
use debugless_unwrap::*;
use embedded_graphics::{
    image::{Image, ImageRawLE},
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::{DrawTarget, Point, RgbColor},
    text::Text,
    Drawable,
};

use embedded_hal::digital::v2::InputPin;
use fixedstr::{str8, try_format};
#[allow(unused_imports)]
use panic_halt as _;
use waveshare_rp2040_lcd_0_96::entry;

mod system;
use system::System;
mod myferris;
use myferris::MyFerris;

#[entry]
fn main() -> ! {
    let mut system = System::new();
    // let my_ferris = MyFerris::new();
    system.display.clear(Rgb565::BLACK).debugless_unwrap();
    let mut img_bytes = include_bytes!("../assets/ferris.raw").clone();
    let mut frame_count = 0usize;

    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);

    let num_chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut frame_str = [' '; 10];

    loop {
        // if frame_count >= 1 << 16 {
        //     continue;
        // }

        frame_count += 1;
        // for i in 0..img_bytes.len() {
        //     img_bytes[i] += 1;
        // }

        let img_raw: ImageRawLE<Rgb565> = ImageRawLE::new(&img_bytes, 86);
        let img = Image::with_center(&img_raw, Point { x: 64, y: 32 });
        img.draw(&mut system.display).debugless_unwrap();

        // frame_str = [' '; 10];
        // let mut frame_count_copy = frame_count;
        // let mut i = 8;
        // while frame_count_copy > 0 {
        //     frame_str[i] = num_chars[frame_count_copy % 10];
        //     frame_count_copy = frame_count_copy / 10;
        //     i -= 1;
        // }
        // frame_str[9] = '\0';

        // let frame_count_str = char_array_to_str(&frame_str).unwrap();

        // let frame_count_str = format!("{}", frame_count);

        let frame_count_str = try_format!(str8, "{}", frame_count).unwrap();

        Text::new(frame_count_str.as_str(), Point::new(20, 128 - 12), style)
            .draw(&mut system.display)
            .debugless_unwrap();
        // if system.key0.is_low().debugless_unwrap() {
        //     system.display.clear(Rgb565::RED).debugless_unwrap();
        // } else if system.key1.is_low().debugless_unwrap() {
        //     system.display.clear(Rgb565::GREEN).debugless_unwrap();
        // } else if system.key2.is_low().debugless_unwrap() {
        //     system.display.clear(Rgb565::BLUE).debugless_unwrap();
        // } else if system.key3.is_low().debugless_unwrap() {
        //     system.display.clear(Rgb565::WHITE).debugless_unwrap();
        // } else {
        //     continue;
        // }
        img.draw(&mut system.display).debugless_unwrap();
        // system.display.clear(Rgb565::RED).debugless_unwrap();
        // system.display.clear(Rgb565::GREEN).debugless_unwrap();
        // system.display.clear(Rgb565::BLUE).debugless_unwrap();
        system.delay.delay_ms(500);
        system.display.clear(Rgb565::BLACK).debugless_unwrap();
    }
}
