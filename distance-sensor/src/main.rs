#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::Instant;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    embassy::{self},
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
};

#[main]
async fn main(_spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    // setup system timer
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timg0);
    let delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut led = io.pins.gpio2.into_push_pull_output();
    led.set_low();

    // trig: D34 (gpio34)
    let mut trig = io.pins.gpio12.into_push_pull_output();
    trig.set_low();

    // rx: D35 (gpio35)
    let mut echo = io.pins.gpio34.into_pull_down_input();

    esp_println::logger::init_logger_from_env();

    loop {
        trig.set_low();
        delay.delay(2.micros());
        // send pulse for 10us
        trig.set_high();
        delay.delay(10.micros());
        trig.set_low();

        // Wait for rising edge
        echo.wait_for_high().await;

        let start_time = Instant::now();

        // wait for falling edge
        echo.wait_for_low().await;

        let duration = start_time.elapsed().as_micros();
        log::info!("Duration: {duration}us");

        // let duration = instant.elapsed().as_micros();
        if duration >= 38000 {
            continue; // out of bounds
        }
        let distance = duration / 58;
        // print distance
        log::info!("Distance: {distance}cm");
        if distance <= 5 {
            led.set_high();
        } else {
            led.set_low();
        }
        delay.delay(10000.micros());
    }
}
