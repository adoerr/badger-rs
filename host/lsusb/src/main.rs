use anyhow::{anyhow, Result};
use futures_lite::future::block_on;
use log::info;
use nusb::transfer::{ControlIn, ControlOut, ControlType, Recipient};

const USB_VENDOR_ID: u16 = 0x4242;
const USB_PRODUCT_ID: u16 = 0x4242;

fn main() -> Result<()> {
    env_logger::init();

    // check the available devices and pick the GreatFET One
    let device_info = nusb::list_devices()?
        .find(|info| info.vendor_id() == USB_VENDOR_ID && info.product_id() == USB_PRODUCT_ID)
        .ok_or(anyhow!("Device not found"))?;

    info!("Device: {}", device_info.product_string().unwrap());
    info!("Serial: {}", device_info.serial_number().unwrap());

    let device = device_info.open()?;
    let iface = device.claim_interface(0)?;

    // Send "hello world" to device
    let result = block_on(iface.control_out(ControlOut {
        control_type: ControlType::Vendor,
        recipient: Recipient::Interface,
        request: 100,
        value: 200,
        index: 0,
        data: b"hello world",
    }));

    info!("{result:?}");

    // Receive "hello" from device
    let result = block_on(iface.control_in(ControlIn {
        control_type: ControlType::Vendor,
        recipient: Recipient::Interface,
        request: 101,
        value: 201,
        index: 0,
        length: 5,
    }));

    info!("{result:?}");

    Ok(())
}
