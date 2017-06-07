extern crate rust_pigpio;

use std::thread::sleep;
use rust_pigpio::pigpio::*;

const PIN: u32 = 21;

fn main() {
  initialize();
  setMode(PIN, PI_OUTPUT).unwrap();
  write(PIN, 1).unwrap();
  sleep(std::time::Duration::from_secs(1));
  write(PIN, 0).unwrap();
  sleep(std::time::Duration::from_secs(1));
  write(PIN, 1).unwrap();
  sleep(std::time::Duration::from_secs(1));
  write(PIN, 0).unwrap();
  sleep(std::time::Duration::from_secs(1));
  terminate();
}
