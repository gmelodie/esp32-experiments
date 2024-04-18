use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi};

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
        auth_method: AuthMethod::WPA3Personal,
        password: PASSWORD.try_into().unwrap(),
        channel: None,
        ..Default::default()
    });

    connect_wifi(&mut wifi, &wifi_configuration);

    let ip_info = wifi.wifi().sta_netif().get_ip_info().unwrap();
    log::info!("DHCP info: {:?}", ip_info);
}

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>, wifi_configuration: &Configuration) {
    wifi.set_configuration(wifi_configuration).unwrap();

    wifi.start().unwrap();
    log::info!("Wifi started");

    wifi.connect().unwrap();
    log::info!("Wifi connected");

    wifi.wait_netif_up().unwrap();
    log::info!("Wifi interface up");
}
