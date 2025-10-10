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
            "disasm_range" => Self::set_disasm_range(app, command),
            "dump_range" => Self::set_range(app, command),
            "command_history_size" => Self::set_command_history_size(app, command),
            "output_history_size" => Self::set_output_history_size(app, command),
            _ => {
                app.messages
                    .push(format!("Error: Unknown parameter {}", command[1]));
                app.messages
                    .push("- Available parameters are:".to_string());
                app.messages
                    .push(format!("    cpu <{CPU_LIST}>"));
                app.messages
                    .push("    dump_range <range>".to_string());
                app.messages
                    .push("    disasm_range <range>".to_string());
                app.messages
                    .push("    output_history_size <size>".to_string());
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
            app.messages.push("  Usage: set dsasm_range <size>.".to_string());
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
    /// Set size of history of Output window
    ///
    /// Usage:
    ///   set output_history_size 255
    ///   set output_history_size 0ffh
    ///   set output_history_size $ff
    ///   aet output_history_size 0xff
    fn set_output_history_size(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        if command.len() != 3 {
            app.messages
                .push("Error: Wrong number of parameters.".to_string());
            app.messages.push("  Usage: set output_history_size <size>.".to_string());
            return Ok(AppState::Home);
        }
        let range = Memory::from_hex_string(command[2].to_string())?;
        app.get_output_view_status().set_output_history_size(range as usize);
        Ok(AppState::Home)
    }
    /// Set size of history of command window
    ///
    /// Usage:
    ///   set command_history_size 255
    ///   set command_history_size 0ffh
    ///   set command_history_size $ff
    ///   aet command_history_size 0xff
    fn set_command_history_size(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        if command.len() != 3 {
            app.messages
                .push("Error: Wrong number of parameters.".to_string());
            app.messages.push("  Usage: set command_history_size <size>.".to_string());
            return Ok(AppState::Home);
        }
        let size = Memory::from_hex_string(command[2].to_string())?;
        app.set_command_history_size(size as usize);
        Ok(AppState::Home)
    }

}
