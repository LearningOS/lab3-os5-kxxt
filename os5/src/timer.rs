//! RISC-V timer-related functionality

use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use riscv::register::time;

const TICKS_PER_SEC: usize = 100;
const MICRO_PER_SEC: usize = 1_000_000;

/// read the `mtime` register
pub fn get_time() -> usize {
    time::read()
}

/// get current time in microseconds
pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}

/// get current time in milliseconds
pub fn get_time_ms() -> usize {
    let us = get_time_us();
    let sec = us / 1_000_000;
    let usec = us % 1_000_000;
    (sec & 0xffff) * 1000 + usec / 1000
}

/// set the next timer interrupt
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}
