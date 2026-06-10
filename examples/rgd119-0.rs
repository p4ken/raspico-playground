#![no_std]
#![no_main]

use core::convert::Infallible;
use core::hint::spin_loop;

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use panic_probe as _;

use p4pico::{rgd119::Rgd119, Font, Pico};

const JISKAN24: Font = Font::new(include_bytes!("../rom/jiskan24.bin"));

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
    ab: char,
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
    let glyph = &JISKAN24['驚'];
    for h in 0..24 {
        for i in 0..24 {
            // JISKAN24 is bit-packed: 24*24 = 576 bits = 72 bytes.
            // Compute linear bit index and extract the bit from the byte array.
            // Rotate 90 degrees clockwise: map display (h,i) to original
            // coordinate (orig_row, orig_col) = (23 - i, h).
            let idx = (23 - i) * 24 + h;
            let byte = glyph[idx / 8];
            let bit = (byte >> (7 - (idx % 8))) & 1;

            if bit == 1 {
                set_pin(red, false);
                set_pin(green, true);
            } else {
                set_pin(red, false);
                set_pin(green, false);
            }

            set_pin(clock, true);
            set_pin(clock, false);
        }

        set_address(address0, address1, address2, address3, address4, h);

        set_pin(latch, true);
        set_pin(we, true);
        set_pin(we, false);
        set_pin(latch, false);
    }
}

fn drive(pico: Pico) -> Result<(), Infallible> {
    let pins = pico.pins;

    let mut rgd119 = Rgd119::new(
        pins.gpio1,
        pins.gpio2,
        pins.gpio3,
        pins.gpio4,
        pins.gpio5,
        pins.gpio6,
        pins.gpio7,
        pins.gpio9,
        pins.gpio10,
        pins.gpio11,
        pins.gpio12,
        pins.gpio13,
    );
    let mut led = pins.led.into_push_pull_output();

    set_pin(&mut rgd119.se, true);
    set_pin(&mut rgd119.abb, true);
    set_pin(&mut rgd119.abb, false);
    set_pin(&mut rgd119.clk, false);
    set_pin(&mut rgd119.ale, false);
    set_pin(&mut rgd119.we, false);

    loop {
        set_pin(&mut rgd119.abb, true);
        set_pin(&mut led, true);
        led_display(
            &mut rgd119.dr,
            &mut rgd119.dg,
            &mut rgd119.ale,
            &mut rgd119.clk,
            &mut rgd119.we,
            &mut rgd119.a0,
            &mut rgd119.a1,
            &mut rgd119.a2,
            &mut rgd119.a3,
            &mut rgd119.a4,
            'a',
        );

        set_pin(&mut rgd119.abb, false);
        set_pin(&mut led, false);
        led_display(
            &mut rgd119.dr,
            &mut rgd119.dg,
            &mut rgd119.ale,
            &mut rgd119.clk,
            &mut rgd119.we,
            &mut rgd119.a0,
            &mut rgd119.a1,
            &mut rgd119.a2,
            &mut rgd119.a3,
            &mut rgd119.a4,
            'b',
        );
    }
}

#[entry]
fn main() -> ! {
    info!("Program start");
    let pico = Pico::new().unwrap();
    drive(pico).unwrap();
    loop {}
}
