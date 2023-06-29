//! Keyscanner types and functionality
//!
//! Types and functionality for scanning the key matrix, and debouncing key activation state.

use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use avr_device::asm;
use usbd_hid::descriptor::KeyboardReport;

use crate::{key_is_modifier, key_to_modifier, KeyMatrix, COLS, COL_KEYS, ROWS};

/// Maximum number of columns of in a [RowState].
pub const MAX_COLS: usize = 16;

/// Blank [KeyboardReport].
pub const BLANK_REPORT: KeyboardReport = KeyboardReport {
    modifier: 0,
    reserved: 0,
    leds: 0,
    keycodes: [0; 6],
};

bitfield! {
    /// Activated status for a row of keys.
    ///
    /// Each row-column key is represented by a single bit to save space.
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct RowState(u16);

    /// Getter/setter for column 0
    pub col0, set_col0: 0;
    /// Getter/setter for column 1
    pub col1, set_col1: 1;
    /// Getter/setter for column 2
    pub col2, set_col2: 2;
    /// Getter/setter for column 3
    pub col3, set_col3: 3;
    /// Getter/setter for column 4
    pub col4, set_col4: 4;
    /// Getter/setter for column 5
    pub col5, set_col5: 5;
    /// Getter/setter for column 6
    pub col6, set_col6: 6;
    /// Getter/setter for column 7
    pub col7, set_col7: 7;
    /// Getter/setter for column 8
    pub col8, set_col8: 8;
    /// Getter/setter for column 9
    pub col9, set_col9: 9;
    /// Getter/setter for column 10
    pub col10, set_col10: 10;
    /// Getter/setter for column 11
    pub col11, set_col11: 11;
    /// Getter/setter for column 12
    pub col12, set_col12: 12;
    /// Getter/setter for column 13
    pub col13, set_col13: 13;
    /// Getter/setter for column 14
    pub col14, set_col14: 14;
    /// Getter/setter for column 15
    pub col15, set_col15: 15;
}

impl RowState {
    /// Creates a new [RowState].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Gets the underlying integer representation of the [RowState].
    pub const fn as_inner(&self) -> u16 {
        self.0
    }

    /// Creates a new [RowState] from a `u16`.
    pub const fn from_u16(val: u16) -> Self {
        Self(val)
    }

    /// Gets the column activation state.
    pub fn column(&self, index: usize) -> bool {
        match index % MAX_COLS {
            0 => self.col0(),
            1 => self.col1(),
            2 => self.col2(),
            3 => self.col3(),
            4 => self.col4(),
            5 => self.col5(),
            6 => self.col6(),
            7 => self.col7(),
            8 => self.col8(),
            9 => self.col9(),
            10 => self.col10(),
            11 => self.col11(),
            12 => self.col12(),
            13 => self.col13(),
            14 => self.col14(),
            _ => self.col15(),
        }
    }

    /// Sets the column activation state.
    pub fn set_column(&mut self, index: usize, val: bool) {
        match index % MAX_COLS {
            0 => self.set_col0(val),
            1 => self.set_col1(val),
            2 => self.set_col2(val),
            3 => self.set_col3(val),
            4 => self.set_col4(val),
            5 => self.set_col5(val),
            6 => self.set_col6(val),
            7 => self.set_col7(val),
            8 => self.set_col8(val),
            9 => self.set_col9(val),
            10 => self.set_col10(val),
            11 => self.set_col11(val),
            12 => self.set_col12(val),
            13 => self.set_col13(val),
            14 => self.set_col14(val),
            _ => self.set_col15(val),
        }
    }

    /// Gets whether the [RowState] has active columns.
    pub const fn is_active(&self) -> bool {
        self.0 != 0
    }

    /// Gets whether the [RowState] has no active columns.
    pub const fn is_inactive(&self) -> bool {
        self.0 == 0
    }
}

impl From<u16> for RowState {
    fn from(val: u16) -> Self {
        Self::from_u16(val)
    }
}

impl From<&u16> for RowState {
    fn from(val: &u16) -> Self {
        (*val).into()
    }
}

impl From<&RowState> for u16 {
    fn from(val: &RowState) -> Self {
        val.as_inner()
    }
}

impl From<RowState> for u16 {
    fn from(val: RowState) -> Self {
        (&val).into()
    }
}

impl BitAnd for RowState {
    type Output = RowState;

    fn bitand(self, rhs: Self) -> Self::Output {
        (self.0 & rhs.0).into()
    }
}

impl BitAndAssign for RowState {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for RowState {
    type Output = RowState;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self.0 | rhs.0).into()
    }
}

impl BitOrAssign for RowState {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for RowState {
    type Output = RowState;

    fn bitxor(self, rhs: Self) -> Self::Output {
        (self.0 ^ rhs.0).into()
    }
}

impl BitXorAssign for RowState {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for RowState {
    type Output = RowState;

    fn not(self) -> Self::Output {
        (!self.0).into()
    }
}

/// Debounce state for the keyscanner matrix.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Debounce {
    /// Debounce counter 0 bits for a given [RowState].
    db0: RowState,
    /// Debounce counter 1 bits for a given [RowState].
    db1: RowState,
    /// Debounced [RowState].
    debounced: RowState,
}

impl Debounce {
    /// Creates a new [Debounce] state.
    pub const fn new() -> Self {
        Self {
            db0: RowState::new(),
            db1: RowState::new(),
            debounced: RowState::new(),
        }
    }

    /// Gets the debounce counter bit zero.
    pub const fn db0(&self) -> RowState {
        self.db0
    }

    /// Gets the debounce counter bit one.
    pub const fn db1(&self) -> RowState {
        self.db1
    }

    /// Gets the debounced [RowState].
    pub const fn debounced(&self) -> RowState {
        self.debounced
    }

    /// Debounce the sampled [RowState].
    pub fn debounce(&mut self, sample: RowState) -> RowState {
        // Use xor to detect changes from last stable state:
        // if a key has changed, its bit will be 1, otherwise 0
        let delta = sample ^ self.debounced;

        // Increment counters and reset any unchanged bits:
        // increment bit 1 for all changed keys
        self.db1 = (self.db1 ^ self.db0) & delta;
        // increment bit 0 for all changed keys
        self.db0 = !self.db0 & delta;

        // Calculate returned change set: if delta is still true
        // and the counter has wrapped back to 0, the key is changed.
        let changes = !(!delta | self.db0 | self.db1);
        // Update state: in this case use xor to flip any bit that is true in changes.
        self.debounced ^= changes;

        changes.into()
    }
}

/// Represents the previous, current, and debounced state for a given row.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DebounceRowState {
    /// Previous [RowState].
    previous: RowState,
    /// Current [RowState].
    current: RowState,
    /// [Debounce]r for this [RowState].
    debouncer: Debounce,
}

impl DebounceRowState {
    /// Creates a new [DebounceRowState].
    pub const fn new() -> Self {
        Self {
            previous: RowState::new(),
            current: RowState::new(),
            debouncer: Debounce::new(),
        }
    }

    /// Gets the previous [RowState].
    pub const fn previous(&self) -> RowState {
        self.previous
    }

    /// Sets the previous [RowState].
    pub fn set_previous(&mut self, state: RowState) {
        self.previous = state;
    }

    /// Builder function that sets the previous [RowState].
    pub fn with_previous(mut self, state: RowState) -> Self {
        self.set_previous(state);
        self
    }

    /// Gets the current [RowState].
    pub const fn current(&self) -> RowState {
        self.current
    }

    /// Sets the current [RowState].
    pub fn set_current(&mut self, state: RowState) {
        self.current = state;
    }

    /// Builder function that sets the current [RowState].
    pub fn with_current(mut self, state: RowState) -> Self {
        self.set_current(state);
        self
    }

    /// Gets the `[Debounce]r` for the [RowState].
    pub const fn debouncer(&self) -> Debounce {
        self.debouncer
    }

    /// Sets the `[Debounce]r` for the [RowState].
    pub fn set_debouncer(&mut self, state: Debounce) {
        self.debouncer = state;
    }

    /// Builder function that sets the `[Debounce]r` for the [RowState].
    pub fn with_debouncer(mut self, state: Debounce) -> Self {
        self.set_debouncer(state);
        self
    }
}

/// Represents the key matrix scanner for reading row and column pin sctivation.
///
/// Uses a debouncing algorithm to normalize reads, and avoid producing multiple reports for a
/// single key press.
pub struct KeyScanner {
    matrix_pins: KeyMatrix,
    matrix_state: [DebounceRowState; ROWS],
    do_scan: bool,
}

fn small_delay(count: usize) {
    for _ in 0..count {
        asm::nop();
    }
}

impl KeyScanner {
    pub fn new(matrix_pins: KeyMatrix) -> Self {
        Self {
            matrix_pins,
            matrix_state: [DebounceRowState::new(); ROWS],
            do_scan: true,
        }
    }

    pub fn set_do_scan(&mut self, val: bool) {
        self.do_scan = val;
    }

    /// Reads the [KeyMatrix] pins, and updates the debouncer state.
    pub fn read_matrix(&mut self) {
        let mut any_debounced_changes = RowState::new();

        for (i, row) in self.matrix_pins.rows.iter_mut().enumerate() {
            // pull the row pin low to "activate" the row
            row.set_low();

            let mut hot_pins = RowState::new();
            for (j, col) in self.matrix_pins.cols.iter().enumerate() {
                // add a slight delay to allow for stable read of the input pin
                small_delay(768);
                // if the column pin is low, the key was pressed
                if col.is_low() {
                    hot_pins.set_column(j, true);
                }
            }

            // pull the row pin high to "deactivate" the row, and avoid electrical interference
            // with following reads
            row.set_high();

            any_debounced_changes = RowState::from(
                any_debounced_changes.as_inner()
                    | self.matrix_state[i].debouncer.debounce(hot_pins).as_inner(),
            );
        }

        if any_debounced_changes.is_active() {
            for s in 0..ROWS {
                let debounced = self.matrix_state[s].debouncer.debounced();
                self.matrix_state[s].set_current(debounced);
            }
        }
    }

    /// Gets the debounced [KeyboardReports] from the most recent matrix scan.
    pub fn matrix_scan_reports<const N: usize>(&mut self) -> [KeyboardReport; N] {
        let mut reports = [BLANK_REPORT; N];
        let mut report_idx = 0;
        let mut keycodes = 0;

        for (row, row_state) in self.matrix_state.iter_mut().enumerate() {
            for col in 0..COLS {
                if row_state.previous.column(col) || row_state.current.column(col) {
                    // read the key value from the key map
                    let key = COL_KEYS[col][row];

                    if key_is_modifier(key) {
                        reports[report_idx].modifier |= key_to_modifier(key);
                    } else {
                        reports[report_idx].keycodes[keycodes] = key;
                        keycodes += 1;
                    }

                    // if the current report has the max non-modifier keys, move to the next report
                    if keycodes >= 6 {
                        report_idx += 1;
                        keycodes = 0;
                    }
                }
            }

            row_state.previous = row_state.current;
        }

        reports
    }

    /// Perform a debounced [KeyMatrix] scan, and return any [KeyboardReport]s.
    pub fn scan<const N: usize>(&mut self) -> [KeyboardReport; N] {
        let do_scan = self.do_scan;
        if do_scan {
            self.read_matrix();
            // FIXME: the original algorithm toggles this flag using the timer,
            // but we simply call `scan_matrix`. TBD if we can just remove this flag.
            self.do_scan = false;
        }

        self.matrix_scan_reports::<N>()
    }
}
