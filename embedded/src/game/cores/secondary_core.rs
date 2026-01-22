use waveshare_rp2040_lcd_0_96::hal::multicore::Stack;
use waveshare_rp2040_lcd_0_96::pac;

use crate::game::hardware::audio::AudioFrequency;

pub static mut CORE1_STACK: Stack<4096> = Stack::new();
pub fn secondary_main_loop(sys_freq: u32) -> ! {
    let core = unsafe { pac::CorePeripherals::steal() };

    let mut delay = cortex_m::delay::Delay::new(core.SYST, sys_freq);
    loop {
        // cortex_m::asm::wfi();
        let hardware = crate::game::globals::get_hardware();
        hardware.start_tone(&AudioFrequency::Cs5);
        delay.delay_ms(1000);
    }
}
