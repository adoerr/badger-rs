#![no_std]
#![no_main]

#[allow(unused_imports)]
use panic_probe as _;
use rp_pico::entry;

#[entry]
fn main() -> ! {
    loop {}
}
