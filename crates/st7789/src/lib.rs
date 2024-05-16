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
    di: DI,
    // backlight pin
    bl: BL,
    // current orientation
    orientation: Orientation,
}

impl<DI, BL> ST7789<DI, BL>
where
    DI: WriteOnlyDataCommand,
    BL: OutputPin,
{
    /// Create a new ST7789 driver
    pub fn new(di: DI, bl: BL) -> Self {
        Self {
            di,
            bl,
            orientation: Orientation::Portrait,
        }
    }

    /// Set the display orientation
    pub fn orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    /// Initialize the display
    pub fn init(&mut self, delay: &mut Delay) {
        // backlight off
        let _ = self.bl.set_low().is_ok();
        delay.delay_ms(10);
        // backlight on
        let _ = self.bl.set_high().is_ok();
        // software reset
        self.write_command(Instruction::SWRESET);
        delay.delay_ms(150);
        // sleep out
        self.write_command(Instruction::SLPOUT);
        delay.delay_ms(10);
        // display inversion off
        self.write_command(Instruction::INVOFF);
        // pixel and color format
        self.write_command(Instruction::COLMOD);
        // 65K colors and 16 bit/pixel
        self.write_data(&[0b0101_0101]);
        // normal display mode on (no partial mode)
        self.write_command(Instruction::NORON);
        delay.delay_ms(10);
        // display on
        self.write_command(Instruction::DISPON);
        delay.delay_ms(10);
    }

    fn write_command(&mut self, command: Instruction) {
        self.di
            .send_commands(U8Iter(&mut once(command as u8)))
            .unwrap();
    }

    fn write_data(&mut self, data: &[u8]) {
        self.di
            .send_data(U8Iter(&mut data.iter().cloned()))
            .unwrap()
    }
}
