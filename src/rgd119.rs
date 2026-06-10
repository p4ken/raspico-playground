use core::convert::Infallible;

use rp_pico::hal::gpio::{
    DynPinId, Function, FunctionSio, Pin, PullDown, PullType, SioOutput, ValidFunction,
};

type Rgd119Pin = Pin<DynPinId, FunctionSio<SioOutput>, PullDown>;

pub struct Rgd119 {
    pub se: Rgd119Pin,
    pub abb: Rgd119Pin,
    pub a4: Rgd119Pin,
    pub a3: Rgd119Pin,
    pub a2: Rgd119Pin,
    pub a1: Rgd119Pin,
    pub a0: Rgd119Pin,
    pub dg: Rgd119Pin,
    pub clk: Rgd119Pin,
    pub we: Rgd119Pin,
    pub dr: Rgd119Pin,
    pub ale: Rgd119Pin,
}

impl Rgd119 {
    pub fn new(
        se: Pin<impl ValidFunction<FunctionSio<SioOutput>>, impl Function, impl PullType>,
        abb: Pin<impl ValidFunction<FunctionSio<SioOutput>>, impl Function, impl PullType>,
        a4: Pin<impl ValidFunction<FunctionSio<SioOutput>>, impl Function, impl PullType>,
        a3: Pin<impl ValidFunction<FunctionSio<SioOutput>>, impl Function, impl PullType>,
        a2: Pin<impl ValidFunction<FunctionSio<SioOutput>>, impl Function, impl PullType>,
        a1: Pin<impl ValidFunction<FunctionSio<SioOutput>>, impl Function, impl PullType>,
        a0: Pin<impl ValidFunction<FunctionSio<SioOutput>>, impl Function, impl PullType>,
        dg: Pin<impl ValidFunction<FunctionSio<SioOutput>>, impl Function, impl PullType>,
        clk: Pin<impl ValidFunction<FunctionSio<SioOutput>>, impl Function, impl PullType>,
        we: Pin<impl ValidFunction<FunctionSio<SioOutput>>, impl Function, impl PullType>,
        dr: Pin<impl ValidFunction<FunctionSio<SioOutput>>, impl Function, impl PullType>,
        ale: Pin<impl ValidFunction<FunctionSio<SioOutput>>, impl Function, impl PullType>,
    ) -> Self {
        Self {
            se: se.into_push_pull_output().into_dyn_pin().into_pull_type(),
            abb: abb.into_push_pull_output().into_dyn_pin().into_pull_type(),
            a4: a4.into_push_pull_output().into_dyn_pin().into_pull_type(),
            a3: a3.into_push_pull_output().into_dyn_pin().into_pull_type(),
            a2: a2.into_push_pull_output().into_dyn_pin().into_pull_type(),
            a1: a1.into_push_pull_output().into_dyn_pin().into_pull_type(),
            a0: a0.into_push_pull_output().into_dyn_pin().into_pull_type(),
            /* vss: gnd */
            dg: dg.into_push_pull_output().into_dyn_pin().into_pull_type(),
            clk: clk.into_push_pull_output().into_dyn_pin().into_pull_type(),
            we: we.into_push_pull_output().into_dyn_pin().into_pull_type(),
            dr: dr.into_push_pull_output().into_dyn_pin().into_pull_type(),
            ale: ale.into_push_pull_output().into_dyn_pin().into_pull_type(),
        }
    }

    pub fn tick(&mut self) -> Result<(), Infallible> {
        Ok(())
    }
}
