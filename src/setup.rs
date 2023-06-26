use arduino_hal::pac;

use crate::F_CPU;

/// Setup the timer used to trigger a keyscan.
pub fn setup_timer(tc1: pac::TC1, interval: u32) {
    tc1.tccr1b.write(|w| w.wgm1().bits(0b10));
    tc1.tccr1a.write(|w| unsafe { w.bits(0) });

    let cycles = (F_CPU / 2_000_000) * interval;

    tc1.icr1.write(|w| w.bits(cycles as u16));

    tc1.tccr1b.write(|w| w.wgm1().bits(0b10).cs1().bits(0b01));
    tc1.timsk1.modify(|_, w| w.toie1().bit(true));
}
