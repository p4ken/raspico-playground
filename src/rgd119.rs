use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use rp_pico::hal::gpio::{
    DynPinId, Function, FunctionSio, Pin, PullDown, PullType, SioOutput, ValidFunction,
};

pub struct DigitalOut(pub Pin<DynPinId, FunctionSio<SioOutput>, PullDown>);

impl DigitalOut {
    fn on(&mut self) {
        self.set(true)
    }

    fn off(&mut self) {
        self.set(false)
    }

    fn set(&mut self, on: bool) {
        self.0.set_state(on.into()).unwrap()
    }
}

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

pub struct CN1 {
    pub se: DigitalOut,
    pub abb: DigitalOut,
    pub a4: DigitalOut,
    pub a3: DigitalOut,
    pub a2: DigitalOut,
    pub a1: DigitalOut,
    pub a0: DigitalOut,
    pub dg: DigitalOut,
    pub clk: DigitalOut,
    pub we: [DigitalOut; 2],
    pub dr: DigitalOut,
    pub ale: DigitalOut,
}

impl CN1 {
    pub fn into_host(self) -> Host {
        Host::new(self)
    }
}

pub struct Host {
    abb: DigitalOut,
    a4: DigitalOut,
    a3: DigitalOut,
    a2: DigitalOut,
    a1: DigitalOut,
    a0: DigitalOut,
    dg: DigitalOut,
    clk: DigitalOut,
    we: [DigitalOut; 2],
    dr: DigitalOut,
    ale: DigitalOut,
}

impl Host {
    fn new(mut cn1: CN1) -> Self {
        cn1.se.on();
        cn1.abb.on();
        cn1.abb.off();
        cn1.clk.off();
        cn1.ale.off();
        cn1.we[0].off();
        cn1.we[1].off();

        Self {
            abb: cn1.abb,
            a4: cn1.a4,
            a3: cn1.a3,
            a2: cn1.a2,
            a1: cn1.a1,
            a0: cn1.a0,
            dg: cn1.dg,
            clk: cn1.clk,
            we: cn1.we,
            dr: cn1.dr,
            ale: cn1.ale,
        }
    }

    pub fn draw(&mut self, glyphs: [&[u8]; 2], color: Color) {
        self.abb.0.toggle();

        for g in 0..2 {
            let glyph = glyphs[g];
            for h in 0..24 {
                for i in 0..24 {
                    let idx = (23 - i) * 24 + h;
                    let byte = glyph[idx / 8];
                    let bit = (byte >> (7 - (idx % 8))) & 1;

                    self.dr.set(bit == 1 && color.emits_red());
                    self.dg.set(bit == 1 && color.emits_green());

                    self.clk.on();
                    self.clk.off();
                }

                self.a0.set(h & 0b00001 != 0);
                self.a1.set(h & 0b00010 != 0);
                self.a2.set(h & 0b00100 != 0);
                self.a3.set(h & 0b01000 != 0);
                self.a4.set(h & 0b10000 != 0);

                self.ale.on();
                self.we[g].on();
                self.we[g].off();
                self.ale.off();
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Green,
    Red,
    Orange,
}

impl Color {
    fn emits_red(self) -> bool {
        matches!(self, Color::Red | Color::Orange)
    }

    fn emits_green(self) -> bool {
        matches!(self, Color::Green | Color::Orange)
    }
}
