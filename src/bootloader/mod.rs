use std::io::Error;
use crate::memory::Memory;

#[derive(Clone)]
pub struct Bootloader {
    filename: String,
    start_address: u16,
}

impl Bootloader {
    pub fn new(filename: String) -> Self {
        Self { filename, start_address: 0x0000 }
    }
    pub fn get_start_address(&self) -> u16 {
        0x0000u16
    }
    pub fn get_filename(&self) -> String {
        self.filename.clone()
    }
    pub fn load(&mut self, memory: &mut Memory)  -> Result<(), Error> {
        match memory.load_data_from_intelhex_file(&self.filename) {
            Ok(region) => {
                self.start_address = region.start;
                Ok(())
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}