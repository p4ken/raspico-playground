#![no_std]
#![no_main]

use core::convert::Infallible;
use core::hint::spin_loop;

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use panic_probe as _;

use p4pico::Pico;

pub const LED_BUFFER_KARAS: &[u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
    0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
    0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, //
    0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, //
    0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, //
    0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, //
    0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, //
    1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, //
    1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, //
    0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, //
    0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
    0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
    0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, //
    0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, //
    0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, //
    0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, //
    0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, //
    0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, //
    0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, //
    1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, //
    1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, //
    1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, //
    0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, //
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[inline(always)]
fn set_pin<P>(pin: &mut P, high: bool)
where
    P: OutputPin<Error = Infallible>,
{
    if high {
        let _ = pin.set_high();
    } else {
        let _ = pin.set_low();
    }
}

#[inline(always)]
fn wait_ms(ms: u32) {
    // RP2040 is configured to 125 MHz in pico.rs.
    // 1 ms ≒ 125,000 CPU cycles.
    cortex_m::asm::delay(125_000 * ms);
    spin_loop();
}

#[inline(always)]
fn set_address<A0, A1, A2, A3, A4>(
    a0: &mut A0,
    a1: &mut A1,
    a2: &mut A2,
    a3: &mut A3,
    a4: &mut A4,
    value: usize,
) where
    A0: OutputPin<Error = Infallible>,
    A1: OutputPin<Error = Infallible>,
    A2: OutputPin<Error = Infallible>,
    A3: OutputPin<Error = Infallible>,
    A4: OutputPin<Error = Infallible>,
{
    set_pin(a0, value & 0b00001 != 0);
    set_pin(a1, value & 0b00010 != 0);
    set_pin(a2, value & 0b00100 != 0);
    set_pin(a3, value & 0b01000 != 0);
    set_pin(a4, value & 0b10000 != 0);
}

#[inline(always)]
fn led_display<R, G, L, C, W, A0, A1, A2, A3, A4>(
    red: &mut R,
    green: &mut G,
    latch: &mut L,
    clock: &mut C,
    we: &mut W,
    address0: &mut A0,
    address1: &mut A1,
    address2: &mut A2,
    address3: &mut A3,
    address4: &mut A4,
) where
    R: OutputPin<Error = Infallible>,
    G: OutputPin<Error = Infallible>,
    L: OutputPin<Error = Infallible>,
    C: OutputPin<Error = Infallible>,
    W: OutputPin<Error = Infallible>,
    A0: OutputPin<Error = Infallible>,
    A1: OutputPin<Error = Infallible>,
    A2: OutputPin<Error = Infallible>,
    A3: OutputPin<Error = Infallible>,
    A4: OutputPin<Error = Infallible>,
{
    for h in 0..24 {
        for i in 0..24 {
            // JISKAN24 is bit-packed: 24*24 = 576 bits = 72 bytes.
            // Compute linear bit index and extract the bit from the byte array.
            let idx = h * 24 + i;
            let byte = p4pico::JISKAN24[idx / 8];
            let bit = (byte >> (7 - (idx % 8))) & 1;

            if bit == 1 {
                set_pin(red, false);
                set_pin(green, true);
            } else {
                set_pin(red, false);
                set_pin(green, false);
            }

            wait_ms(1);
            set_pin(clock, true);
            wait_ms(1);
            set_pin(clock, false);
            wait_ms(1);
        }

        wait_ms(1);
        set_address(address0, address1, address2, address3, address4, h);

        wait_ms(1);
        set_pin(latch, true);
        wait_ms(1);
        set_pin(we, true);
        wait_ms(1);
        set_pin(we, false);
        wait_ms(1);
        set_pin(latch, false);
        wait_ms(1);
    }
}

fn drive(pico: Pico) -> Result<(), Infallible> {
    let pins = pico.pins;

    let mut se = pins.gpio1.into_push_pull_output();
    let mut abb = pins.gpio2.into_push_pull_output();
    let mut a4 = pins.gpio3.into_push_pull_output();
    let mut a3 = pins.gpio4.into_push_pull_output();
    let mut a2 = pins.gpio5.into_push_pull_output();
    let mut a1 = pins.gpio6.into_push_pull_output();
    let mut a0 = pins.gpio7.into_push_pull_output();
    let _vss = (); // gnd
    let mut dg = pins.gpio9.into_push_pull_output();
    let mut clk = pins.gpio10.into_push_pull_output();
    let mut we = pins.gpio11.into_push_pull_output();
    let mut dr = pins.gpio12.into_push_pull_output();
    let mut ale = pins.gpio13.into_push_pull_output();
    let mut led = pins.led.into_push_pull_output();

    set_pin(&mut se, true);
    set_pin(&mut abb, true);
    wait_ms(1);
    set_pin(&mut abb, false);
    set_pin(&mut clk, false);
    set_pin(&mut ale, false);
    set_pin(&mut we, false);

    wait_ms(1);

    loop {
        set_pin(&mut abb, true);
        set_pin(&mut led, true);
        led_display(
            &mut dr, &mut dg, &mut ale, &mut clk, &mut we, &mut a0, &mut a1, &mut a2, &mut a3,
            &mut a4,
        );

        set_pin(&mut abb, false);
        set_pin(&mut led, false);
        led_display(
            &mut dr, &mut dg, &mut ale, &mut clk, &mut we, &mut a0, &mut a1, &mut a2, &mut a3,
            &mut a4,
        );

        wait_ms(200);
    }
}

#[entry]
fn main() -> ! {
    info!("Program start");
    let pico = Pico::new().unwrap();
    drive(pico).unwrap();
    loop {}
}
