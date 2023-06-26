use atmega_usbd::UsbBus;
use usb_device::device::UsbDevice;
use usbd_hid::hid_class::HIDClass;

use crate::{KeyScanner, BLANK_REPORT};

/// Maximum number of [KeyboardReport]s that can be returned by a matrix scan.
///
/// There are 4 rows, 12 columns, and each report holds 6 key codes: 4 * 12 / 6 = 8
pub const MAX_KEYBOARD_REPORTS: usize = 8;

/// Represents the USB context used for scanning the key matrix,
/// and sending keyboard reports to the host.
pub struct UsbContext {
    pub usb_device: UsbDevice<'static, UsbBus>,
    pub hid_class: HIDClass<'static, UsbBus>,
    pub key_scanner: KeyScanner,
}

impl UsbContext {
    pub fn scan_matrix(&mut self) {
        let reports = self.key_scanner.scan::<MAX_KEYBOARD_REPORTS>();

        for report in reports.iter() {
            self.hid_class.push_input(report).ok();

            if self.usb_device.poll(&mut [&mut self.hid_class]) {
                let mut report_buf = [0u8; 1];

                self.hid_class.pull_raw_output(&mut report_buf).ok();
            }

            self.poll();
        }
    }

    pub fn poll(&mut self) {
        self.hid_class.push_input(&BLANK_REPORT).ok();

        if self.usb_device.poll(&mut [&mut self.hid_class]) {
            let mut report_buf = [0u8; 1];

            self.hid_class.pull_raw_output(&mut report_buf).ok();
        }
    }
}
