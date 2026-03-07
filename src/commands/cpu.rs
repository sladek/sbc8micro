use crate::commands::memory::Memory;
use crate::commands::{CPU_LIST, cpu_not_set_error};
use crate::cpu;
use crate::ui::app::{App, AppState};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, poll};

use std::time::Duration;

pub struct Cpu;

impl Cpu {
    /// Set default cpu
    ///
    /// Usage:
    ///   cpu <i80808 | 8080 | mos6502 | 6502>
    pub fn set_cpu(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        match command.len() {
            1 => {
                app.is_cpu_set()?;
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
    /// Executes one step
    ///
    /// Executes one step and if debug flag is set, it also displays opcode of executed instruction
    pub fn step(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        app.is_cpu_set()?; // Check if cpu is defined
        if command.len() != 1 {
            return Err("Invalid number of parameters. Usage: s or step".to_string());
        }
        match &mut app.cpu_ui {
            Some(cpu) => {
                if let Some(disasm) = cpu.one_step() {
                    app.messages.push(disasm);
                }
            }
            None => return cpu_not_set_error(),
        }
        Ok(AppState::Home)
    }
    /// Starts execution of program from address in PC register
    ///
    /// Starts execution of program from address in PC register and stops if it
    /// reaches breakpoint or CTRL-C is pressed. If no breakpoint is reached and pc reaches
    /// end of memory (0xffff) it rolles over to 0x0000 and continues execution.
    pub fn go(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        app.is_cpu_set()?; // Check if cpu is defined
        match &mut app.cpu_ui {
            Some(cpu) => {
                // Stores debug flag
                let debug = cpu.get_debug_flag();
                // and disable debugging (opcode) output
                let pc: u16 = match command.len() {
                    1 => cpu.get_pc(),
                    2 => {
                        // Set PC from command line value
                        Memory::from_hex_string(command[1].to_string())?
                    }
                    _ => return Err("ERROR - Invalid number of parameters. Usage: g or go".to_string()),
                };
                cpu.set_debug_flag(false);
                cpu.set_pc(pc); // Set PC before run
                loop {
                    if let Ok(true) = poll(Duration::from_secs(0)) {
                        match event::read() {
                            Ok(event) => {
                                if let Event::Key(key) = event
                                    && key.kind == KeyEventKind::Press
                                    && let KeyCode::Char(c) = key.code
                                    && c.eq_ignore_ascii_case(&'C')
                                    && key.modifiers.contains(KeyModifiers::CONTROL)
                                {
                                    app.messages
                                        .push(format!("CTRL-C pressed: PC: 0x{:04X}", cpu.get_pc()));
                                    cpu.set_debug_flag(debug);
                                    return Ok(AppState::Home);
                                }
                            }
                            Err(err) => {
                                cpu.set_debug_flag(debug);
                                return Err(format!("ERROR occured: {}", err));
                            }
                        };
                    }
                    if let Some(disasm) = cpu.one_step() {
                        app.messages.push(disasm);
                    }
                    let pc = cpu.get_pc();
                    if cpu.get_breakpoints().unwrap().contains(&pc) {
                        app.messages.push(format!("b: [0x{:04X}]", pc));
                        cpu.set_debug_flag(debug);
                        return Ok(AppState::Home);
                    };
                }
            }
            None => cpu_not_set_error(),
        }
    }
}
