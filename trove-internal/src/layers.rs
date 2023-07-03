//! Types and functionality for keyboard layers.
//!
//! Layers allow the user to switch between multiple key layouts with pass-through for transparent
//! keys.
//!
//! For more information, see the [Kaleidoscope Layer docs](https://kaleidoscope.readthedocs.io/en/latest/layers.html).

use core::sync::atomic::{AtomicU8, Ordering};

mod key_defs;

pub use key_defs::*;

/// Represents a layer selection.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Layer {
    /// Base layer key layout.
    #[default]
    Base = 0,
    /// Function layer key layout.
    Fun = 1,
    /// Upper layer key layout.
    Upper = 2,
}

impl Layer {
    /// Creates a base [Layer](Layer::Base).
    pub const fn base() -> Self {
        Self::Base
    }

    /// Creates a function [Layer](Layer::Fun).
    pub const fn fun() -> Self {
        Self::Fun
    }

    /// Creates a upper [Layer](Layer::Upper).
    pub const fn upper() -> Self {
        Self::Upper
    }

    /// Converts the [Layer] to a `usize`.
    pub const fn index(&self) -> usize {
        *self as usize
    }
}

impl From<u8> for Layer {
    fn from(val: u8) -> Self {
        match val % 3 {
            0 => Self::Base,
            1 => Self::Fun,
            2 => Self::Upper,
            _ => Self::Base, // technically unreachable, but let's not panic
        }
    }
}

impl From<&Layer> for u8 {
    fn from(val: &Layer) -> Self {
        val.index() as u8
    }
}

impl From<Layer> for u8 {
    fn from(val: Layer) -> Self {
        (&val).into()
    }
}

impl From<usize> for Layer {
    fn from(val: usize) -> Self {
        (val as u8).into()
    }
}

impl From<&Layer> for usize {
    fn from(val: &Layer) -> Self {
        val.index()
    }
}

impl From<Layer> for usize {
    fn from(val: Layer) -> Self {
        (&val).into()
    }
}

pub type LayerKeys = [[u8; 12]; 4];

/// Base layer of keys on the default Atreus layout.
#[rustfmt::skip]
const LAYER0_KEYS: LayerKeys = [
    [ Q,   W,   E,   R,     T,    0,        0,     Y,   U,     I,     O,     P ],
    [ A,   S,   D,   F,     G,    0,        0,     H,   J,     K,     L,  SEMI ],
    [ Z,   X,   C,   V,     B,    TICK,  PIPE,     N,   M, COMMA,   DOT, SLASH ],
    [ ESC, TAB, CMD, SHIFT, BKSP, CTRL,   ALT, SPACE, FUN,  DASH, QUOTE, ENTER ],
];

/// Function layer of keys on the default Atreus layout.
#[rustfmt::skip]
const LAYER1_KEYS: LayerKeys = [
    [ EXCL,    AT,       U_ARROW, DOLLAR,  MOD,     0,         0,  PGUP, SEVEN, EIGHT,  NINE,  BKSP ],
    [ L_PAREN, L_ARROW,  D_ARROW, R_ARROW, R_PAREN, 0,         0,  PGDN,  FOUR,  FIVE,   SIX, TRANS ],
    [ L_BRACK, R_BRACK,  HASH,    L_BRACE, R_BRACE, CARET,   AMP,  STAR,   ONE,   TWO, THREE,  PLUS ],
    [ UPPER,   INS,      TRANS,   TRANS,   TRANS,   TRANS, TRANS, TRANS,   FUN,   DOT,  ZERO, EQUAL ],
];

/// Upper layer of keys on the default Atreus layout.
#[rustfmt::skip]
const LAYER2_KEYS: LayerKeys = [
    [ INS,   HOME,   TRANS,   END, PGUP,  0,         0,  U_ARROW, F7,     F8,     F9,     F10 ],
    [ DEL,   TRANS,  TRANS, TRANS, PGDN,  0,         0,  D_ARROW, F4,     F5,     F6,     F11 ],
    [ TRANS, VOL_UP, TRANS, TRANS, TRANS, TRANS, TRANS,  TRANS,   F1,     F2,     F3,     F12 ],
    [ UPPER, VOL_DN, TRANS, TRANS, TRANS, TRANS, TRANS,  TRANS,  FUN, PRT_SC, SCR_LK, PLAY_PS ],
];

/// Collection of all the layers.
static LAYERS: [LayerKeys; 3] = [LAYER0_KEYS, LAYER1_KEYS, LAYER2_KEYS];

/// Currently active layer.
static ACTIVE_LAYER: AtomicU8 = AtomicU8::new(0);

/// Get the key for a given `layer` and `index` (both zero-indexed).
///
/// The index is modulo the number of keys in a layer. For example, the Atreus has 4 rows of 12
/// keys = 48 keys total (with 4 blank keys). So, any index at or above 48 will start wrapping
/// around to the beginning.
pub fn layer_key(layer: usize, index: usize) -> u8 {
    // 0-47 => 0..3, mod 4 should be unneeded, but just in case...
    let row = (index / 12) % 4;
    // regardless of the row (since they are multiples of 12), this should give the column
    let col = index % 12;

    LAYERS[layer % LAYERS.len()][row][col]
}

/// Gets the key for a given `layer` and `index`, with pass-through for any transparent keys.
///
/// Transparent keys will pass-through to the next lowest layer, until a non-transparent key is
/// found, or the bottom layer is reached.
pub fn passthrough_key(layer: usize, index: usize) -> u8 {
    let key = layer_key(layer, index);

    if key == TRANS && layer > 0 {
        passthrough_key(layer - 1, index)
    } else {
        key
    }
}

/// Converts a given row and column index into the absolute index for a layer.
pub fn layer_index(row: usize, col: usize) -> usize {
    (row * 12) + col
}

/// Gets the currently active layer.
pub fn active_layer() -> Layer {
    ACTIVE_LAYER.load(Ordering::Relaxed).into()
}

/// Sets the currently active layer.
pub fn set_active_layer(layer: Layer) -> Layer {
    let last = active_layer();
    ACTIVE_LAYER.store(layer.into(), Ordering::SeqCst);
    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_zero_keys() {
        // row 0
        assert_eq!(layer_key(0, 0), Q);
        assert_eq!(layer_key(0, 1), W);
        assert_eq!(layer_key(0, 2), E);
        assert_eq!(layer_key(0, 3), R);
        assert_eq!(layer_key(0, 4), T);
        assert_eq!(layer_key(0, 5), 0);
        assert_eq!(layer_key(0, 6), 0);
        assert_eq!(layer_key(0, 7), Y);
        assert_eq!(layer_key(0, 8), U);
        assert_eq!(layer_key(0, 9), I);
        assert_eq!(layer_key(0, 10), O);
        assert_eq!(layer_key(0, 11), P);

        // row 1
        assert_eq!(layer_key(0, 12), A);
        assert_eq!(layer_key(0, 13), S);
        assert_eq!(layer_key(0, 14), D);
        assert_eq!(layer_key(0, 15), F);
        assert_eq!(layer_key(0, 16), G);
        assert_eq!(layer_key(0, 17), 0);
        assert_eq!(layer_key(0, 18), 0);
        assert_eq!(layer_key(0, 19), H);
        assert_eq!(layer_key(0, 20), J);
        assert_eq!(layer_key(0, 21), K);
        assert_eq!(layer_key(0, 22), L);
        assert_eq!(layer_key(0, 23), SEMI);

        // row 2
        assert_eq!(layer_key(0, 24), Z);
        assert_eq!(layer_key(0, 25), X);
        assert_eq!(layer_key(0, 26), C);
        assert_eq!(layer_key(0, 27), V);
        assert_eq!(layer_key(0, 28), B);
        assert_eq!(layer_key(0, 29), TICK);
        assert_eq!(layer_key(0, 30), PIPE);
        assert_eq!(layer_key(0, 31), N);
        assert_eq!(layer_key(0, 32), M);
        assert_eq!(layer_key(0, 33), COMMA);
        assert_eq!(layer_key(0, 34), DOT);
        assert_eq!(layer_key(0, 35), SLASH);

        // row 3
        assert_eq!(layer_key(0, 36), ESC);
        assert_eq!(layer_key(0, 37), TAB);
        assert_eq!(layer_key(0, 38), CMD);
        assert_eq!(layer_key(0, 39), SHIFT);
        assert_eq!(layer_key(0, 40), BKSP);
        assert_eq!(layer_key(0, 41), CTRL);
        assert_eq!(layer_key(0, 42), ALT);
        assert_eq!(layer_key(0, 43), SPACE);
        assert_eq!(layer_key(0, 44), FUN);
        assert_eq!(layer_key(0, 45), DASH);
        assert_eq!(layer_key(0, 46), QUOTE);
        assert_eq!(layer_key(0, 47), ENTER);
    }

    #[test]
    fn test_layer_one_keys() {
        // row 0
        assert_eq!(layer_key(1, 0), EXCL);
        assert_eq!(layer_key(1, 1), AT);
        assert_eq!(layer_key(1, 2), U_ARROW);
        assert_eq!(layer_key(1, 3), DOLLAR);
        assert_eq!(layer_key(1, 4), MOD);
        assert_eq!(layer_key(1, 5), 0);
        assert_eq!(layer_key(1, 6), 0);
        assert_eq!(layer_key(1, 7), PGUP);
        assert_eq!(layer_key(1, 8), SEVEN);
        assert_eq!(layer_key(1, 9), EIGHT);
        assert_eq!(layer_key(1, 10), NINE);
        assert_eq!(layer_key(1, 11), BKSP);

        // row 1
        assert_eq!(layer_key(1, 12), L_PAREN);
        assert_eq!(layer_key(1, 13), L_ARROW);
        assert_eq!(layer_key(1, 14), D_ARROW);
        assert_eq!(layer_key(1, 15), R_ARROW);
        assert_eq!(layer_key(1, 16), R_PAREN);
        assert_eq!(layer_key(1, 17), 0);
        assert_eq!(layer_key(1, 18), 0);
        assert_eq!(layer_key(1, 19), PGDN);
        assert_eq!(layer_key(1, 20), FOUR);
        assert_eq!(layer_key(1, 21), FIVE);
        assert_eq!(layer_key(1, 22), SIX);
        assert_eq!(layer_key(1, 23), TRANS);

        // row 2
        assert_eq!(layer_key(1, 24), L_BRACK);
        assert_eq!(layer_key(1, 25), R_BRACK);
        assert_eq!(layer_key(1, 26), HASH);
        assert_eq!(layer_key(1, 27), L_BRACE);
        assert_eq!(layer_key(1, 28), R_BRACE);
        assert_eq!(layer_key(1, 29), CARET);
        assert_eq!(layer_key(1, 30), AMP);
        assert_eq!(layer_key(1, 31), STAR);
        assert_eq!(layer_key(1, 32), ONE);
        assert_eq!(layer_key(1, 33), TWO);
        assert_eq!(layer_key(1, 34), THREE);
        assert_eq!(layer_key(1, 35), PLUS);

        // row 3
        assert_eq!(layer_key(1, 36), UPPER);
        assert_eq!(layer_key(1, 37), INS);
        assert_eq!(layer_key(1, 38), TRANS);
        assert_eq!(layer_key(1, 39), TRANS);
        assert_eq!(layer_key(1, 40), TRANS);
        assert_eq!(layer_key(1, 41), TRANS);
        assert_eq!(layer_key(1, 42), TRANS);
        assert_eq!(layer_key(1, 43), TRANS);
        assert_eq!(layer_key(1, 44), FUN);
        assert_eq!(layer_key(1, 45), DOT);
        assert_eq!(layer_key(1, 46), ZERO);
        assert_eq!(layer_key(1, 47), EQUAL);
    }

    #[test]
    fn test_layer_two_keys() {
        // row 0
        assert_eq!(layer_key(2, 0), INS);
        assert_eq!(layer_key(2, 1), HOME);
        assert_eq!(layer_key(2, 2), TRANS);
        assert_eq!(layer_key(2, 3), END);
        assert_eq!(layer_key(2, 4), PGUP);
        assert_eq!(layer_key(2, 5), 0);
        assert_eq!(layer_key(2, 6), 0);
        assert_eq!(layer_key(2, 7), U_ARROW);
        assert_eq!(layer_key(2, 8), F7);
        assert_eq!(layer_key(2, 9), F8);
        assert_eq!(layer_key(2, 10), F9);
        assert_eq!(layer_key(2, 11), F10);

        // row 1
        assert_eq!(layer_key(2, 12), DEL);
        assert_eq!(layer_key(2, 13), TRANS);
        assert_eq!(layer_key(2, 14), TRANS);
        assert_eq!(layer_key(2, 15), TRANS);
        assert_eq!(layer_key(2, 16), PGDN);
        assert_eq!(layer_key(2, 17), 0);
        assert_eq!(layer_key(2, 18), 0);
        assert_eq!(layer_key(2, 19), D_ARROW);
        assert_eq!(layer_key(2, 20), F4);
        assert_eq!(layer_key(2, 21), F5);
        assert_eq!(layer_key(2, 22), F6);
        assert_eq!(layer_key(2, 23), F11);

        // row 2
        assert_eq!(layer_key(2, 24), TRANS);
        assert_eq!(layer_key(2, 25), VOL_UP);
        assert_eq!(layer_key(2, 26), TRANS);
        assert_eq!(layer_key(2, 27), TRANS);
        assert_eq!(layer_key(2, 28), TRANS);
        assert_eq!(layer_key(2, 29), TRANS);
        assert_eq!(layer_key(2, 30), TRANS);
        assert_eq!(layer_key(2, 31), TRANS);
        assert_eq!(layer_key(2, 32), F1);
        assert_eq!(layer_key(2, 33), F2);
        assert_eq!(layer_key(2, 34), F3);
        assert_eq!(layer_key(2, 35), F12);

        // row 3
        assert_eq!(layer_key(2, 36), UPPER);
        assert_eq!(layer_key(2, 37), VOL_DN);
        assert_eq!(layer_key(2, 38), TRANS);
        assert_eq!(layer_key(2, 39), TRANS);
        assert_eq!(layer_key(2, 40), TRANS);
        assert_eq!(layer_key(2, 41), TRANS);
        assert_eq!(layer_key(2, 42), TRANS);
        assert_eq!(layer_key(2, 43), TRANS);
        assert_eq!(layer_key(2, 44), FUN);
        assert_eq!(layer_key(2, 45), PRT_SC);
        assert_eq!(layer_key(2, 46), SCR_LK);
        assert_eq!(layer_key(2, 47), PLAY_PS);
    }

    #[test]
    fn test_passthrough_keys() {
        // layer 1
        assert_eq!(passthrough_key(1, 23), SEMI);
        assert_eq!(passthrough_key(1, 38), CMD);
        assert_eq!(passthrough_key(1, 39), SHIFT);
        assert_eq!(passthrough_key(1, 40), BKSP);
        assert_eq!(passthrough_key(1, 41), CTRL);
        assert_eq!(passthrough_key(1, 42), ALT);
        assert_eq!(passthrough_key(1, 43), SPACE);

        // layer 2
        assert_eq!(passthrough_key(2, 2), U_ARROW);
        assert_eq!(passthrough_key(2, 13), L_ARROW);
        assert_eq!(passthrough_key(2, 14), D_ARROW);
        assert_eq!(passthrough_key(2, 15), R_ARROW);
        assert_eq!(passthrough_key(2, 24), L_BRACK);
        assert_eq!(passthrough_key(2, 26), HASH);
        assert_eq!(passthrough_key(2, 27), L_BRACE);
        assert_eq!(passthrough_key(2, 28), R_BRACE);
        assert_eq!(passthrough_key(2, 29), CARET);
        assert_eq!(passthrough_key(2, 30), AMP);
        assert_eq!(passthrough_key(2, 31), STAR);
        assert_eq!(passthrough_key(2, 38), CMD);
        assert_eq!(passthrough_key(2, 39), SHIFT);
        assert_eq!(passthrough_key(2, 40), BKSP);
        assert_eq!(passthrough_key(2, 41), CTRL);
        assert_eq!(passthrough_key(2, 42), ALT);
        assert_eq!(passthrough_key(2, 43), SPACE);
    }
}
