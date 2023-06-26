use usbd_hid::descriptor::{KeyboardUsage, SystemControlKey};

/// Gets whether the keycode is for a modifier key.
pub fn key_is_modifier(key: u8) -> bool {
    let left_ctl = KeyboardUsage::KeyboardLeftControl as u8;
    let right_gui = KeyboardUsage::KeyboardRightGUI as u8;

    (left_ctl..=right_gui).contains(&key)
}

pub const fn key_to_modifier(key: u8) -> u8 {
    1 << (key - (KeyboardUsage::KeyboardLeftControl as u8))
}

/// Key layout (by column) of an Atreus keyboard.
pub const COL_KEYS: [[u8; 4]; 12] = [
    [
        KeyboardUsage::KeyboardQq as u8,
        KeyboardUsage::KeyboardAa as u8,
        KeyboardUsage::KeyboardZz as u8,
        KeyboardUsage::KeyboardEscape as u8,
    ],
    [
        KeyboardUsage::KeyboardWw as u8,
        KeyboardUsage::KeyboardSs as u8,
        KeyboardUsage::KeyboardXx as u8,
        KeyboardUsage::KeyboardTab as u8,
    ],
    [
        KeyboardUsage::KeyboardEe as u8,
        KeyboardUsage::KeyboardDd as u8,
        KeyboardUsage::KeyboardCc as u8,
        KeyboardUsage::KeyboardLeftGUI as u8,
    ],
    [
        KeyboardUsage::KeyboardRr as u8,
        KeyboardUsage::KeyboardFf as u8,
        KeyboardUsage::KeyboardVv as u8,
        KeyboardUsage::KeyboardLeftShift as u8,
    ],
    [
        KeyboardUsage::KeyboardTt as u8,
        KeyboardUsage::KeyboardGg as u8,
        KeyboardUsage::KeyboardBb as u8,
        KeyboardUsage::KeyboardBackspace as u8,
    ],
    [
        0,
        0,
        KeyboardUsage::KeyboardBacktickTilde as u8,
        KeyboardUsage::KeyboardSpacebar as u8,
    ],
    [
        0,
        0,
        KeyboardUsage::KeyboardBackslashBar as u8,
        KeyboardUsage::KeyboardRightAlt as u8,
    ],
    [
        KeyboardUsage::KeyboardYy as u8,
        KeyboardUsage::KeyboardHh as u8,
        KeyboardUsage::KeyboardNn as u8,
        KeyboardUsage::KeyboardRightControl as u8,
    ],
    [
        KeyboardUsage::KeyboardUu as u8,
        KeyboardUsage::KeyboardJj as u8,
        KeyboardUsage::KeyboardMm as u8,
        SystemControlKey::SystemFunctionShift as u8,
    ],
    [
        KeyboardUsage::KeyboardIi as u8,
        KeyboardUsage::KeyboardKk as u8,
        KeyboardUsage::KeyboardCommaLess as u8,
        KeyboardUsage::KeyboardDashUnderscore as u8,
    ],
    [
        KeyboardUsage::KeyboardOo as u8,
        KeyboardUsage::KeyboardLl as u8,
        KeyboardUsage::KeyboardPeriodGreater as u8,
        KeyboardUsage::KeyboardSingleDoubleQuote as u8,
    ],
    [
        KeyboardUsage::KeyboardPp as u8,
        KeyboardUsage::KeyboardSemiColon as u8,
        KeyboardUsage::KeyboardSlashQuestion as u8,
        KeyboardUsage::KeyboardEnter as u8,
    ],
];
