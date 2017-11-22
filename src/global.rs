use std::sync::{Arc,Mutex};
use std::collections::BTreeSet;
use c_bindings::*;

lazy_static! {
    pub static ref SINGLETON: Arc<Mutex<PigpioSingleton>> = Arc::new(Mutex::new(PigpioSingleton::new()));
}

pub struct PigpioSingleton {
    instances: BTreeSet<u32>,
    initialized: bool,
}

impl PigpioSingleton {
    pub fn new() -> PigpioSingleton {
        PigpioSingleton {
            instances: BTreeSet::new(),
            initialized: false,
        }
    }

    pub fn add_pin(&mut self, pin: u32) -> Result<(), &'static str> {
        if self.instances.contains(&pin) {
            return Err("Pin in use.");
        }
        if self.instances.len() == 0 && self.initialized == false {
            unsafe { gpioInitialise() };
            self.initialized = true;
        }
        self.instances.insert(pin);
        Ok(())
    }

    pub fn drop_pin(&mut self, pin: u32) {
        self.instances.remove(&pin);
        if self.instances.len() == 0 && self.initialized == true {
            unsafe { gpioTerminate() };
            self.initialized = false;
        }
    }
}
