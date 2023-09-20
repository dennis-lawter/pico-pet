pub mod primary_core;
pub mod secondary_core;
pub mod state_manager;

use self::primary_core::primary_main_loop;
use self::secondary_core::secondary_main_loop;

use waveshare_rp2040_lcd_0_96::hal::multicore::Multicore;

pub fn spawn_secondary_core_worker() {
    unsafe {
        let hardware = crate::globals::get_hardware();
        let mut mc = Multicore::new(
            &mut *hardware.psm_ptr,
            &mut *hardware.ppb_ptr,
            &mut *hardware.fifo_ptr,
        );
        let cores = &mut mc.cores();
        let core1 = &mut cores[1];
        let sys_freq = hardware.sys_freq;
        let _test = core1.spawn(&mut secondary_core::CORE1_STACK.mem, move || {
            secondary_main_loop(sys_freq)
        });
    }
}

pub fn run_primary_main_loop() -> ! {
    primary_main_loop()
}
