use std::collections::VecDeque;
use std::{fs::File};
use std::io::{Read, Write};

use crate::io::IoPort;
use crate::io::ErrorIndicators;

const FILENAME_DATA_RDR: &str = "data/data.rdr";
const RDR_BUFFER_LENGTH: usize = 128;
const FILENAME_DATA_PUN: &str = "data/data.pun";
const NAME: &str = "RDR/PUN User Defined Device";
const EOF: u8 = 26;

/// User defined device for plm RDR and PUN mapping in CP/M 2.2.
/// 
/// 
pub struct RdrPun {
    base_address: Option<u8>,
    memory_base_address: Option<u16>,
    port_offsets: [u8; 1], // base_address that is used to read data from RDR or write data to PUN    
    name: Option<String>,
    reader_data_filename: Option<String>,
    reader_data_file: Option<File>,
    reader_data_buffer: VecDeque<u8>,
    reader_current_buffer_index: usize,
    puncher_data_filename: Option<String>,
    puncher_data_file: Option<File>,
}
impl Default for RdrPun {
    fn default() -> Self {
        RdrPun {
            base_address: None,
            memory_base_address: None,
            port_offsets: [0],
            name:  Some(NAME.to_string()),
            reader_data_filename: Some(FILENAME_DATA_RDR.to_string()),
            reader_data_file: None,
            reader_data_buffer: VecDeque::new(),
            reader_current_buffer_index: 0,
            puncher_data_filename: Some(FILENAME_DATA_PUN.to_string()),
            puncher_data_file: None,
        }
    }
}

impl RdrPun {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn get_name(&self) -> String {
        match &self.name {
            Some(name) => {
                name.to_string()
            },
            None => {
                "No name specified".to_string()
            }
        }
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string())
    }
    pub fn set_base_address(&mut self, address: u8) {
        self.base_address = Some(address)
    }
    pub fn set_memory_base_address(&mut self, address: u16) {
        self.memory_base_address = Some(address)
    }
    pub fn set_reader_data_filename(&mut self, filename: &str) {
        self.reader_data_filename = Some(filename.to_string())
    }
    pub fn set_puncher_data_filename(&mut self, filename: &str) {
        self.puncher_data_filename = Some(filename.to_string())
    }
    fn open_rdr_pun_file(name: &str, read_only: bool) -> std::io::Result<File> {
        let file: std::io::Result<File> = if read_only {
            std::fs::OpenOptions::new().read(true).open(name)
        } else {
            std::fs::OpenOptions::new()
                .write(true)
                .read(true)
                .open(name)
        };
        file
    }
    /// Read next byte from RDR file
    /// 
    /// Reads next byte from file defined for RDR (default is data/data.rdr).
    /// It returns either byte or EOF in case the end of file is reached or error occures.
    fn read_next_byte(&mut self) -> u8 {
        // Read next bytes from RDR file. If it is not open open it
        if self.reader_data_file.is_none() {
            // file is not open try to open it
            let filename = match &self.reader_data_filename {
                Some(filename) => {
                    filename
                }
                None => {
                    FILENAME_DATA_RDR
                }
            };
            match Self::open_rdr_pun_file(filename, true) {
                Ok(file) => {
                    self.reader_current_buffer_index = 0;
                    self.reader_data_buffer.clear();
                    self.reader_data_file = Some(file);
                }
                Err(_) => {
                    return EOF;
                }
            }
        };
        if self.reader_data_buffer.is_empty() {
            // Read from file
            let file = self.reader_data_file.as_mut().unwrap();
            let mut buff = [0u8; RDR_BUFFER_LENGTH];
            match file.read(&mut buff) {
                Ok(n) => {
                    self.reader_data_buffer.clear();
                    if n == 0 {
                        // End of file reached
                        self.reader_data_file = None;
                        return EOF;
                    }
                    self.reader_data_buffer.extend(&buff[0..n]);
                }
                Err(_) => {
                    self.reader_data_file = None;
                    return EOF;
                }
            };
        }
        match self.reader_data_buffer.pop_front() {
            Some(data) => {
                data
            }
            _ => {
                self.reader_data_file = None;
                EOF
            }
        }
    }
    /// Writes byte to PUN file
    /// 
    /// Writes byte to PUN file (default is data/data.pun). If EOF is received, the file is closed
    /// and name is appended by the timestamp so that it is not overwritten.
    fn write_byte(&mut self, data: u8) {
        if self.puncher_data_file.is_none() {
                // File doesn't exists let open the new one

                let now = chrono::offset::Local::now();
                let custom_datetime_format = now.format("%Y%m%d_%H%M%S");
                let mut new_name = FILENAME_DATA_PUN.to_string();
                new_name.push('.');
                new_name.push_str(custom_datetime_format.to_string().as_str());

                match File::create(new_name) {
                    Ok(f) => {
                        self.puncher_data_file = Some(f);
                    }
                    Err(_) => {
                        // At this point we cannot do anything so we just quit
                        return;
                    }
                };
        };
        // At this point we should have a file that is open for writing
        // Let's check for EOF
        if data == EOF {
            // Close the file
            self.puncher_data_file = None;
        }
        if self.puncher_data_file.is_none() {
            // File is already closed
            return;
        }
        _ = self.puncher_data_file.as_mut().unwrap().write(&[data]);
    }

}

impl IoPort for RdrPun {
    fn get_base_address(&self) -> Option<u8> {
        self.base_address
    }
    fn get_memory_base_address(&self) -> Option<u16> {
        self.memory_base_address
    }
    fn get_ports_offset(&self) -> &[u8] {
        &self.port_offsets
    }
    fn get_io_port_info(&self) -> String {
        let base_address = match (self.get_base_address(), self.get_memory_base_address()) {
            (Some(address), None) => {
                format!("0x{:02X}", address)
            }
            (None, Some(address)) => {
                format!("M0x{:04X}", address)
            }
            _ => "Not defined".to_string(),
        };
        format!(
            "RDR/PUN User defined device: base address[{base_address}], name[{}]", self.get_name()
        )
    }

    /// Read from IO address
    /// 
    /// Reads one byte from base address
    fn read_from_address(&mut self, address: u8) -> Option<u8> {
        if address != self.base_address.unwrap() {
            return Some(EOF);
        }
        Some(self.read_next_byte())
    }
    fn read_from_mem_address(&mut self, address: u16) -> Option<u8> {
        if address != self.memory_base_address.unwrap() {
            return Some(EOF);
        }
        Some(self.read_next_byte())
    }
    fn write_to_address(&mut self, _memory: &mut [crate::memory::MemCell], address: u8, data: u8) -> Result<Option<crate::memory::dma::Dma>, ErrorIndicators> {
        if address != self.base_address.unwrap() {
            return Ok(None);
        };
        self.write_byte(data);
        Ok(None)
    }
    fn write_to_memory_address(&mut self, _memory: &mut [crate::memory::MemCell], address: u16, data: u8) -> Result<Option<crate::memory::dma::Dma>, ErrorIndicators> {
        if address != self.memory_base_address.unwrap() {
            return Ok(None);
        }
        self.write_byte(data);
        Ok(None)        
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Write};
    use ihex::Record;
    use crate::cpu::CpuUi;
    use crate::io::{rdr_pun::{EOF, FILENAME_DATA_RDR, RdrPun}};
    use crate::cpu::i8080::{self};
    use crate::disassembler::i8080_opcode_consts::*;
    use glob::glob;

    const BASE: u8 = 0xF0;
    const MEMORY_BASE: u16 = 0x1000;

    /// Initialize test file
    /// 
    /// Initialises a test file in INTELHEX format
    fn init_rdr() {
        const DATA_SIZE: usize = 128;
        remove_rdr_file();
        let mut file = fs::File::create(FILENAME_DATA_RDR).unwrap();
        let mut data = [0u8; DATA_SIZE];
        for i in 0..DATA_SIZE {
            data[i] = i as u8;
        };
        let records = &[
            Record::Data { offset: 0x0000, value: data.to_vec() },
            Record::EndOfFile
        ];
        if let Ok(file_content) = ihex::create_object_file_representation(records) {
            _ = file.write_all(file_content.as_bytes());
        }
    }
    fn remove_rdr_file() {
        _ = fs::remove_file(FILENAME_DATA_RDR);
    }

    #[test]
    fn test_read() {
        init_rdr();
        let mut rdr = RdrPun::new();
        let mut data: Vec<u8> = Vec::new();

        loop {
            let value = rdr.read_next_byte();
            if value == EOF {
                break;
            }
            data.push(value);
        }
        assert_eq!(*data.get(0).unwrap(), ':' as u8);
        assert_eq!(data.len(), 280);
        // Assert the end of the first record in INTELHEX file 
        assert_eq!(*data.get(263).unwrap(), '7' as u8);
        assert_eq!(*data.get(264).unwrap(), 'F' as u8);
        assert_eq!(*data.get(265).unwrap(), 'C' as u8);
        assert_eq!(*data.get(266).unwrap(), '0' as u8);
        assert_eq!(*data.get(279).unwrap(), 10u8);
        remove_rdr_file();
    }
    #[test]
    fn read_rdr_io_mapped() {
        init_rdr();
        // Let's read from cpu
        let mut cpu = i8080::Cpu::new();
        let mut rdr_pun = Box::new(RdrPun::new());
        rdr_pun.set_base_address(BASE);
        let io_memory = cpu.get_io_memory().unwrap();
        let res = io_memory.map_port(rdr_pun);
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(false);
        let program_address = 0x100;
        let data_address = 0x1000u16;
        let io_address = BASE;
        // let read data 
        let program: &[u8] = &[
            LXI_H, (data_address & 0xff) as u8, ((data_address & 0xff00) >> 8) as u8,  
            IN, io_address,
            CPI, EOF,
            JZ, 0x0F, 0x01,
            MOV_M_A,
            INX_H,
            JMP, 0x03, 0x01,
            HLT,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            cpu.one_step();
            if opcode == HLT {
                break;
            }
        }
        remove_rdr_file();
        let mut data = cpu.get_memory().read_byte(0x1000);
        // Assert the beginning of the file
        assert_eq!(data, ':' as u8);
        // Assert the end of the file
        data = cpu.get_memory().read_byte(0x1000 + 263);
        assert_eq!(data, '7' as u8);
        data = cpu.get_memory().read_byte(0x1000 + 264);
        assert_eq!(data, 'F' as u8);
        data = cpu.get_memory().read_byte(0x1000 + 265);
        assert_eq!(data, 'C' as u8);
        data = cpu.get_memory().read_byte(0x1000 + 266);
        assert_eq!(data, '0' as u8);
        data = cpu.get_memory().read_byte(0x1000 + 267);
        assert_eq!(data, 10u8);
    }
    #[test]
    fn read_rdr_memory_mapped() {
        init_rdr();
        // Let's read from cpu
        let mut cpu = i8080::Cpu::new();
        let mut rdr_pun = Box::new(RdrPun::new());
        rdr_pun.set_memory_base_address(MEMORY_BASE);
        let res = cpu.get_memory().map_port(rdr_pun);
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(false);
        let program_address = 0x100;
        let data_address = 0x2000u16;
        let io_address = MEMORY_BASE;
        // let read data 
        let program: &[u8] = &[
            LXI_D, (data_address & 0xff) as u8, ((data_address & 0xff00) >> 8) as u8, 
            LXI_H, (io_address & 0xff) as u8, ((io_address & 0xff00) >> 8) as u8, 
            MOV_A_M,
            CPI, EOF,
            JZ, 0x13, 0x01,
            XCHG,
            MOV_M_A,
            XCHG,
            INX_D,
            JMP, 0x06, 0x01,
            HLT,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            cpu.one_step();
            if opcode == HLT {
                break;
            }
        }
        remove_rdr_file();
        let mut data = cpu.get_memory().read_byte(0x2000);
        // Assert the beginning of the file
        assert_eq!(data, ':' as u8);
        // Assert the end of the file
        data = cpu.get_memory().read_byte(0x2000 + 263);
        assert_eq!(data, '7' as u8);
        data = cpu.get_memory().read_byte(0x2000 + 264);
        assert_eq!(data, 'F' as u8);
        data = cpu.get_memory().read_byte(0x2000 + 265);
        assert_eq!(data, 'C' as u8);
        data = cpu.get_memory().read_byte(0x2000 + 266);
        assert_eq!(data, '0' as u8);
        data = cpu.get_memory().read_byte(0x2000 + 267);
        assert_eq!(data, 10u8);
    }
    #[test]
    fn test_write() {
        let mut pun = RdrPun::new();
        for data in 0x20 .. 0x7f {
            pun.write_byte(data);
        }
        pun.write_byte(EOF);
        let paths = glob("data/data.pun.*").expect("Failed to read glob pattern");
        let last = paths.last().unwrap().unwrap();
        let content_as_string = fs::read_to_string(last.clone()).unwrap();
        // Remove test file (which is the last in the list)
        _ = fs::remove_file(last);
        let bytes = content_as_string.as_bytes();
        assert_eq!(bytes[0], ' ' as u8);
        assert_eq!(*bytes.last().unwrap(), '~' as u8);
    }
    #[test]
    fn write_pun_io_mapped() {

        let paths = glob("data/data.pun.*").expect("Failed to read glob pattern");
        for path in paths {
            let name = path.unwrap();
            _ = fs::remove_file(name);

        }
        // Let's read from cpu
        let mut cpu = i8080::Cpu::new();
        let mut rdr_pun = Box::new(RdrPun::new());
        rdr_pun.set_base_address(BASE);
        let io_memory = cpu.get_io_memory().unwrap();
        let res = io_memory.map_port(rdr_pun);
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(false);
        let program_address = 0x100;
        let io_address = BASE;
        // let read data 
        let program: &[u8] = &[
            MVI_A, ' ' as u8,  
            OUT, io_address,
            INR_A,
            CPI, '~' as u8 + 1, // Just after the last printable character '~'
            JNZ, 0x02, 0x01,
            MVI_A, EOF, // Send EOF
            OUT, io_address,
            HLT,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            cpu.one_step();
            if opcode == HLT {
                break;
            }
        }
        let paths = glob("data/data.pun.*").expect("Failed to read glob pattern");
        let last = paths.last().unwrap().unwrap();
        let content_as_string = fs::read_to_string(last.clone()).unwrap();
        // Remove test file (which is the last in the list)
        _ = fs::remove_file(last);
        let bytes = content_as_string.as_bytes();
        assert_eq!(bytes[0], ' ' as u8);
        assert_eq!(*bytes.last().unwrap(), '~' as u8);
    }
    #[test]
    fn write_pun_memory_mapped() {
        // Let's read from cpu
        let mut cpu = i8080::Cpu::new();
        let mut rdr_pun = Box::new(RdrPun::new());
        rdr_pun.set_memory_base_address(MEMORY_BASE);
        let res = cpu.get_memory().map_port(rdr_pun);
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(false);
        let program_address = 0x100;
        let io_address = MEMORY_BASE;
        // let read data 
        let program: &[u8] = &[
            LXI_H, (io_address & 0xff) as u8, ((io_address & 0xff00) >> 8) as u8,
            MVI_A, ' ' as u8,  
            MOV_M_A,
            INR_A,
            CPI, '~' as u8 + 1, // Just after the last printable character '~'
            JNZ, 0x05, 0x01,
            MVI_A, EOF, // Send EOF
            MOV_M_A,
            HLT,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            cpu.one_step();
            if opcode == HLT {
                break;
            }
        }
        let paths = glob("data/data.pun.*").expect("Failed to read glob pattern");
        let last = paths.last().unwrap().unwrap();
        let content_as_string = fs::read_to_string(last.clone()).unwrap();
        // Remove test file (which is the last in the list)
        _ = fs::remove_file(last);
        let bytes = content_as_string.as_bytes();
        assert_eq!(bytes[0], ' ' as u8);
        assert_eq!(*bytes.last().unwrap(), '~' as u8);
    }
}