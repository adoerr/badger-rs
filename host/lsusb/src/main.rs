use anyhow::{anyhow, Result};
use log::info;

const USB_VENDOR_ID: u16 = 0x4242;
const USB0_PRODUCT_ID: u16 = 0x4242;

fn main() -> Result<()> {
    env_logger::init();

    // check the available devices and pick the GreatFET One
    let device_info = nusb::list_devices()?
        .find(|info| info.vendor_id() == USB_VENDOR_ID && info.product_id() == USB0_PRODUCT_ID)
        .ok_or(anyhow!("Device not found"))?;

    info!("Device: {}", device_info.product_string().unwrap());
    info!("Serial: {}", device_info.serial_number().unwrap());

    // open the device
    let device = device_info.open()?;
    // the GreatFET One has only one configuration which should be active
    let _config = device.active_configuration()?;
    // claim the interface
    let _iface = device.claim_interface(1)?;

    info!("Interface claimed");

    Ok(())
}
