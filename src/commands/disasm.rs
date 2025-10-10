use crate::commands::CPU_LIST;
use crate::commands::memory::Memory;
use crate::ui::app::{App, AppState};

pub struct Disasm;

impl Disasm {
    pub fn disasm(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        if command.len() > 3 {
            app.messages.push(
                "Invalid number of parameters. Usage: disasm <start address> <end_address>"
                    .to_string(),
            );
        }
        let cpu = &mut app.cpu_ui;
        let start_address: u16;
        let mut end_address: u16;
        match cpu {
            Some(cpu) => {
                if command.len() == 1 {
                    start_address = app.disasm.start;
                    end_address = app.disasm.end + 1;
                } else if command.len() == 2 {
                    start_address = Memory::from_hex_string(command[1].to_string())?;
                    end_address = start_address + app.disasm.range;
                } else {
                    start_address = Memory::from_hex_string(command[1].to_string())?;
                    end_address = Memory::from_hex_string(command[2].to_string())?;
                    end_address += 1;
                }
                if start_address > end_address {
                    app.messages
                        .push("End address must be bigger than start address.".to_string());
                    return Ok(AppState::Home);
                }
                if command.len() == 3 {
                    // Save values from command line so they can be used later
                    app.disasm.set_start_address(start_address);
                    app.disasm.set_end_address(end_address);
                }
                let mut disasm = cpu.disasm(start_address, end_address);
                app.messages.append(&mut disasm);
            }
            None => {
                app.messages.push(format!("Error: Cpu is not defined. Use set cpu <{CPU_LIST}> to set default cpu first or use opcodes <{CPU_LIST}>"));
            }
        }
        Ok(AppState::Home)
    }
}
