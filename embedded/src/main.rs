#![no_std]
#![no_main]

mod game;

#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[allow(unused_imports)]
use game::cores::spawn_secondary_core_worker;

use game::cores::run_primary_main_loop;
use game::globals::init_globals;

use rp2040_hal::entry;

#[entry]
fn main() -> ! {
    init_globals();

    // spawn_secondary_core_worker();

    run_primary_main_loop()
}

// Enable hardware interrupt to wake the CPU from wfi()
use rp2040_hal::pac::interrupt;
#[interrupt]
fn IO_IRQ_BANK0() {
    unsafe {
        let p = rp2040_hal::pac::Peripherals::steal();

        // Clear all GPIO interrupt sources
        p.IO_BANK0.intr[0].write(|w| w.bits(0xffffffff));
        p.IO_BANK0.intr[1].write(|w| w.bits(0xffffffff));
    }
}
