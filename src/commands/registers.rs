use crate::commands::CPU_LIST;
use crate::ui::app::App;
use crate::ui::app::AppState;

pub struct Registers;

impl Registers {
    pub fn get_registers(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
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
                app.messages.push(format!("Error: Cpu is not defined. Use set cpu <{CPU_LIST}> to set default cpu first or use opcodes <{CPU_LIST}>"));
            }
        }
        Ok(AppState::Home)
    }
}
