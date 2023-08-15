//! # GPIO 'Blinky' Example
//!
//! This application demonstrates how to control a GPIO pin on the RP2040.
//!
//! It may need to be adapted to your particular board layout and/or pin assignment.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]
extern crate embedded_graphics;
extern crate embedded_hal;
extern crate embedded_time;
extern crate fugit;
extern crate panic_halt;
extern crate rp2040_hal;
extern crate st7735_lcd;

use embedded_graphics::primitives::{Circle, PrimitiveStyle};
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
#[allow(unused_imports)]
use panic_halt as _;

// Alias for our HAL crate
use rp2040_hal as hal;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

// Some traits we need
use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::pixelcolor::{BinaryColor, Rgb565};
use embedded_graphics::prelude::*;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::rate::Extensions;
use rp2040_hal::clocks::Clock;
// use embedded_time::fixed_point::FixedPoint;
use embedded_graphics::prelude::*;
use st7735_lcd::Orientation;

use fugit::RateExtU32;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

/// Entry point to our bare-metal application.
///
/// The `#[rp2040_hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
///
/// The function configures the RP2040 peripherals, then toggles a GPIO pin in
/// an infinite loop. If there is an LED connected to that pin, it will blink.
#[rp2040_hal::entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut lcd_led = pins.gpio13.into_push_pull_output();

    // These are implicitly used by the spi driver if they are in the correct mode
    let _spi_sclk = pins.gpio10.into_mode::<hal::gpio::FunctionSpi>();
    let _spi_mosi = pins.gpio11.into_mode::<hal::gpio::FunctionSpi>();
    // let _spi_miso = pins.gpio4.into_mode::<hal::gpio::FunctionSpi>();

    let spi = hal::Spi::<_, _, 8>::new(pac.SPI0);
    let dc = pins.gpio8.into_push_pull_output();
    let rst = pins.gpio12.into_push_pull_output();

    // Exchange the uninitialised SPI driver for an initialised one
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        RateExtU32::Hz(12_000_000u32),
        &embedded_hal::spi::MODE_0,
    );
    let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, true, false, 132, 162);

    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    disp.clear(Rgb565::BLUE).unwrap();
    // disp.set_offset(0, 0);


    // Configure mode
    let mut led_pin = pins.gpio9.into_push_pull_output();
    led_pin.set_low().unwrap();

    // let image_raw: ImageRawLE<Rgb565> =
    //     ImageRaw::new(include_bytes!("assets/ferris.raw"), 86);

    // let image: Image<_> = Image::new(&image_raw, Point::new(34, 8));
    // lcd_led.set_high().unwrap();
    // image.draw(&mut disp).unwrap();

    // Wait until the background and image have been rendered otherwise
    // the screen will show random pixels for a brief moment
    // lcd_led.set_high().unwrap();

    // let thick_stroke = PrimitiveStyle::with_stroke(Rgb565::RED, 3);
    // disp.clear(Rgb565::BLUE).unwrap();
    // lcd_led.set_high().unwrap();
    // Circle::new(Point::new(64, 64), 16)
    //     .into_styled(thick_stroke)
    //     .draw(&mut disp)
    //     .unwrap();

    loop {
        // disp.clear(Rgb565::BLUE).unwrap();
        // led_pin.set_high().unwrap();
        // disp.clear(Rgb565::BLACK).unwrap();
        delay.delay_ms(500);
        lcd_led.set_high().unwrap();
        delay.delay_ms(500);
        lcd_led.set_low().unwrap();

        // Circle::new(Point::new(64, 64), 16)
        //     .into_styled(thick_stroke)
        //     .draw(&mut disp)
        //     .unwrap();
        // continue;
    }
}
