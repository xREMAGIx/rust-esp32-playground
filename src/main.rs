use esp_idf_hal::{prelude::Peripherals, delay::Delay};
use esp_idf_hal::gpio::*;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");

    // Get the peripherals
    let peripherals = Peripherals::take().expect("Failed to obtain peripherals");

    // Configure the GPIOs
    let mut led = PinDriver::output(peripherals.pins.gpio4).expect("Failed to initialize LED GPIO");
    let mut led1 = PinDriver::output(peripherals.pins.gpio5).expect("Failed to initialize LED GPIO");
    let mut button = PinDriver::input(peripherals.pins.gpio0).expect("Failed to initialize button GPIO");

    button.set_pull(Pull::Down).expect("Failed to set button pull-down");

    loop {
        // Blinky LED
        led.set_high().expect("Failed to set LED GPIO high");
        Delay::delay_ms(1000);

        led.set_low().expect("Failed to set LED GPIO low");
        Delay::delay_ms(1000);

        // Push button LED
        if button.is_high() {
            led1.set_high().expect("Failed to set LED GPIO high");
        } else {
            led1.set_low().expect("Failed to set LED GPIO low");
        }
    }
}
