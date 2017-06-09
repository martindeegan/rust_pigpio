use GpioResult;
use GpioResponse;

const OK: i32 = 0;
const BAD_USER_GPIO: i32 = -2;
const BAD_GPIO: i32 = -3;
const BAD_PULSEWIDTH: i32 = -7;
const BAD_DUTYCYCLE: i32 = -8;
const BAD_DUTYRANGE: i32 = -21;
const NOT_SERIAL_GPIO: i32 = -38;
const NOT_PWM_GPIO: i32 = -92;
const NOT_SERVO_GPIO: i32 = -93;

const DEFAULT_ERROR: &str = "Unknown error.";


#[link(name = "pigpio", kind = "dylib")]
extern "C" {
    fn gpioPWM(user_gpio: u32, dutycycle: u32) -> i32;
    fn gpioGetPWMdutycycle(user_gpio: u32) -> i32;
    fn gpioSetPWMrange(user_gpio: u32, range: u32) -> i32;
    fn gpioGetPWMrange(user_gpio: u32) -> i32;
    fn gpioGetPWMrealRange(user_gpio: u32) -> i32;
    fn gpioSetPWMfrequency(user_gpio: u32, frequency: u32) -> i32;
    fn gpioGetPWMfrequency(user_gpio: u32) -> i32;

    fn gpioServo(user_gpio: u32, pulsewidth: u32) -> i32;
    fn gpioGetServoPulsewidth(user_gpio: u32) -> i32;

    fn gpioHardwareClock(gpio: u32, clkfreq: u32) -> i32;
    fn gpioHardwarePWM(gpio: u32, PWMfreq: u32, PWMduty: u32) -> i32;
}

/// Starts PWM on the GPIO, dutycycle between 0 (off) and range (fully on). Range defaults to 255.
///
/// This and the servo functionality use the DMA and PWM or PCM peripherals to control and schedule the pulse lengths and dutycycles.
/// The 'set_pwm_range' function may be used to change the default range of 255.
///
/// The real value set by gpioPWM is (dutycycle * real range) / range.
pub fn pwm(user_gpio: u32, dutycycle: u32) -> GpioResult {
    match unsafe { gpioPWM(user_gpio, dutycycle) } {
        OK => Ok(()),
        BAD_USER_GPIO => Err("Bad user gpio".to_string()),
        BAD_DUTYCYCLE => Err("Bad dutycycle".to_string()),
        _ => Err(DEFAULT_ERROR.to_string()),
    }
}

/// Returns the PWM dutycycle setting for the GPIO.
///
/// For normal PWM the dutycycle will be out of the defined range for the GPIO (see 'get_pwm_range').
/// If a hardware clock is active on the GPIO the reported dutycycle will be 500000 (500k) out of 1000000 (1M).
/// If hardware PWM is active on the GPIO the reported dutycycle will be out of a 1000000 (1M).
/// Normal PWM range defaults to 255.
pub fn get_pwm_duty_cycle(user_gpio: u32) -> GpioResponse {
    let result = unsafe { gpioGetPWMdutycycle(user_gpio) };
    match result {
        BAD_USER_GPIO => Err("Bad user gpio".to_string()),
        NOT_PWM_GPIO => Err("Not PWM gpio".to_string()),
        _ => Ok(result as u32),
    }
}

/// Starts servo pulses on the GPIO, 0 (off), 500 (most anti-clockwise) to 2500 (most clockwise).
///
/// The range supported by servos varies and should probably be determined by experiment. A value of 1500 should always be safe and represents the mid-point of rotation. You can DAMAGE a servo if you command it to move beyond its limits.
/// The following causes an on pulse of 1500 microseconds duration to be transmitted on GPIO 17 at a rate of 50 times per second. This will command a servo connected to GPIO 17 to rotate to its mid-point.
///
/// This function updates servos at 50Hz. If you wish to use a different update frequency you will have to use the PWM functions.
/// Firstly set the desired PWM frequency using 'set_pwm_frequency'.
/// Then set the PWM range using set_pwm_range to 1E6/frequency. Doing this allows you to use units of microseconds when setting the servo pulsewidth.
/// ~~~
/// set_pwm_frequency(25, 400);
/// set_pwm_range(25, 2500);
/// ~~~
/// Thereafter use the PWM command to move the servo, e.g. 'pwm(25, 1500)' will set a 1500 us pulse.
pub fn servo(user_gpio: u32, pulse_width: u32) -> GpioResult {
    match unsafe { gpioServo(user_gpio, pulse_width) } {
        OK => Ok(()),
        BAD_USER_GPIO => Err("Bad user gpio".to_string()),
        BAD_PULSEWIDTH => Err("Bad pulse width".to_string()),
        _ => Err(DEFAULT_ERROR.to_string()),
    }
}

/// Returns the servo pulsewidth setting for the GPIO.
pub fn get_servo_pulse_width(user_gpio: u32) -> GpioResponse {
    let result = unsafe { gpioGetServoPulsewidth(user_gpio) };
    match  result {
        BAD_USER_GPIO => Err("Bad user gpio".to_string()),
        NOT_SERIAL_GPIO => Err("Not server gpio".to_string()),
        _ => Ok(result as u32),
    }
}



/// Selects the dutycycle range to be used for the GPIO.
///
/// Subsequent calls to pwm will use a dutycycle between 0 (off) and range (fully on).
pub fn set_pwm_range(user_gpio: u32, range: u32) -> GpioResponse {
    let result = unsafe { gpioSetPWMrange(user_gpio, range) };
    match result {
        BAD_USER_GPIO => Err("Bad user gpio".to_string()),
        BAD_DUTYRANGE => Err("Bad range".to_string()),
        _ => Ok(result as u32)
    }
}

/// Returns the dutycycle range used for the GPIO.
///
/// If a hardware clock or hardware PWM is active on the GPIO the reported range will be 1000000 (1M).
pub fn get_pwm_range(user_gpio: u32) -> GpioResponse {
    let result = unsafe { gpioGetPWMrange(user_gpio) };
    match result {
        BAD_USER_GPIO => Err("Bad user gpio".to_string()),
        _ => Ok(result as u32),
    }
}

/// Returns the real range used for the GPIO.
///
/// If a hardware clock is active on the GPIO the reported real range will be 1000000 (1M).
/// If hardware PWM is active on the GPIO the reported real range will be approximately 250M divided by the set PWM frequency.
pub fn get_pwm_real_range(user_gpio: u32) -> GpioResponse {
    let result = unsafe { gpioGetPWMrealRange(user_gpio) };
    match result {
        BAD_USER_GPIO => Err("Bad user gpio".to_string()),
        _ => Ok(result as u32),
    }
}

/// Sets the frequency in hertz to be used for the GPIO.
pub fn set_pwm_frequency(user_gpio: u32, frequency: u32) -> GpioResponse {
    let result = unsafe { gpioSetPWMfrequency(user_gpio, frequency) };
    match result {
        BAD_USER_GPIO => Err("Bad user gpio".to_string()),
        _ => Ok(result as u32),
    }
}

/// Returns the frequency in hertz used for the GPIO.
pub fn get_pwm_frequency(user_gpio: u32) -> GpioResponse {
    let result = unsafe { gpioGetPWMfrequency(user_gpio) };
    match result {
        BAD_USER_GPIO => Err("Bad user gpio".to_string()),
        _ => Ok(result as u32),
    }
}

const NOT_HCLK_GPIO: i32 = -94;
const NOT_HPWM_GPIO: i32 = -95;
const BAD_HPWM_FREQ: i32 = -96;
const BAD_HPWM_DUTY: i32 = -97;
const BAD_HCLK_FREQ: i32 = -98;
const BAD_HCLK_PASS: i32 = -99;
const HPWM_ILLEGAL: i32 = -100;

/// Starts a hardware clock on a GPIO at the specified frequency. Frequencies above 30MHz are unlikely to work.
///
/// The same clock is available on multiple GPIO. The latest frequency setting will be used by all GPIO which share a clock.
/// The GPIO must be one of the following:
/// ~~~
/// 4   clock 0  All models
/// 5   clock 1  All models but A and B (reserved for system use)
/// 6   clock 2  All models but A and B
/// 20  clock 0  All models but A and B
/// 21  clock 1  All models but A and Rev.2 B (reserved for system use)
///
/// 32  clock 0  Compute module only
/// 34  clock 0  Compute module only
/// 42  clock 1  Compute module only (reserved for system use)
/// 43  clock 2  Compute module only
/// 44  clock 1  Compute module only (reserved for system use)
/// ~~~
/// Access to clock 1 is protected by a password as its use will likely crash the Pi. The password is given by or'ing 0x5A000000 with the GPIO number.
pub fn hardware_clock(gpio: u32, frequency: u32) -> GpioResult {
    match unsafe { gpioHardwareClock(gpio, frequency) } {
        OK => Ok(()),
        BAD_GPIO => Err("Bad gpio".to_string()),
        NOT_HCLK_GPIO => Err("Not hardware clock gpio".to_string()),
        BAD_HCLK_FREQ => Err("Bad hardware clock frequency".to_string()),
        BAD_HCLK_PASS => Err("Bad hardware clock pass".to_string()),
        _ => Err(DEFAULT_ERROR.to_string()),
    }
}


/// Starts hardware PWM on a GPIO at the specified frequency and dutycycle. Frequencies above 30MHz are unlikely to work.
///
//NOTE: Any waveform started by gpioWaveTxSend, or gpioWaveChain will be cancelled.
// This function is only valid if the pigpio main clock is PCM. The main clock defaults to PCM but may be overridden by a call to gpioCfgClock.
///
/// The same PWM channel is available on multiple GPIO. The latest frequency and dutycycle setting will be used by all GPIO which share a PWM channel.
/// The GPIO must be one of the following:
///
/// ~~~
/// 12  PWM channel 0  All models but A and B
/// 13  PWM channel 1  All models but A and B
/// 18  PWM channel 0  All models
/// 19  PWM channel 1  All models but A and B
///
/// 40  PWM channel 0  Compute module only
/// 41  PWM channel 1  Compute module only
/// 45  PWM channel 1  Compute module only
/// 52  PWM channel 0  Compute module only
/// 53  PWM channel 1  Compute module only
/// ~~~
pub fn hardware_pwm(gpio: u32, frequency: u32, dutycycle: u32) -> GpioResult {
    match unsafe { gpioHardwarePWM(gpio, frequency, dutycycle) } {
        OK => Ok(()),
        BAD_GPIO => Err("Bad gpio".to_string()),
        BAD_HPWM_DUTY => Err("Bad gpio".to_string()),
        BAD_HPWM_FREQ => Err("Bad gpio".to_string()),
        HPWM_ILLEGAL => Err("Bad gpio".to_string()),
        NOT_HPWM_GPIO => Err("Bad gpio".to_string()),
        _ => Err(DEFAULT_ERROR.to_string()),
    }
}
