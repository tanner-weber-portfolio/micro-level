# Level

Tanner Weber 2026

This program uses the accelerometer from the LSM303AGR on the Microbit to 
light up a single cell on the 5x5 LED grid like a bubble level.

# 🚀 Build and Run

```cargo embed --release```

# 📖 Writeup

My implementation uses the LSM303AGR crate to access the accelerometer values.
The x, y, and z values are obtained each frame (200ms), then used to 
calculate which cell on the 5x5 grid to light up. There is a course mode and 
fine mode that can be switched to by pressing the A and B buttons on the 
Microbit. I put the calculations for each mode in their own functions.

I initially tried to not use the LSM303AGR crate and just use the i2c read 
and write methods, because I wanted to get better at working with the LSM303AGR 
datasheet, but it was a pain to set up and convert the values and deal with 
a bunch of constants.

Each frame, the accelerometer values and LED grid values are printed.

# License

Copyright (C) 2026 Tanner Weber

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
