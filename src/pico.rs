use core::fmt::Debug;

use rp_pico::hal;

pub struct Pico {
    pub pins: rp_pico::Pins,
    pub pwms: hal::pwm::Slices,
    pub timer: hal::Timer,
}
impl Pico {
    pub fn new() -> Result<Self, InitError> {
        let mut pac = hal::pac::Peripherals::take().ok_or(InitError::Singlton)?;
        let mut watchdog = hal::watchdog::Watchdog::new(pac.WATCHDOG);

        // Configure the clocks
        let clocks = hal::clocks::init_clocks_and_plls(
            rp_pico::XOSC_CRYSTAL_FREQ,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .map_err(|_| InitError::Clock)?;
        let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

        // The single-cycle I/O block controls our GPIO pins
        let sio = hal::Sio::new(pac.SIO);

        // Set the pins to their default state
        let pins = rp_pico::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        // Init PWMs
        let mut pwms = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);
        Self::init_pwm(&mut pwms.pwm0);
        Self::init_pwm(&mut pwms.pwm1);
        Self::init_pwm(&mut pwms.pwm2);
        Self::init_pwm(&mut pwms.pwm3);
        Self::init_pwm(&mut pwms.pwm4);
        Self::init_pwm(&mut pwms.pwm5);
        Self::init_pwm(&mut pwms.pwm6);
        Self::init_pwm(&mut pwms.pwm7);

        Ok(Self { pins, pwms, timer })
    }

    fn init_pwm<I: hal::pwm::SliceId, M: hal::pwm::ValidSliceMode<I>>(
        pwm: &mut hal::pwm::Slice<I, M>,
    ) {
        pwm.set_ph_correct();
        pwm.enable();
    }
}

pub enum InitError {
    Singlton,
    Clock,
}
impl Debug for InitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use InitError::*;
        match self {
            Singlton => f.write_str("singlton violation"),
            Clock => f.write_str("failed to init clock"),
        }
    }
}
