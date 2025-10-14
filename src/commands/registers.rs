use crate::commands::cpu_not_set_error;
use crate::commands::memory::Memory;
use crate::ui::app::App;
use crate::ui::app::AppState;

pub struct Registers;

impl Registers {
    /// Gets or sets register via terminal UI
    pub fn set_get_reg(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        if app.cpu_ui.is_none() {
            return  cpu_not_set_error();
        }
        match &mut app.cpu_ui {
            Some(cpu) => {
                match command.len() {
                    1 => {
                        let mut regs = cpu.show_registers();
                        app.messages.append(&mut regs);
                    }
                    2 => {
                        let reg = command[1];
                        match cpu.get_register_by_name(reg) {
                            Ok(s_value) => {
                                app.messages.push(s_value);
                            }
                            Err(err) => return Err(err),
                        }
                    }
                    3 => {
                        let reg = command[1];
                        let value = Memory::from_hex_string(command[2].to_string())?;
                        cpu.set_register_by_name(reg, value)?;
                    }
                    _ => {
                        app.messages.push(
                            "Invalid number of parameters. Usage: \'reg <reg> [value]\' or \'r <reg> [value]\'"
                                .to_string(),
                        );
                        return Ok(AppState::Home);
                    }
                }
            }
            None => {
                return  cpu_not_set_error();
            }
        }
        Ok(AppState::Home)
    }
}
