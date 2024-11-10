#![no_std]
#![no_main]

use core::convert::Infallible;

use defmt::info;
use defmt_rtt as _;
use embedded_hal::{blocking::delay::DelayUs, PwmPin};
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
    let _button = pico.pins.gpio16.into_pull_up_input();

    let mut led_green = pico.pwms.pwm7.channel_b;
    led_green.output_to(pico.pins.gpio15);

    let mut led_red = pico.pwms.pwm7.channel_a;
    led_red.output_to(pico.pins.gpio14);

    loop {
        const PHASE_USEC: i32 = 5_000_000;
        const MAX_GREEN_DUTY: u16 = u16::MAX;
        const MAX_RED_DUTY: u16 = 10000;

        for i in 0..MAX_GREEN_DUTY {
            led_green.set_duty(i);
            pico.timer.delay_us(PHASE_USEC / MAX_GREEN_DUTY as i32);
        }
        for i in 0..MAX_RED_DUTY {
            led_red.set_duty(i);
            pico.timer.delay_us(PHASE_USEC / MAX_RED_DUTY as i32);
        }
        for i in (0..MAX_GREEN_DUTY).rev() {
            led_green.set_duty(i);
            pico.timer.delay_us(PHASE_USEC / MAX_GREEN_DUTY as i32);
        }
        for i in (0..MAX_RED_DUTY).rev() {
            led_red.set_duty(i);
            pico.timer.delay_us(PHASE_USEC / MAX_RED_DUTY as i32);
        }
    }
}
