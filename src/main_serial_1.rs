//! Simple example that echoes received serial traffic to stdout
use sbc8micro::io::serial::Serial;
use std::io;

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM3";
const DEFAULT_BAUD: u32 = 9600;
pub fn main() -> io::Result<()> {
    // Create the serial port
    println!("Opening {} at 9600,8N1", DEFAULT_TTY);

    let serial = &mut Serial::new();
    serial.open(DEFAULT_TTY, DEFAULT_BAUD, serialport::DataBits::Eight, serialport::StopBits::One, serialport::Parity::None)?;
    let _handle = serial.start();

    let mut received_data: Option<u8>;
    loop {
        if let Some(data) = serial.read_data() {
            println!("Received {data}H");
            received_data = Some(data);
            let rd = received_data.take();
            if let Some(data) = rd {
                let str = format!("Received some byte from serial: [{:02X}H]\n\r", data);
                let buf = str.as_bytes();
                for data in buf {
                    _ = serial.write_data(*data);
                }
            }
        }
    }
}
