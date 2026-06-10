use core::convert::Infallible;

use rp_pico::hal::gpio::{
    DynPinId, FunctionNull, FunctionSio, Pin, PullDown, SioOutput, ValidFunction,
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
        se: Pin<impl ValidFunction<FunctionSio<SioOutput>>, FunctionNull, PullDown>,
        abb: Pin<impl ValidFunction<FunctionSio<SioOutput>>, FunctionNull, PullDown>,
        a4: Pin<impl ValidFunction<FunctionSio<SioOutput>>, FunctionNull, PullDown>,
        a3: Pin<impl ValidFunction<FunctionSio<SioOutput>>, FunctionNull, PullDown>,
        a2: Pin<impl ValidFunction<FunctionSio<SioOutput>>, FunctionNull, PullDown>,
        a1: Pin<impl ValidFunction<FunctionSio<SioOutput>>, FunctionNull, PullDown>,
        a0: Pin<impl ValidFunction<FunctionSio<SioOutput>>, FunctionNull, PullDown>,
        dg: Pin<impl ValidFunction<FunctionSio<SioOutput>>, FunctionNull, PullDown>,
        clk: Pin<impl ValidFunction<FunctionSio<SioOutput>>, FunctionNull, PullDown>,
        we: Pin<impl ValidFunction<FunctionSio<SioOutput>>, FunctionNull, PullDown>,
        dr: Pin<impl ValidFunction<FunctionSio<SioOutput>>, FunctionNull, PullDown>,
        ale: Pin<impl ValidFunction<FunctionSio<SioOutput>>, FunctionNull, PullDown>,
    ) -> Self {
        Self {
            se: se.into_push_pull_output().into_dyn_pin(),
            abb: abb.into_push_pull_output().into_dyn_pin(),
            a4: a4.into_push_pull_output().into_dyn_pin(),
            a3: a3.into_push_pull_output().into_dyn_pin(),
            a2: a2.into_push_pull_output().into_dyn_pin(),
            a1: a1.into_push_pull_output().into_dyn_pin(),
            a0: a0.into_push_pull_output().into_dyn_pin(),
            /* vss: gnd */
            dg: dg.into_push_pull_output().into_dyn_pin(),
            clk: clk.into_push_pull_output().into_dyn_pin(),
            we: we.into_push_pull_output().into_dyn_pin(),
            dr: dr.into_push_pull_output().into_dyn_pin(),
            ale: ale.into_push_pull_output().into_dyn_pin(),
        }
    }

    pub fn tick(&mut self) -> Result<(), Infallible> {
        Ok(())
    }
}
