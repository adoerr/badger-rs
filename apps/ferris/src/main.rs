#![no_std]
#![no_main]

use defmt::info;
use rp_pico::entry;
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    info!("board init");

    loop {}
}
