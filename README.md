# esp32-experiments
My experiments with ESP32 (mostly in Rust hehe)

Install dependencies
```bash
sudo apt install libudev-dev
cargo install espup ldproxy espflash cargo-generate cargo-espflash
espup install
source ~/export-esp.sh
```

Generate new project with (and fill up project name, etc.)
```bash
cargo generate esp-rs/esp-idf-template cargo
```

## Ideas
1. ESP takes commands from serial port + CLI to send commands to the serial port
    1. Wifi connect <ssid> + <passwd>: ESP saves wifi information in a file to reconnect
    2. HTTP server: ESP spawns an HTTP Server
