use rppal::gpio::Gpio;
use rppal::uart::{Parity, Uart};
use std::time::Duration;

const MOTOR_CONTROL_PIN: u8 = 18;
const UART_BAUD_RATE: u32 = 115_200;

fn main() {
    let gpio = Gpio::new().expect("Failed to setup GPIO");
    let mut motor_pin = gpio.get(MOTOR_CONTROL_PIN).expect("Failed to establish motor pin").into_output();

    motor_pin.set_high();
    // Setup UART
    let mut uart = Uart::new(UART_BAUD_RATE, Parity::None, 8, 1).expect("Failed to set UART channel");
    uart.set_read_mode(255, Duration::from_secs(1)).expect("Failed to set read mode");

    // Send start scan command
    let start_scan_cmd: [u8; 2] = [0xA5, 0x20];
    uart.write(&start_scan_cmd).expect("Failed to send start cmd");

    println!("RPLidar A1 scanning...");

    let mut buffer: Vec<u8> = vec![0; 5];  // Buffer to hold one complete response packet

    loop {
        match uart.read(&mut buffer) {
            Ok(_size) => {
                // Parse a single data packet (5 bytes)
                if buffer.len() == 5 {
                    let start_flag = buffer[0] & 0x01;  // S bit is the LSB of byte 0
                    let quality = buffer[0] >> 2;       // Quality is bits 2-7 of byte 0

                    let angle_q6: u16 = ((buffer[1] as u16) >> 1) | ((buffer[2] as u16) << 7);
                    let actual_angle = angle_q6 as f32 / 64.0;  // Convert fixed-point to float

                    let distance_q2: u16 = (buffer[3] as u16) | ((buffer[4] as u16) << 8);
                    let actual_distance = distance_q2 as f32 / 4.0;  // Convert fixed-point to float

                    println!(
                        "Start Flag: {}, Quality: {}, Angle: {:.2}°, Distance: {:.2} mm",
                        start_flag, quality, actual_angle, actual_distance
                    );

                    if start_flag == 1 {
                        println!("New 360° scan started!");
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read from UART: {}", e);
                break;
            }
        }
    }
}
