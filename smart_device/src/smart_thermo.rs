#![allow(unused, dead_code)]

use std::{
    error::Error,
    mem::size_of,
    net::{ToSocketAddrs, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};

pub struct SmartThermo {
    temperature: Arc<MutTemperature>,
    finished: Arc<AtomicBool>,
}

impl SmartThermo {
    pub fn new(addr: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let temperature = Arc::new(MutTemperature(Mutex::new(0.)));
        let finished = Arc::new(AtomicBool::new(false));
        let t_clone = temperature.clone();
        let f_clone = finished.clone();

        let socket = UdpSocket::bind(addr)?;
        thread::spawn(move || loop {
            if f_clone.load(Ordering::SeqCst) {
                break;
            }
            let mut buf = [0; size_of::<f64>()];
            socket.recv_from(&mut buf).unwrap();
            t_clone.set(f64::from_be_bytes(buf));
        });

        Ok(Self {
            temperature,
            finished,
        })
    }

    pub fn get_temperature(&self) -> f64 {
        self.temperature.get()
    }
}

impl Drop for SmartThermo {
    fn drop(&mut self) {
        self.finished.store(true, Ordering::SeqCst);
    }
}

struct MutTemperature(Mutex<f64>);

impl MutTemperature {
    pub fn get(&self) -> f64 {
        *self.0.lock().unwrap()
    }

    pub fn set(&self, t: f64) {
        *self.0.lock().unwrap() = t;
    }
}
