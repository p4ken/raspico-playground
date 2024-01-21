#![no_std]
#![no_main]

use core::convert::Infallible;

use defmt::info;
use defmt_rtt as _;
use embedded_hal::{blocking::delay::DelayUs, PwmPin};
use panic_probe as _;

use crate::pico::PicoW;

mod pico;

#[pico::entry]
fn main() -> ! {
    info!("Program start");
    let pico = PicoW::new().unwrap();
    blink(pico).unwrap();
    loop {}
}

fn blink(mut pico: PicoW) -> Result<(), Infallible> {
    let _button = pico.pins.gpio15.into_pull_up_input();
    let mut led_green = pico.pwms.pwm0.channel_a;
    led_green.output_to(pico.pins.gpio16);

    let mut led_red = pico.pwms.pwm1.channel_a;
    led_red.output_to(pico.pins.gpio18);

    // wifi
    let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 230321) };
    let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };
    let pwr = pico
        .pins
        .gpio23
        .into_push_pull_output_in_state(rp_pico::hal::pio::PinState::Low);
    let cs = pico
        .pins
        .gpio23
        .into_push_pull_output_in_state(rp_pico::hal::pio::PinState::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    async move {
        let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    };

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
            pico.timer.delay_us(PHASE_USEC * 2 / MAX_RED_DUTY as i32);
        }
        for i in (0..MAX_GREEN_DUTY).rev() {
            led_green.set_duty(i);
            pico.timer.delay_us(PHASE_USEC / MAX_GREEN_DUTY as i32);
        }
        for i in (0..MAX_RED_DUTY).rev() {
            led_red.set_duty(i);
            pico.timer.delay_us(PHASE_USEC / 2 / MAX_RED_DUTY as i32);
        }
    }
}
