#![no_std]

pub mod ld61;
mod pico;

pub use pico::Pico;
pub use rp_pico::entry;

pub const JISKAN24: &[u8] = include_bytes!("font/jiskan24.bin");
