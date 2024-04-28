#![no_std]
#![no_main]

mod task;

use bsp::{
    entry,
    hal::{clocks::init_clocks_and_plls, Watchdog},
    pac,
};
use defmt::*;
use rp_pico as bsp;
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    info!("init board");

    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let _ = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    info!("main stack pointer: {:?}", task::stack_ptr() as usize);

    task::init_scheduler();
    task::task_switch()
}
