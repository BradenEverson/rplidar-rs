use rppal::gpio::Gpio;
use rppal::uart::{Parity, Uart};
use std::error::Error;
use std::time::Duration;

const MOTOR_CONTROL_PIN: u8 = 18;
fn main() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new()?;
    let mut motor_pin = gpio.get(MOTOR_CONTROL_PIN)?.into_output();

    motor_pin.set_high();

    let mut uart = Uart::new(115_200, Parity::None, 8, 1)?;

    uart.set_read_mode(255, Duration::from_millis(1000))?;

    let start_scan_cmd: [u8; 2] = [0xA5, 0x20];
    uart.write(&start_scan_cmd)?;

    println!("RPLidar A1 scanning...");

    let mut buffer: Vec<u8> = vec![0; 512];

    loop {
        match uart.read(&mut buffer) {
            Ok(n) if n > 0 => {
                println!("Received {} bytes: {:X?}", n, &buffer[..n]);
            }
            _ => panic!("Read error!")
        }
    }
}
