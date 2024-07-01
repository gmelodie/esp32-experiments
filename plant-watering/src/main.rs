use chrono::{DateTime, Duration, Utc};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::sntp::{EspSntp, SyncStatus};
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi};
use std::time::SystemTime;

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs)).unwrap(),
        sys_loop,
    )
    .unwrap();

    let wifi_configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        bssid: None,
        // auth_method: AuthMethod::WPA3Personal,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.try_into().unwrap(),
        channel: None,
        ..Default::default()
    });

    let mut led = PinDriver::output(peripherals.pins.gpio2).unwrap();
    led.set_low().unwrap();

    connect_wifi(&mut wifi, &wifi_configuration);
    led.set_high().unwrap();

    let ip_info = wifi.wifi().sta_netif().get_ip_info().unwrap();
    log::info!("DHCP info: {:?}", ip_info);

    // Create Handle and Configure SNTP
    let ntp = EspSntp::new_default().unwrap();

    // Synchronize NTP
    log::info!("Synchronizing with NTP Server");
    while ntp.get_sync_status() != SyncStatus::Completed {}
    log::info!("Time Sync Completed");

    let mut water = PinDriver::output(peripherals.pins.gpio15).unwrap();

    loop {
        // Obtain System Time
        let st_now = SystemTime::now();
        // Convert to dateTime
        let mut dt_now: DateTime<Utc> = st_now.clone().into();
        dt_now = dt_now - Duration::hours(3);

        let formatted = format!("{}", dt_now.format("%d/%m/%Y %H:%M:%S"));
        println!("{}", formatted);

        water.set_high().unwrap();
        println!("Watering plants for 30s (pin D15)");

        FreeRtos::delay_ms(30_000); // 30s on

        water.set_low().unwrap();
        println!("Shutting down");

        FreeRtos::delay_ms(30_000); // 30s off

        // TODO: 1. check if last_watered file exsits
        //          if present:
        //              read dateTime
        //              parse datetime
        //              if now - inFile < 24h:
        //                  wait remaining time (if any)
        //
        //          truncate file
        //          put current datetime in file
        //          water plants
        //          sleep for a day (with led on)
    }
}

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>, wifi_configuration: &Configuration) {
    wifi.set_configuration(wifi_configuration).unwrap();

    wifi.start().unwrap();
    log::info!("Wifi started");

    while wifi.connect().is_err() {
        log::info!("Error connecting to Wifi, sleeping for 10mins");
        FreeRtos::delay_ms(600_000); // 10mins == 600s == 600_000 ms
    }
    log::info!("Wifi connected");

    wifi.wait_netif_up().unwrap();
    log::info!("Wifi interface up");
}
