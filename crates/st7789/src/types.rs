//!
//! ST7789 display driver type definitions
//!

/// Instruction set used to communicate with the display.
///
/// Note that some instructions do have multiple encodings.
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Instruction {
    /// No operation
    NOP = 0x00,
    /// Software reset
    SWRESET = 0x01,
    /// Read display ID
    RDDID = 0x04,
    /// Read display status
    RDDST = 0x09,
    /// Sleep in
    SLPIN = 0x10,
    /// Sleep out
    SLPOUT = 0x11,
    /// Partial mode on
    PTLON = 0x12,
    /// Partial mode off (Normal)
    NORON = 0x13,
    /// Display inversion off
    INVOFF = 0x20,
    /// Display inversion on
    INVON = 0x21,
    /// Display off
    DISPOFF = 0x28,
    /// Display on
    DISPON = 0x29,
    /// Column address set
    CASET = 0x2A,
    /// Row address set
    RASET = 0x2B,
    /// Write to RAM
    RAMWR = 0x2C,
    /// Read from RAM
    RAMRD = 0x2E,
    /// Partial address
    PTLAR = 0x30,
    /// Vertical scrolling definition
    VSCRDER = 0x33,
    /// Tearing effect line off
    TEOFF = 0x34,
    /// Tearing effect line on
    TEON = 0x35,
    /// Memory data access control
    MADCTL = 0x36,
    /// Vertical scrolling start address
    VSCRSADD = 0x37,
    /// Interface pixel and color format
    COLMOD = 0x3A,
    /// Voltage common offset set
    VCMOFSET = 0xC5,
}

/// Display orientation.
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Orientation {
    Portrait = 0b0000_0000,
    Landscape = 0b0110_0000,
    PortraitSwapped = 0b1100_0000,
    LandscapeSwapped = 0b1010_0000,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Landscape
    }
}

/// Display backlight state.
#[derive(Copy, Clone, Debug)]
pub enum BacklightState {
    On,
    Off,
}
