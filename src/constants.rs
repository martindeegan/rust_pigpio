
pub enum GpioMode {
    INPUT = 0,
    OUTPUT = 1,
    ALT0 = 4,
    ALT1 = 5,
    ALT2 = 6,
    ALT3 = 7,
    ALT4 = 3,
    ALT5 = 2,
}

pub enum Pud {
    OFF = 0,
    DOWN = 1,
    UP = 2,
}

pub enum Level {
    ON = 1,
    OFF = 0,
}
