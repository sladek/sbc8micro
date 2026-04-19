use std::fs::read_to_string;

use crate::ui::app::AppState;
use crate::ui::app::App;
pub struct Script;

impl Script {
    pub fn script(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        if command.len() != 2 {
            app.messages.push(
                "ERROR - Invalid number of parameters. Usage: script <script name>"
                    .to_string(),
            );
            return Ok(AppState::Home);
        }
        let mut path = command[1].to_string();
        if !path.ends_with(".scr") {
            path += ".scr";
        }
        match read_to_string(&path){
            Ok(content) => {
                for line in content.lines() {
                    if line.trim().starts_with("#") || line.trim().starts_with(";") {
                        continue;
                    };
                    let characters = line.as_bytes();
                    for character in characters {
                        app.enter_char(*character as char);
                    }
                    app.submit_message();
                    // Let's check the message from the command for ERROR
                    let msg = app.messages.last().unwrap();
                    if msg.to_uppercase().starts_with("ERROR") {
                        break;
                    };
                }
            }
            Err(err) => {
                app.messages.push(
                    format!("ERROR - Cannot open script file: {path}. ERROR - {err}")
                );

            }
        };
        Ok(AppState::Home)
    }
}