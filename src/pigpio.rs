#![crate_type = "lib"]
#![crate_name = "rust_pigpio"]

use std::string::String;

#[link(name = "pigpio", kind = "dylib")]
#[link(name = "rt", kind = "dylib")]
extern {
  fn gpioInitialise() -> i32;
  fn gpioTerminate();

  fn gpioSetMode(gpio: u32, mode: u32) -> i32;
  fn gpioGetMode(gpio: u32) -> i32;
  fn gpioRead(gpio: u32) -> i32;
  fn gpioWrite(gpio: u32, level: u32) -> i32;
  fn gpioPWM(user_gpio: u32, dutycycle: u32) -> i32;
  fn gpioGetPWMdutycycle(user_gpio: u32) -> i32;
  fn gpioServo(user_gpio: u32, pulsewidth: u32) -> i32;
  fn gpioGetServoPulsewidth(user_gpio: u32) -> i32;
  fn gpioDelay(micros: u32) -> u32;

  fn gpioSetPWMrange(user_gpio: u32, range: u32) -> i32;
  fn gpioGetPWMrange(user_gpio: u32) -> i32;
  fn gpioSetPWMfrequency(user_gpio: u32, frequency: u32) -> i32;
  fn gpioGetPWMfrequency(user_gpio: u32) -> i32;

  fn gpioHardwareClock(gpio: u32, clkfreq: u32) -> i32;
  fn gpioHardwarePWM(gpio: u32, PWMfreq: u32, PWMduty: u32) -> i32;
}

//Response codes
const OK                  : i32 = 0;
const PI_INIT_FAILED      : i32 =-1;
const PI_BAD_USER_GPIO    : i32 =-2;
const PI_BAD_GPIO         : i32 =-3;
const PI_BAD_MODE         : i32 =-4;
const PI_BAD_LEVEL        : i32 =-5;
const PI_BAD_PUD          : i32 =-6;
const PI_BAD_PULSEWIDTH   : i32 =-7;
const PI_BAD_DUTYCYCLE    : i32 =-8;
const PI_BAD_TIMER        : i32 =-9;
const PI_BAD_MS           : i32 =-10;
const PI_BAD_TIMETYPE     : i32 =-11;
const PI_BAD_SECONDS      : i32 =-12;
const PI_BAD_MICROS       : i32 =-13;
const PI_TIMER_FAILED     : i32 =-14;
const PI_BAD_WDOG_TIMEOUT : i32 =-15;
const PI_NO_ALERT_FUNC    : i32 =-16;
const PI_BAD_CLK_PERIPH   : i32 =-17;
const PI_BAD_CLK_SOURCE   : i32 =-18;
const PI_BAD_CLK_MICROS   : i32 =-19;
const PI_BAD_BUF_MILLIS   : i32 =-20;
const PI_BAD_DUTYRANGE    : i32 =-21;
const PI_BAD_SIGNUM       : i32 =-22;
const PI_BAD_PATHNAME     : i32 =-23;
const PI_NO_HANDLE        : i32 =-24;
const PI_BAD_HANDLE       : i32 =-25;
const PI_BAD_IF_FLAGS     : i32 =-26;
const PI_BAD_CHANNEL      : i32 =-27;
const PI_BAD_PRIM_CHANNEL : i32 =-27;
const PI_BAD_SOCKET_PORT  : i32 =-28;
const PI_BAD_FIFO_COMMAND : i32 =-29;
const PI_BAD_SECO_CHANNEL : i32 =-30;
const PI_NOT_INITIALISED  : i32 =-31;
const PI_INITIALISED      : i32 =-32;
const PI_BAD_WAVE_MODE    : i32 =-33;
const PI_BAD_CFG_INTERNAL : i32 =-34;
const PI_BAD_WAVE_BAUD    : i32 =-35;
const PI_TOO_MANY_PULSES  : i32 =-36;
const PI_TOO_MANY_CHARS   : i32 =-37;
const PI_NOT_SERIAL_GPIO  : i32 =-38;
const PI_BAD_SERIAL_STRUC : i32 =-39;
const PI_BAD_SERIAL_BUF   : i32 =-40;
const PI_NOT_PERMITTED    : i32 =-41;
const PI_SOME_PERMITTED   : i32 =-42;
const PI_BAD_WVSC_COMMND  : i32 =-43;
const PI_BAD_WVSM_COMMND  : i32 =-44;
const PI_BAD_WVSP_COMMND  : i32 =-45;
const PI_BAD_PULSELEN     : i32 =-46;
const PI_BAD_SCRIPT       : i32 =-47;
const PI_BAD_SCRIPT_ID    : i32 =-48;
const PI_BAD_SER_OFFSET   : i32 =-49;
const PI_GPIO_IN_USE      : i32 =-50;
const PI_BAD_SERIAL_COUNT : i32 =-51;
const PI_BAD_PARAM_NUM    : i32 =-52;
const PI_DUP_TAG          : i32 =-53;
const PI_TOO_MANY_TAGS    : i32 =-54;
const PI_BAD_SCRIPT_CMD   : i32 =-55;
const PI_BAD_VAR_NUM      : i32 =-56;
const PI_NO_SCRIPT_ROOM   : i32 =-57;
const PI_NO_MEMORY        : i32 =-58;
const PI_SOCK_READ_FAILED : i32 =-59;
const PI_SOCK_WRIT_FAILED : i32 =-60;
const PI_TOO_MANY_PARAM   : i32 =-61;
const PI_SCRIPT_NOT_READY : i32 =-62;
const PI_BAD_TAG          : i32 =-63;
const PI_BAD_MICS_DELAY   : i32 =-64;
const PI_BAD_MILS_DELAY   : i32 =-65;
const PI_BAD_WAVE_ID      : i32 =-66;
const PI_TOO_MANY_CBS     : i32 =-67;
const PI_TOO_MANY_OOL     : i32 =-68;
const PI_EMPTY_WAVEFORM   : i32 =-69;
const PI_NO_WAVEFORM_ID   : i32 =-70;
const PI_I2C_OPEN_FAILED  : i32 =-71;
const PI_SER_OPEN_FAILED  : i32 =-72;
const PI_SPI_OPEN_FAILED  : i32 =-73;
const PI_BAD_I2C_BUS      : i32 =-74;
const PI_BAD_I2C_ADDR     : i32 =-75;
const PI_BAD_SPI_CHANNEL  : i32 =-76;
const PI_BAD_FLAGS        : i32 =-77;
const PI_BAD_SPI_SPEED    : i32 =-78;
const PI_BAD_SER_DEVICE   : i32 =-79;
const PI_BAD_SER_SPEED    : i32 =-80;
const PI_BAD_PARAM        : i32 =-81;
const PI_I2C_WRITE_FAILED : i32 =-82;
const PI_I2C_READ_FAILED  : i32 =-83;
const PI_BAD_SPI_COUNT    : i32 =-84;
const PI_SER_WRITE_FAILED : i32 =-85;
const PI_SER_READ_FAILED  : i32 =-86;
const PI_SER_READ_NO_DATA : i32 =-87;
const PI_UNKNOWN_COMMAND  : i32 =-88;
const PI_SPI_XFER_FAILED  : i32 =-89;
const PI_BAD_POINTER      : i32 =-90;
const PI_NO_AUX_SPI       : i32 =-91;
const PI_NOT_PWM_GPIO     : i32 =-92;
const PI_NOT_SERVO_GPIO   : i32 =-93;
const PI_NOT_HCLK_GPIO    : i32 =-94;
const PI_NOT_HPWM_GPIO    : i32 =-95;
const PI_BAD_HPWM_FREQ    : i32 =-96;
const PI_BAD_HPWM_DUTY    : i32 =-97;
const PI_BAD_HCLK_FREQ    : i32 =-98;
const PI_BAD_HCLK_PASS    : i32 =-99;
const PI_HPWM_ILLEGAL     : i32 =-100;
const PI_BAD_DATABITS     : i32 =-101;
const PI_BAD_STOPBITS     : i32 =-102;
const PI_MSG_TOOBIG       : i32 =-103;
const PI_BAD_MALLOC_MODE  : i32 =-104;
const PI_TOO_MANY_SEGS    : i32 =-105;
const PI_BAD_I2C_SEG      : i32 =-106;
const PI_BAD_SMBUS_CMD    : i32 =-107;
const PI_NOT_I2C_GPIO     : i32 =-108;
const PI_BAD_I2C_WLEN     : i32 =-109;
const PI_BAD_I2C_RLEN     : i32 =-110;
const PI_BAD_I2C_CMD      : i32 =-111;
const PI_BAD_I2C_BAUD     : i32 =-112;
const PI_CHAIN_LOOP_CNT   : i32 =-113;
const PI_BAD_CHAIN_LOOP   : i32 =-114;
const PI_CHAIN_COUNTER    : i32 =-115;
const PI_BAD_CHAIN_CMD    : i32 =-116;
const PI_BAD_CHAIN_DELAY  : i32 =-117;
const PI_CHAIN_NESTING    : i32 =-118;
const PI_CHAIN_TOO_BIG    : i32 =-119;
const PI_DEPRECATED       : i32 =-120;
const PI_BAD_SER_INVERT   : i32 =-121;
const PI_BAD_EDGE         : i32 =-122;
const PI_BAD_ISR_INIT     : i32 =-123;
const PI_BAD_FOREVER      : i32 =-124;
const PI_BAD_FILTER       : i32 =-125;
const PI_BAD_PAD          : i32 =-126;
const PI_BAD_STRENGTH     : i32 =-127;
const PI_FIL_OPEN_FAILED  : i32 =-128;
const PI_BAD_FILE_MODE    : i32 =-129;
const PI_BAD_FILE_FLAG    : i32 =-130;
const PI_BAD_FILE_READ    : i32 =-131;
const PI_BAD_FILE_WRITE   : i32 =-132;
const PI_FILE_NOT_ROPEN   : i32 =-133;
const PI_FILE_NOT_WOPEN   : i32 =-134;
const PI_BAD_FILE_SEEK    : i32 =-135;
const PI_NO_FILE_MATCH    : i32 =-136;
const PI_NO_FILE_ACCESS   : i32 =-137;
const PI_FILE_IS_A_DIR    : i32 =-138;
const PI_BAD_SHELL_STATUS : i32 =-139;
const PI_BAD_SCRIPT_NAME  : i32 =-140;
const PI_BAD_SPI_BAUD     : i32 =-141;
const PI_NOT_SPI_GPIO     : i32 =-142;
const PI_BAD_EVENT_ID     : i32 =-143;

//Modes
pub const PI_INPUT: u32 = 0;
pub const PI_OUTPUT: u32 = 1;
pub const ALT0: u32   = 4;
pub const ALT1: u32   = 5;
pub const ALT2: u32   = 6;
pub const ALT3: u32   = 7;
pub const ALT4: u32   = 3;
pub const ALT5: u32   = 2;

//Values
pub const PI_ON: i32 = 1;
pub const PI_OFF: i32 = 0;

type GPIO_Result = Result<(), String>;
type GPIO_Response = Result<i32, String>;

const DEFAULT_ERROR: &str = "Unknown error.";

//Initializes the pigpio library. Must be called before other pigpio library functions.
//Returns: Result<()>
pub fn initialize() -> GPIO_Result {
  match unsafe{ gpioInitialise() } {
    OK => Ok(()),
    PI_INIT_FAILED => Err("Initializ failed".to_string()),
    _ => Err(DEFAULT_ERROR.to_string())
  }
}

//Terminates the pigpio library.
pub fn terminate() {
  unsafe{ gpioTerminate() };
}

//modes:
//PI_INPUT, PI_OUTPUT, ALT(0-5)
pub fn setMode(gpio: u32, mode: u32) -> GPIO_Result {
  match unsafe{ gpioSetMode(gpio, mode) } {
    OK => Ok(()),
    PI_BAD_GPIO => Err("Bad gpio".to_string()),
    PI_BAD_MODE => Err("Bad mode".to_string()),
    _ => Err(DEFAULT_ERROR.to_string())
  }
}

pub fn getMode(gpio: u32) -> GPIO_Response {
  let response = unsafe{ gpioGetMode(gpio)};
  match response {
    PI_BAD_GPIO => Err("Bad gpio".to_string()),
    _ => Ok(response)
  }
}

// pub fn setPullUpDown(gpio: u32, pud: u32) -> Result<(), std::String> {
//   match unsafe{ gpioSetPullUpDown(gpio, pud) } {
//     OK => return Ok(()),
//     PI_BAD_GPIO => return Err("Bad gpio"),
//     PI_BAD_PUD => return Err("Bad pud")
//   }
// }

pub fn read(gpio: u32) -> GPIO_Response {
  match unsafe{ gpioRead(gpio) } {
    PI_ON => Ok(PI_ON),
    PI_OFF => Ok(PI_OFF),
    PI_BAD_GPIO => Err("Bad gpio".to_string()),
    _ => Err(DEFAULT_ERROR.to_string())
  }
}

pub fn write(gpio: u32, level: u32) -> GPIO_Result {
  match unsafe{ gpioWrite(gpio, level) } {
    OK => Ok(()),
    PI_BAD_GPIO => Err("Bad gpio".to_string()),
    PI_BAD_LEVEL => Err("Bad level".to_string()),
    _ => Err(DEFAULT_ERROR.to_string())
  }
}

pub fn PWM(user_gpio: u32, dutycycle: u32) -> GPIO_Result {
  match unsafe { gpioPWM(user_gpio, dutycycle) } {
    OK => Ok(()),
    PI_BAD_USER_GPIO => Err("Bad user gpio".to_string()),
    PI_BAD_DUTYCYCLE => Err("Bad dutycycle".to_string()),
    _ => Err(DEFAULT_ERROR.to_string())
  }
}

pub fn getPWMDutyCycle(user_gpio: u32) -> GPIO_Response {
  let result = unsafe{ gpioGetPWMdutycycle(user_gpio) };
  match result {
    PI_BAD_USER_GPIO => Err("Bad user gpio".to_string()),
    PI_NOT_PWM_GPIO => Err("Not PWM gpio".to_string()),
    _ => Ok(result)
  }
}

pub fn servo(user_gpio: u32, pulsewidth: u32) -> GPIO_Result {
  match unsafe{ gpioServo(user_gpio, pulsewidth) } {
    OK => Ok(()),
    PI_BAD_USER_GPIO => Err("Bad user gpio".to_string()),
    PI_BAD_PULSEWIDTH => Err("Bad pulse width".to_string()),
    _ => Err(DEFAULT_ERROR.to_string())
  }
}

pub fn getServoPulseWidth(user_gpio: u32) -> GPIO_Result {
  match unsafe{ gpioGetServoPulsewidth(user_gpio) } {
    OK => Ok(()),
    PI_BAD_USER_GPIO => Err("Bad user gpio".to_string()),
    PI_NOT_SERIAL_GPIO => Err("Not server gpio".to_string()),
    _ => Err(DEFAULT_ERROR.to_string())
  }
}

pub fn delay(microseconds: u32) -> u32 {
  unsafe { gpioDelay(microseconds) }
}

pub fn setPWMRange(user_gpio: u32, range: u32) -> GPIO_Result {
  match unsafe { gpioSetPWMrange(user_gpio, range) } {
    OK => Ok(()),
    PI_BAD_USER_GPIO => Err("Bad user gpio".to_string()),
    PI_BAD_DUTYRANGE => Err("Bad range".to_string()),
    _ => Err(DEFAULT_ERROR.to_string())
  }
}

pub fn getPWMRange(user_gpio: u32) -> GPIO_Response {
  let result = unsafe { gpioGetPWMrange(user_gpio) };
  match result {
    PI_BAD_USER_GPIO => Err("Bad user gpio".to_string()),
    _ => Ok(result)
  }
}

pub fn setPWMFrequency(user_gpio: u32, frequency: u32) -> GPIO_Result {
  match unsafe { gpioSetPWMfrequency(user_gpio, frequency) } {
    OK => Ok(()),
    PI_BAD_USER_GPIO => Err("Bad user gpio".to_string()),
    _ => Err(DEFAULT_ERROR.to_string())
  }
}

pub fn getPWMFrequency(user_gpio: u32) -> GPIO_Response {
  let result = unsafe { gpioGetPWMfrequency(user_gpio) };
  match result {
    PI_BAD_USER_GPIO => Err("Bad user gpio".to_string()),
    _ => Ok(result)
  }
}

pub fn hardwareClock(gpio: u32, frequency: u32) -> GPIO_Result {
  match unsafe { gpioHardwareClock(gpio, frequency) } {
    OK => Ok(()),
    PI_BAD_GPIO => Err("Bad gpio".to_string()),
    PI_NOT_HCLK_GPIO => Err("Not hardware clock gpio".to_string()),
    PI_BAD_HCLK_FREQ => Err("Bad hardware clock frequency".to_string()),
    PI_BAD_HCLK_PASS => Err("Bad hardware clock pass".to_string()),
    _ => Err(DEFAULT_ERROR.to_string())
  }
}

pub fn hardwarePWM(gpio: u32, frequency: u32, dutycycle: u32) -> GPIO_Result {
  match unsafe { gpioHardwarePWM(gpio, frequency, dutycycle) } {
    OK => Ok(()),
    PI_BAD_GPIO => Err("Bad gpio".to_string()),
    PI_BAD_HPWM_DUTY => Err("Bad gpio".to_string()),
    PI_BAD_HPWM_FREQ => Err("Bad gpio".to_string()),
    PI_HPWM_ILLEGAL => Err("Bad gpio".to_string()),
    PI_NOT_HPWM_GPIO => Err("Bad gpio".to_string()),
    _ => Err(DEFAULT_ERROR.to_string())
  }
}