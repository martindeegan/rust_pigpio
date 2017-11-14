#![crate_type = "lib"]
#![crate_name = "rust_pigpio"]
#![allow(dead_code)]
#![allow(non_snake_case)]

//! #Rust PiGPIO
//!
//! The Rust wrapper of the C library functions
pub mod pwm;
pub mod constants;

use std::string::String;

use constants::*;

const OK: i32 = 0;
const INIT_FAILED: i32 = -1;
const BAD_USER_GPIO: i32 = -2;
const BAD_GPIO: i32 = -3;
const BAD_MODE: i32 = -4;
const BAD_LEVEL: i32 = -5;
const BAD_PUD: i32 = -6;
const DEFAULT_ERROR: &str = "Unknown error.";

pub const INPUT: GpioMode = GpioMode::INPUT;
pub const OUTPUT: GpioMode = GpioMode::OUTPUT;

pub const ON: Level = Level::ON;
pub const OFF: Level = Level::OFF;

pub type GpioResult = Result<(), String>;
pub type GpioResponse = Result<u32, String>;

#[link(name = "pigpio", kind = "dylib")]
extern "C" {
    fn gpioInitialise() -> i32;
    fn gpioTerminate();

    fn gpioSetMode(gpio: u32, mode: u32) -> i32;
    fn gpioGetMode(gpio: u32) -> i32;
    fn gpioSetPullUpDown(gpio: u32, pud: u32) -> i32; //
    fn gpioRead(gpio: u32) -> i32;
    fn gpioWrite(gpio: u32, level: u32) -> i32;

    fn gpioDelay(micros: u32) -> u32;

//    fn gpioSetAlertFunc(user_gpio: u32, f: gpioAlertFunc_t) -> i32;
//    fn gpioSetAlertFuncEx(user_gpio: u32, f: gpioAlertFuncEx_t, void* userdata) -> i32;

    fn gpioTrigger(user_gpio: u32, pulseLen: u32, level: u32) -> i32; //
    fn gpioSetWatchdog(user_gpio: u32, timeout: u32) -> i32; //
}

#[derive(Debug)]
pub struct Pigpio {
    pub version: i32,
}

impl Drop for Pigpio {
    /// Terminates the library.
    ///
    /// This function resets the used DMA channels, releases memory, and terminates any running threads.
    fn drop(&mut self) {
        unsafe { gpioTerminate() };
    }
}

impl Pigpio {
    /// Initializes the library.
    pub fn new() -> Result<Pigpio, String> {
        let result = unsafe { gpioInitialise() };
        match result {
            INIT_FAILED => Err("Initialize failed".to_string()),
            _ => Ok(Pigpio{ version: result })
        }

    }

    /// Sets the GPIO mode, typically input or output.
    pub fn set_mode(&self, gpio: u32, mode: GpioMode) -> GpioResult {
        match unsafe { gpioSetMode(gpio, mode as u32) } {
            OK => Ok(()),
            BAD_GPIO => Err("Bad gpio".to_string()),
            BAD_MODE => Err("Bad mode".to_string()),
            _ => Err(DEFAULT_ERROR.to_string()),
        }
    }

    /// Gets the GPIO mode.
    pub fn get_mode(&self, gpio: u32) -> GpioResponse {
        let response = unsafe { gpioGetMode(gpio) };
        match response {
            BAD_GPIO => Err("Bad gpio".to_string()),
            _ => Ok(response as u32),
        }
    }

    /// Sets or clears resistor pull ups or downs on the GPIO.
    pub fn set_pull_up_down(&self, gpio: u32, pud: Pud) -> GpioResult {
        match unsafe { gpioSetPullUpDown(gpio, pud as u32) } {
            OK => Ok(()),
            BAD_GPIO => Err("Bad gpio".to_string()),
            BAD_PUD => Err("Bad pud".to_string()),
            _ => Err(DEFAULT_ERROR.to_string())
        }
    }

    /// Reads the GPIO level, on or off.
    pub fn read(&self, gpio: u32) -> GpioResponse {
        match unsafe { gpioRead(gpio) } {
            1 => Ok(1),
            0 => Ok(0),
            BAD_GPIO => Err("Bad gpio".to_string()),
            _ => Err(DEFAULT_ERROR.to_string()),
        }
    }

    /// Sets the GPIO level, on or off.
    /// If PWM or servo pulses are active on the GPIO they are switched off.
    pub fn write(&self, gpio: u32, level: Level) -> GpioResult {
        match unsafe { gpioWrite(gpio, level as u32) } {
            OK => Ok(()),
            BAD_GPIO => Err("Bad gpio".to_string()),
            BAD_LEVEL => Err("Bad level".to_string()),
            _ => Err(DEFAULT_ERROR.to_string()),
        }
    }

    /// Delays for at least the number of microseconds specified by microseconds.
    pub fn delay(&self, microseconds: u32) -> u32 {
        unsafe { gpioDelay(microseconds) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_gpio_mode_before_init_should_fail() {
        let response = unsafe { gpioSetMode(8, INPUT as u32) };

        assert_ne!(0, response);
    }

   #[test]
   fn set_mode_after_drop_should_fail() {
       {
           let pigpio = Pigpio::new().unwrap();
           pigpio.set_mode(8, INPUT).unwrap();
       }

       let response = unsafe { gpioSetMode(8, INPUT as u32) };

       assert_ne!(0, response);
   }
}
