use std::{
    thread,
    time::{Duration, Instant},
};

mod thermo;
mod udp_observer;

fn main() {
    let mut thermo = thermo::Thermometer::new(20.);
    let observer = udp_observer::UdpThermo::new("127.0.0.1:4084");
    thermo.add_observer(observer);
    mock_thermo(&mut thermo);
}

fn mock_thermo(thermo: &mut thermo::Thermometer) {
    let started = Instant::now();
    loop {
        let passed = Instant::now() - started;
        thermo.set_temperature(20. + (passed.as_secs_f64() / 5.).sin());
        thread::sleep(Duration::from_secs_f64(rand::random::<f64>() * 5.));
    }
}
