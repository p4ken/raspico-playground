#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use embedded_hal::{delay::DelayNs, digital::StatefulOutputPin};
use panic_probe as _;

use p4pico::{
    rgd119::{Color, Rgd119},
    Font, Pico,
};

const JISKAN24: Font = Font::new(include_bytes!("../rom/jiskan24.bin"));

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pico = Pico::new().unwrap();

    let mut rgd119 = Rgd119::new(
        pico.pins.gpio1,
        pico.pins.gpio2,
        pico.pins.gpio3,
        pico.pins.gpio4,
        pico.pins.gpio5,
        pico.pins.gpio6,
        pico.pins.gpio7,
        pico.pins.gpio9,
        pico.pins.gpio10,
        pico.pins.gpio11,
        pico.pins.gpio12,
        pico.pins.gpio13,
    );
    let mut led = pico.pins.led.into_push_pull_output();

    loop {
        led.toggle();
        pico.timer.delay_ms(500);
        rgd119.draw(&JISKAN24['鷹'], Color::Green);
    }
}
