extern crate rust-pigpio;

use std::thread::sleep;
use rust_pigpio::pigpio::*;

const PIN: u32 = 21;

fn main() {
    initialize();
    println!("Initialized!");
    set_mode(PIN, PI_OUTPUT).unwrap();
    println!("Mode set!");
    write(PIN, 0).unwrap();
    pwm(PIN, 30).unwrap();
    sleep(std::time::Duration::from_secs(2));
    pwm(PIN, 50).unwrap();
    sleep(std::time::Duration::from_secs(2));
    pwm(PIN, 90).unwrap();
    sleep(std::time::Duration::from_secs(2));
    pwm(PIN, 130).unwrap();
    sleep(std::time::Duration::from_secs(2));
    pwm(PIN, 170).unwrap();
    sleep(std::time::Duration::from_secs(2));
    pwm(PIN, 255).unwrap();
    sleep(std::time::Duration::from_secs(2));
    terminate();
}