#![no_std]
#![no_main]

use defmt::info;
use display_interface_spi::SPIInterface;
use embedded_hal::{digital::OutputPin, spi::MODE_3};
use rp_pico::{
    entry, hal,
    hal::{fugit::RateExtU32, gpio::FunctionSpi, Clock, Spi},
    pac,
};
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    info!("board init");

    // RP2040 peripherals
    let mut pac = pac::Peripherals::take().unwrap();
    // Cortex-M core peripherals
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
    // display master out slave in
    let lcd_mosi = pins.gpio19.into_function::<FunctionSpi>();
    // display slave clock
    let lcd_sclk = pins.gpio18.into_function::<FunctionSpi>();
    // display direct current
    let lcd_dc = pins.gpio16.into_push_pull_output();
    // display chip select
    let lcd_cs = pins.gpio17.into_push_pull_output();
    // display backlight enable
    let _lcd_bl = pins.gpio20.into_push_pull_output();

    // serial interface minimal clock cycle for a write command is 16ns -> 65__000_000 Hz
    const SPI_BAUD: u32 = 65_000_000;

    // SPI bus with 8 bits per data frame
    let spi = Spi::<_, _, _, 8u8>::new(pac.SPI0, (lcd_mosi, lcd_sclk));
    // init SPI bus in master mode
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        SPI_BAUD.Hz(),
        MODE_3,
    );

    // create SPI display interface
    let _di = SPIInterface::new(spi, lcd_dc, lcd_cs);
    // create display driver
    //let mut lcd = ST7789::new(di, None, Some(lcd_bl), 320, 240);

    // delay provider for the display driver
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // initialize the display driver
    //lcd.init(&mut delay).unwrap();
    //lcd.set_orientation(Orientation::Landscape).unwrap();

    loop {
        pico_led.set_high().unwrap();
        delay.delay_ms(2000);
        pico_led.set_low().unwrap();
        delay.delay_ms(2000);
    }
}
