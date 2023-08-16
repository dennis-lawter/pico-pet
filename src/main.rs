#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate debugless_unwrap;
extern crate defmt_rtt;
extern crate embedded_graphics;
extern crate embedded_hal;
extern crate embedded_time;
extern crate fugit;
extern crate panic_halt;
extern crate st7735_lcd;
extern crate waveshare_rp2040_lcd_0_96;

use debugless_unwrap::*;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{DrawTarget, RgbColor},
};

use embedded_hal::digital::v2::OutputPin;
#[allow(unused_imports)]
use panic_halt as _;
use waveshare_rp2040_lcd_0_96::entry;

mod system;
use system::System;

#[entry]
fn main() -> ! {
    let mut system = System::new();
    system.display.clear(Rgb565::BLACK).debugless_unwrap();
    loop {
        system.delay.delay_ms(990);
        system.lcd_bl.set_low().debugless_unwrap();
        system.display.clear(Rgb565::RED).debugless_unwrap();
        system.delay.delay_ms(10);
        system.lcd_bl.set_high().debugless_unwrap();
        system.delay.delay_ms(990);
        system.lcd_bl.set_low().debugless_unwrap();
        system.display.clear(Rgb565::GREEN).debugless_unwrap();
        system.delay.delay_ms(10);
        system.lcd_bl.set_high().debugless_unwrap();
        system.delay.delay_ms(990);
        system.lcd_bl.set_low().debugless_unwrap();
        system.display.clear(Rgb565::BLUE).debugless_unwrap();
        system.delay.delay_ms(10);
        system.lcd_bl.set_high().debugless_unwrap();
        system.delay.delay_ms(990);
        system.lcd_bl.set_low().debugless_unwrap();
        system.display.clear(Rgb565::BLACK).debugless_unwrap();
        system.delay.delay_ms(10);
        system.lcd_bl.set_high().debugless_unwrap();
    }
}
