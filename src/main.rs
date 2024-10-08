use rppal::gpio::Gpio;
use rppal::uart::{Parity, Uart};
use std::time::Duration;

const MOTOR_CONTROL_PIN: u8 = 18;

fn main() {
    let gpio = Gpio::new().expect("Failed to setup GPIO");
    let mut motor_pin = gpio.get(MOTOR_CONTROL_PIN).expect("Failed to establish motor pin").into_output();

    motor_pin.set_high();

    let mut uart = Uart::new(115_200, Parity::None, 8, 1).expect("Failed to set UART channel");

    uart.set_read_mode(16, Duration::from_secs(1)).expect("Failed to set read mode");

    let start_scan_cmd: [u8; 2] = [0xA5, 0x20];
    uart.write(&start_scan_cmd).expect("Failed to send start cmd");

    std::thread::sleep(Duration::from_millis(100));
    println!("RPLidar A1 scanning...");

    let mut buffer: Vec<u8> = vec![0; 16];

    loop {
        match uart.read(&mut buffer) {
            Ok(n) if n > 0 => {
                println!("Received {} bytes: {:02X?}", n, &buffer[..n]);
                
                if n == 5 && buffer[0] == 0xA5 && buffer[1] == 0x5A {
                    println!("Received RPLidar response header.");
                }
            }
            Ok(_) => {
                println!("No data received, retrying...");
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                eprintln!("Failed to read from UART: {}", e);
                break;
            }
        }
    }
}
