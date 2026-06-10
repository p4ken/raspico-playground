use core::convert::Infallible;

use rp_pico::hal::gpio::{DynPinId, FunctionSio, Pin, PullDown, SioOutput};

type DigitalOut = Pin<DynPinId, FunctionSio<SioOutput>, PullDown>;

pub struct Driver {
    interface: Interface,
}

pub struct Interface {
    se: DigitalOut,
    abb: DigitalOut,
    a4: DigitalOut,
    a3: DigitalOut,
    a2: DigitalOut,
    a1: DigitalOut,
    a0: DigitalOut,
    dg: DigitalOut,
    clk: DigitalOut,
    we: DigitalOut,
    dr: DigitalOut,
    ale: DigitalOut,
    led: DigitalOut,
}

impl Interface {
    fn new() -> Self {
        let pins = crate::Pico::new().unwrap().pins;

        let _: &Pin<rp_pico::hal::gpio::bank0::Gpio1, rp_pico::hal::gpio::FunctionNull, PullDown> =
            &pins.gpio1;
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
        todo!()
    }

    fn tick(&self) -> Result<(), Infallible> {
        Ok(())
    }
}
