use crate::ui::app::App;
use crate::ui::app::AppState;
use crate::commands::memory::Memory;
use crate::io::conf_switch::ConfSwitch as Switch;

pub struct ConfSwitch;

impl ConfSwitch {
    pub fn set_switch(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        app.is_cpu_set()?; // Check if cpu is defined
        let cpu = app.cpu_ui.as_mut().unwrap();
        if command.len() < 3 || command.len() > 4 {
            app.messages.push(
                "ERROR - Invalid number of parameters. Usage: cf <base address> <data> [name] or conf_switch <base address> <data> [name]"
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
        let data = Memory::from_hex_string(command[2].to_string())?;
        if data  > 0xff {
            app.messages.push(format!(
                "ERROR - Data cannot be bigger than 0xff, but it is 0x{:02X}",
                base_address
            ));
            return Ok(AppState::Home);               
        }
        let mut conf_switch = Switch::new(data as u8);
        if m_flag == "M" {
            conf_switch.set_memory_base_address(base_address);
        } else {
            conf_switch.set_base_address(base_address as u8)
        };
        if command.len() == 4 {
            conf_switch.set_name(command[3]);
        }
        let name = conf_switch.get_name();
        let parameters = format!(
            "Parameters: base address[{}], data[0x{:02X}], name[{name}]",
            command[1], data
        );
        app.messages.push(parameters);
        if m_flag == "M" {
            cpu.get_memory().map_port(Box::new(conf_switch))?;
        } else {
            match cpu.get_io_memory() {
                Some(io_memory) => {
                    io_memory.map_port(Box::new(conf_switch))?;
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