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

use setting_value::Setting;
use sprite::SpriteFactory;
use text_writer::FontStyle;
use waveshare_rp2040_lcd_0_96::{
    entry,
    hal::multicore::{Multicore, Stack},
    pac,
};

#[allow(unused_imports)]
use panic_halt as _;

mod font;
mod render;
mod rgb_converter;
mod setting_value;
mod sprite;
mod system;
mod text_writer;
use system::System;

#[entry]
fn main() -> ! {
    let mut system = System::new();
    system.display.clear(Rgb565::BLACK).debugless_unwrap();

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

    main_loop(&mut system)
}

static mut CORE1_STACK: Stack<4096> = Stack::new();
fn side_loop(sys_freq: u32) -> ! {
    let core = unsafe { pac::CorePeripherals::steal() };

    let mut delay = cortex_m::delay::Delay::new(core.SYST, sys_freq);
    loop {
        delay.delay_ms(1000);
    }
}

fn main_loop(system: &mut System) -> ! {
    let mut ferris = SpriteFactory::new_ferris_sprite();
    ferris.x = 32;
    ferris.y = 32;
    let mut corro = SpriteFactory::new_corro_sprite();
    corro.x = 64;
    corro.y = 64;
    let mut frame_count = 0;

    let mut brightness = Setting::new_max();

    // clear the LCD
    render::flood(0b000_000_00);
    render::draw(&mut system.display);
    let mut key_repeat_slowdown_timer = 0;

    let mut in_menu = false;
    loop {
        frame_count += 1;
        render::flood(0b000_000_00);

        match in_menu {
            true => {
                let title = "BRIGHTNESS";
                let menu_body = "";
                text_writer::full_dialog_box(title, menu_body);
                text_writer::draw_text(
                    24,
                    18,
                    FontStyle::Icon,
                    0b000_000_11,
                    brightness.generate_bar(),
                );
            }
            false => {
                corro.draw(0);
                ferris.draw((frame_count / 20) % 2);

                let text = "DIALOG\\b700!\\b703 so \\c700smol\\c003\\\\ so cute";
                text_writer::bottom_dialog_box(text);
            }
        }

        system.set_backlight(&brightness);
        render::draw(&mut system.display);

        match in_menu {
            true => {
                if system.key0_pressed() {
                    in_menu = false;
                }
                if system.key1_pressed() && !system.key2_pressed() {
                    if key_repeat_slowdown_timer == 0 {
                        key_repeat_slowdown_timer = 5;
                        brightness.dec();
                    } else {
                        key_repeat_slowdown_timer -= 1;
                    }
                } else if system.key2_pressed() && !system.key1_pressed() {
                    if key_repeat_slowdown_timer == 0 {
                        key_repeat_slowdown_timer = 5;
                        brightness.inc();
                    } else {
                        key_repeat_slowdown_timer -= 1;
                    }
                } else {
                    key_repeat_slowdown_timer = 0;
                }
            }
            false => {
                if system.key2_pressed() && system.key3_pressed() {
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
