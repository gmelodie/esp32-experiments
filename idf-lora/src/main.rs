use esp_idf_svc::hal::gpio::Gpio18;
use esp_idf_svc::hal::{
    delay::{Ets, FreeRtos},
    gpio::PinDriver,
    // gpio::Pins,
    prelude::*,
    spi::config::{Config, DriverConfig},
    spi::{SpiDeviceDriver, SpiDriver},
};

const BAND: i64 = 866;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let spi = peripherals.spi2;

    let config = Config::new().baudrate(200.kHz().into());

    let sclk = peripherals.pins.gpio5;
    let miso = peripherals.pins.gpio19;
    let mosi = peripherals.pins.gpio27;
    let cs = peripherals.pins.gpio18;
    let rst = peripherals.pins.gpio14;

    let driver = SpiDriver::new(spi, sclk, mosi, Some(miso), &DriverConfig::new()).unwrap();
    let spi_device = SpiDeviceDriver::new(driver, Option::<Gpio18>::None, &config).unwrap();
    let config = sx127x::Config {
        chip: Sx1276,
        tcxo_used: true,
        rx_boost: false,
        tx_boost: false,
    };
    let iv = GenericSx127xInterfaceVariant::new(reset, irq, None, None).unwrap();
    let lora = LoRa::new(Sx127x::new(spi, iv, config), true, Delay)
        .await
        .unwrap();

    let radio: LorawanRadio<_, _, MAX_TX_POWER> = lora.into();
    let region: region::Configuration = region::Configuration::new(LORAWAN_REGION);
}
