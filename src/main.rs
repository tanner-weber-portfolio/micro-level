/* File: main.rs
 * Author: Tanner Weber, tannerw@pdx.edu
 * Date: 11 February 2026
 */

//! Turns the Microbit V2 into a level.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use microbit::{
    display::blocking::Display, hal::timer::Timer, hal::twim::Twim,
    pac::twim0::frequency::FREQUENCY_A,
};
use panic_rtt_target as _;
use rtt_target::rprintln;

const FRAMETIME_MS: u32 = 100;
const ACCELEROMETER_ADDR: u8 = 0b_0011001;
const MAGNETOMETER_ADDR: u8 = 0b_0011110;
const ACCELEROMETER_ID_REG: u8 = 0x_0f;
const MAGNETOMETER_ID_REG: u8 = 0x_4f;

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    let board = microbit::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds: [[u8; 5]; 5] = [[0; 5]; 5];
    let mut button_b = board.buttons.button_b;
    let mut i2c =
        Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut acc = [0u8];
    let mut mag = [0u8];

    i2c.write_then_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc)
        .unwrap();
    i2c.write_then_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag)
        .unwrap();

    rprintln!("The accelerometer chip's id is: {:#b}", acc[0]);
    rprintln!("The magnetometer chip's id is: {:#b}", mag[0]);

    loop {
        if button_b.is_low().unwrap() {
            rprintln!("B Pressed");
        }
        rprintln!("        Board {:?}", leds);
        display.show(&mut timer, leds, FRAMETIME_MS);
        timer.delay_ms(FRAMETIME_MS);
    }
}
