#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("init keyboard");
    let _p = embassy_rp::init(Default::default());

    loop {}
}
