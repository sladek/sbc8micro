use crate::commands::memory::Memory;
use crate::ui::app::{App, AppState};
use crate::commands::push_cpu_not_set;

pub struct Breakpoint;
use crate::cpu::CpuUi;

impl Breakpoint {
    pub fn breakpoint(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        if app.cpu_ui.is_none(){
            push_cpu_not_set(app);
            return Ok(AppState::Home);
        }
        let cpu: &mut Box<dyn CpuUi> = app.cpu_ui.as_mut().unwrap();
        match command.len() {
            1 => {
                let breakpoints = cpu.get_breakpoints()?;
                if breakpoints.is_empty() {
                    app.messages.push("No breakpoints defined.".to_string());
                }
                for (i, val) in breakpoints.iter().enumerate() {
                    app.messages.push(format!("{}: 0x{:04X?} [{val}]", i, val));
                }

            }
            2 => {
                if command[1].eq_ignore_ascii_case("x") {
                    cpu.clear_breakpoints();
                    app.messages.push("All breakpoint has been deleted.".to_string());
                    return Ok(AppState::Home);
                } 
                match Memory::from_hex_string(command[1].to_string()) {
                    Ok(address) => {
                        cpu.set_breakpoints(address)?;
                    }
                    Err(err) => {
                        return Err(err)
                    }
                }
            }
            _ => {
                app.messages.push("Invalid number of arguments: Usage: b or b <address>".to_string());
            }
        }
        Ok(AppState::Home)
    }
}