#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub type GpioResult = Result<(), String>;
pub type GpioResponse = Result<u32, String>;

enum_from_primitive! {
    #[derive(Debug)]
    pub enum PigpioResponse {
        Ok = 0,
        InitFailed = -1,
        BAD_USER_GPIO = -2,
        BAD_GPIO = -3,
        BAD_MODE = -4,
        BAD_LEVEL = -5,
        BAD_PUD = -6,
        BAD_PULSEWIDTH = -7,
        BAD_DUTYCYCLE = -8,
        BAD_TIMER = -9,
        BAD_MS = -10,
        BAD_TIMETYPE = -11,
        BAD_SECONDS = -12,
        BAD_MICROS = -13,
        TIMER_FAILED = -14,
        BAD_WDOG_TIMEOUT = -15,
        NO_ALERT_FUNC = -16,
        BAD_CLK_PERIPH = -17,
        BAD_CLK_SOURCE = -18,
        BAD_CLK_MICROS = -19,
        BAD_BUF_MILLIS = -20,
        BAD_DUTYRANGE = -21,
        BAD_SIGNUM = -22,
        BAD_PATHNAME = -23,
        NO_HANDLE = -24,
        BAD_HANDLE = -25,
        BAD_IF_FLAGS = -26,
        BAD_CHANNEL = -27,
        BAD_SOCKET_PORT = -28,
        BAD_FIFO_COMMAND = -29,
        BAD_SECO_CHANNEL = -30,
        NOT_INITIALISED = -31,
        INITIALISED = -32,
        BAD_WAVE_MODE = -33,
        BAD_CFG_INTERNAL = -34,
        BAD_WAVE_BAUD = -35,
        TOO_MANY_PULSES = -36,
        TOO_MANY_CHARS = -37,
        NOT_SERIAL_GPIO = -38,
        BAD_SERIAL_STRUC = -39,
        BAD_SERIAL_BUF = -40,
        NOT_PERMITTED = -41,
        SOME_PERMITTED = -42,
        BAD_WVSC_COMMND = -43,
        BAD_WVSM_COMMND = -44,
        BAD_WVSP_COMMND = -45,
        BAD_PULSELEN = -46,
        BAD_SCRIPT = -47,
        BAD_SCRIPT_ID = -48,
        BAD_SER_OFFSET = -49,
        GPIO_IN_USE = -50,
        BAD_SERIAL_COUNT = -51,
        BAD_PARAM_NUM = -52,
        DUP_TAG = -53,
        TOO_MANY_TAGS = -54,
        BAD_SCRIPT_CMD = -55,
        BAD_VAR_NUM = -56,
        NO_SCRIPT_ROOM = -57,
        NO_MEMORY = -58,
        SOCK_READ_FAILED = -59,
        SOCK_WRIT_FAILED = -60,
        TOO_MANY_PARAM = -61,
        SCRIPT_NOT_READY = -62,
        BAD_TAG = -63,
        BAD_MICS_DELAY = -64,
        BAD_MILS_DELAY = -65,
        BAD_WAVE_ID = -66,
        TOO_MANY_CBS = -67,
        TOO_MANY_OOL = -68,
        EMPTY_WAVEFORM = -69,
        NO_WAVEFORM_ID = -70,
        I2C_OPEN_FAILED = -71,
        SER_OPEN_FAILED = -72,
        SOPEN_FAILED = -73,
        BAD_I2C_BUS = -74,
        BAD_I2C_ADDR = -75,
        BAD_SCHANNEL = -76,
        BAD_FLAGS = -77,
        BAD_SSPEED = -78,
        BAD_SER_DEVICE = -79,
        BAD_SER_SPEED = -80,
        BAD_PARAM = -81,
        I2C_WRITE_FAILED = -82,
        I2C_READ_FAILED = -83,
        BAD_SCOUNT = -84,
        SER_WRITE_FAILED = -85,
        SER_READ_FAILED = -86,
        SER_READ_NO_DATA = -87,
        UNKNOWN_COMMAND = -88,
        SXFER_FAILED = -89,
        BAD_POINTER = -90,
        NO_AUX_SPI = -91,
        NOT_PWM_GPIO = -92,
        NOT_SERVO_GPIO = -93,
        NOT_HCLK_GPIO = -94,
        NOT_HPWM_GPIO = -95,
        BAD_HPWM_FREQ = -96,
        BAD_HPWM_DUTY = -97,
        BAD_HCLK_FREQ = -98,
        BAD_HCLK_PASS = -99,
        HPWM_ILLEGAL = -100,
        BAD_DATABITS = -101,
        BAD_STOPBITS = -102,
        MSG_TOOBIG = -103,
        BAD_MALLOC_MODE = -104,
        TOO_MANY_SEGS = -105,
        BAD_I2C_SEG = -106,
        BAD_SMBUS_CMD = -107,
        NOT_I2C_GPIO = -108,
        BAD_I2C_WLEN = -109,
        BAD_I2C_RLEN = -110,
        BAD_I2C_CMD = -111,
        BAD_I2C_BAUD = -112,
        CHAIN_LOOP_CNT = -113,
        BAD_CHAIN_LOOP = -114,
        CHAIN_COUNTER = -115,
        BAD_CHAIN_CMD = -116,
        BAD_CHAIN_DELAY = -117,
        CHAIN_NESTING = -118,
        CHAIN_TOO_BIG = -119,
        DEPRECATED = -120,
        BAD_SER_INVERT = -121,
        BAD_EDGE = -122,
        BAD_ISR_INIT = -123,
        BAD_FOREVER = -124,
        BAD_FILTER = -125,
        BAD_PAD = -126,
        BAD_STRENGTH = -127,
        FIL_OPEN_FAILED = -128,
        BAD_FILE_MODE = -129,
        BAD_FILE_FLAG = -130,
        BAD_FILE_READ = -131,
        BAD_FILE_WRITE = -132,
        FILE_NOT_ROPEN = -133,
        FILE_NOT_WOPEN = -134,
        BAD_FILE_SEEK = -135,
        NO_FILE_MATCH = -136,
        NO_FILE_ACCESS = -137,
        FILE_IS_A_DIR = -138,
        BAD_SHELL_STATUS = -139,
        BAD_SCRIPT_NAME = -140,
        BAD_SBAUD = -141,
        NOT_SGPIO = -142,
        BAD_EVENT_ID = -143,
        UNKNOWN_RESULT = -666,
    }
}

/// Raw bindings. Must be wrapped in unsafe.
#[link(name = "pigpio", kind = "dylib")]
extern "C" {
    pub fn gpioInitialise() -> i32;
    pub fn gpioTerminate();

    pub fn gpioSetMode(gpio: u32, mode: u32) -> i32;
    pub fn gpioGetMode(gpio: u32) -> i32;
    pub fn gpioSetPullUpDown(gpio: u32, pud: u32) -> i32; //
    pub fn gpioRead(gpio: u32) -> i32;
    pub fn gpioWrite(gpio: u32, level: u32) -> i32;

    pub fn gpioDelay(micros: u32) -> u32;

    //    fn gpioSetAlertFunc(user_gpio: u32, f: gpioAlertFunc_t) -> i32;
    //    fn gpioSetAlertFuncEx(user_gpio: u32, f: gpioAlertFuncEx_t, void* userdata) -> i32;

    pub fn gpioTrigger(user_gpio: u32, pulseLen: u32, level: u32) -> i32; //
    pub fn gpioSetWatchdog(user_gpio: u32, timeout: u32) -> i32; //

    pub fn gpioPWM(user_gpio: u32, dutycycle: u32) -> i32;
    pub fn gpioGetPWMdutycycle(user_gpio: u32) -> i32;
    pub fn gpioSetPWMrange(user_gpio: u32, range: u32) -> i32;
    pub fn gpioGetPWMrange(user_gpio: u32) -> i32;
    pub fn gpioGetPWMrealRange(user_gpio: u32) -> i32;
    pub fn gpioSetPWMfrequency(user_gpio: u32, frequency: u32) -> i32;
    pub fn gpioGetPWMfrequency(user_gpio: u32) -> i32;

    pub fn gpioServo(user_gpio: u32, pulsewidth: u32) -> i32;
    pub fn gpioGetServoPulsewidth(user_gpio: u32) -> i32;

    pub fn gpioHardwareClock(gpio: u32, clkfreq: u32) -> i32;
    pub fn gpioHardwarePWM(gpio: u32, PWMfreq: u32, PWMduty: u32) -> i32;
}
