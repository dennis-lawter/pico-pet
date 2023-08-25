use cortex_m::delay::Delay;

use embedded_hal::{digital::v2::InputPin, PwmPin};
use fugit::RateExtU32;

use waveshare_rp2040_lcd_0_96::{
    hal::{
        self,
        clocks::{init_clocks_and_plls, Clock},
        gpio::Pins,
        pac,
        pio::PIOExt,
        sio::SioFifo,
        watchdog::Watchdog,
        Sio,
    },
    pac::{PPB, PSM},
    XOSC_CRYSTAL_FREQ,
};

use st7735_lcd::{Orientation, ST7735};

pub const LCD_WIDTH: usize = 128;
pub const LCD_HEIGHT: usize = 128;

type DisplaySdi = hal::Spi<hal::spi::Enabled, pac::SPI1, 8>;
type DisplayDc = hal::gpio::Pin<hal::gpio::bank0::Gpio8, hal::gpio::Output<hal::gpio::PushPull>>;
type DisplayRst = hal::gpio::Pin<hal::gpio::bank0::Gpio12, hal::gpio::Output<hal::gpio::PushPull>>;

pub type Lcd = ST7735<DisplaySdi, DisplayDc, DisplayRst>;

type LcdBlPinChannel = hal::pwm::Channel<hal::pwm::Pwm6, hal::pwm::FreeRunning, hal::pwm::B>;

type Key0Pin = hal::gpio::Pin<hal::gpio::bank0::Gpio15, hal::gpio::Input<hal::gpio::PullUp>>;
type Key1Pin = hal::gpio::Pin<hal::gpio::bank0::Gpio17, hal::gpio::Input<hal::gpio::PullUp>>;
type Key2Pin = hal::gpio::Pin<hal::gpio::bank0::Gpio2, hal::gpio::Input<hal::gpio::PullUp>>;
type Key3Pin = hal::gpio::Pin<hal::gpio::bank0::Gpio3, hal::gpio::Input<hal::gpio::PullUp>>;

pub struct System {
    pub display: Lcd,
    pub sys_freq: u32,
    pub backlight_channel_ptr: *mut LcdBlPinChannel,
    pub delay: Delay,
    pub key0: Key0Pin,
    pub key1: Key1Pin,
    pub key2: Key2Pin,
    pub key3: Key3Pin,
    pub psm_ptr: *mut PSM,
    pub ppb_ptr: *mut PPB,
    pub fifo_ptr: *mut SioFifo,
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

        let mut sio = Sio::new(pac.SIO);
        let pins = Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        // disable backlight ASAP to hide boot artifacts
        // let lcd_bl = pins.gpio13.into_mode();
        // Init PWMs
        let mut pwm_slices: hal::pwm::Slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

        // Configure PWM4
        let pwm = &mut pwm_slices.pwm6;
        pwm.set_ph_correct();
        pwm.enable();

        // Output channel B on PWM6 to GPIO 13
        let backlight_channel_ptr = &mut pwm.channel_b as *mut LcdBlPinChannel;
        unsafe {
            (*backlight_channel_ptr).output_to(pins.gpio13);
            (*backlight_channel_ptr).set_duty(0);
        }
        let key0 = pins.gpio15.into_pull_up_input();
        let key1 = pins.gpio17.into_pull_up_input();
        let key2 = pins.gpio2.into_pull_up_input();
        let key3 = pins.gpio3.into_pull_up_input();

        let sys_freq = clocks.system_clock.freq().to_Hz();
        let mut delay = Delay::new(core.SYST, sys_freq);

        let (mut _pio, _sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

        let lcd_dc = pins.gpio8.into_push_pull_output();
        let mut _lcd_cs = pins.gpio9.into_mode::<hal::gpio::FunctionSpi>();
        let mut _lcd_clk = pins.gpio10.into_mode::<hal::gpio::FunctionSpi>();
        let mut _lcd_mosi = pins.gpio11.into_mode::<hal::gpio::FunctionSpi>();
        let lcd_rst = pins
            .gpio12
            .into_push_pull_output_in_state(hal::gpio::PinState::High);

        let spi = hal::Spi::<_, _, 8>::new(pac.SPI1);

        let spi = spi.init(
            &mut pac.RESETS,
            clocks.peripheral_clock.freq(),
            10.MHz(),
            &embedded_hal::spi::MODE_0,
        );

        let mut display = ST7735::new(
            spi,
            lcd_dc,
            lcd_rst,
            false,
            false,
            LCD_WIDTH as u32,
            LCD_HEIGHT as u32,
        );

        display.init(&mut delay).unwrap();
        display.set_orientation(&Orientation::Portrait).unwrap();

        display.set_offset(2, 1);

        let psm_ptr: *mut PSM = &mut pac.PSM as *mut PSM;
        let ppb_ptr: *mut PPB = &mut pac.PPB as *mut PPB;
        let fifo_ptr: *mut SioFifo = &mut sio.fifo as *mut SioFifo;

        Self {
            display,
            sys_freq,
            backlight_channel_ptr,
            delay,
            key0,
            key1,
            key2,
            key3,
            psm_ptr,
            ppb_ptr,
            fifo_ptr,
        }
    }

    pub fn key0_pressed(&self) -> bool {
        self.key0.is_low().unwrap()
    }

    pub fn key1_pressed(&self) -> bool {
        self.key1.is_low().unwrap()
    }

    pub fn key2_pressed(&self) -> bool {
        self.key2.is_low().unwrap()
    }

    pub fn key3_pressed(&self) -> bool {
        self.key3.is_low().unwrap()
    }

    const BRIGHTNESS_LUT: [u16; 22] = [
        45, 64, 91, 128, 181, 256, 362, 512, 724, 1024, 1448, 2048, 2896, 4096, 5793, 8192, 11585,
        16384, 23170, 32768, 46341, 65535,
    ];

    pub fn set_backlight(&mut self, brightness: u8) {
        let effective_brightness = Self::BRIGHTNESS_LUT[brightness as usize];
        unsafe {
            self.backlight_channel_ptr
                .as_mut()
                .unwrap()
                .set_duty(effective_brightness)
        }
    }
}

impl Drop for System {
    fn drop(&mut self) {
        unsafe { core::ptr::drop_in_place(self.psm_ptr) };
        unsafe { core::ptr::drop_in_place(self.ppb_ptr) };
        unsafe { core::ptr::drop_in_place(self.fifo_ptr) };
    }
}
