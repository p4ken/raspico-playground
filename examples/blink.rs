#![no_std]
#![no_main]

use core::convert::Infallible;

use defmt::info;
use defmt_rtt as _;
use embedded_hal::{blocking::delay::DelayUs, digital::v2::ToggleableOutputPin};
use panic_probe as _;

use p4pico::Pico;

#[p4pico::entry]
fn main() -> ! {
    info!("Program start");
    let pico = Pico::new().unwrap();
    blink(pico).unwrap();
    loop {}
}

fn blink(mut pico: Pico) -> Result<(), Infallible> {
    let mut led = pico.pins.gpio25.into_push_pull_output();
    loop {
        led.toggle()?;
        pico.timer.delay_us(1 * 1000 * 1000);
    }
}
