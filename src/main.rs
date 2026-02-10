use serialport::{StopBits, Parity, DataBits};
use std::io::Write;
use std::time::Duration;
use std::thread;

fn main() {
    let mut port = loop {
        match serialport::new("COM1", 9600)
            .data_bits(DataBits::Eight)
            .parity(Parity::Even)
            .stop_bits(StopBits::One)
            .timeout(Duration::from_secs(1))
            .open()
        {
            Ok(p) => {
                println!("Connected to COM1");
                thread::sleep(Duration::from_secs(10));
                break p;
            }
            Err(e) => {
                println!("Error: {}. Retry in 2s...", e);
                thread::sleep(Duration::from_secs(10));
            }
        }
    };

    let frame: [u8; 5] = [0xFC, 0x05, 0x40, 0x2B, 0x15];
    port.write_all(&frame).unwrap();

    println!("Sent ID003 frame");
}
