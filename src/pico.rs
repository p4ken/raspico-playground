pub use rp_pico::entry;

use rp_pico::hal;

pub struct PicoW {
    pub pins: rp_pico::Pins,
    pub timer: hal::Timer,
}
impl PicoW {
    pub fn new() -> Self {
        let mut pac = hal::pac::Peripherals::take().unwrap();
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
        .ok()
        .expect("failed to initialize clock");

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

        Self { pins, timer }
    }
}
