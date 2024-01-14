/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

pub struct PicoW {
    pub pins: rp2040_hal::gpio::Pins,
    pub timer: rp2040_hal::Timer,
}
impl PicoW {
    pub fn new() -> Self {
        let Rp2040 { pins, timer } = Rp2040::new();
        Self { pins, timer }
    }
}

struct Rp2040 {
    pub pins: rp2040_hal::gpio::Pins,
    pub timer: rp2040_hal::Timer,
}
impl Rp2040 {
    pub fn new() -> Self {
        let mut pac = rp2040_hal::pac::Peripherals::take().unwrap();
        let mut watchdog = rp2040_hal::watchdog::Watchdog::new(pac.WATCHDOG);

        // Configure the clocks
        let clocks = rp2040_hal::clocks::init_clocks_and_plls(
            XTAL_FREQ_HZ,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .ok()
        .expect("failed to initialize clock");

        let timer = rp2040_hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

        // The single-cycle I/O block controls our GPIO pins
        let sio = rp2040_hal::Sio::new(pac.SIO);

        // Set the pins to their default state
        let pins = rp2040_hal::gpio::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        Self { pins, timer }
    }
}
