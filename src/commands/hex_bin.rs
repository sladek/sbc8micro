use std::fs;
use std::fs::File;
use std::io::Write;
use crate::ui::app::App;
use crate::ui::app::AppState;
use crate::commands::memory::Memory;
use ihex::Reader;
use std::collections::BTreeMap;

pub struct HexBin{}

impl HexBin {
    pub fn hex_bin(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        let source_filename: String;
        let mut destination_filename: String;
        let mut destination_file: File;

        if command.len() < 3 || command.len() > 4 {
            app.messages
                .push("ERROR - Invalid number of parameters. Usage: hex_bin <offset> <source filename> [destination filename]".to_string());
            return Ok(AppState::Home);
        }
        let hex_offset = Memory::from_hex_string(command[1].to_uppercase())?;
        match command.len(){
            4 => {
                source_filename = command[2].to_string();
                destination_filename = command[3].to_string();
                if !destination_filename.contains(".") {
                        destination_filename.push_str(".com");

                }
            }
            // Here it can be only 2
            _ => {
                source_filename = command[2].to_string();
                destination_filename = source_filename.clone();
                destination_filename.push_str(".com");
            }
        }
        match fs::File::create(destination_filename.clone()){
            Ok(file) => {
                destination_file = file;
            }
            Err(err) => {
                return Err(err.to_string());
            }
        };
        let ihex_data: String = match std::fs::read_to_string(source_filename) {
            Ok(ihdata) =>{
                ihdata
            }
            Err(err) => {
                return Err(err.to_string());
            }
        };
        let mut memory_map: BTreeMap<u32, u8> = BTreeMap::new();
        let mut max_addr = 0;

        // Parse Intel HEX records
        for record in Reader::new(&ihex_data) {
            match record {
                Ok(ihex::Record::Data { offset, value }) => {
                    let off: u16 = offset.saturating_sub(hex_offset);
                     for (i, byte) in value.iter().enumerate() {
                        let addr = off + i as u16;
                        memory_map.insert(addr.into(), *byte);
                        if addr > max_addr { max_addr = addr; }
                    }
                }
                Ok(ihex::Record::EndOfFile) => break,
                _ => {}
            }
        }

        // Generate binary data, filling gaps with 0xFF
        let mut binary_data = vec![0xFFu8; max_addr as usize + 1];
        for (addr, byte) in memory_map {
            binary_data[addr as usize] = byte;
        }

        // Write to file
        if let Err(err) = destination_file.write_all(&binary_data) {
            return Err(err.to_string());
            };

    Ok(AppState::Home)
    }
}
