use crate::commands::{MIN_DISASM_RANGE, memory::Memory};
use crate::ui::app::{App, AppState};

pub struct Disasm;

impl Disasm {
    pub fn disasm(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        app.is_cpu_set()?; // Check if cpu is defined
        let mut start_address = 0u16;
        let end_address: u16;
        match command.len() {
            1 => {
                if let Some(cpu) = &mut app.cpu_ui {
                    start_address = cpu.get_pc();
                }
                end_address =
                    if start_address as usize + app.disasm.range as usize > u16::MAX as usize {
                        u16::MAX
                    } else {
                        start_address + app.disasm.range - 1
                    };
            }
            2 => {
                start_address = Memory::from_hex_string(command[1].to_string())?;
                end_address = if start_address as u32 + app.disasm.range as u32 > u16::MAX.into() {
                    u16::MAX
                } else {
                    start_address + app.disasm.range - 1
                };
            }
            3 => {
                // Save values from command line so they can be used later
                start_address = Memory::from_hex_string(command[1].to_string())?;
                end_address = Memory::from_hex_string(command[2].to_string())?;
                if start_address > end_address {
                    app.messages
                        .push("ERROR - End address must be bigger than start address.".to_string());
                    return Ok(AppState::Home);
                }
                app.disasm.set_start_address(start_address);
                app.disasm.set_range(end_address - start_address + 1);
            }
            _ => {
                return Err(
                    "ERROR - Invalid number of parameters. Usage: disasm <start address> <end_address>"
                        .to_string(),
                );
            }
        }
        if let Some(cpu) = &mut app.cpu_ui {
            let mut disasm = cpu.disasm(start_address, end_address);
            app.messages.append(&mut disasm);
        }

        Ok(AppState::Home)
    }
    /// Set memory range for dissasembler
    ///
    /// Usage:
    ///   disasm_range 127
    ///   disasm_range 0ffh
    ///   dr $ff
    ///   dr 0xff
    pub fn disasm_range(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        match command.len() {
            1 => {
                let range = app.disasm.range;
                app.messages
                    .push(format!("Disasembler range: {:04x}H [{range}]", range));
            }
            2 => {
                let range = Memory::from_hex_string(command[1].to_string())?;
                if range < MIN_DISASM_RANGE {
                    return Err(format!(
                        "ERROR - Minimum allowed disassembler range is {MIN_DISASM_RANGE}"
                    ));
                }
                //range cannot increase end address beyond max memory size
                app.disasm.set_range(range);
            }
            _ => {
                app.messages
                    .push("ERROR - Wrong number of parameters.".to_string());
                app.messages
                    .push("  Usage: set dsasm_range <size>.".to_string());
                return Ok(AppState::Home);
            }
        }
        Ok(AppState::Home)
    }
}
