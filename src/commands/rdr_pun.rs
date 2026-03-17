use crate::commands::memory::Memory;
use crate::ui::app::App;
use crate::ui::app::AppState;
use crate::io::rdr_pun;

pub struct RdrPun;

impl RdrPun {
    pub fn rdr_pun(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        app.is_cpu_set()?; // Check if cpu is defined
        let cpu = app.cpu_ui.as_mut().unwrap();
        if command.len() != 2 {
            app.messages.push(
                "ERROR - Invalid number of parameters. Usage: rdr <base address> or pun <base address> or rdr_pun <base address>"
                    .to_string(),
            );
            return Ok(AppState::Home);
        }
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
        let mut rdr_pun = rdr_pun::RdrPun::new();
        if m_flag == "M" {
            rdr_pun.set_memory_base_address(base_address);
        } else {
            rdr_pun.set_base_address(base_address as u8)
        };
        let name = rdr_pun.get_name();
        let parameters = format!(
            "Parameters: base address[{}], name[{name}]",
            command[1]
        );
        app.messages.push(parameters);
        if m_flag == "M" {
            cpu.get_memory().map_port(Box::new(rdr_pun))?;
        } else {
            match cpu.get_io_memory() {
                Some(io_memory) => {
                    io_memory.map_port(Box::new(rdr_pun))?;
                }
                None => {
                    app.messages.push(
                        "ERROR - This CPU doesn't suppor Io mapping, please use memory mapping"
                            .to_string(),
                    );
                }
            }
        }
        Ok(AppState::Home)
    }
}
