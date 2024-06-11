//! Blinks the onboard LED of esp32c3

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{Io, Level, Output},
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};
use esp_println::println;
use fugit::ExtU64;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // gpio8 for the onboard led
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = Output::new(io.pins.gpio8, Level::High);

    let delay = Delay::new(&clocks);

    println!("hello world");

    loop {
        led.toggle();
        delay.delay(1.secs());
        led.toggle();
        delay.delay(1.secs());
    }
}