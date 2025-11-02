//! Generic memory implementation
use crate::io::IoPort;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};
use crate::ui::app::AppState;
use intelhex::IntelHexFile;
use intelhex::file::RecordType;

const CAPACITY: usize = 0x10000;

#[derive(Default)]
pub struct Region {
    pub start: u16,
    pub end: u16,
}

impl Region {
    pub fn new() -> Self {
        Self { start: 0, end: 0 }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MemCell {
    Memory(u8), // Direct value
    Io(u16), // Address in the memory
}
/// This can be used only for comparing 8 bit content of memory
/// For 16 bit it always return false
impl PartialEq<u8> for MemCell {
    fn eq(&self, other: &u8) -> bool {
        match (self, other) {
            (MemCell::Memory(data), other) => data == other,
            (MemCell::Io(_data), _other) => false,
        }
    }
}

//#[derive(Clone)]
/// Memory that can be assigned to specific CPU
pub struct Memory {
    /// Data of the memory
    data: [MemCell; CAPACITY], // 64KB
    ports: HashMap<u16, Box<dyn IoPort>>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: [MemCell::Memory(0); CAPACITY],
            ports: HashMap::new(),
        }
    }
    pub fn map_port(&mut self, port: Box<dyn IoPort>) -> Result<(), String> {
        let offset = port.get_ports_offset();
        match port.get_memory_base_address() {
            Some(address) => {
                for i in offset {
                    self.data[(address + *i as u16) as usize] = MemCell::Io(address);
                }
                self.ports.insert(address, port);
                Ok(())
            }
            None => Err("Base address is not defined".to_string()),
        }
    }
    /// Removes ports mapped to base address
    pub fn remove(&mut self, base_address: u16) -> Result<AppState, String> {
        match self.ports.get(&base_address) {
            Some(port) => {
                // Remove ports from memory
                let offsets = port.get_ports_offset();
                for offset in offsets {
                    let address = base_address + *offset as u16;
                    self.data[address as usize] = MemCell::Memory(0);
                }
                // remove port from ports HashMap
                self.ports.remove(&base_address);
            }
            None => {
                return Err(format!("No device mapped to this address [{base_address}]"));
            }
        }
        Ok(AppState::Home)
    }
    /// Gets io port info
    pub fn get_io_ports_info(&self) -> Vec<String> {
        let ports = &self.ports;
        let mut info: Vec<String> = Vec::new();
        for v in ports.values() {
            info.push(v.get_io_port_info());
        }
        info
    }
    /// Gets ports mapped into memory
    pub fn get_ports(&mut self) -> &mut HashMap<u16, Box<dyn IoPort>> {
        &mut self.ports
    }
    /// Reads a byte from specific address
    pub fn read_byte(&mut self, addr: u16) -> u8 {
        self.get_byte(addr, self.data[addr as usize])
    }
    /// Gets a byte from MemCell
    fn get_byte(&mut self, addr: u16, cell: MemCell) -> u8 {
        match cell {
            MemCell::Memory(value) => value,
            MemCell::Io(address) => {
                if let Some(port) = self.ports.get_mut(&address) {
                    port.read_from_mem_address(addr).unwrap_or(0xff)
                } else {
                    0xff // Not yet implemented
                }
            }
        }
    }
    /// Writes a byte to specific address
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match self.data[address as usize] {
            MemCell::Memory(_) => {
                self.data[address as usize] = MemCell::Memory(value);
            }
            MemCell::Io(addr) => {
                if let Some(port) = self.ports.get_mut(&addr) {
                        port.write_to_memory_address(address, value);
                }
            }
        }
    }
    /// Reads a word from specific address
    pub fn read_word(&mut self, addr: u16) -> u16 {
        let lo = self.read_byte(addr) as u16;
        let hi = self.read_byte(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }
    /// Write a word to specific address
    pub fn write_word(&mut self, addr: u16, value: u16) {
        self.write_byte(addr, (value & 0xFF) as u8);
        self.write_byte(addr.wrapping_add(1), (value >> 8) as u8);
    }
    /// Reads a byte from zero_page [0, 0xff]
    ///
    /// Address is only 8bit long so it can read a data from address range of 0x00 .. 0xff.
    /// This is used by the CPU like mos6502
    pub fn read_byte_zero_page(&mut self, addr: u8) -> u8 {
        self.read_byte(addr as u16)
    }
    /// Writes a byte to zero_page [0, 0xff]
    ///
    /// Address is only 8bit long so it can write a data to address range of 0x00 .. 0xff.
    /// This is used by the CPU like mos6502
    pub fn write_byte_zero_page(&mut self, addr: u8, value: u8) {
        self.write_byte(addr as u16, value);
    }
    /// Reads a word from zero_page [0, 0xff]
    ///
    /// Address is only 8bit long so it can read a data from address range of 0x00 .. 0xff.
    /// This is used by the CPU like mos6502
    pub fn read_word_zero_page(&mut self, addr: u8) -> u16 {
        let lo = self.read_byte(addr as u16) as u16;
        let hi = self.read_byte(addr.wrapping_add(1) as u16) as u16;
        (hi << 8) | lo
    }
    /// Writes a byte to zero_page [0, 0xff]
    ///
    /// Address is only 8bit long so it can write a data to address range of 0x00 .. 0xff.
    /// This is used by the CPU like mos6502
    pub fn write_word_zero_page(&mut self, addr: u8, value: u16) {
        self.write_word(addr as u16, value);
    }
    /// Loads a binary code from a file
    ///
    /// Loads a binary code from a file to specific location in memory starting
    /// at start_addr
    pub fn load_data(&mut self, data: &[u8], start_addr: u16) -> Result<Region, Error> {
        let mut end_addr = start_addr;
        for (i, &byte) in data.iter().enumerate() {
            end_addr = start_addr.saturating_add(i as u16);
            self.write_byte(end_addr, byte);
            if end_addr == 0xffffu16 {
                // End of memory reached
                return Ok(Region {
                    start: start_addr,
                    end: end_addr,
                });
            }
        }
        Ok(Region {
            start: start_addr,
            end: end_addr,
        })
    }
    /// Loads data from ACME file
    ///
    /// Loads .obj file to memory. The first 2 bytes of the file contain load address.
    /// This format is generated by ACME 6502 compiler
    pub fn load_data_from_acme_file(&mut self, file_name: &str) -> Result<Region, Error> {
        // Open the binary file
        let mut file = File::open(file_name)?;
        // Create a buffer to hold the data
        let mut buffer: Vec<u8> = Vec::new();
        // Read the file into the buffer
        file.read_to_end(&mut buffer)?;
        let start_addr_lo = buffer[0];
        let start_addr_hi = buffer[1];
        let start_addr = (start_addr_hi as u16) << 8 | start_addr_lo as u16;
        self.load_data(&buffer[2..], start_addr)
    }
    /// Loads data from IntelHex file
    pub fn load_data_from_intelhex_file(&mut self, file_name: &str) -> Result<Region, Error> {
        let mut start: u16 = 0;
        let mut end: u16 = 0;
        match IntelHexFile::load_file(file_name) {
            Ok(file) => {
                let n_records = file.records.len();
                start = file.records[0].addr;                
                for i in 0..n_records {
                    let record = &file.records[i];                    
                    match  &record.rtype {
                        RecordType::Data => {
                            let addr = record.addr;
                            let data = record.data.to_vec();
                            self.load_data(&data, addr)?;                        }
                        RecordType::EndOfFile => {
                            end = record.addr;
                            break;
                        }
                        _ => {}
                    }
                }
            },
            Err(err) => println!("{:?}", err)
        }
        Ok(Region { start, end })
    }
    /// Retusrns memory data as an array
    pub fn get_data(&self) -> [MemCell; CAPACITY] {
        self.data
    }
    /// Prints content of memory slice [start_addr, end_addr] as hexadecimal dump via println!
    pub fn print_hex_dump(&mut self, start_addr: u16, end_addr: u16) {
        let hex_dump = self.hex_dump(start_addr, end_addr);
        for line in hex_dump {
            println!("{line}");
        }
    }
    /// Gets content of memory slice [start_addr, end_addr] as hexadecimal dump in Vector<String>
    pub fn hex_dump(&mut self, start_addr: u16, end_addr: u16) -> Vec<String> {
        let mut hex_dump: Vec<String> = Vec::new();
        let mut input_data: Vec<(u16, MemCell)> = Vec::new();
        // To prevent multiple mutable borrows of self later, we clone the array to data variable
        for (i, cell) in self.data[start_addr as usize..=end_addr as usize].iter().enumerate() {
            input_data.push((i as u16, *cell));
        }
        for (i, chunk) in input_data.chunks(16).enumerate() {
            let mut line = String::new();
            line += &format!("{:08X}: ", (i * 16) + start_addr as usize);
            // Print hex values
            for cell in chunk {
                let (addr, c) = *cell;
                let value = self.get_byte(addr, c);
                line += &format!("{:02X} ", value);
            }

            // Pad if less than 16 bytes
            for _ in 0..(16 - chunk.len()) {
                line += "   ";
            }
            // Print ASCII representation
            line += "|";
            let mut c_count = 16;
            for &cell in chunk {
                let (addr, c) = cell;
                let byte = self.get_byte(addr, c);
                let c = if byte.is_ascii_graphic() || byte == b' ' {
                    byte as char
                } else {
                    '.'
                };
                line.push(c);
                c_count -= 1;
            }
            // Padding of the last row for number of characters smaller than 16
            while c_count > 0 {
                line.push(' ');
                c_count -= 1;
            }
            line.push('|');
            hex_dump.push(line);
        }
        hex_dump
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::{self, MemCell, Memory};
    #[test]
    ///
    /// Writes and reads back byte from memory
    ///
    fn write_read_byte() {
        let mut memory = Memory::new();
        let addr = 0x0100u16;
        let value = 0x55u8;
        memory.write_byte(addr, value);
        let result = memory.read_byte(addr);
        assert_eq! {result, value};
    }
    #[test]
    ///
    /// Writes and reads back word from memory
    ///
    fn write_read_word() {
        let mut memory = Memory::new();
        let addr = 0x0100u16;
        let value = 0x55AAu16;
        memory.write_word(addr, value);
        let result = memory.read_word(addr);
        assert_eq!(result, value);
    }
    #[test]
    ///
    /// Writes word and reads back bytes from memory
    /// to verify proper order
    ///
    fn write_word_read_bytes() {
        let mut memory = Memory::new();
        let addr = 0x0100u16;
        let value = 0x55AAu16;
        memory.write_word(addr, value);
        let byte0 = memory.read_byte(addr) as u16;
        let byte1 = memory.read_byte(addr.wrapping_add(1)) as u16;
        let result: u16 = byte1 << 8 | byte0;
        assert_eq!(result, value);
    }
    #[test]
    ///
    /// Writes bytes and reads back word from memory
    /// to verify proper order
    ///
    fn write_bytes_read_word() {
        let mut memory = Memory::new();
        let addr = 0x0100u16;
        let value0 = 0x55u16;
        let value1 = 0xAAu16;
        memory.write_byte(addr, value0 as u8);
        memory.write_byte(addr.wrapping_add(1), value1 as u8);
        let result = memory.read_word(addr);
        let value = value1 << 8 | value0;
        assert_eq!(result, value);
    }
    #[test]
    ///
    /// Writes word to the end of memory (0xffff)
    /// to verify wrapping to the beginning (0x0000)
    ///
    fn write_word_end_of_memory() {
        let mut memory = Memory::new();
        let addr = 0xFFFFu16;
        let value = 0x55AAu16;
        memory.write_word(addr, value);
        let result1 = memory.read_byte(addr) as u16;
        let result2 = memory.read_byte(0x0) as u16;
        let result = result2 << 8 | result1;
        assert_eq!(result, value);
    }
    #[test]
    ///
    /// Writes byte to the zero page and then reads it back
    ///
    fn read_byte_zero_page() {
        let mut memory = Memory::new();
        let addr = 0x0010u16;
        let value = 0x55u8;
        memory.write_byte(addr, value);
        let result = memory.read_byte_zero_page(addr as u8);
        assert_eq!(result, value);
    }

    #[test]
    ///
    /// Writes word to the zero page and then reads it back
    ///
    fn read_word_zero_page() {
        let mut memory = Memory::new();
        let addr = 0x0010u16;
        let value = 0x55AAu16;
        memory.write_word(addr, value);
        let result = memory.read_word_zero_page(addr as u8);
        assert_eq!(result, value);
    }
    #[test]
    ///
    /// Writes word to the end of zero page (0xff)
    /// to verify wrapping to the beginning (0x0000)
    ///
    fn read_word_zero_page_wrap_around() {
        let mut memory = Memory::new();
        let addr = 0xFFu8;
        let value = 0x55AAu16;
        memory.write_byte(addr as u16, (value & 0x00ff) as u8);
        memory.write_byte(0x0000u16, ((value & 0xff00) >> 8) as u8);
        let result = memory.read_word_zero_page(addr);
        assert_eq!(result, value);
    }
    #[test]
    ///
    /// Load a short probram and verifies if it is written properly
    ///
    fn load_program() {
        let mut memory = memory::Memory::new();
        let program = vec![
            0xA9, 0x01, // LDA #$01
            0x8D, 0x00, 0x02, // STA $0200
            0xE8, // INX
            0xF0, 0xFD, // BEQ $0600
            0x00, // BRK
        ];
        let start = 0x0000;
        let _ = memory.load_data(&program, start);
        let mem_slice = &memory.data[0..=8];
        assert_eq!(mem_slice, program);
    }
    #[test]
    fn test_io_map_remove() {
        use crate::io::memory::DummyIo;
        let base_memory_address = 0x1234;
        let mut memory = Memory::new();
        let _ = memory.map_port(Box::new(DummyIo::new())).unwrap();
        let port = memory.ports.get(&base_memory_address);
        assert!(!port.is_none());
        let offsets = port.unwrap().get_ports_offset();
        let mem1 = base_memory_address + offsets[0] as u16; 
        let mem2 = base_memory_address + offsets[1] as u16;
        match memory.data[mem1 as usize] {
            MemCell::Io(address) => {
                assert!(address == base_memory_address);
            }
            _ => {
                assert!(false);
            }
        }
        match memory.data[mem2 as usize] {
            MemCell::Io(address) => {
                assert!(address == base_memory_address);
            }
            _ => {
                assert!(false);
            }
        }
        let result = memory.remove(base_memory_address);
        assert!(result.is_ok());
        match memory.data[mem1 as usize] { // Is Memory cell back?
            MemCell::Memory(data) => {
                assert_eq!(data, 0); // Default data should be 0
            }
            _ => {
                assert!(false);
            }
        }
        match memory.data[mem2 as usize] { // Is memory cell back?
            MemCell::Memory(data) => {
                assert_eq!(data, 0); // Default data should be 0
            }
            _ => {
                assert!(false);
            }
        }
    }

}
