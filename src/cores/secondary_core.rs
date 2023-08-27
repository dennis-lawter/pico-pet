use waveshare_rp2040_lcd_0_96::{hal::multicore::Stack, pac};

pub static mut CORE1_STACK: Stack<4096> = Stack::new();
pub fn secondary_main_loop(sys_freq: u32) -> ! {
    let core = unsafe { pac::CorePeripherals::steal() };

    let mut delay = cortex_m::delay::Delay::new(core.SYST, sys_freq);
    loop {
        delay.delay_ms(1000);
    }
}
