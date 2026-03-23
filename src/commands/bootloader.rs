use crate::bootloader::Bootloader as Bl;
use crate::ui::app::App;
use crate::ui::app::AppState;
use std::path::Path;

pub struct Bootloader {}

impl Bootloader {
    pub fn bootloader(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        app.is_cpu_set()?; // Check if cpu is defined
        let cpu = app.cpu_ui.as_mut().unwrap();
        match command.len() {
            1 => {
                match cpu.get_bootloader() {
                    Some(bootloader) => {
                        app.messages.push(format!("Bootloader: {}", bootloader.get_filename()));
                    }
                    None => {
                        app.messages.push("Bootloader is not set.".to_string());
                    }
                };
                Ok(AppState::Home)
            }
            2 => {
                if !Path::exists(Path::new(command[1])) {
                    app.messages
                        .push(format!("Bootloader file '{}' doesn't exist.", command[1]));
                    return Ok(AppState::Home)
                }
                cpu.set_bootloader(Bl::new(command[1].to_string()));
                Ok(AppState::Home)
            }
            _ => {
                app.messages
                    .push("Error: Wrong number of parameters.".to_string());
                app.messages.push("  Usage: bl <intelhex filename> or bootloader <intelhex filename>".to_string());
                Ok(AppState::Home)
            }
        }
    }
}