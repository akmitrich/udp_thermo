use std::{thread, time::Duration};

mod smart_thermo;

fn main() {
    let udp_receiver = smart_thermo::SmartThermo::new("127.0.0.1:4084").unwrap();
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Temperature is: {}", udp_receiver.get_temperature());
    }
}
