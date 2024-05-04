#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::gpio::IO;
use esp_hal::{clock::ClockControl, delay::Delay, peripherals::Peripherals, prelude::*};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio2.into_push_pull_output();
    let button = io.pins.gpio15.into_pull_down_input();

    esp_println::logger::init_logger_from_env();

    led.set_low();

    loop {
        // log::info!("Loop");
        // delay.delay(500.millis());
        if button.is_high() {
            // log::info!("Button high");
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
