#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

const ANIM_LENGTH: usize = 16;
const FRAME_TIME_MS: u32 = 20;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let frames = generate_frames();
    let mut frame_no = 0;

    loop {
        display.show(&mut timer, frames[frame_no], FRAME_TIME_MS);
        frame_no = (frame_no + 1) % ANIM_LENGTH;

        // clear the display again
        display.clear();
        // timer.delay_ms(FRAME_TIME_MS);
    }
}

type Frame = [[u8; 5]; 5];

fn generate_frames() -> [Frame; ANIM_LENGTH] {
    let mut frames: [Frame; ANIM_LENGTH] = [[[0; 5]; 5]; ANIM_LENGTH];
    let mut n = 0;
    for x in 0..=4 {
        frames[n][0][x] = 1;
        n += 1;
    }
    for y in 1..=3 {
        frames[n][y][4] = 1;
        n += 1;
    }
    for x in (0..=4).rev() {
        frames[n][4][x] = 1;
        n += 1;
    }
    for y in (1..=3).rev() {
        frames[n][y][0] = 1;
        n += 1;
    }
    frames
}
