use cortex_m::delay::Delay;

use embedded_hal::digital::v2::OutputPin;
use fugit::RateExtU32;

use waveshare_rp2040_lcd_0_96::{
    hal::{
        self,
        clocks::{init_clocks_and_plls, Clock},
        pac,
        pio::PIOExt,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};

use st7735_lcd::{Orientation, ST7735};

const LCD_WIDTH: u32 = 128;
const LCD_HEIGHT: u32 = 128;

pub struct System {
    pub display: ST7735<
        hal::Spi<hal::spi::Enabled, pac::SPI1, 8>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio8, hal::gpio::Output<hal::gpio::PushPull>>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio12, hal::gpio::Output<hal::gpio::PushPull>>,
    >,
    pub lcd_bl: hal::gpio::Pin<hal::gpio::bank0::Gpio13, hal::gpio::Output<hal::gpio::PushPull>>,
    pub delay: Delay,
}
impl System {
    pub fn new() -> Self {
        let mut pac = pac::Peripherals::take().unwrap();
        let core = pac::CorePeripherals::take().unwrap();

        let mut watchdog = Watchdog::new(pac.WATCHDOG);

        let clocks = init_clocks_and_plls(
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

        let sio = Sio::new(pac.SIO);
        let pins = Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );
        let mut lcd_bl = pins
            .gp13
            .into_push_pull_output_in_state(hal::gpio::PinState::Low);

        let sys_freq = clocks.system_clock.freq().to_Hz();
        let mut delay = Delay::new(core.SYST, sys_freq);

        let (mut _pio, _sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

        let lcd_dc = pins.gp8.into_push_pull_output();
        let mut _lcd_cs = pins.gp9.into_mode::<hal::gpio::FunctionSpi>();
        let mut _lcd_clk = pins.gp10.into_mode::<hal::gpio::FunctionSpi>();
        let mut _lcd_mosi = pins.gp11.into_mode::<hal::gpio::FunctionSpi>();
        let lcd_rst = pins
            .gp12
            .into_push_pull_output_in_state(hal::gpio::PinState::High);

        let spi = hal::Spi::<_, _, 8>::new(pac.SPI1);

        let spi = spi.init(
            &mut pac.RESETS,
            clocks.peripheral_clock.freq(),
            10.MHz(),
            &embedded_hal::spi::MODE_0,
        );

        let mut display = ST7735::new(spi, lcd_dc, lcd_rst, false, false, LCD_WIDTH, LCD_HEIGHT);

        display.init(&mut delay).unwrap();
        display.set_orientation(&Orientation::Landscape).unwrap();

        display.set_offset(1, 2);

        lcd_bl.set_high().unwrap();

        Self {
            display,
            lcd_bl,
            delay,
        }
    }
}
