#![no_std]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]

#[macro_use(bitfield)]
extern crate bitfield;

pub use trove_internal::layers;

mod key_matrix;
mod key_scanner;
mod lock;
mod setup;
mod std_stub;
mod usb_context;

pub use key_matrix::*;
pub use key_scanner::*;
pub use lock::*;
pub use setup::*;
pub use usb_context::*;

/// CPU frequency of the ATmega32u4 (16Mhz).
pub const F_CPU: u32 = 16_000_000;

/// Global USB context for scanning the key matrix, and handling device-host communication.
pub static mut USB_CTX: Option<UsbContext> = None;
