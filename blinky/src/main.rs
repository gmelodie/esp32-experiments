#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    peripherals::{self, Peripherals},
    prelude::*,
    Delay, IO,
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    println!("here1");
    let mut led = io.pins.gpio2.into_push_pull_output();

    led.set_high().unwrap();

    println!("Hello world!");
    loop {
        println!("Loop...");
        delay.delay_ms(500u32);
        led.toggle().unwrap();
    }
}
