extern crate rust_pigpio;

use std::thread::sleep;
use rust_pigpio::pigpio::*;

const PIN: u32 = 21;

fn main() {
  initialize();
  println!("Initialized!");
  setMode(PIN, PI_OUTPUT).unwrap();
  println!("Mode set!");
  write(PIN, 0).unwrap();
  PWM(PIN, 30).unwrap();
  sleep(std::time::Duration::from_secs(2));
  PWM(PIN, 50).unwrap();
  sleep(std::time::Duration::from_secs(2));
  PWM(PIN, 90).unwrap();
  sleep(std::time::Duration::from_secs(2));
  PWM(PIN, 130).unwrap();
  sleep(std::time::Duration::from_secs(2));
  PWM(PIN, 170).unwrap();
  sleep(std::time::Duration::from_secs(2));
  PWM(PIN, 255).unwrap();
  sleep(std::time::Duration::from_secs(2));
  terminate();
}