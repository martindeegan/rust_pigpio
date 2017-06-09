extern crate rust_pigpio;

use std::thread::sleep;
use std::time::Duration;
use rust_pigpio::pwm::*;

const PIN: u32 = 21;

fn main() {
    println!("Initialized pigpio. Version: {}", initialize().unwrap());
    set_mode(PIN, MODE_OUTPUT).unwrap();
    println!("Mode set!");
    write(PIN, 0).unwrap();
    pwm(PIN, 30).unwrap();
    sleep(Duration::from_secs(2));
    pwm(PIN, 50).unwrap();
    sleep(Duration::from_secs(2));
    pwm(PIN, 90).unwrap();
    sleep(Duration::from_secs(2));
    pwm(PIN, 130).unwrap();
    sleep(Duration::from_secs(2));
    pwm(PIN, 170).unwrap();
    sleep(Duration::from_secs(2));
    pwm(PIN, 255).unwrap();
    sleep(Duration::from_secs(2));
    terminate();
}