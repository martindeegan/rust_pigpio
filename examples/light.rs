extern crate rust_pigpio;

use std::thread::sleep;
use rust_pigpio::pigpio::*;

const PIN: u32 = 21;

//Turns light on and off
fn main() {
  println!("Initialized pigpio. Version: {}", initialize().unwrap());
  set_mode(PIN, PI_OUTPUT).unwrap();
  write(PIN, 1).unwrap();
  sleep(std::time::Duration::from_secs(1));
  write(PIN, 0).unwrap();
  sleep(std::time::Duration::from_secs(1));
  write(PIN, 1).unwrap();
  sleep(std::time::Duration::from_secs(1));
  write(PIN, 0).unwrap();
  terminate();
}
