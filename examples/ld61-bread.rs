#![no_std]
#![no_main]

use core::convert::Infallible;

use defmt::info;
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use panic_probe as _;

use p4pico::Pico;

#[p4pico::entry]
fn main() -> ! {
    info!("Program start");
    let pico = Pico::new().unwrap();
    blink(pico).unwrap();
    unreachable!()
}

fn blink(pico: Pico) -> Result<(), Infallible> {
    pico.pins.led.into_push_pull_output().set_high()?;

    let gray = pico.pins.gpio14;
    let brown = pico.pins.gpio13;
    let orange = pico.pins.gpio12;
    let white = pico.pins.gpio11;
    let blue = pico.pins.gpio10;

    let mut enable = gray.into_push_pull_output();
    enable.set_low()?;
    let mut latch_speed = brown.into_push_pull_output();
    latch_speed.set_high()?;
    let mut serial = orange.into_push_pull_output();
    let mut clock = white.into_push_pull_output();
    clock.set_low()?;
    let mut latch_stations = blue.into_push_pull_output();
    latch_stations.set_high()?;

    const DATA: &str = "1001010001001001001001001XXXXXXXYYYY0010";
    for c in DATA.chars() {
        match c {
            '0' => serial.set_low()?,
            _ => serial.set_high()?,
        };
        clock.set_high()?;
        clock.set_low()?;
    }

    loop {}
}
