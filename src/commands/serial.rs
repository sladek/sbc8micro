use crate::commands::memory::Memory;
use crate::io::i8251a::{Async8251, StopBits};
use crate::ui::app::App;
use crate::ui::app::AppState;

pub struct Serial;

impl Serial {
    pub fn serial(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        app.is_cpu_set()?; // Check if cpu is defined
        let cpu = app.cpu_ui.as_mut().unwrap();
        if command.len() > 4 || command.len() < 3 {
            app.messages.push(
                "ERROR - Invalid number of parameters. Usage: serial <port address> <port name> [clock frequency]"
                    .to_string(),
            );
            return Ok(AppState::Home);
        }
        let name = command[2];
        let mut port_address = command[1].to_uppercase();
        let mut m_flag = "";
        if port_address.starts_with("M") {
            m_flag = "M";
            port_address = port_address[1..].to_string();
        }
        let base_address = Memory::from_hex_string(port_address)?;
        if base_address > 0xff && m_flag.is_empty() {
            app.messages.push(format!(
                "ERROR - Address cannot be bigger than 0xff, but it is 0x{:02X}",
                base_address
            ));
            return Ok(AppState::Home);
        }
        let mut serial = Async8251::new();
        serial.set_name(name.to_string());
        if m_flag == "M" {
            serial.set_memory_base_address(base_address);
        } else {
            serial.set_base_address(base_address as u8)
        };
        if command.len() == 4 {
            match command[2].parse::<u32>() {
                Ok(clock) => {
                    serial.set_clock(clock);
                }
                Err(err) => {
                    let error = format!("{} - {}", command[3], err);
                    app.messages.push(error);
                    return Ok(AppState::Home);
                }
            }
        }
        let name = serial.get_name().unwrap();
        let baud_rate = serial.get_baud_rate();
        let char_len = serial.get_character_length();
        let stop_bits: &str = match serial.get_stop_bits() {
            StopBits::One => "one",
            StopBits::OneAndHalf => "one and half",
            StopBits::Two => "two",
            _ => "invalid",
        };
        let parity: &str = if !serial.get_parity_enable() {
            "none"
        } else if serial.is_even_parity() {
            "even"
        } else {
            "odd"
        };
        let parameters = format!(
            "Parameters: base address[{}], name[{name}], baud rate[{baud_rate}], character length[{char_len}], parity[{parity}], stop bits[{stop_bits}]",
            command[1]
        );
        app.messages.push(parameters);
        //        cpu.get_io_memory().remove(base_address as u8);
        match serial.open_port(&name) {
            Ok(serial) => {
                if m_flag == "M" {
                    cpu.get_memory().map_port(Box::new(serial))?;
                } else {
                    match cpu.get_io_memory() {
                        Some(io_memory) => {
                            io_memory.map_port(Box::new(serial))?;
                        }
                        None => {
                            app.messages.push(
                                "ERROR - This CPU doesn't suppor Io mapping, please use memory mapping"
                                    .to_string(),
                            );
                        }
                    }
                }
            }
            Err(err) => {
                app.messages.push(format!("{name}: {}", err));
            }
        };
        Ok(AppState::Home)
    }
}
