#![no_std]
#![no_main]

use defmt::info;
use embedded_hal::digital::OutputPin;
use rp_pico::{entry, hal, hal::Clock, pac};
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    info!("board init");

    // get RP2040 peripherals
    let mut pac = pac::Peripherals::take().unwrap();
    // ger Cortex-M core peripherals
    let core = pac::CorePeripherals::take().unwrap();
    // watchdog peripheral needed for clock setup
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // clock setup
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // delay provider
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    // single-cycle I/O (SIO) peripheral
    let sio = hal::Sio::new(pac.SIO);

    // init pins and set GPIO default functions
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // pico on board LED
    let mut pico_led = pins.led.into_push_pull_output();

    loop {
        pico_led.set_high().unwrap();
        delay.delay_ms(500);
        pico_led.set_low().unwrap();
        delay.delay_ms(500);
    }
}
