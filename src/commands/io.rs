use crate::commands::memory::Memory;
use crate::ui::app::App;
use crate::ui::app::AppState;

pub struct Io;

impl Io {
    pub fn io(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        app.is_cpu_set()?; // Check if cpu is defined
        let cpu = app.cpu_ui.as_mut().unwrap();
        if command.len() > 3 {
            app.messages.push(
                "ERROR - Invalid number of parameters. Usage: io <io address> [data]"
                    .to_string(),
            );
            return Ok(AppState::Home);
        }
        let io_memory = cpu.get_io_memory();
        if io_memory.is_none()  {
            app.messages.push(
                "ERROR - This CPU doesn't suppor Io mapping, please use memory mapping"
                    .to_string(),
            );
        }
        let address = Memory::from_hex_string(command[1].to_uppercase())?;
        if address > 0xff {
            app.messages.push(format!(
                "ERROR - Address cannot be bigger than 0xff, but it is 0x{:04X}",
                address
            ));
            return Ok(AppState::Home);
        }
        if command.len() == 3 {
            // Data is present so we will do iu write
            let data = Memory::from_hex_string(command[2].to_uppercase())?;
            if data > 0xff {
                app.messages.push(format!(
                    "ERROR - Address cannot be bigger than 0xff, but it is 0x{:04X}",
                    address
                ));
                return Ok(AppState::Home);
            }
            cpu.io_write(address as u8, data as u8);
        } else {
            // Data is not present so we will do read
            let read_data = cpu.io_read(address as u8);
            app.messages.push(format!("0x{:02X}", read_data));
        }
        Ok(AppState::Home)
    }
}
