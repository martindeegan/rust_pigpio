use GpioResult;
use GpioResponse;
use super::lib_constants::*;

#[link(name = "pigpio", kind = "dylib")]
extern "C" {
    fn gpioSerialReadOpen(user_gpio: u32, baud: u32, data_bits: u32) -> i32;
    fn gpioSerialReadInvert(user_gpio: u32, invert: u32) -> i32;
    fn gpioSerialRead(user_gpio: u32, buf: *const u32, buf_size: u32) -> i32;
    fn gpioSerialReadClose(user_gpio: u32) -> i32;
}

/// Starts bit-banging serial on GPIO
pub fn serial(user_gpio: u32, baud: u32, data_bits: u32) -> GpioResult {
    match unsafe { gpioSerialReadOpen(user_gpio, baud, data_bits) } {
        OK => Ok(()),
        BAD_USER_GPIO => Err("Bad user gpio".to_string()),
        BAD_WAVE_BAUD => Err("Bad wave baud".to_string()),
        BAD_DATABITS => Err("Bad data bits".to_string()),
        GPIO_IN_USE => Err("Gpio in use".to_string()),
        _ => Err(DEFAULT_ERROR.to_string()),
    }
}
