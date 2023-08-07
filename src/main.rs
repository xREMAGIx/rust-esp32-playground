use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;
use esp_idf_hal::{prelude::Peripherals, gpio::*, delay::{Ets, Delay}};
use dht11::Dht11;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Initialize the peripherals
    let peripherals = Peripherals::take().unwrap();
    
    // Configure the GPIO pin
    let dht11_pin = PinDriver::input_output_od(peripherals.pins.gpio4).expect("Failed to initialize GPIO");
    let mut dht11 = Dht11::new(dht11_pin);

    info!("Hello, world!");

    //Wait for the sensor to be ready
    println!("Waiting for sensor to be ready...");
    Delay::delay_ms(1000);

    loop {
        let mut dht11_delay = Ets;
        match dht11.perform_measurement(&mut dht11_delay) {
            Ok(measurement) => println!(
                "temp: {}C, humidity: {}%",
                (measurement.temperature as f32 / 10.0),
                (measurement.humidity as f32 / 10.0)
            ),
            Err(e) => println!("{:?}", e),
        }
        // Delay of at least 500ms before polling the sensor again, 1 second or more advised
        Delay::delay_ms(1000);
    }
}