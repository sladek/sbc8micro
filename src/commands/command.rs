//! Command processor
//! Processing a command from command line

use crate::commands::directory::Directory;
use crate::commands::help::Help;
use crate::commands::load::Load;
use crate::commands::memory::Memory;
use crate::commands::opcodes::Opcodes;
use crate::commands::set::Parameter;
use crate::commands::disasm::Disasm;
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
            "cd" => {
                Directory::cd(app, command)
            }
            "cls" | "clear" => {
                app.messages.clear();
                Ok(AppState::Home)
            }
            "disasm" => {
                Disasm::disasm(app, command)
            }
            "dump" => {
                Memory::dump(app, command)
            }
            "help" | "?" => {
                Help::help(app, command)
            }
            "load" => {
                Load::load_file(app, command)
            }
            "loada" => {
                Load::load_acme_file(app, command)
            }
            "ls" | "dir" => {
                Directory::ls(app, command)
            }
            "opcodes" => {
                Opcodes::list_opcodes(app, command)
            }
            "pwd" => {
                Directory::pwd(app, command)
            }
            "set" => {
                Parameter::set(app, command)
            }
            "" => {
                Ok(AppState::Home)
            }
            _ => {
                Self::get_usage(app)
            }
        }
    }
    /// Shows usage
    fn get_usage(app: &mut App) -> Result<AppState, String> {
        let help = help::Help::new();
        let items = help.help_items;
        app.messages.push(String::from("Unknown command."));
        app.messages.push(String::from("  Available commands:"));
        for item in items {
            let mut msg = String::from("    ");
            msg.push_str(&item.usage);
            app.messages.push(msg);
        }
        Ok(AppState::Home)
    }
}
