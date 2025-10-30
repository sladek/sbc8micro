use crate::io::serial::{Async8251, StopBits};
use crate::ui::app::App;
use crate::ui::app::AppState;
use crate::commands::memory::Memory;

pub struct Serial;

impl Serial {
    pub fn serial(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        app.check_cpu()?; // Check if cpu is defined
        let cpu = app.cpu_ui.as_mut().unwrap();
        if command.len() == 1 { 
            let io_memory = cpu.get_io_memory();
            let info = io_memory.get_io_ports_info();
            if info.is_empty() {
                app.messages.push("No device mapped to io memory.".to_string());
            }
            else {
                for info in info {
                    app.messages.push(info);
                }
            }
            return Ok(AppState::Home); 
        };
        if command.len() > 4 || command.len() < 3{
            app.messages.push(
                "Invalid number of parameters. Usage: serial <port name> [clock frequency]".to_string(),
            );
            return Ok(AppState::Home);           
        }
        let name = command[2];
        let base_address = Memory::from_hex_string(command[1].to_string())?;
        if base_address > 0xff {
            app.messages.push(format!("Address cannot be bigger than 0xff, but it is 0x{:02X}", base_address));
            return Ok(AppState::Home);
        }
        let mut serial = Async8251::new();
        serial.set_name(name.to_string());
        serial.set_base_address(base_address as u8);
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
        let stop_bits: &str = match serial.get_stop_bits(){
            StopBits::One => {
                "one"
            }
            StopBits::OneAndHalf => {
                "one and half"
            }
            StopBits::Two => {
                "two"
            }
            _ => {
                "invalid"
            }
        };
        let parity: &str = if !serial.get_parity_enable() {
            "none"
        } else if serial.is_even_parity() {"even"}
        else {"odd"};        
        let parameters = format!("Parameters: base address[0x{:02X}], name[{name}], baud rate[{baud_rate}], character length[{char_len}], parity[{parity}], stop bits[{stop_bits}]", base_address);
        app.messages.push(parameters);
        cpu.get_io_memory().remove(base_address as u8);
        match serial.open_port(&name) {
            Ok(serial) => {
                cpu.get_io_memory().map_port(Box::new(serial))?;
            }
            Err(err) => {
                app.messages.push(format!("{name}: {}", err));
            }
        };
        
        Ok(AppState::Home)
    }
}