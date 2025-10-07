use crate::commands::CPU_LIST;
use crate::commands::memory::Memory;
use crate::cpu::Cpu;
use crate::cpu::i8080;
use crate::cpu::mos6502;
use crate::ui::app::App;
use crate::ui::app::AppState;

pub struct Parameter {}
impl Parameter {
    /// Set command
    ///
    /// Parses "set" command
    /// Usage:
    ///   set <parameter> .. <parameter>
    ///   example set cpu <8080 | i8080 | 6502 | mos6502>
    pub fn set(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        if command.len() < 2 {
            app.messages.push(
                "Error: Parameter is not defined. Use set <parameter> <other_parameters>"
                    .to_string(),
            );
            app.messages.push(format!("  Example: set cpu <{CPU_LIST}> to set default cpu first or use opcodes <{CPU_LIST}>"));
            return Ok(AppState::Home);
        }
        match command[1] {
            "cpu" => Self::set_cpu(app, command),
            "dump_range" => Self::set_range(app, command),
            "disasm_range" => Self::set_disasm_range(app, command),
            _ => {
                app.messages.push(format!("Error: Unknown parameter {}", command[1]));
                app.messages
                    .push(format!("  Available parameters are: cpu <{CPU_LIST}>"));
                Ok(AppState::Home)
            }
        }
    }
    /// Set default cpu
    ///
    /// Usage:
    ///   set cpu <i80808 | 8080 | mos6502 | 6502>
    fn set_cpu(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        if command.len() != 3 {
            app.messages
                .push("Error: Wrong number of parameters.".to_string());
            app.messages.push(format!("  Usage: set cpu <{CPU_LIST}>."));
            return Ok(AppState::Home);
        }
        match command[2] {
            "i8080" | "8080" => {
                app.cpu_ui = i8080::Cpu::get_cpu_ui();
                app.cpu = Cpu::I8080;
            }
            "mos6502" | "6502" => {
                app.cpu_ui = mos6502::Cpu::get_cpu_ui();
                app.cpu = Cpu::Mos6502;
            }
            _ => {
                app.messages.push(format!(
                    "Error: wrong cpu defined. Use set cpu <{CPU_LIST}> to set default cpu."
                ));
                return Ok(AppState::Home);
            }
        }
        Ok(AppState::Home)
    }
    /// Set memory dump range
    ///
    /// Usage:
    ///   set dump_range 127
    ///   set dump_range 0ffh
    ///   set dump_range $ff
    ///   aet dump_range 0xff
    fn set_range(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        if command.len() != 3 {
            app.messages
                .push("Error: Wrong number of parameters.".to_string());
            app.messages.push("  Usage: set range <size>.".to_string());
            return Ok(AppState::Home);
        }
        let mut range = Memory::from_hex_string(command[2].to_string())?;
        range -= 1;
        app.dump.set_range(range);
        let start_address = app.dump.start;
        if (start_address as u32 + range as u32) > 0xff {
            app.dump.set_end_address(0xffu16);
        }
        app.dump.set_end_address(start_address + range);
        Ok(AppState::Home)
    }

    /// Set memory range for dissasembler
    ///
    /// Usage:
    ///   set disasm_range 127
    ///   set disasm_range 0ffh
    ///   set disasm_range $ff
    ///   aet disasm_range 0xff
    fn set_disasm_range(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        if command.len() != 3 {
            app.messages
                .push("Error: Wrong number of parameters.".to_string());
            app.messages.push("  Usage: set range <size>.".to_string());
            return Ok(AppState::Home);
        }
        let range = Memory::from_hex_string(command[2].to_string())?;
        app.disasm.set_range(range);
        let start_address = app.disasm.start;
        if (start_address as u32 + range as u32) > 0xff {
            app.disasm.set_end_address(0xffu16);
        }
        app.disasm.set_end_address(start_address + range);
        Ok(AppState::Home)
    }

}
