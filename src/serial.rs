use GpioResult;
use GpioResponse;
use super::lib_constants::*;

use serial_core::{CharSize, PortSettings, SerialDevice};
use std::io;

#[link(name = "pigpio", kind = "dylib")]
extern "C" {
    fn gpioSerialReadOpen(user_gpio: u32, baud: u32, data_bits: u32) -> i32;
    fn gpioSerialReadInvert(user_gpio: u32, invert: u32) -> i32;
    fn gpioSerialRead(user_gpio: u32, buf: *mut u8, buf_size: u32) -> i32;
    fn gpioSerialReadClose(user_gpio: u32) -> i32;
}

/// Bit-banged Serial on a GPIO pin
pub struct GPIOSerial {
    pin: u32,
    settings: PortSettings,
    invert: bool,
}

impl GPIOSerial {
    pub fn new(user_gpio: u32, settings: PortSettings, invert: bool) -> Result<Self, String> {
        let data_bits = match settings.char_size {
            CharSize::Bits5 => 5,
            CharSize::Bits6 => 6,
            CharSize::Bits7 => 7,
            CharSize::Bits8 => 8,
        };
        match unsafe {
            gpioSerialReadOpen(user_gpio, settings.baud_rate.speed() as u32, data_bits)
        } {
            OK => {
                Ok(GPIOSerial {
                    pin: user_gpio,
                    settings,
                    invert,
                })
            }
            BAD_USER_GPIO => Err("Bad user gpio".to_string()),
            BAD_WAVE_BAUD => Err("Bad wave baud".to_string()),
            BAD_DATABITS => Err("Bad data bits".to_string()),
            GPIO_IN_USE => Err("Gpio in use".to_string()),
            _ => Err(DEFAULT_ERROR.to_string()),
        }.and_then(|s| match unsafe {
            gpioSerialReadInvert(user_gpio, invert as u32)
        } {
            OK => Ok(s),
            BAD_USER_GPIO => Err("Bad user gpio".to_string()),
            GPIO_IN_USE => Err("Gpio in use".to_string()),
            NOT_SERIAL_GPIO => Err("Not serial gpio".to_string()),
            BAD_SER_INVERT => Err("Bad serial invert parameter".to_string()),
            _ => Err(DEFAULT_ERROR.to_string()),
        })
    }
}

impl Drop for GPIOSerial {
    fn drop(&mut self) {
        unsafe { gpioSerialReadClose(self.pin) };
    }
}

impl io::Read for GPIOSerial {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match unsafe { gpioSerialRead(self.pin, buf.as_mut_ptr(), buf.len() as u32) } {
            BAD_USER_GPIO => Err(io::Error::new(io::ErrorKind::InvalidInput, "Bad user gpio")),
            NOT_SERIAL_GPIO => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Not serial gpio",
            )),
            read => Ok(read as usize),
        }
    }
}
