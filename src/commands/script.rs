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
        let path = command[1].to_string();
        match read_to_string(&path){
            Ok(content) => {
                for line in content.lines() {
                    let characters = line.as_bytes();
                    for character in characters {
                        app.enter_char(*character as char);
                    }
                    app.submit_message();
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