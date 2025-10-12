use crate::commands::memory::Memory;
use crate::commands::push_cpu_not_set;
use crate::ui::app::App;
use crate::ui::app::AppState;

pub struct Registers;

impl Registers {
    /// Appends register map to application's output area of terminal UI
    pub fn show_registers(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        if app.cpu_ui.is_none() {
            push_cpu_not_set(app);
            return Ok(AppState::Home)
        }
        if command.len() > 1 {
            app.messages
                .push("Invalid number of parameters. Usage: registers or regs".to_string());
            return Ok(AppState::Home);
        }
        let cpu = &mut app.cpu_ui;
        match cpu {
            Some(cpu) => {
                let mut regs = cpu.show_registers();
                app.messages.append(&mut regs);
            }
            None => {
                push_cpu_not_set(app);
            }
        }
        Ok(AppState::Home)
    }
    /// Gets or sets register via terminal UI
    pub fn set_get_reg(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        if app.cpu_ui.is_none() {
            push_cpu_not_set(app);
            return Ok(AppState::Home)
        }
        if command.len() > 3 || command.len() < 2 {
            app.messages
                .push("Invalid number of parameters. Usage: \'reg <reg> [value]\' or \'r <reg> [value]\'".to_string());
            return Ok(AppState::Home);
        }
        // If parameter is just a registre name then display a content of the register
        if command.len() == 2 {
            let reg = command[1];
            let cpu = &mut app.cpu_ui;
                match cpu {
                    Some(cpu) => {
                        match cpu.get_register_by_name(reg) {
                            Ok(s_value) => {
                                app.messages.push(s_value);
                            }
                            Err(err) => {
                                return Err(err)
                            }
                        }

                    }
                    None => {
                        push_cpu_not_set(app);
                    }
                }            
        }
        // Register value is provided
        if command.len() == 3 {
            let reg = command[1];
            let value = Memory::from_hex_string(command[2].to_string())?;
            let cpu = &mut app.cpu_ui;
                match cpu {
                    Some(cpu) => {
                        cpu.set_register_by_name(reg, value)?
                    }
                    None => {
                        push_cpu_not_set(app);
                    }
                }
        }
        Ok(AppState::Home)
    }
}
