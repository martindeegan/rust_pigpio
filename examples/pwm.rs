extern crate rust_pigpio;

use std::thread::sleep;
use std::time::Duration;
use rust_pigpio::*;
use rust_pigpio::pwm::*;

const PIN: u32 = 21;

fn main() {
    let pigpio = Pigpio::new().unwrap();
    println!("Initialized pigpio. Version: {}", pigpio.version);
    pigpio.set_mode(PIN, OUTPUT).unwrap();
    println!("Mode set!");
    pigpio.write(PIN, OFF).unwrap();
    println!("Light off.");

    set_pwm_frequency(PIN, 500).unwrap(); // Set to modulate at 500hz.
    set_pwm_range(PIN, 1000).unwrap(); // Set range to 1000. 1 range = 2 us;

    pwm(PIN, 100).unwrap();
    println!("10%");

    sleep(Duration::from_secs(2));
    pwm(PIN, 250).unwrap();
    println!("25%");

    sleep(Duration::from_secs(2));
    pwm(PIN, 500).unwrap();
    println!("50%");

    sleep(Duration::from_secs(2));
    pwm(PIN, 735).unwrap();
    println!("73.5%");

    sleep(Duration::from_secs(2));
    pwm(PIN, 50).unwrap();
    println!("5%");

    sleep(Duration::from_secs(2));
    pwm(PIN, 1000).unwrap();
    println!("100%");

    sleep(Duration::from_secs(2));
    pigpio.write(PIN, OFF).unwrap();
    println!("Light off.");
}