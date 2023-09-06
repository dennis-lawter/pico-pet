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

use cortex_m::delay::Delay;

use embedded_hal::{digital::v2::OutputPin, prelude::_embedded_hal_blocking_i2c_Read};
use fugit::RateExtU32;

use waveshare_rp2040_lcd_0_96::{
    hal::{self, pac, Clock},
    XOSC_CRYSTAL_FREQ,
};

use waveshare_rp2040_lcd_0_96::entry;

#[allow(unused_imports)]
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let sys_freq = clocks.system_clock.freq().to_Hz();
    let mut timer = Delay::new(core.SYST, sys_freq);

    // Configure two pins as being I²C, not GPIO
    let sda_pin = pins.gpio0.into_mode::<hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio1.into_mode::<hal::gpio::FunctionI2C>();
    // let not_an_scl_pin = pins.gpio20.into_function::<hal::gpio::FunctionI2C>();

    // Create the I²C drive, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let mut i2c = hal::I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin, // Try `not_an_scl_pin` here
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    let mut buffer = [0u8; 1];
    match i2c.read(0x86, &mut buffer) {
        Ok(_) => {}
        Err(_) => panic!(),
        // Err(_) => {}
    }

    let mut led_pin = pins.gpio25.into_push_pull_output();
    loop {
        led_pin.set_high().unwrap();
        timer.delay_ms(500);
        led_pin.set_low().unwrap();
        timer.delay_ms(500);
    }
}
