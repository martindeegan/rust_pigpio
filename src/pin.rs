use global::SINGLETON;
use c_bindings;
use c_bindings::PigpioResponse;
use enum_primitive::FromPrimitive;

type PigpioResult<T> = Result<T, PigpioResponse>;

enum_from_primitive! {
pub enum GpioMode {
    INPUT = 0,
    OUTPUT = 1,
    ALT0 = 4,
    ALT1 = 5,
    ALT2 = 6,
    ALT3 = 7,
    ALT4 = 3,
    ALT5 = 2
}
}

enum_from_primitive! {
pub enum Pud {
    OFF = 0,
    DOWN = 1,
    UP = 2
}
}

enum_from_primitive! {
pub enum Level {
    ON = 1,
    OFF = 0
}
}

pub struct Pin {
    pin: u32,
}

const DEFAULT_ERROR: &str = "Unknown error.";

impl Pin {
    pub fn new(pin: u32) -> Pin {
        SINGLETON.lock().unwrap().add_pin(pin);
        Pin { pin: pin }
    }

    /// Sets the GPIO mode, typically input or output.
    /// http://abyz.me.uk/rpi/pigpio/cif.html#gpioSetMode
    pub fn set_mode(&self, mode: GpioMode) -> PigpioResult<()> {
        let response = unsafe { PigpioResponse::from_i32(c_bindings::gpioSetMode(self.pin, mode as u32)) };
        match response {
            Some(PigpioResponse::Ok) => Ok(()),
            Some(_) => Err(response.unwrap()),
            None => Err(PigpioResponse::UNKNOWN_RESULT),
        }
    }
    
    /// Gets the GPIO mode.
    fn get_mode(&self) -> PigpioResult<GpioMode> {
        let response = unsafe { c_bindings::gpioGetMode(self.pin) };
        let mode = GpioMode::from_i32(response);
        match mode {
            Some(_) => Ok(mode.unwrap()),
            None => Err(PigpioResponse::from_i32(response).unwrap()),
        }
    }

    // /// Sets or clears resistor pull ups or downs on the GPIO.
    // fn set_pull_up_down(&self, pud: Pud) -> GpioResult {
    //     match unsafe { c_bindings::gpioSetPullUpDown(self.pin, pud as u32) } {
    //         OK => Ok(()),
    //         BAD_GPIO => Err("Bad gpio".to_string()),
    //         BAD_PUD => Err("Bad pud".to_string()),
    //         _ => Err(DEFAULT_ERROR.to_string())
    //     }
    // }

    // /// Reads the GPIO level, on or off.
    // fn read(&self) -> GpioResponse {
    //     match unsafe { c_bindings::gpioRead(self.pin) } {
    //         1 => Ok(1),
    //         0 => Ok(0),
    //         BAD_GPIO => Err("Bad gpio".to_string()),
    //         _ => Err(DEFAULT_ERROR.to_string()),
    //     }
    // }

    // /// Sets the GPIO level, on or off.
    // /// If PWM or servo pulses are active on the GPIO they are switched off.
    // fn write(&self, gpio: u32, level: Level) -> GpioResult {
    //     match unsafe { c_bindings::gpioWrite(gpio, level as u32) } {
    //         OK => Ok(()),
    //         BAD_GPIO => Err("Bad gpio".to_string()),
    //         BAD_LEVEL => Err("Bad level".to_string()),
    //         _ => Err(DEFAULT_ERROR.to_string()),
    //     }
    // }

    // /// Delays for at least the number of microseconds specified by microseconds.
    // fn delay(&self, microseconds: u32) -> u32 {
    //     unsafe { c_bindings::gpioDelay(microseconds) }
    // }


    // /// Sets or clears resistor pull ups or downs on the GPIO.
    // pub fn set_pull_up_down(&self);
    // /// Reads the GPIO level, on or off.
    // fn read(&self, gpio: u32) -> GpioResponse {
    // /// Sets the GPIO level, on or off.
    // /// If PWM or servo pulses are active on the GPIO they are switched off.
    // fn write(&self, gpio: u32, level: Level) -> GpioResult {
    // /// Delays for at least the number of microseconds specified by microseconds.
    // fn delay(&self, microseconds: u32) -> u32 {

}

impl Drop for Pin {
    fn drop(&mut self) {
        SINGLETON.lock().unwrap().drop_pin(self.pin);
    }
}


// fn new() -> Result<Pigpio, String> {
//         let result = unsafe { gpioInitialise() };
//         match result {
//             INIT_FAILED => Err("Initialize failed".to_string()),
//             _ => Ok(Pigpio{ version: result })
//         }

//     }

    // /// Sets the GPIO mode, typically input or output.
    // fn set_mode(&self, gpio: u32, mode: GpioMode) -> GpioResult {
    //     match unsafe { gpioSetMode(gpio, mode as u32) } {
    //         OK => Ok(()),
    //         BAD_GPIO => Err("Bad gpio".to_string()),
    //         BAD_MODE => Err("Bad mode".to_string()),
    //         _ => Err(),
    //     }
    // }

