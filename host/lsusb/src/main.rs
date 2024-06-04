use anyhow::{anyhow, Result};
use futures_lite::future::block_on;
use log::info;
use nusb::transfer::{ControlIn, ControlOut, ControlType, Recipient};

const VENDOR_ID: u16 = 0x4242;
const PRODUCT_ID: u16 = 0x4242;

fn main() -> Result<()> {
    env_logger::init();

    let dev_info = nusb::list_devices()?
        .find(|info| info.vendor_id() == VENDOR_ID && info.product_id() == PRODUCT_ID)
        .ok_or(anyhow!("Device not found"))?;

    info!("Device: {}", dev_info.product_string().unwrap());
    info!("Serial: {}", dev_info.serial_number().unwrap());

    // open device so we can interact with and claim interfaces
    let dev = dev_info.open()?;
    let iface = dev.claim_interface(0)?;

    // Send "hello world" to device
    let res = block_on(iface.control_out(ControlOut {
        control_type: ControlType::Vendor,
        recipient: Recipient::Interface,
        request: 100,
        value: 200,
        index: 0,
        data: b"hello world",
    }));

    info!("{res:?}");

    // Receive "hello" from device
    let res = block_on(iface.control_in(ControlIn {
        control_type: ControlType::Vendor,
        recipient: Recipient::Interface,
        request: 101,
        value: 201,
        index: 0,
        length: 5,
    }));

    info!("{res:?}");

    Ok(())
}
