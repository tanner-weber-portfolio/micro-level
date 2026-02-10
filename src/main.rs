/* File: main.rs
 * Author: Tanner Weber, tannerw@pdx.edu
 * Date: 11 February 2026
 */

//! Turns the Microbit V2 into a level.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};
use microbit::{
    display::blocking::Display, hal::timer::Timer, hal::twim::Twim,
    pac::twim0::frequency::FREQUENCY_A,
};
use panic_rtt_target as _;
use rtt_target::rprintln;

const FRAMETIME_MS: u32 = 200;

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    let board = microbit::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut button_b = board.buttons.button_b;
    let i2c =
        Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut sensor = Lsm303agr::new_with_i2c(i2c);

    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut timer,
            AccelMode::HighResolution,
            AccelOutputDataRate::Hz50,
        )
        .unwrap();

    loop {
        if button_b.is_low().unwrap() {
            rprintln!("B Pressed");
        }

        let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();
        rprintln!("Accelerometer: x {:?} y {:?} z {:?}", x, y, z);
        let leds = get_bubble_pos(x, y, z);

        rprintln!("        Board {:?}", leds);
        display.show(&mut timer, leds, FRAMETIME_MS);
    }
}

/// Calculates the position of the bubble on the 5x5 LED grid.
/// Takes 3 mG values (-500mG to 500mG).
/// Returns the LED grid with a single lit up cell.
fn get_bubble_pos(x: i32, y: i32, z: i32) -> [[u8; 5]; 5] {
    if z > 0 || !(-500..=500).contains(&x) || !(-500..=500).contains(&y) {
        return [[0u8; 5]; 5];
    }
    let mut leds = [[0u8; 5]; 5];
    let pos_y = (-x / 200 + 2) as usize;
    let pos_x = (y / 200 + 2) as usize;
    leds[pos_x][pos_y] = 1;

    leds
}
