use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::i2c::I2cConfig;
use esp_idf_svc::hal::i2c::I2cDriver;
// use esp_idf_svc::hal::prelude::FromValueType;
// use esp_idf_svc::hal::prelude::Peripherals;
// use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};
use esp_idf_svc::hal::prelude::*;
use ssd1306;
use ssd1306::mode::DisplayConfig;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("About to initialize the Heltec SSD1306 I2C LED driver");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let sda = pins.gpio4;
    let scl = pins.gpio15;
    let di = ssd1306::I2CDisplayInterface::new(
        I2cDriver::new(
            peripherals.i2c0,
            sda,
            scl,
            &I2cConfig::new().baudrate((400 as u32).kHz().into()),
        )
        .unwrap(),
    );

    let mut reset = PinDriver::output(pins.gpio16).unwrap();

    reset.set_high().unwrap();
    Ets::delay_ms(1 as u32);

    reset.set_low().unwrap();
    Ets::delay_ms(10 as u32);

    reset.set_high().unwrap();

    log::info!("About to initialize the Heltec SSD1306 I2C LED driver");

    let mut display = ssd1306::Ssd1306::new(
        di,
        ssd1306::size::DisplaySize128x64,
        ssd1306::rotation::DisplayRotation::Rotate0,
    )
    .into_buffered_graphics_mode();

    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    loop {
        Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        display.flush().unwrap();
    }
}
