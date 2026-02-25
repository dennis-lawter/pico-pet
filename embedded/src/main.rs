#![no_std]
#![no_main]

mod game;

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
