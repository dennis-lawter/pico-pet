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

use crate::setting_value::Setting;

pub const LCD_WIDTH: usize = 128;
pub const LCD_HEIGHT: usize = 128;

type DisplaySdi = hal::Spi<hal::spi::Enabled, pac::SPI1, 8>;
type DisplayDc = hal::gpio::Pin<hal::gpio::bank0::Gpio8, hal::gpio::Output<hal::gpio::PushPull>>;
type DisplayRst = hal::gpio::Pin<hal::gpio::bank0::Gpio12, hal::gpio::Output<hal::gpio::PushPull>>;

pub type Lcd = ST7735<DisplaySdi, DisplayDc, DisplayRst>;

type LcdBlPinChannel = hal::pwm::Channel<hal::pwm::Pwm6, hal::pwm::FreeRunning, hal::pwm::B>;
type BuzzerPinChannel = hal::pwm::Channel<hal::pwm::Pwm0, hal::pwm::FreeRunning, hal::pwm::A>;

type Key0Pin = hal::gpio::Pin<hal::gpio::bank0::Gpio15, hal::gpio::Input<hal::gpio::PullUp>>;
type Key1Pin = hal::gpio::Pin<hal::gpio::bank0::Gpio17, hal::gpio::Input<hal::gpio::PullUp>>;
type Key2Pin = hal::gpio::Pin<hal::gpio::bank0::Gpio2, hal::gpio::Input<hal::gpio::PullUp>>;
type Key3Pin = hal::gpio::Pin<hal::gpio::bank0::Gpio3, hal::gpio::Input<hal::gpio::PullUp>>;

pub struct System {
    pub display: Lcd,
    pub sys_freq: u32,
    pub backlight_channel_ptr: *mut LcdBlPinChannel,
    pub buzzer_channel_ptr: *mut BuzzerPinChannel,
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
        let pwm0 = &mut pwm_slices.pwm0;
        pwm0.set_ph_correct();
        // pwm0.set_div_int(4);
        // pwm0.set_div_int(64);
        // pwm0.set_div_int(255);
        // pwm0.set_div_frac(0);
        // pwm0.set_top(4456 * 4 * 4);
        // pwm0.set_top(65_535 - 15_520);
        // pwm0.set_top(10702); //   4393 hz
        // pwm0.set_top(20702); //   4522 hz
        // pwm0.set_top(30802); //    3531 hz
        // pwm0.set_top(5702); //   10939 hz

        // pwm0.set_div_int(4);
        // pwm0.set_div_frac(0);
        // 4522 hz
        // pwm0.set_top(65535);
        // 7838 hz
        // pwm0.set_top(6_000);
        // 3842 hz
        // pwm0.set_top(5_500);
        // 3144 hz
        // pwm0.set_top(5_000);
        // 3919 hz
        // pwm0.set_top(4_000);
        // 10422 hz
        // pwm0.set_top(3000);

        pwm0.set_div_int(2);
        pwm0.set_div_frac(0);

        // trying to find a note
        // C 7 = 2093.00
        // C#7 = 2217.46
        // D 7 = 2349.32
        // D#7 = 2489.02
        // E 7 = 2637.02
        // F 7 = 2793.83
        // F#7 = 2959.96
        // G 7 = 3135.96
        // G#7 = 3322.44
        // A 7 = 3520.00
        // A#7 = 3729.31r
        // B 7 = 3951.07
        // C 8 = 4186.01

        pwm0.set_top(11236);
        // the number at the upper range flickers my frequency finder between the 2 threshholds
        // 3316 hz =      ? -   9_484   G#7
        // 3273 hz =  9_485 -   9_609
        // 3230 hz =  9_610 -   9_738
        // 3187 hz =  9_739 -   9_871
        // 3144 hz =  9_872 -  10_007   G 7
        // 3101 hz = 10_008 -  10_147
        // 3058 hz = 10_148 -  10_291
        // 3015 hz = 10_292 -  10_441
        // 2972 hz = 10_442 -       ?
        // 2929 hz = 10_596 -       ?
        // theoretical
        // 2886 hz = 10_750 -       ?
        // 2843 hz = 10_908 -       ?
        // 2800 hz = 11_070 -       ?
        //      hz =        -       ?
        //      hz =        -       ?
        pwm0.enable();

        // Output channel B on PWM6 to GPIO 13
        let backlight_channel_ptr = &mut pwm6.channel_b as *mut LcdBlPinChannel;
        let buzzer_channel_ptr = &mut pwm0.channel_a as *mut BuzzerPinChannel;
        unsafe {
            // disable backlight ASAP to hide boot artifacts
            (*backlight_channel_ptr).output_to(pins.gpio13);
            (*backlight_channel_ptr).set_duty(0);

            (*buzzer_channel_ptr).output_to(pins.gpio0);
            (*buzzer_channel_ptr).set_duty(512);
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
            buzzer_channel_ptr,
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

    /// LOG SCALE TO 65535 BY 24 STEPS, DROP LOWEST 8
    /// fn main() {
    ///     const M: f64 = 65535.0;
    ///     const TOTAL_VALUES: usize = 24;
    ///     const NEEDED_VALUES: usize = 16;
    ///     let r = M.powf(1.0 / (TOTAL_VALUES as f64 - 1.0));
    ///     let mut values = Vec::with_capacity(TOTAL_VALUES);
    ///     for i in 0..TOTAL_VALUES {
    ///         values.push((r.powf(i as f64) + 0.5) as u32);
    ///     }
    ///     let values: Vec<_> = values.into_iter().rev().take(NEEDED_VALUES).collect();
    ///     println!("{:?}", values);
    /// }
    const BRIGHTNESS_LUT: [u16; 16] = [
        47, 77, 124, 201, 326, 528, 855, 1384, 2242, 3631, 5880, 9524, 15425, 24983, 40463, 65535,
    ];

    pub fn set_backlight(&mut self, brightness: &Setting) {
        let effective_brightness = Self::BRIGHTNESS_LUT[brightness.get_value() as usize];
        unsafe {
            self.backlight_channel_ptr
                .as_mut()
                .unwrap()
                .set_duty(effective_brightness)
        }
    }
}
