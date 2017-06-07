extern crate rust_pigpio;

use std::thread::sleep;
use rust_pigpio::pigpio::*;

fn main() {
  initialize();
  terminate();
}
