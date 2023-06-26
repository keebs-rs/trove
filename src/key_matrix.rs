use arduino_hal::hal::port::Pins;
use arduino_hal::port::{
    mode::{Input, Output, PullUp},
    Pin,
};

pub const ROWS: usize = 4;
pub const COLS: usize = 12;

/// Represents the rows and columns of the key matrix.
///
/// Rows are made of `Output` pins that are driven low to "activate" them.
///
/// Columns are made of `Input` pins connected to pull-up resistors, that are driven low when a key
/// is pressed.
pub struct KeyMatrix {
    pub(crate) rows: [Pin<Output>; ROWS],
    pub(crate) cols: [Pin<Input<PullUp>>; COLS],
}

impl KeyMatrix {
    /// Creates a new [KeyMatrix] of the rows and columns of key switch pins.
    ///
    /// Columns are pull-up resistor input pins, and rows are pulled-low output pins.
    ///
    /// This setup allows for "activating" rows by pulling the pin low, and checking each column in
    /// that row for a low state. Because the output (row) pin is pulled low, when a key is pressed the
    /// pull-up resistor (column) will go from pulled-high to pulled-low.
    ///
    /// After the row has been scanned, its pin is reset to pulled-high, and the process repeats for every
    /// other row. This all happens very quickly, so it appears simultaneous.
    ///
    /// For more information, see the great writeup by [Technomancy](https://atreus.technomancy.us/firmware).
    pub fn new(pins: Pins) -> Self {
        Self {
            rows: [
                // Row 0
                pins.pf6.into_output().downgrade(),
                // Row 1
                pins.pf5.into_output().downgrade(),
                // Row 3
                pins.pf4.into_output().downgrade(),
                // Row 4
                pins.pf1.into_output().downgrade(),
            ],
            cols: [
                // Col 0
                pins.pf7.into_pull_up_input().downgrade(),
                // Col 1
                pins.pe2.into_pull_up_input().downgrade(),
                // Col 2
                pins.pc7.into_pull_up_input().downgrade(),
                // Col 3
                pins.pc6.into_pull_up_input().downgrade(),
                // Col 4
                pins.pb6.into_pull_up_input().downgrade(),
                // Col 5
                pins.pb5.into_pull_up_input().downgrade(),
                // Col 6 is a blank column
                //pins.pb4.into_pull_up_input().downgrade(),
                // Col 7
                pins.pd7.into_pull_up_input().downgrade(),
                // Col 8
                pins.pd6.into_pull_up_input().downgrade(),
                // Col 9
                pins.pd4.into_pull_up_input().downgrade(),
                // Col 10
                pins.pd5.into_pull_up_input().downgrade(),
                // Col 11
                pins.pd3.into_pull_up_input().downgrade(),
                // Col 12
                pins.pd2.into_pull_up_input().downgrade(),
            ],
        }
    }

    /// Gets a reference to the row pins.
    pub fn rows(&self) -> &[Pin<Output>] {
        self.rows.as_ref()
    }

    /// Gets a mutable reference to the row pins.
    pub fn rows_mut(&mut self) -> &mut [Pin<Output>] {
        self.rows.as_mut()
    }

    /// Gets a reference to the column pins.
    pub fn cols(&self) -> &[Pin<Input<PullUp>>] {
        self.cols.as_ref()
    }

    /// Gets a mutable reference to the column pins.
    pub fn cols_mut(&mut self) -> &mut [Pin<Input<PullUp>>] {
        self.cols.as_mut()
    }
}
