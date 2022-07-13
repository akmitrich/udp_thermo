#![allow(unused, dead_code)]

pub trait ThermoObserver {
    fn observe(&self, thermo: &Thermometer);
}

pub struct Thermometer {
    temperature: f64,
    observers: Vec<Box<dyn ThermoObserver>>,
}

impl Thermometer {
    pub fn new(temperature: f64) -> Self {
        let observers = vec![];
        Self {
            temperature,
            observers,
        }
    }

    pub fn add_observer(&mut self, observer: impl ThermoObserver + 'static) {
        self.observers.push(Box::new(observer));
    }

    pub fn get_temperature(&self) -> f64 {
        self.temperature
    }

    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = temperature;
        for observer in self.observers.iter() {
            observer.observe(self);
        }
    }
}
