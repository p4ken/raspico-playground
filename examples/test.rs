#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use p4pico::{ld61, Pico};
use panic_probe as _;

#[p4pico::entry]
fn main() -> ! {
    info!("Program start");
    let pico = Pico::new().unwrap();
    let mut light = ld61::Light::new();
    light.select_local();

    light.stations_main();
    *light.stations_izumino() = [1; 7];
    light.toggle_stations();
    assert_eq!(*light.stations_main(), [1; 18]);
    assert_eq!(*light.stations_izumino(), [0; 7]);

    pico.pins.led.into_push_pull_output().set_high().unwrap();
    info!("All tests passed.");
    loop {}
}
