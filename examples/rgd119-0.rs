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

#[entry]
fn main() -> ! {
    info!("Program start");

    // probe-rs download rom/jiskan24.bin --base-address 0x10040000 --chip rp2040 --binary-format=bin
    let jiskan24 = Font::new(unsafe {
        core::slice::from_raw_parts(
            0x10040000 as *const u8,
            include_bytes!("../rom/jiskan24.bin").len(),
        )
    });

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
        rgd119.draw(&jiskan24['凰'], Color::Green);
    }
}
