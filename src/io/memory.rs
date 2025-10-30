//! Input output memory
//!
//! Simulates input output memory with address space of 0xff bytes as in Intel 8080 CPU
//! Periferal that implements IoPort can be mapped to the memory and then read and write
//! instructions can be used to transfer data to/from io.
//!
//! ```
//! use sbc8micro::io::memory::{DummyIo, Memory};
//!
//! let address = 0x40;
//! let value = 0x55;
//! let mut memory = Memory::new();
//! memory.map_port(Box::new(DummyIo::new()));
//! memory.write(address, value);
//! let result = memory.read(address);
//! assert_eq!(value, result)
//! ```
use std::{collections::HashMap};
use crate::io::*;

#[derive(Default)]
pub struct Memory {
    addresses: Vec<Box<dyn IoPort>>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            addresses: Vec::new(),
        }
    }
    pub fn map_port(&mut self, port: Box<dyn IoPort>) {
        self.addresses.push(port);
    }
    pub fn clear(&mut self) {
        self.addresses.clear();
    }
    /// Reads data from io address.
    ///
    /// Reads data from io address and returns data as u8. If the port is not present on that address
    /// it returns 0xff which simulates real behavior of reading from non existing periferals.
    pub fn read(&mut self, address: u8) -> u8 {
        let mut i = 0;
        while i < self.addresses.len() {
            let port = &mut self.addresses[i];
            if let Some(data) = port.read_from_address(address) {
                return data;
            };
            i += 1;
        }
        0xff
    }
    /// Writes data to specific io address
    ///
    /// Writes data to specific io address. As in real system, it doesn't indicate success or failure
    /// and it is on the user to make sure that periferal exist on that address.
    pub fn write(&mut self, address: u8, data: u8) {
        let mut i = 0;
        while i < self.addresses.len(){
            let port = &mut self.addresses[i];
            port.write_to_address(address, data);
            i += 1;
        }
    }
}


const MEMORY_SIZE: usize = 256;
pub struct IoMemory{
    port_map: [Option<u8>; MEMORY_SIZE],
    ports: HashMap<u8, Box<dyn IoPort>>,
}

impl IoMemory {
    pub fn new() -> Self {
        Self {
            port_map: [None; MEMORY_SIZE],
            ports: HashMap::new(),
        }
    }
    pub fn map_port(&mut self, port: Box<dyn IoPort>) -> Result<(), String> {
        let offset = port.get_ports_offset();
        match port.get_base_address() {
            Some(address) => {
                for i in offset {
                    self.port_map[(address + *i) as usize] = Some(address);
                }
                self.ports.insert(address, port);
                Ok(())
            },
            None => {
                Err("Base address is not defined".to_string())
            }
        }
    }
    /// Reads data from io address.
    ///
    /// Reads data from io address and returns data as u8. If the port is not present on that address
    /// it returns 0xff which simulates real behavior of reading from non existing periferals.
    pub fn read(&mut self, address: u8) -> u8 {
        if let Some(base_address) = self.port_map[address as usize]
            && let Some(port) = self.ports.get_mut(&base_address) {
                match port.read_from_address(address) {
                    Some(data) => {
                        return data;
                    },
                    None => {
                        return 0xff;
                    }
                };
            };
        0xff
    }
    /// Writes data to specific io address
    ///
    /// Writes data to specific io address. As in real system, it doesn't indicate success or failure
    /// and it is on the user to make sure that periferal exist on that address.
    pub fn write(&mut self, address: u8, data: u8) {
        if let Some(base_address) = self.port_map[address as usize]
            && let Some(port) = self.ports.get_mut(&base_address) {
                port.write_to_address(address, data);
            };
    }
    /// Removes ports mapped to base address
    pub fn remove(&mut self, base_address: u8) {
        // Remove base addresses from port map
        for i in 0 .. (self.port_map.len() - 1) { 
             if let Some(address) = self.port_map[i] 
                && address == base_address {
                    self.port_map[i] = None;
            }
           
        }
        // remove port from ports HashMap
        self.ports.remove(&base_address);

    }
    /// Gets io port info
    pub fn get_io_ports_info(&self) -> Vec<String> {
        let ports = &self.ports;
        let mut info: Vec<String> = Vec::new();
        for (k,v) in ports {
            info.push(v.get_io_port_info());            
        }
        return info;
    }
}
impl Default for IoMemory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct DummyIo {
    base_address: u8,
    ports_offset: [u8; 2],
    data: u8,
    control: u8
}
impl DummyIo {
    pub fn new() -> Self {
        DummyIo { 
            base_address: 0x40, 
            ports_offset: [0, 1],
            data: 0, 
            control: 0 }
    }
}
impl IoPort for DummyIo {
    fn read_from_address(&mut self, address: u8) -> Option<u8> {
        let offset_data = self.ports_offset[0];
        let offset_control = self.ports_offset[1];
        if address == self.base_address + offset_data {
            let data = self.data;
            println!("Read data from dummy io: address 0x{:02X}, data 0x{:02X}", address, data);
            return Some(data)
        }
        if address == self.base_address + offset_control {
            let data = self.control;
            println!("Read control from dummy io: address 0x{:02X}, data 0x{:02X}", address, data);
            return Some(data)
        }
        None
    }
    fn write_to_address(&mut self, address: u8, data: u8) {
        let offset_data = self.ports_offset[0];
        let offset_control = self.ports_offset[1];
        if address == self.base_address + offset_data {
            self.data = data;
            println!("Written data (0x{:02X}) to dummy io address 0x{:2X}", data, address);
        }
        if address == self.base_address + offset_control {
            self.control = data;
            println!("Written control (0x{:04X}) to dummy io address 0x{:2X}", data, address);
        }
    }
    fn get_ports_offset (& self) -> &[u8] {
        &self.ports_offset
    }
    fn get_base_address(& self) -> Option<u8> {
        Some(self.base_address)
    }
    fn get_io_port_info(&self) -> String {
        "Dumy device".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::io::memory::{DummyIo, IoMemory};

    #[test]
    /// Test of dummy interface
    fn test_dummy_io() {
        let address_data: u8 = 0x40;
        let address_control: u8 = 0x41;
        let data: u8 = 0x55;
        let control: u8 = 0xaa;
        let mut memory = IoMemory::new();
        let _ = memory.map_port(Box::new(DummyIo::new()));
        memory.write(address_data, data);
        memory.write(address_control, control);
        let mut result = memory.read(address_data);
        assert_eq!(data, result);
        result = memory.read(address_control);
        assert_eq!(control, result);
    }
    #[test]
    /// Test of nonexisting interface
    fn test_nonexisting_io() {
        let address_data: u8 = 0x40;
        let address_control: u8 = 0x41;
        let data: u8 = 0x55;
        let control: u8 = 0xaa;
        let mut memory = IoMemory::new();
        let _ = memory.map_port(Box::new(DummyIo::new()));
        memory.write(address_data, data);
        memory.write(address_control, control);
        let mut result = memory.read(address_data);
        assert_eq!(data, result);
        result = memory.read(address_control);
        assert_eq!(control, result);
    }
    #[test]
    fn test_io_memory() {
        let address_data: u8 = 0x40;
        let address_control: u8 = 0x41;
        let data: u8 = 0x55;
        let control: u8 = 0xaa;
        let mut memory = IoMemory::new();
        let _ = memory.map_port(Box::new(DummyIo::new())).unwrap();
        memory.write(address_data, data);
        memory.write(address_control, control);
        let mut result = memory.read(address_data);
        assert_eq!(data, result);
        result = memory.read(address_control);
        assert_eq!(control, result);
    }
}
