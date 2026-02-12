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

enum State {
    Coarse,
    Fine,
}

const FRAMETIME_MS: u32 = 200;

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    let board = microbit::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;
    let mut leds: [[u8; 5]; 5];
    let mut state = State::Coarse;
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
        if button_a.is_low().unwrap() {
            rprintln!("A Pressed");
            state = State::Coarse;
        }
        if button_b.is_low().unwrap() {
            rprintln!("B Pressed");
            state = State::Fine;
        }

        let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();
        rprintln!("Accelerometer: x {:?} y {:?} z {:?}", x, y, z);

        match state {
            State::Coarse => {
                leds = get_bubble_pos_course(x, y, z);
            }
            State::Fine => {
                leds = get_bubble_pos_fine(x, y, z);
            }
        }

        rprintln!("        Board {:?}", leds);
        display.show(&mut timer, leds, FRAMETIME_MS);
    }
}

/// Calculates the position of the bubble on the 5x5 LED grid.
/// Takes 3 mG values (-500mG to 500mG) directly from lsm303agr crate function.
/// Returns the LED grid with a single lit up cell.
fn get_bubble_pos_course(x: i32, y: i32, z: i32) -> [[u8; 5]; 5] {
    let (x, y, z) = convert_axes(x, y, z);
    if z < 0 {
        return [[0u8; 5]; 5];
    }
    let mut leds = [[0u8; 5]; 5];
    let mut pos_x = (x as f64 / 200_f64 + 2.5) as usize;
    let mut pos_y = (y as f64 / 200_f64 + 2.5) as usize;

    if pos_x > leds.len() {
        pos_x = leds.len();
    }
    if pos_y > leds.len() {
        pos_y = leds.len();
    }

    leds[pos_x][pos_y] = 1;
    leds
}

/// Calculates the position of the bubble on the 5x5 LED grid.
/// Takes 3 mG values (-50mG to 50mG) directly from lsm303agr crate function.
/// Returns the LED grid with a single lit up cell.
fn get_bubble_pos_fine(x: i32, y: i32, z: i32) -> [[u8; 5]; 5] {
    let (x, y, z) = convert_axes(x, y, z);
    if z < 0 {
        return [[0u8; 5]; 5];
    }
    let mut leds = [[0u8; 5]; 5];
    let mut pos_x = (x as f64 / 20_f64 + 2.5) as usize;
    let mut pos_y = (y as f64 / 20_f64 + 2.5) as usize;

    if pos_x > leds.len() {
        pos_x = leds.len();
    }
    if pos_y > leds.len() {
        pos_y = leds.len();
    }

    leds[pos_x][pos_y] = 1;
    leds
}

/// Converts the 3 axes from the lsm303agr crate acceleration() and xyz_mg()
/// functions and flips the axes to match the microbit board, such that the
/// the top of the board is the LED grid and the axes correspond to a 2D array
/// representing the LED grid.
fn convert_axes(x: i32, y: i32, z: i32) -> (i32, i32, i32) {
    (y, -x, -z)
}
