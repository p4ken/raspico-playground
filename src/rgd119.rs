use core::convert::Infallible;

use rp_pico::hal::gpio::{
    DynPinId, Function, FunctionSio, Pin, PullDown, PullType, SioOutput, ValidFunction,
};

pub struct DigitalOut(pub Pin<DynPinId, FunctionSio<SioOutput>, PullDown>);

impl<I, F, P> From<Pin<I, F, P>> for DigitalOut
where
    I: ValidFunction<FunctionSio<SioOutput>>,
    F: Function,
    P: PullType,
{
    fn from(source: Pin<I, F, P>) -> Self {
        Self(
            source
                .into_push_pull_output()
                .into_dyn_pin()
                .into_pull_type(),
        )
    }
}

pub struct Rgd119 {
    pub se: DigitalOut,
    pub abb: DigitalOut,
    pub a4: DigitalOut,
    pub a3: DigitalOut,
    pub a2: DigitalOut,
    pub a1: DigitalOut,
    pub a0: DigitalOut,
    pub dg: DigitalOut,
    pub clk: DigitalOut,
    pub we: DigitalOut,
    pub dr: DigitalOut,
    pub ale: DigitalOut,
}

impl Rgd119 {
    pub fn new(
        se: impl Into<DigitalOut>,
        abb: impl Into<DigitalOut>,
        a4: impl Into<DigitalOut>,
        a3: impl Into<DigitalOut>,
        a2: impl Into<DigitalOut>,
        a1: impl Into<DigitalOut>,
        a0: impl Into<DigitalOut>,
        dg: impl Into<DigitalOut>,
        clk: impl Into<DigitalOut>,
        we: impl Into<DigitalOut>,
        dr: impl Into<DigitalOut>,
        ale: impl Into<DigitalOut>,
    ) -> Self {
        Self {
            se: se.into(),
            abb: abb.into(),
            a4: a4.into(),
            a3: a3.into(),
            a2: a2.into(),
            a1: a1.into(),
            a0: a0.into(),
            dg: dg.into(),
            clk: clk.into(),
            we: we.into(),
            dr: dr.into(),
            ale: ale.into(),
        }
    }

    pub fn tick(&mut self) -> Result<(), Infallible> {
        Ok(())
    }
}
