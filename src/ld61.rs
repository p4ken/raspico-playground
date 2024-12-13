//! LD61, the LED display controller of Sotetsu 7050 series.
use embedded_hal::digital::OutputPin;

/// Connector to LED driver board.
pub struct Driver<X, C, S, Y, E> {
    pub xlat: X,
    pub clk: C,
    pub ser: S,
    pub ylat: Y,
    pub en: E,
}
impl<X, C, S, Y, E, ER> Driver<X, C, S, Y, E>
where
    X: OutputPin<Error = ER>,
    C: OutputPin<Error = ER>,
    S: OutputPin<Error = ER>,
    Y: OutputPin<Error = ER>,
    E: OutputPin<Error = ER>,
{
    pub fn clear(&mut self) -> Result<(), ER> {
        self.en.set_high()
    }
    pub fn display(&mut self, light: Light) -> Result<(), ER> {
        self.ylat.set_high()?;
        self.clk.set_low()?;
        self.xlat.set_high()?;

        for c in light.0 {
            match c {
                0 => self.ser.set_low()?,
                _ => self.ser.set_high()?,
            };
            self.clk.set_high()?;
            self.clk.set_low()?;
        }
        self.en.set_low()?;
        Ok(())
    }
}

/// Lighting pattern.
#[derive(Clone, Copy)]
pub struct Light([u8; 40]);
impl Light {
    pub fn new() -> Self {
        Self([0; 40])
    }
    pub fn stations(&mut self) -> &mut [u8] {
        &mut self.0[0..25]
    }
    pub fn stations_izumino(&mut self) -> &mut [u8; 7] {
        (&mut self.0[0..7]).try_into().unwrap()
    }
    pub fn stations_main(&mut self) -> &mut [u8; 18] {
        (&mut self.0[7..25]).try_into().unwrap()
    }
    pub fn toggle_stations(&mut self) {
        for x in self.stations() {
            *x = (*x == 0) as u8;
        }
    }
    pub fn select_local(&mut self) {
        self.set_speed(Speed::Local);
    }
    pub fn set_speed(&mut self, speed: Speed) {
        self.0[36] = 0;
        self.0[37] = 0;
        self.0[38] = 0;
        self.0[39] = 0;
        let i = match speed {
            Speed::Express => 36,
            Speed::Rapid => 37,
            Speed::Local => 38,
        };
        self.0[i] = 1;
    }
    // pub fn ebina(mut self, on: bool) -> Self {}
}

pub enum Speed {
    Express,
    Rapid,
    Local,
}
