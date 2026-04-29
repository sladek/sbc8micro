use crate::disk::Utils;
use crate::ui::app::App;
use crate::ui::app::AppState;

pub struct Imd2Raw{}

impl Imd2Raw {
    pub fn imd2raw(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        let source_filename: String;
        let mut destination_filename: String;
        match command.len() {
            2 => {
                source_filename = command[1].to_string();
                destination_filename = source_filename.clone();
                if destination_filename.to_ascii_uppercase().ends_with(".IMD") {
                    destination_filename = destination_filename.replace(".imd", ".raw");
                }
                else {
                    destination_filename.push_str(".raw");
                }
            }
            3 => {
                source_filename = command[1].to_string();
                destination_filename = command[2].to_string();
            }
            _ => {
                app.messages
                    .push("ERROR - Invalid number of parameters. Usage: imd2raw <input file> [output file]".to_string());
                return Ok(AppState::Home);
            }
        }

        match Utils::imd2raw(source_filename, destination_filename) {
            Ok(report) => {
                app.messages.push(report);
                Ok(AppState::Home)
            }
            Err(err) => {
                Err(format!("Error - {:?}", err.to_string()))
            }
        }       
    }
}
