# 🐿️ squirrel-rs 🦀

<p align="center">
    <img src="./facedancer_logo.png" widht="350" height="150">
</p>

<br>

Experiment with and explore USB device emulation in Rust.

This project is inspired and influenced by the current iteration of [FaceDancer](https://github.com/greatscottgadgets/facedancer)
and the original [GoodFET](https://github.com/travisgoodspeed/goodfet) based FaceDancer.

For more information on the original FaceDancer project,  see
- [Travis Goodspeed's blog post on FaceDancer](http://travisgoodspeed.blogspot.com/2012/07/emulating-usb-devices-with-python.html)
- [The FaceDancer 21, the original supported board](http://goodfet.sourceforge.net/hardware/facedancer21/)
<br>

The basic idea of `squirrel-rs` is to evaluate, if it is possible to achieve FaceDancer's functionality using a 
[Raspberry Pi Pico](https://www.raspberrypi.com/products/raspberry-pi-pico/) board and Rust.

The RPi Pico obviously only has a single USB phy, which is needed to connect to the USB host. For this reason, using the
[Debug Probe](https://www.raspberrypi.com/products/debug-probe/) is **mandatory**. We need both, the USB to SWD bridge as
well as the USB to UART bridge. The SWD bridge is used to flash the RPi Pico and the UART bridge is used to communicate with
the USB device emulation.

<p align="center">
<b>🚧 squirrel-rs is under construction - a hardhat 👷 is recommended beyond this point 🚧</b>
</p>