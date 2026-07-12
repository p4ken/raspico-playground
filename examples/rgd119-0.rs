#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use embedded_hal::{delay::DelayNs, digital::StatefulOutputPin};
use panic_probe as _;

use p4pico::{
    rgd119::{self, Color},
    Font, Pico,
};

#[entry]
fn main() -> ! {
    info!("Program start");

    let mut pico = Pico::new().unwrap();

    let mut rgd119 = rgd119::CN1 {
        se: pico.pins.gpio1.into(),
        abb: pico.pins.gpio2.into(),
        a4: pico.pins.gpio3.into(),
        a3: pico.pins.gpio4.into(),
        a2: pico.pins.gpio5.into(),
        a1: pico.pins.gpio6.into(),
        a0: pico.pins.gpio7.into(),
        dg: pico.pins.gpio9.into(),
        clk: pico.pins.gpio10.into(),
        we: pico.pins.gpio11.into(),
        dr: pico.pins.gpio12.into(),
        ale: pico.pins.gpio13.into(),
    }
    .into_host();
    let mut led = pico.pins.led.into_push_pull_output();

    // probe-rs download rom/jiskan24.bin --base-address 0x10040000 --chip rp2040 --binary-format=bin
    let jiskan24 = Font::new(unsafe {
        core::slice::from_raw_parts(
            0x10040000 as *const u8,
            include_bytes!("../rom/jiskan24.bin").len(),
        )
    });

    loop {
        led.toggle();
        pico.timer.delay_ms(500);
        rgd119.draw(&jiskan24['凰'], Color::Green);
    }
}
