extern crate rust_pigpio;

use std::thread::sleep;
use rust_pigpio::*;
const PIN: u32 = 21;

//Turns light on and off
fn main() {
  let pigpio = Pigpio::new().unwrap();
  println!("Initialized pigpio. Version: {}", pigpio.version);
  pigpio.set_mode(PIN, OUTPUT).unwrap();
  pigpio.write(PIN, ON).unwrap();
  sleep(std::time::Duration::from_secs(1));
  pigpio.write(PIN, OFF).unwrap();
  sleep(std::time::Duration::from_secs(1));
  pigpio.write(PIN, ON).unwrap();
  sleep(std::time::Duration::from_secs(1));
  pigpio.write(PIN, OFF).unwrap();
}
