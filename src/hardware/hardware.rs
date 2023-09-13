use cortex_m::delay::Delay;

use debugless_unwrap::DebuglessUnwrap;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{DrawTarget, RgbColor},
};
use embedded_hal::{
    digital::v2::InputPin,
    prelude::{_embedded_hal_blocking_i2c_Read, _embedded_hal_blocking_i2c_Write},
    PwmPin,
};
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

use crate::globals;

use super::{audio::AudioFrequency, rtc::RealTime};

pub const LCD_WIDTH: usize = 128;
pub const LCD_HEIGHT: usize = 128;

type DisplaySdi = hal::Spi<hal::spi::Enabled, pac::SPI1, 8>;
type DisplayDc = hal::gpio::Pin<hal::gpio::bank0::Gpio8, hal::gpio::Output<hal::gpio::PushPull>>;
type DisplayRst = hal::gpio::Pin<hal::gpio::bank0::Gpio12, hal::gpio::Output<hal::gpio::PushPull>>;

pub type Lcd = ST7735<DisplaySdi, DisplayDc, DisplayRst>;

type LcdBlPinChannel = hal::pwm::Channel<hal::pwm::Pwm6, hal::pwm::FreeRunning, hal::pwm::B>;
type BuzzerPinChannel = hal::pwm::Channel<hal::pwm::Pwm2, hal::pwm::FreeRunning, hal::pwm::A>;
type BuzzerPwmSlice = hal::pwm::Slice<hal::pwm::Pwm2, hal::pwm::FreeRunning>;

type Key0Pin = hal::gpio::Pin<hal::gpio::bank0::Gpio15, hal::gpio::Input<hal::gpio::PullUp>>;
type Key1Pin = hal::gpio::Pin<hal::gpio::bank0::Gpio17, hal::gpio::Input<hal::gpio::PullUp>>;
type Key1AltPin = hal::gpio::Pin<hal::gpio::bank0::Gpio29, hal::gpio::Input<hal::gpio::PullUp>>;
type Key2Pin = hal::gpio::Pin<hal::gpio::bank0::Gpio2, hal::gpio::Input<hal::gpio::PullUp>>;
type Key3Pin = hal::gpio::Pin<hal::gpio::bank0::Gpio3, hal::gpio::Input<hal::gpio::PullUp>>;

type I2CBus = hal::I2C<
    pac::I2C0,
    (
        hal::gpio::Pin<hal::gpio::bank0::Gpio0, hal::gpio::Function<hal::gpio::I2C>>,
        hal::gpio::Pin<hal::gpio::bank0::Gpio1, hal::gpio::Function<hal::gpio::I2C>>,
    ),
>;

pub struct HardwareComponents {
    pub display: Lcd,
    pub sys_freq: u32,
    pub backlight_channel_ptr: *mut LcdBlPinChannel,
    pub buzzer_channel_ptr: *mut BuzzerPinChannel,
    pub buzzer_pwm_slice_ptr: *mut BuzzerPwmSlice,
    pub delay: Delay,
    pub key0: Key0Pin,
    pub key1: Key1Pin,
    pub key1_alt: Key1AltPin,
    pub key2: Key2Pin,
    pub key3: Key3Pin,
    pub psm_ptr: *mut PSM,
    pub ppb_ptr: *mut PPB,
    pub fifo_ptr: *mut SioFifo,
    pub i2c_bus: I2CBus,
}
impl HardwareComponents {
    pub fn new() -> Self {
        unsafe {
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

            // Init PWMs
            let mut pwm_slices: hal::pwm::Slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

            // Configure LCD PWM slice
            let pwm6 = &mut pwm_slices.pwm6;
            pwm6.set_ph_correct();
            // these numbers are around 55hz, aka the note A1
            // pwm6.set_div_int(255);
            // pwm6.set_top(4456);
            pwm6.set_top(65535);
            pwm6.set_div_int(1);
            pwm6.set_div_frac(0);
            pwm6.enable();

            // Configure buzzer PWM slice
            let buzzer_pwm_slice_ptr: *mut BuzzerPwmSlice =
                &mut pwm_slices.pwm2 as *mut BuzzerPwmSlice;

            // Output channel B on PWM6 to GPIO 13
            let backlight_channel_ptr = &mut pwm6.channel_b as *mut LcdBlPinChannel;
            // disable backlight ASAP to hide boot artifacts
            (*backlight_channel_ptr).output_to(pins.gpio13);
            (*backlight_channel_ptr).set_duty(0);

            let buzzer_channel_ptr =
                &mut (*buzzer_pwm_slice_ptr).channel_a as *mut BuzzerPinChannel;
            (*buzzer_channel_ptr).output_to(pins.gpio4);
            (*buzzer_channel_ptr).set_duty(0);

            (*buzzer_pwm_slice_ptr).set_ph_correct();
            (*buzzer_pwm_slice_ptr).set_div_int(0);
            (*buzzer_pwm_slice_ptr).set_div_frac(0);
            (*buzzer_pwm_slice_ptr).set_top(0);
            (*buzzer_pwm_slice_ptr).enable();

            let key0 = pins.gpio15.into_pull_up_input();
            let key1 = pins.gpio17.into_pull_up_input();
            let key1_alt = pins.gpio29.into_pull_up_input();
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

            display.clear(Rgb565::BLACK).debugless_unwrap();

            let sda_pin = pins.gpio0.into_mode::<hal::gpio::FunctionI2C>();
            let scl_pin = pins.gpio1.into_mode::<hal::gpio::FunctionI2C>();
            // let not_an_scl_pin = pins.gpio20.into_function::<hal::gpio::FunctionI2C>();

            // Create the I²C drive, using the two pre-configured pins. This will fail
            // at compile time if the pins are in the wrong mode, or if this I²C
            // peripheral isn't available on these pins!
            let i2c_bus: I2CBus = hal::I2C::i2c0(
                pac.I2C0,
                sda_pin,
                scl_pin, // Try `not_an_scl_pin` here
                400.kHz(),
                &mut pac.RESETS,
                &clocks.system_clock,
            );

            Self {
                display,
                sys_freq,
                backlight_channel_ptr,
                buzzer_channel_ptr,
                buzzer_pwm_slice_ptr,
                delay,
                key0,
                key1,
                key1_alt,
                key2,
                key3,
                psm_ptr,
                ppb_ptr,
                fifo_ptr,
                i2c_bus,
            }
        }
    }

    pub fn key0_pressed(&self) -> bool {
        self.key0.is_low().unwrap()
    }

    pub fn key1_pressed(&self) -> bool {
        self.key1.is_low().unwrap() || self.key1_alt.is_low().unwrap()
    }

    pub fn key2_pressed(&self) -> bool {
        self.key2.is_low().unwrap()
    }

    pub fn key3_pressed(&self) -> bool {
        self.key3.is_low().unwrap()
    }

    const BRIGHTNESS_LUT: [u16; 16] = [
        306, 438, 626, 895, 1281, 1831, 2619, 3746, 5357, 7660, 10955, 15667, 22406, 32043, 45825,
        65535,
    ];

    pub fn set_backlight(&mut self) {
        let brightness = unsafe { &globals::BRIGHTNESS_SETTING };
        let effective_brightness = Self::BRIGHTNESS_LUT[brightness.get_value() as usize];
        unsafe { (*self.backlight_channel_ptr).set_duty(effective_brightness) }
    }

    const VOLUME_LUT: [u16; 6] = [0, 32768 / 256, 32768 / 128, 32768 / 64, 32768 / 32, 32768];

    pub fn start_tone(&mut self, tone: &AudioFrequency) {
        let volume = unsafe { &globals::VOLUME_SETTING };
        let tone = if volume.get_value() == 0 {
            &AudioFrequency::None
        } else {
            tone
        };
        let tone_settings = tone.get_registers();
        let effective_volume = if tone == &AudioFrequency::None {
            0
        } else {
            Self::VOLUME_LUT[volume.get_value() as usize]
        };
        unsafe {
            (*self.buzzer_pwm_slice_ptr).set_top(tone_settings.0);
            (*self.buzzer_pwm_slice_ptr).set_div_int(tone_settings.1);
            (*self.buzzer_pwm_slice_ptr).set_div_frac(tone_settings.2);
            (*self.buzzer_channel_ptr).set_duty(effective_volume);
        }
    }

    pub fn end_tone(&mut self) {
        self.start_tone(&AudioFrequency::None);
    }

    pub fn get_time(&mut self) -> RealTime {
        let mut buffer = [0u8; 7];
        self.i2c_bus.write(0x68, &[0x00]).unwrap();
        self.i2c_bus.read(0x68, &mut buffer).unwrap();
        let sec = RealTime::bcd_to_dec(buffer[0]);
        let min = RealTime::bcd_to_dec(buffer[1]);
        let hr = RealTime::bcd_to_dec(buffer[2]);

        RealTime { sec, min, hr }
    }

    pub fn set_time(&mut self, new_time: &RealTime) {
        let sec_bcd = RealTime::dec_to_bcd(new_time.sec);
        let min_bcd = RealTime::dec_to_bcd(new_time.min);
        let hr_bcd = RealTime::dec_to_bcd(new_time.hr);

        let data = [0x00, sec_bcd, min_bcd, hr_bcd];

        self.i2c_bus.write(0x68, &data).unwrap();
    }
}
