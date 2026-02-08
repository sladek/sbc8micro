//! Command processor
//! Processing a command from command line

use crate::commands::breakpoints::Breakpoint;
use crate::commands::cpu::Cpu;
use crate::commands::dev::Dev;
use crate::commands::directory::Directory;
use crate::commands::disasm::Disasm;
use crate::commands::fdc::Fdc;
use crate::commands::help::Help;
use crate::commands::load::Load;
use crate::commands::memory::Memory;
use crate::commands::opcodes::Opcodes;
use crate::commands::registers::Registers;
use crate::commands::script::Script;
use crate::commands::serial::Serial;
use crate::commands::io::Io;
use crate::commands::{MIN_COMMAND_HISTORY_LENGTH, MIN_OUTPUT_HISTORY_LENGTH};
use crate::help;
use crate::ui::app::{App, AppState};
use regex::Regex;

#[derive(Default)]
pub struct Command {}

impl Command {
    pub fn new() -> Self {
        Self {}
    }
    pub fn command(&self, app: &mut App, input: String) -> Result<AppState, String> {
        let re = Regex::new(r"\s+").unwrap(); // Matches one or more whitespace characters 
        let params = re.replace_all(&input.to_owned(), " ").to_string();
        let command: Vec<&str> = params.trim().split(" ").collect();
        match command[0] {
            "b" => Breakpoint::breakpoint(app, command),
            "cd" => Directory::cd(app, command),
            "ch" | "command_history_length" => Self::command_history(app, command),
            "cls" | "clear" => {
                app.messages.clear();
                Ok(AppState::Home)
            }
            "cpu" => Cpu::set_cpu(app, command),
            "d" | "dump" => Memory::dump(app, command),
            "da" | "disasm" => Disasm::disasm(app, command),
            "dev" => Dev::list_devices(app, command),
            "dr" | "disasm_range" => Disasm::disasm_range(app, command),
            "fdc" | "floppy" => Fdc::fdc(app, command),
            "g" | "go" => Cpu::go(app, command),
            "h" | "help" | "?" => Help::help(app, command),
            "io" => Io::io(app, command),
            "l" | "load" => Load::load_file(app, command),
            "la" | "loada" => Load::load_acme_file(app, command),
            "lh" | "loadh" => Load::load_hex_file(app, command),
            "ls" | "dir" => Directory::ls(app, command),
            "m" | "mem" => Memory::set_memory(app, command),
            "mr" | "memory_range" => Memory::memory_range(app, command),
            "oh" | "output_history_length" => Self::output_history(app, command),
            "op" | "opcodes" => Opcodes::list_opcodes(app, command),
            "pwd" => Directory::pwd(app, command),
            "r" | "reg" => Registers::set_get_reg(app, command),
            "s" | "step" => Cpu::step(app, command),
            "scr" | "script" => Script::script(app, command),
            "ser" | "serial" => Serial::serial(app, command),
            "" => Ok(AppState::Home),
            _ => Self::get_usage(app),
        }
    }
    /// Shows usage
    fn get_usage(app: &mut App) -> Result<AppState, String> {
        let help = help::Help::new();
        let items = help.help_items;
        app.messages.push(String::from("ERROR - Unknown command."));
        app.messages.push(String::from("  Available commands:"));
        for item in items {
            let mut msg = String::from("    ");
            msg.push_str(&item.usage);
            app.messages.push(msg);
        }
        Ok(AppState::Home)
    }
    /// Set or displays size of history of Output window
    ///
    /// Usage:
    ///   output_history 255
    ///   output_history 0ffh
    ///   oh $ff
    ///   oh 0xff
    fn output_history(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        match command.len() {
            1 => {
                let length = app.get_output_view_status().get_output_history_size();
                app.messages.push(format!(
                    "Output window history length: 0x{:04x} [{length}]",
                    length
                ));
                return Ok(AppState::Home);
            }
            2 => {
                let range = Memory::from_hex_string(command[1].to_string())?;
                if range < MIN_OUTPUT_HISTORY_LENGTH {
                    return Err(format!(
                        "ERROR - Minimal output history length is {MIN_OUTPUT_HISTORY_LENGTH}"
                    ));
                }
                app.get_output_view_status()
                    .set_output_history_size(range as usize);
            }
            _ => {
                app.messages
                    .push("ERROR - Wrong number of parameters.".to_string());
                app.messages
                    .push("  Usage: output_history [length] or oh [length]".to_string());
                return Ok(AppState::Home);
            }
        }
        Ok(AppState::Home)
    }
    /// Set or displays the length of history of command window
    ///
    /// Usage:
    ///   ch
    ///   ch 255
    ///   ch 0ffh
    ///   command_history
    ///   command_history $ff
    ///   command_history 0xff
    fn command_history(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        match command.len() {
            1 => {
                let length = app.get_command_history_size();
                app.messages
                    .push(format!("Command history length: {length}"));
                return Ok(AppState::Home);
            }
            2 => {
                let size = Memory::from_hex_string(command[1].to_string())?;
                if size < MIN_COMMAND_HISTORY_LENGTH {
                    return Err(format!(
                        "ERROR - Minimal command history length is {MIN_COMMAND_HISTORY_LENGTH}"
                    ));
                }
                app.set_command_history_size(size as usize);
            }
            _ => {
                app.messages
                    .push("ERROR - Wrong number of parameters.".to_string());
                app.messages
                    .push("  Usage: set command_history_size <size>.".to_string());
                return Ok(AppState::Home);
            }
        }
        Ok(AppState::Home)
    }
}
