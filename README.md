trove: Rust firmware for Keyboardio Atreus
==========================================

Experimental Rust firmware for the [Technomancy/Keyboardio Atreus](https://atreus.technomancy.us/).

The firmware is largely based on the [arduino_keyboard](https://github.com/agausmann/atmega-usbd/blob/master/examples/arduino_keyboard.rs) example from [atmega-usbd](https://github.com/agausmann/atmega-usbd), and the [Kaleidoscope](https://github.com/keyboardio/Kaleidoscope) firmware from [Keyboardio](https://keyboard.io).

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build --release` to build the firmware.

3. Run `cargo run --release` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.
    - This may require specifying the serial port of the keyboard (e.g. `RAVEDUDE_PORT=/dev/ttyACM0` on linux).

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
