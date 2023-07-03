#![no_std]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]

#[macro_use(bitfield)]
extern crate bitfield;

use core::cell::RefCell;

use avr_device::interrupt::Mutex;

pub use trove_internal::layers;

pub mod key_matrix;
pub mod key_scanner;
pub mod lock;
pub mod setup;
pub mod std_stub;
pub mod usb_context;

pub use key_matrix::*;
pub use key_scanner::*;
pub use lock::*;
pub use setup::*;
pub use usb_context::*;

/// CPU frequency of the ATmega32u4 (16Mhz).
pub const F_CPU: u32 = 16_000_000;

/// Global USB context for scanning the key matrix, and handling device-host communication.
pub static USB_CTX: Mutex<RefCell<Option<UsbContext>>> = Mutex::new(RefCell::new(None));
