use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use crate::commands::memory::Memory;
use crate::ui::app::App;
use crate::ui::app::AppState;
use ihex::Record;

const RECORD_LENGTH: usize = 16;

pub struct Ihex{}

impl Ihex {
    pub fn ihex(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        let source_filename: String;
        let mut destination_filename: String;
        let mut source_file: File;
        let mut destination_file: File;

        if command.len() < 3 || command.len() > 4 {
            app.messages
                .push("ERROR - Invalid number of parameters. Usage: ihex <offset> <source filename> [destination filename]".to_string());
            return Ok(AppState::Home);
        }
        let mut offset = Memory::from_hex_string(command[1].to_uppercase())?;
        match command.len(){
            4 => {
                source_filename = command[2].to_string();
                destination_filename = command[3].to_string();
                if !destination_filename.to_lowercase().ends_with(".rdr") {
                    if !destination_filename.ends_with(".hex") {
                        destination_filename.push_str(".hex");
                    }                
                }
            }
            // Here it can be only 2
            _ => {
                source_filename = command[2].to_string();
                destination_filename = source_filename.clone();
                destination_filename.push_str(".hex");
            }
        }
        match fs::File::open(source_filename){
            Ok(file) => {
                source_file = file;
            }
            Err(err) => {
                return Err(err.to_string());
            }
        };

        match fs::File::create(destination_filename){
            Ok(file) => {
                destination_file = file;
            }
            Err(err) => {
                return Err(err.to_string());
            }
        };
        let mut buff= [0 ;RECORD_LENGTH];
        let mut records :Vec<Record> = Vec::new();
        loop  {
            match source_file.read(&mut buff) {
                Ok(bytes) => {
                    if bytes == 0 {
                        break;
                    }
                    let data = Vec::from(&buff[..bytes]);
                    records.push(Record::Data {offset: offset, value: data});
                    offset += RECORD_LENGTH as u16;
                }
                Err(err) => {
                    return Err(err.to_string());
                }
            }
        }
        records.push(Record::EndOfFile);
        match ihex::create_object_file_representation(&records){
            Ok(value) => {
                match destination_file.write_all(value.as_bytes()) {
                    Ok(()) => {
                        return Ok(AppState::Home);
                    }
                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            }
            Err(err) => {
                return Err(err.to_string());
            }
        }
    }
}
