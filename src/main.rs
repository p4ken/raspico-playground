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

    let mut led_pin = pico.pins.gpio16.into_push_pull_output();

    loop {
        led_pin.set_high().unwrap();
        pico.timer.delay_ms(1000);
        led_pin.set_low().unwrap();
        pico.timer.delay_ms(500);
    }
}
