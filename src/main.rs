/* File: main.rs
 * Author: Tanner Weber, tannerw@pdx.edu
 * Date: 11 February 2026
 */

//! Turns the Microbit V2 into a level.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use microbit::display::blocking::Display;
use microbit::hal::timer::Timer;
use panic_rtt_target as _;
use rtt_target::rprintln;

const FRAMETIME_MS: u32 = 100;

#[entry]
fn init() -> ! {
    rtt_target::rtt_init_print!();

    let board = microbit::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds: [[u8; 5]; 5] = [[0; 5]; 5];
    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

    loop {
        if button_a.is_low().unwrap() {
            rprintln!("A Pressed");
        }
        if button_b.is_low().unwrap() {
            rprintln!("B Pressed");
        }
        rprintln!("        Board {:?}", leds);
        display.show(&mut timer, leds, FRAMETIME_MS);
        timer.delay_ms(FRAMETIME_MS);
    }
}
