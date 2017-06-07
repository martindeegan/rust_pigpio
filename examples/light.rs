extern crate rust_pigpio;

use std::thread::sleep;
use rust_pigpio::pigpio::*;

const PIN: u32 = 21;

//Turns light on and off
fn main() {
  println!("Initialized pigpio. Version: {}", initialize().unwrap());
  set_mode(PIN, MODE_OUTPUT).unwrap();
  write(PIN, PI_ON).unwrap();
  sleep(std::time::Duration::from_secs(1));
  write(PIN, PI_OFF).unwrap();
  sleep(std::time::Duration::from_secs(1));
  write(PIN, PI_ON).unwrap();
  sleep(std::time::Duration::from_secs(1));
  write(PIN, PI_OFF).unwrap();
  terminate();
}
