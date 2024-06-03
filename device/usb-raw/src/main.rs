#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    peripherals::USB,
    usb::{Driver, InterruptHandler},
};
use embassy_usb::{Builder, Config};
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

const USB_VENDOR_ID: u16 = 0x4242;
const USB_PRODUCT_ID: u16 = 0x4242;

bind_interrupts!(struct Irqs{
     USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("init raw USB device");

    let p = embassy_rp::init(Default::default());
    // USB driver
    let driver = Driver::new(p.USB, Irqs);
    // USB config
    let config = Config::new(USB_VENDOR_ID, USB_PRODUCT_ID);
    // USB device builder
    let builder = Builder::new(driver, config, &mut [], &mut [], &mut [], &mut []);

    let mut usb = builder.build();
    usb.run().await;
}
