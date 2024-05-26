#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::Delay;
use esp_backtrace as _;
use esp_hal::spi::master::prelude::_esp_hal_spi_master_dma_WithDmaSpi2;
use esp_hal::{
    clock::ClockControl,
    dma::{Dma, DmaPriority},
    dma_buffers, dma_descriptors, embassy,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    spi::{master::Spi, SpiMode},
    timer::TimerGroup,
    FlashSafeDma,
};
use esp_println::println;

// use embedded_hal_async::spi::SpiBus;
use embedded_hal_bus::spi::ExclusiveDevice;

use lora_phy::iv::GenericSx127xInterfaceVariant;
use lora_phy::lorawan_radio::LorawanRadio;
use lora_phy::sx127x::{self, Sx1276, Sx127x};
use lora_phy::LoRa;
use lorawan_device::async_device::{region, Device, EmbassyTimer, JoinMode};
use lorawan_device::default_crypto::DefaultFactory as Crypto;
use lorawan_device::{AppEui, AppKey, DevEui};

// warning: set these appropriately for the region
const LORAWAN_REGION: region::Region = region::Region::AU915;
const MAX_TX_POWER: u8 = 14;

#[main]
async fn main(_spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    // setup system timer
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    esp_println::logger::init_logger_from_env();

    embassy::init(&clocks, timg0);
    // let delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let sclk = io.pins.gpio5;
    let miso = io.pins.gpio19;
    let mosi = io.pins.gpio27;
    let cs = io.pins.gpio18.into_push_pull_output();
    let rst = io.pins.gpio14.into_push_pull_output();
    let dio1 = io.pins.gpio26.into_pull_down_input();

    let dma = Dma::new(peripherals.DMA);

    let dma_channel = dma.spi2channel;
    let (mut descriptors, mut rx_descriptors) = dma_descriptors!(32000);

    let spi = Spi::new(peripherals.SPI2, 100.kHz(), SpiMode::Mode0, &clocks)
        .with_sck(sclk)
        .with_mosi(mosi)
        .with_miso(miso)
        .with_dma(dma_channel.configure_for_async(
            false,
            &mut descriptors,
            &mut rx_descriptors,
            DmaPriority::Priority0,
        ));

    // let spi = FlashSafeDma::new(spi);
    let spi = ExclusiveDevice::new(spi, cs, Delay);

    let config = sx127x::Config {
        chip: Sx1276,
        tcxo_used: false,
        tx_boost: false,
        rx_boost: false,
    };

    println!("Setting up iv");

    let iv = GenericSx127xInterfaceVariant::new(rst, dio1, None, None).unwrap();
    println!("Setting up lora");
    let lora = LoRa::new(Sx127x::new(spi, iv, config), false, Delay)
        .await
        .unwrap();

    println!("Setting up radio");
    let radio: LorawanRadio<_, _, MAX_TX_POWER> = lora.into();
    let region: region::Configuration = region::Configuration::new(LORAWAN_REGION);

    println!("Setting up device");

    let mut device: Device<_, Crypto, _, _> = Device::new(
        region,
        radio,
        EmbassyTimer::new(),
        esp_hal::rng::Rng::new(peripherals.RNG),
    );

    println!("Joining LoRaWAN network");

    // TODO: Adjust the EUI and Keys according to your network credentials
    let resp = device
        .join(&JoinMode::OTAA {
            deveui: DevEui::from([0, 0, 0, 0, 0, 0, 0, 0]),
            appeui: AppEui::from([0, 0, 0, 0, 0, 0, 0, 0]),
            appkey: AppKey::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        })
        .await
        .unwrap();

    println!("LoRaWAN network joined: {:?}", resp);
}
