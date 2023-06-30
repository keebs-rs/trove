//! Key definitions

use usbd_hid::descriptor::{KeyboardUsage as KB, MediaKey as MD, SystemControlKey as SC};

/// Number of columns in the keyboard layout.
pub const COLS: usize = 12;
/// Number of rows in the keyboard layout.
pub const ROWS: usize = 4;

pub const A: u8 = KB::KeyboardAa as u8;
pub const B: u8 = KB::KeyboardBb as u8;
pub const C: u8 = KB::KeyboardCc as u8;
pub const D: u8 = KB::KeyboardDd as u8;
pub const E: u8 = KB::KeyboardEe as u8;
pub const F: u8 = KB::KeyboardFf as u8;
pub const G: u8 = KB::KeyboardGg as u8;
pub const H: u8 = KB::KeyboardHh as u8;
pub const I: u8 = KB::KeyboardIi as u8;
pub const J: u8 = KB::KeyboardJj as u8;
pub const K: u8 = KB::KeyboardKk as u8;
pub const L: u8 = KB::KeyboardLl as u8;
pub const M: u8 = KB::KeyboardMm as u8;
pub const N: u8 = KB::KeyboardNn as u8;
pub const O: u8 = KB::KeyboardOo as u8;
pub const P: u8 = KB::KeyboardPp as u8;
pub const Q: u8 = KB::KeyboardQq as u8;
pub const R: u8 = KB::KeyboardRr as u8;
pub const S: u8 = KB::KeyboardSs as u8;
pub const T: u8 = KB::KeyboardTt as u8;
pub const U: u8 = KB::KeyboardUu as u8;
pub const V: u8 = KB::KeyboardVv as u8;
pub const X: u8 = KB::KeyboardXx as u8;
pub const W: u8 = KB::KeyboardWw as u8;
pub const Y: u8 = KB::KeyboardYy as u8;
pub const Z: u8 = KB::KeyboardZz as u8;
pub const SEMI: u8 = KB::KeyboardSemiColon as u8;
pub const TICK: u8 = KB::KeyboardBacktickTilde as u8;
pub const PIPE: u8 = KB::KeyboardBackslashBar as u8;
pub const COMMA: u8 = KB::KeyboardCommaLess as u8;
pub const DOT: u8 = KB::KeyboardPeriodGreater as u8;
pub const SLASH: u8 = KB::KeyboardSlashQuestion as u8;
pub const ESC: u8 = KB::KeyboardEscape as u8;
pub const TAB: u8 = KB::KeyboardTab as u8;
pub const CMD: u8 = KB::KeyboardLeftGUI as u8;
pub const SHIFT: u8 = KB::KeyboardLeftShift as u8;
pub const BKSP: u8 = KB::KeyboardBackspace as u8;
pub const SPACE: u8 = KB::KeyboardSpacebar as u8;
pub const ALT: u8 = KB::KeyboardLeftAlt as u8;
pub const CTRL: u8 = KB::KeyboardLeftControl as u8;
pub const QUOTE: u8 = KB::KeyboardSingleDoubleQuote as u8;
pub const ENTER: u8 = KB::KeyboardEnter as u8;
pub const DASH: u8 = KB::KeyboardDashUnderscore as u8;

pub const SHIFTED: u8 = 0b1000_0000;

pub const EXCL: u8 = KB::Keyboard1Exclamation as u8 | SHIFTED;
pub const AT: u8 = KB::Keyboard2At as u8 | SHIFTED;
pub const HASH: u8 = KB::Keyboard3Hash as u8 | SHIFTED;
pub const DOLLAR: u8 = KB::Keyboard4Dollar as u8 | SHIFTED;
pub const MOD: u8 = KB::Keyboard5Percent as u8 | SHIFTED;
pub const CARET: u8 = KB::Keyboard6Caret as u8 | SHIFTED;
pub const AMP: u8 = KB::Keyboard7Ampersand as u8 | SHIFTED;
pub const STAR: u8 = KB::Keyboard8Asterisk as u8 | SHIFTED;
pub const L_PAREN: u8 = KB::Keyboard9OpenParens as u8 | SHIFTED;
pub const R_PAREN: u8 = KB::Keyboard0CloseParens as u8 | SHIFTED;
pub const L_BRACE: u8 = KB::KeyboardOpenBracketBrace as u8 | SHIFTED;
pub const R_BRACE: u8 = KB::KeyboardCloseBracketBrace as u8 | SHIFTED;

pub const R_ARROW: u8 = KB::KeyboardRightArrow as u8;
pub const L_ARROW: u8 = KB::KeyboardLeftArrow as u8;
pub const D_ARROW: u8 = KB::KeyboardDownArrow as u8;
pub const U_ARROW: u8 = KB::KeyboardUpArrow as u8;
pub const L_BRACK: u8 = KB::KeyboardOpenBracketBrace as u8;
pub const R_BRACK: u8 = KB::KeyboardCloseBracketBrace as u8;
pub const INS: u8 = KB::KeyboardInsert as u8;
pub const DEL: u8 = KB::KeyboardDelete as u8;
pub const PGUP: u8 = KB::KeyboardPageUp as u8;
pub const PGDN: u8 = KB::KeyboardPageDown as u8;
pub const ONE: u8 = KB::Keyboard1Exclamation as u8;
pub const TWO: u8 = KB::Keyboard2At as u8;
pub const THREE: u8 = KB::Keyboard3Hash as u8;
pub const FOUR: u8 = KB::Keyboard4Dollar as u8;
pub const FIVE: u8 = KB::Keyboard5Percent as u8;
pub const SIX: u8 = KB::Keyboard6Caret as u8;
pub const SEVEN: u8 = KB::Keyboard7Ampersand as u8;
pub const EIGHT: u8 = KB::Keyboard8Asterisk as u8;
pub const NINE: u8 = KB::Keyboard9OpenParens as u8;
pub const ZERO: u8 = KB::Keyboard0CloseParens as u8;
pub const PAD_DOT: u8 = KB::KeypadPeriodDelete as u8;
pub const PLUS: u8 = KB::KeypadPlus as u8;
pub const EQUAL: u8 = KB::KeypadEqual as u8;

pub const HOME: u8 = KB::KeyboardHome as u8;
pub const END: u8 = KB::KeyboardEnd as u8;
pub const PRT_SC: u8 = KB::KeyboardPrintScreen as u8;
pub const SCR_LK: u8 = KB::KeyboardScrollLock as u8;
pub const PLAY_PS: u8 = MD::PlayPause as u8;

pub const VOL_UP: u8 = KB::KeyboardVolumeUp as u8; 
pub const VOL_DN: u8 = KB::KeyboardVolumeDown as u8; 

pub const F1: u8 = KB::KeyboardF1 as u8; 
pub const F2: u8 = KB::KeyboardF2 as u8; 
pub const F3: u8 = KB::KeyboardF3 as u8; 
pub const F4: u8 = KB::KeyboardF4 as u8; 
pub const F5: u8 = KB::KeyboardF5 as u8; 
pub const F6: u8 = KB::KeyboardF6 as u8; 
pub const F7: u8 = KB::KeyboardF7 as u8; 
pub const F8: u8 = KB::KeyboardF8 as u8; 
pub const F9: u8 = KB::KeyboardF9 as u8; 
pub const F10: u8 = KB::KeyboardF10 as u8; 
pub const F11: u8 = KB::KeyboardF11 as u8; 
pub const F12: u8 = KB::KeyboardF12 as u8; 

pub const FUN: u8 = SC::SystemFunctionShift as u8;
pub const UPPER: u8 = 0xfe;
pub const TRANS: u8 = 0xff;

/// Gets whether the key is the function key.
pub fn key_is_fun(key: u8) -> bool {
    key == FUN
}

/// Gets whether the key is the upper key.
pub fn key_is_upper(key: u8) -> bool {
    key == UPPER
}

/// Gets whether the key is a transparent key.
pub fn key_is_trans(key: u8) -> bool {
    key == TRANS
}

/// Gets whether the key is shifted.
pub fn key_is_shifted(key: u8) -> bool {
    match key {
        L_BRACE | R_BRACE | EXCL | AT | HASH | DOLLAR | MOD | CARET | AMP | STAR | L_PAREN | R_PAREN => true,
        _ => false,
    }
}

/// Gets a shifted keycode.
///
/// Useful for sending the base keycode with the shift modifier in a [KeyboardReport](usbd_hid::descriptor::KeyboardReport).
pub fn shifted_key(key: u8) -> u8 {
    key & !SHIFTED
}

/// Gets whether the keycode is for a modifier key.
pub fn key_is_modifier(key: u8) -> bool {
    let left_ctl = KB::KeyboardLeftControl as u8;
    let right_gui = KB::KeyboardRightGUI as u8;

    (left_ctl..=right_gui).contains(&key)
}

/// Converts the key to a modifier bitfield for a [KeyboardReport](usbd_hid::descriptor::KeyboardReport).
pub const fn key_to_modifier(key: u8) -> u8 {
    1 << (key - (KB::KeyboardLeftControl as u8))
}
