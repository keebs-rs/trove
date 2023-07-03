#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
#![deny(unsafe_op_in_unsafe_fn)]

use arduino_hal::{entry, hal::pins, Peripherals};
use atmega_usbd::UsbBus;
use avr_device::{asm::sleep, interrupt};
use usb_device::{
    class_prelude::UsbBusAllocator,
    device::{UsbDeviceBuilder, UsbVidPid},
};
use usbd_hid::{
    descriptor::{KeyboardReport, SerializedDescriptor},
    hid_class::HIDClass,
};

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let pll = dp.PLL;
    let usb = dp.USB_DEVICE;

    // Configure PLL interface
    // prescale 16MHz crystal -> 8MHz
    pll.pllcsr.write(|w| w.pindiv().set_bit());
    // 96MHz PLL output; /1.5 for 64MHz timers, /2 for 48MHz USB
    pll.pllfrq
        .write(|w| w.pdiv().mhz96().plltm().factor_15().pllusb().set_bit());

    // Enable PLL
    pll.pllcsr.modify(|_, w| w.plle().set_bit());

    // Check PLL lock
    while pll.pllcsr.read().plock().bit_is_clear() {}

    trove::setup_timer(dp.TC1, 1500);

    // Safety: mutable static is initialized exactly once, and only borrowed as a shared reference.
    let usb_bus = unsafe {
        static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;
        &*USB_BUS.insert(UsbBus::new(usb))
    };

    let hid_class = HIDClass::new(usb_bus, KeyboardReport::desc(), 1);
    let usb_device = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x1209, 0x2303))
        .manufacturer("Keyboardio")
        .product("Trove Atreus")
        .build();

    let key_scanner = trove::KeyScanner::new(trove::KeyMatrix::new(pins));

    let usb_ctx = trove::UsbContext {
        usb_device,
        hid_class,
        key_scanner,
    };

    interrupt::free(|cs| {
        trove::USB_CTX.borrow(cs).borrow_mut().replace(usb_ctx);
    });

    unsafe { interrupt::enable() };

    loop {
        sleep();
    }
}

#[interrupt(atmega32u4)]
fn USB_GEN() {
    scan_matrix();
}

#[interrupt(atmega32u4)]
fn USB_COM() {
    scan_matrix();
}

#[interrupt(atmega32u4)]
fn TIMER1_OVF() {
    trove::key_scanner::set_do_scan(true);
}

fn scan_matrix() {
    interrupt::free(|cs| {
        if let Some(ctx) = trove::USB_CTX.borrow(cs).borrow_mut().as_mut() {
            ctx.scan_matrix();
        }
    });
}
