use rppal::gpio::Gpio;
use rppal::uart::{Parity, Uart};
use std::time::Duration;
use std::thread;

const CMD_STOP: u8 = 0x25;
const CMD_SCAN: u8 = 0x20;
const CMD_RESET: u8 = 0x40;
const MOTOR_CONTROL_PIN: u8 = 18;

fn main() {
    let gpio = Gpio::new().expect("Failed to setup GPIO");
    let mut motor_pin = gpio.get(MOTOR_CONTROL_PIN).expect("Failed to establish motor pin").into_output();

    motor_pin.set_high();
    let mut uart = Uart::new(115_200, Parity::None, 8, 1).expect("Failed to initialize UART");
    uart.set_read_mode(255, Duration::from_millis(500)).expect("Failed to set timeout");

    send_command(&mut uart, CMD_RESET);
    println!("Sent reset command to RPLIDAR. Waiting 2 seconds...");
    thread::sleep(Duration::from_secs(2));

    send_command(&mut uart, CMD_SCAN);
    println!("Started scanning...");

    let mut buffer: [u8; 255] = [0; 255];
    loop {
        match uart.read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                parse_scan_data(&buffer[..bytes_read]);
            }
            _ => break,
        }
    }

    motor_pin.set_low();
    send_command(&mut uart, CMD_STOP);
}

fn send_command(uart: &mut Uart, command: u8) {
    let cmd_packet = [0xA5, command];
    uart.write(&cmd_packet).expect("Failed to send command");
}

fn parse_scan_data(data: &[u8]) {
    if data.len() < 5 {
        return;
    }

    let mut index = 0;
    while index + 5 <= data.len() {
        let quality = (data[index] >> 2) & 0x3F;  // Extract quality (upper 6 bits)
        let raw_angle = ((data[index + 2] as u16) << 8 | data[index + 1] as u16) >> 1;
        let angle_in_degrees = (raw_angle as f32) / 64.0;  // Convert to degrees

        let raw_distance = (data[index + 4] as u16) << 8 | data[index + 3] as u16;
        let distance_in_mm = raw_distance as f32 / 4.0;  // Convert to mm

        if quality > 10 && raw_distance > 0 && distance_in_mm < 12000.0 {
            println!(
                "Quality: {}, Angle: {:.2}°, Distance: {:.2} mm",
                quality, angle_in_degrees, distance_in_mm
            );
        } else {
            println!("Invalid measurement or noisy data (Quality: {}, Distance: {})", quality, distance_in_mm);
        }

        index += 5;
    }
}
