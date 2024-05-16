#![no_std]
#![allow(dead_code)]

use core::iter::once;

use cortex_m::delay::Delay;
use display_interface::{DataFormat::U8Iter, WriteOnlyDataCommand};
use embedded_hal::digital::v2::OutputPin;

use crate::types::{Instruction, Orientation};

mod types;

pub const WIDTH: u16 = 320;
pub const HEIGHT: u16 = 240;

/// ST7789 driver for graphic type TFT-LCD
///
/// This driver is customized for the [Pimoroni Pico Display Pack 2.0](https://shop.pimoroni.com/products/pico-display-pack-2-0)
pub struct ST7789<DI, BL>
where
    DI: WriteOnlyDataCommand,
    BL: OutputPin,
{
    // display interface
    display_interface: DI,
    // backlight pin
    backlight: BL,
    // current orientation
    orientation: Orientation,
}

impl<DI, BL> ST7789<DI, BL>
where
    DI: WriteOnlyDataCommand,
    BL: OutputPin,
{
    /// Create a new ST7789 driver
    pub fn new(display_interface: DI, backlight: BL) -> Self {
        Self {
            display_interface,
            backlight,
            orientation: Orientation::Portrait,
        }
    }

    /// Set the display orientation
    pub fn orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    /// Initialize the display
    pub fn init(&mut self, delay: &mut Delay) {
        self.write_command(Instruction::SWRESET);
        delay.delay_ms(150)
    }

    fn write_command(&mut self, command: Instruction) {
        self.display_interface
            .send_commands(U8Iter(&mut once(command as u8)))
            .unwrap();
    }
}
