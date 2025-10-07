use crate::commands::CPU_LIST;
use crate::cpu::Cpu;
use crate::ui::app::App;
use crate::ui::app::AppState;

pub struct Opcodes {}
impl Opcodes {
    /// Opcodes command
    ///
    /// Parses "opcodes" command
    /// Usage:
    ///   opcodes
    ///   or
    ///   opcodes <i8080 | 8080 | mos6502 | 6502>
    pub fn list_opcodes(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        if command.len() > 2 {
            app.messages.push(format!(
                "Error: more than 1 argument provided. Usage: opcodes or opcodes <{CPU_LIST}>"
            ));
            return Ok(AppState::Home);
        }
        if command.len() < 2 {
            match app.cpu {
                Cpu::None => {
                    app.messages.push(format!("Error: Cpu is not defined. Use set cpu <{CPU_LIST}> to set default cpu first or use opcodes <{CPU_LIST}>"));
                    return Ok(AppState::Home);
                }
                Cpu::I8080 => {
                    return Ok(AppState::Opcodes8080);
                }
                Cpu::Mos6502 => {
                    return Ok(AppState::Opcodes6502);
                }
            }
        }
        match command[1] {
            "i8080" | "8080" => {
                return Ok(AppState::Opcodes8080);
            }
            "mos6502" | "6502" => {
                return Ok(AppState::Opcodes6502);
            }
            _ => {
                app.messages
                    .push(format!("Unknown cpu. Use opcodes or opcodes <{CPU_LIST}>"));
            }
        }
        Ok(AppState::Home)
    }
}
