#![no_std]
#![no_main]

use core::convert::Infallible;

use defmt::info;
use defmt_rtt as _;
use embedded_hal::{delay::DelayNs, digital::StatefulOutputPin};
use panic_probe as _;

use p4pico::{
    ld61::{Driver, Light},
    Pico,
};

#[p4pico::entry]
fn main() -> ! {
    info!("Program start");
    let pico = Pico::new().unwrap();
    blink(pico).unwrap();
    unreachable!()
}

fn blink(mut pico: Pico) -> Result<(), Infallible> {
    let mut led = pico.pins.led.into_push_pull_output();

    let mut driver = Driver {
        xlat: pico.pins.gpio17.into_push_pull_output(),
        clk: pico.pins.gpio18.into_push_pull_output(),
        ser: pico.pins.gpio19.into_push_pull_output(),
        ylat: pico.pins.gpio20.into_push_pull_output(),
        en: pico.pins.gpio21.into_push_pull_output(),
    };

    let mut light = Light::new();
    light.select_local();
    *light.stations_izumino() = [0, 1, 0, 1, 0, 1, 0];
    *light.stations_main() = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];

    loop {
        driver.display(light)?;
        light.toggle_stations();
        led.toggle()?;
        pico.timer.delay_ms(1500);
    }
}
