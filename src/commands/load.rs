use crate::commands::directory::Directory;
use crate::commands::{self, CPU_LIST};
use crate::memory::Memory;
use crate::ui::app::{App, AppState};
use std::fs;

pub struct Load;

impl Load {
    pub fn load_file(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        if command.len() < 3 {
            app.messages.push(
                "Invalid number of parameters. Usage: load <start address> <file name>".to_string(),
            );
            return Ok(AppState::Home);
        }
        let start_address = commands::memory::Memory::from_hex_string(command[1].to_string())?;
        let filename = Directory::concat(&command[2..]);
        let metadata = fs::metadata(&filename);
        match metadata {
            Ok(metadata) => {
                let file_size = metadata.len();
                if file_size > 0xffff {
                    app.messages.push("File length is too big. Maximum file length is 65535 bytes so it fits to the memory.".to_string());
                    return Ok(AppState::Home);
                }
            }
            Err(err) => {
                app.messages.push(err.to_string());
                return Ok(AppState::Home);
            }
        }
        let bytes = std::fs::read(&filename);
        match bytes {
            Ok(bytes) => {
                let cpu = &mut app.cpu_ui;
                match cpu {
                    Some(cpu) => {
                        let memory: &mut Memory = cpu.get_memory();
                        match memory.load_program(&bytes, start_address) {
                            Ok(region) => {
                                app.messages.push(format!(
                                    "Loaded: start: {:04X}H, end: {:04X}H",
                                    region.start, region.end
                                ));
                            }
                            Err(err) => {
                                app.messages.push(err.to_string());
                                return Ok(AppState::Home);
                            }
                        };
                    }
                    None => {
                        app.messages.push(format!("Error: Cpu is not defined. Use set cpu <{CPU_LIST}> to set default cpu first or use opcodes <{CPU_LIST}>"));
                        return Ok(AppState::Home);
                    }
                }
            }
            Err(err) => {
                app.messages.push(err.to_string());
            }
        }
        Ok(AppState::Home)
    }

    pub fn load_acme_file(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        if command.len() < 2 {
            app.messages
                .push("Invalid number of parameters. Usage: loada <file name>".to_string());
            return Ok(AppState::Home);
        }
        let filename = Directory::concat(&command[1..]);
        let metadata = fs::metadata(&filename);
        match metadata {
            Ok(metadata) => {
                let file_size = metadata.len();
                if file_size > 0xffff {
                    app.messages.push("File length is too big. Maximum file length is 65535 bytes so it fits to the memory.".to_string());
                    return Ok(AppState::Home);
                }
            }
            Err(err) => {
                app.messages.push(err.to_string());
                return Ok(AppState::Home);
            }
        }
        //        let bytes = std::fs::read(&filename);
        let cpu = &mut app.cpu_ui;
        match cpu {
            Some(cpu) => {
                let memory: &mut Memory = cpu.get_memory();
                let region = memory.load_program_from_acme_file(&filename);
                match region {
                    Ok(region) => {
                        app.messages.push(format!(
                            "Loaded: start: 0x{:04X}H, end: 0x{:04X}H",
                            region.start, region.end
                        ));
                    }
                    Err(err) => return Err(err.to_string()),
                }
            }
            None => {
                app.messages.push(format!("Error: Cpu is not defined. Use set cpu <{CPU_LIST}> to set default cpu first or use opcodes <{CPU_LIST}>"));
                //                return Ok(AppState::Home);
            }
        }
        Ok(AppState::Home)
    }
}
