use crate::help;
use crate::ui::app::{App, AppState};

pub struct Help;
impl Help {
    pub fn help(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        let help = help::Help::new();
        if command.len() == 1 {
            app.messages.push("Available commands:".to_string());
            // Show only help items
            for item in help.help_items {
                let mut help_desc = "  ".to_string();
                help_desc.push_str(&item.command);
                help_desc.push_str(" - ");
                help_desc.push_str(&item.description);
                app.messages.push(help_desc);
            }
            return Ok(AppState::Home);
        }
        if command.len() != 2 {
            app.messages
                .push("ERROR - Invalind number of parameters. Usage: help or help <command>".to_string());
            return Ok(AppState::Home);
        }
        let item = help.get_item(command[1]);
        match item {
            Some(item) => {
                let mut command = " Command: ".to_string();
                command.push_str(&item.command.to_string());
                app.messages.push(command);
                let mut description = " Description: ".to_string();
                description.push_str(&item.description.to_string());
                app.messages.push(description);
                let mut usage = " Usage: ".to_string();
                usage.push_str(&item.usage.to_string());
                app.messages.push(usage);
                let mut examples = " Examples: ".to_string();
                examples.push_str(&item.examples.to_string());
                let mut lines = Help::split_line(examples);
                app.messages.append(&mut lines);
            }
            None => {
                app.messages.push("ERROR - Help for this command couldn't be found. Use \"help\" for list of available options.".to_string());
            }
        }
        Ok(AppState::Home)
    }
    /// Splits lines with '\' character
    ///
    /// Splits lines with '\' character and returns Option<Vec<String>>
    fn split_line(line: String) -> Vec<String> {
        let lines = line.split("\\n");
        let mut result = Vec::new();
        for line in lines {
            result.push(line.to_string());
        }
        result
    }
}
