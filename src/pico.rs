use cortex_m::delay::Delay;
use defmt::println;

pub use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::Clock;

pub fn hello() {
    println!("hello usbserial");
}

pub struct PicoW {
    pins: rp_pico::Pins,
    delay: Delay,
}
impl PicoW {
    pub fn new() -> Self {
        let mut rp2040 = Rp2040::new();
        let pins = rp_pico::Pins::new(
            rp2040.pac.IO_BANK0,
            rp2040.pac.PADS_BANK0,
            rp2040.sio.gpio_bank0,
            &mut rp2040.pac.RESETS,
        );
        let delay = rp2040.delay;
        Self { pins, delay }
    }

    pub fn delay_ms(&mut self, millisec: u32) {
        self.delay.delay_ms(millisec)
    }

    pub fn pins(&self) -> &rp_pico::Pins {
        &self.pins
    }
}

struct Rp2040 {
    delay: Delay,
    pac: hal::pac::Peripherals,
    sio: hal::sio::Sio,
}
impl Rp2040 {
    pub fn new() -> Self {
        let mut pac = hal::pac::Peripherals::take().unwrap();
        let core = hal::pac::CorePeripherals::take().unwrap();
        let mut watchdog = hal::watchdog::Watchdog::new(pac.WATCHDOG);
        let sio = hal::sio::Sio::new(pac.SIO);

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

        let delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

        Self { delay, pac, sio }
    }
}
