use waveshare_rp2040_lcd_0_96::hal::rom_data;

use crate::color::Rgb332;
use crate::display::render;
use crate::display::text_writer;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if unsafe { crate::globals::HARDWARE.is_none() } {
        loop {
            rom_data::reset_to_usb_boot(0, 0);
            // if reset fails, just sleep
            cortex_m::asm::wfi();
        }
    }
    let hardware = crate::globals::get_hardware();
    render::flood(Rgb332::RED);
    unsafe {
        embedded_hal::PwmPin::set_duty(&mut (*hardware.backlight_channel_ptr), 32767);
        hardware.end_tone();
    }

    let err_str = fixedstr::str_format!(fixedstr::str256, "error:\n\\b000{:?}", info);

    text_writer::draw_text_centered(
        64,
        4,
        text_writer::FontStyle::BigBold,
        Rgb332::WHITE,
        "PANIC!",
    );

    text_writer::draw_text_wrapped(
        0,
        16,
        text_writer::FontStyle::Small,
        Rgb332::WHITE,
        &err_str,
    );

    text_writer::draw_text_centered(
        64,
        128 - 15,
        text_writer::FontStyle::Small,
        Rgb332::WHITE,
        "press any key to reboot",
    );

    render::draw(&mut hardware.display);

    while !hardware.key0_pressed()
        && !hardware.key1_pressed()
        && !hardware.key2_pressed()
        && !hardware.key3_pressed()
    {}
    // TODO (RELEASE): don't reset to USB
    rom_data::reset_to_usb_boot(0, 0);
    // if reset fails, just reboot
    cortex_m::peripheral::SCB::sys_reset()
}

pub fn reboot() -> ! {
    if unsafe { crate::globals::HARDWARE.is_none() } {
        loop {
            rom_data::reset_to_usb_boot(0, 0);
            // if reset fails, just sleep
            cortex_m::asm::wfi();
        }
    }
    let hardware = crate::globals::get_hardware();
    render::flood(Rgb332::from_u8(0b010_010_11));
    unsafe {
        embedded_hal::PwmPin::set_duty(&mut (*hardware.backlight_channel_ptr), 32767);
        hardware.end_tone();
    }
    text_writer::draw_text_centered(
        64,
        128 - 15,
        text_writer::FontStyle::Small,
        Rgb332::WHITE,
        "Rebooting...",
    );

    render::draw(&mut hardware.display);

    hardware.delay.delay_ms(1_000);
    cortex_m::peripheral::SCB::sys_reset()
}
