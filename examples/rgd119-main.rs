#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use embedded_hal::{delay::DelayNs, digital::StatefulOutputPin};
use panic_probe as _;

use p4pico::{
    rgd119::{self, Color},
    Font, Pico,
};

const CHAR_W: usize = 24; // 1文字ぶんの幅(dot)。jiskan24は24x24
const PANEL_COUNT: usize = 2; // draw()に渡すパネル数(左右2枚で48dot幅)
const DISPLAY_W: usize = CHAR_W * PANEL_COUNT; // 48

const TEXT: [char; 6] = ['電', '車', 'が', 'き', 'ま', 'す'];
const TEXT_LEN: usize = TEXT.len();
const TEXT_W: usize = TEXT_LEN * CHAR_W; // 144

// 表示幅ぶんの空白を前後に挟んだ「仮想バッファ」上をスクロール窓が流れる。
// これにより文字が画面の右外から入り、左外へ抜けてからループする。
const TOTAL_W: usize = DISPLAY_W + TEXT_W + DISPLAY_W; // 240

/// 仮想バッファ座標 (virtual_col, row) が点灯すべきか判定する
fn pixel_on(glyphs: &[&[u8]; TEXT_LEN], virtual_col: usize, row: usize) -> bool {
    if virtual_col < DISPLAY_W || virtual_col >= DISPLAY_W + TEXT_W {
        return false; // 前後の空白部分は常に消灯
    }
    let col_in_text = virtual_col - DISPLAY_W;
    let glyph = glyphs[col_in_text / CHAR_W];
    let col_in_char = col_in_text % CHAR_W;

    // font.rs / rgd119.rs と同じビット並び: idx = row*24 + col, MSBファースト
    let idx = row * CHAR_W + col_in_char;
    (glyph[idx / 8] >> (7 - idx % 8)) & 1 != 0
}

/// offset位置における1パネルぶん(24x24)のビットマップを組み立てる
fn build_panel(glyphs: &[&[u8]; TEXT_LEN], offset: usize, panel: usize, buf: &mut [u8; 72]) {
    buf.fill(0);
    for col in 0..CHAR_W {
        let virtual_col = (offset + panel * CHAR_W + col) % TOTAL_W;
        for row in 0..CHAR_W {
            if pixel_on(glyphs, virtual_col, row) {
                let idx = row * CHAR_W + col;
                buf[idx / 8] |= 1 << (7 - idx % 8);
            }
        }
    }
}

#[entry]
fn main() -> ! {
    info!("Program start");

    let mut pico = Pico::new().unwrap();
    let pins = pico.pins;

    let mut rgd119 = rgd119::CN1 {
        se: pins.gpio1.into(),
        abb: pins.gpio2.into(),
        a4: pins.gpio3.into(),
        a3: pins.gpio4.into(),
        a2: pins.gpio5.into(),
        a1: pins.gpio6.into(),
        a0: pins.gpio7.into(),
        dg: pins.gpio9.into(),
        clk: pins.gpio10.into(),
        we: [pins.gpio11.into(), pins.gpio14.into()],
        dr: pins.gpio12.into(),
        ale: pins.gpio13.into(),
    }
    .into_host();
    let mut led = pins.led.into_push_pull_output();

    // probe-rs download rom/jiskan24.bin --base-address 0x10040000 --chip rp2040 --binary-format=bin
    let jiskan24 = Font::new(unsafe {
        core::slice::from_raw_parts(
            0x10040000 as *const u8,
            include_bytes!("../rom/jiskan24.bin").len(),
        )
    });

    // 毎フレームFontを検索しないよう、使う文字のグリフをあらかじめ引いておく
    let glyphs: [&[u8]; TEXT_LEN] = [
        &jiskan24[TEXT[0]],
        &jiskan24[TEXT[1]],
        &jiskan24[TEXT[2]],
        &jiskan24[TEXT[3]],
        &jiskan24[TEXT[4]],
        &jiskan24[TEXT[5]],
    ];

    let mut left = [0u8; 72];
    let mut right = [0u8; 72];
    let mut offset: usize = 0;

    loop {
        led.toggle();

        build_panel(&glyphs, offset, 0, &mut left);
        build_panel(&glyphs, offset, 1, &mut right);
        rgd119.draw([&left, &right], Color::Green);

        pico.timer.delay_ms(0); // 小さくするほど速くスクロールする
        offset = (offset + 1) % TOTAL_W;
    }
}
