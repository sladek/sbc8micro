use crate::commands::{cpu_not_set_error, CPU_LIST};
use crate::cpu;
use crate::ui::app::{App, AppState};

pub struct Cpu;

impl Cpu {
    /// Set default cpu
    ///
    /// Usage:
    ///   cpu <i80808 | 8080 | mos6502 | 6502>
    pub fn set_cpu(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        match command.len() {
            1 => {
                app.check_cpu()?;
                let cpu = app.cpu_ui.as_mut().unwrap();
                let name = cpu.get_cpu_name().unwrap().to_string();
                app.messages.push(name);
            }
            2 => match command[1] {
                "i8080" | "8080" => {
                    app.cpu_ui = cpu::i8080::Cpu::get_cpu_ui();
                    app.cpu = cpu::Cpu::I8080;
                }
                "mos6502" | "6502" => {
                    app.cpu_ui = cpu::mos6502::Cpu::get_cpu_ui();
                    app.cpu = cpu::Cpu::Mos6502;
                }
                _ => {
                    return Err(format!("Unknown CPU. Use: cpu <{CPU_LIST}>"));
                }
            },
            _ => {
                app.messages
                    .push("Error: Wrong number of parameters.".to_string());
                app.messages.push(format!("  Usage: cpu <{CPU_LIST}>"));
            }
        }
        Ok(AppState::Home)
    }
    pub fn step(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        app.check_cpu()?; // Check if cpu is defined
        if command.len() != 1 {
            return Err("Invalid number of parameters. Usage s or step".to_string())
        }
        match &mut app.cpu_ui {
            Some(cpu) => {
                if let Some(disasm) = cpu.one_step() {
                    app.messages.push(disasm);
                }
            }
            None => {
                return cpu_not_set_error()
            }
        }
        Ok(AppState::Home)
    }
}
