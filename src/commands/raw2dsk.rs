use crate::disk::{Disk, sssd8fd};
use crate::ui::app::App;
use crate::ui::app::AppState;

pub struct Raw2Dsk{}

const DISK_FORMATS: [&str;1] = ["SSSD"];

impl Raw2Dsk {
    pub fn raw2dsk(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        let source_filename: String;
        let destination: String;
        let mut raw_filename:String = String::new();
        let disk_format: String;
        match command.len() {
            3 => {
                source_filename = command[1].to_string();
                destination = command[2].to_string();
                if !destination.contains(":") {
                    app.messages
                        .push("ERROR - <disk format> prefix is missing. Usage: raw2dsk <input file> <disk format>:[output file]".to_string());
                    return Ok(AppState::Home);
                }
                let names: Vec<&str> = destination.split(":").collect();
                disk_format = names[0].to_string();
                let mut format_found = false;
                for item in DISK_FORMATS {
                    if disk_format.to_uppercase().eq( item ){
                        format_found = true;
                    };
                }
                if !format_found {
                    app.messages
                        .push(format!("ERROR - invalid disk format ({disk_format}). Valid disk formats are: {:?}", DISK_FORMATS).to_string());
                    return Ok(AppState::Home);
                }
                // Let process "empty" destination filename
                if names[1].is_empty() {
                    let names:Vec<&str> = source_filename.split(".").collect();
                    let mut name = names[0].to_string();
                    name.push_str(".dsk");
                    raw_filename.push_str(&name);
                }
                else {
                    raw_filename.push_str(names[1]); 
                }
                if !raw_filename.to_uppercase().ends_with(".DSK") {
                    let names: Vec<&str> = raw_filename.split(".").collect();
                    raw_filename = names[0].to_string();
                    raw_filename.push_str(".dsk");
                };
            }
            _ => {
                app.messages
                    .push("ERROR - Invalid number of parameters. Usage: raw2dsk <input file> <disk format>:[output file]".to_string());
                return Ok(AppState::Home);
            }
        }
        match disk_format.to_uppercase().as_str() {
            // This section can be repeated also for ither disks as raw2dsk is a trait implemented fro every disk
            // Just change sssd8fd to new module. Res can stay the same
            "SSSD" => {
                match sssd8fd::Floppy::new(&raw_filename, false) {
                    Ok(mut floppy) => {
                        if let Err(err) = floppy.raw2dsk(source_filename) {
                            app.messages.push(format!("Error: ({:?}) occured while generating the file: {raw_filename}", err));
                            return Ok(AppState::Home);     
                        };
                    }
                    Err(err) => {
                        app.messages.push(format!("Error: ({:?}) occured while creating the file: {raw_filename}", err));
                        return Ok(AppState::Home);     
                    }
                }
            }
            _ => {
                app.messages
                    .push(format!("ERROR - invalid disk format ({disk_format}). Valid disk formats are: {:?}", DISK_FORMATS).to_string());
                return Ok(AppState::Home);
            }
        }
        let output = format!("Generated file: {raw_filename}");
        app.messages.push(output);
        Ok(AppState::Home) 
    }
}
