#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};
use panic_probe as _;

use crate::pico::PicoW;

mod pico;

#[pico::entry]
fn main() -> ! {
    info!("Program start");
    let pico = PicoW::new();
    blink(pico);
}

fn blink(mut pico: PicoW) -> ! {
    let mut led_pin = pico.pins.gpio16.into_push_pull_output();

    loop {
        led_pin.set_high().unwrap();
        pico.timer.delay_ms(1000);
        led_pin.set_low().unwrap();
        pico.timer.delay_ms(500);
    }
}
