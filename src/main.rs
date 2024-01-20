#![no_std]
#![no_main]

use core::convert::Infallible;

use defmt::info;
use defmt_rtt as _;
use embedded_hal::{
    blocking::delay::DelayMs,
    digital::v2::{InputPin, OutputPin},
};
use panic_probe as _;

use crate::pico::PicoW;

mod pico;

#[pico::entry]
fn main() -> ! {
    info!("Program start");
    let pico = PicoW::new();
    blink(pico).unwrap();
    loop {}
}

fn blink(mut pico: PicoW) -> Result<(), Infallible> {
    let button = pico.pins.gpio15.into_pull_up_input();
    let mut led_green = pico.pins.gpio16.into_push_pull_output();
    let mut led_red = pico.pins.gpio17.into_push_pull_output();

    loop {
        led_green.set_high()?;
        pico.timer.delay_ms(500);
        if button.is_low()? {
            led_red.set_high()?;
        }
        pico.timer.delay_ms(500);
        led_green.set_low()?;
        pico.timer.delay_ms(500);
        led_red.set_low()?;
        pico.timer.delay_ms(500);
    }
}
