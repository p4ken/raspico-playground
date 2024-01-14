//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};
use panic_probe as _;

use crate::pico::PicoW;

mod pico;

#[rp2040_hal::entry]
fn main() -> ! {
    info!("Program start");
    let mut pico = PicoW::new();

    // This is the correct pin on the Raspberry Pico board. On other boards, even if they have an
    // on-board LED, it might need to be changed.
    // Notably, on the Pico W, the LED is not connected to any of the RP2040 GPIOs but to the cyw43 module instead. If you have
    // a Pico W and want to toggle a LED with a simple GPIO output pin, you can connect an external
    // LED to one of the GPIO pins, and reference that pin here.
    let mut led_pin = pico.pins.gpio16.into_push_pull_output();

    loop {
        led_pin.set_high().unwrap();
        pico.timer.delay_ms(1000);
        led_pin.set_low().unwrap();
        pico.timer.delay_ms(500);
    }
}
