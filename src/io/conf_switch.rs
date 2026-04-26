use crate::io::IoPort;

pub struct ConfSwitch {
        name: Option<String>,
        value: u8,
        base_address: Option<u8>,
        memory_base_address: Option<u16>,
        port_offsets: [u8; 1],
}

impl ConfSwitch {
    pub fn new(value: u8) -> Self {
        Self {
            name: None,
            value,
            base_address: None,
            memory_base_address: None,
            port_offsets: [0],
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
}

impl IoPort for ConfSwitch {
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
        let data = format!("0x{:02X}", self.value);
        format!(
            "Configuration switch: base address[{base_address}], data[{data}], name[{}]", self.get_name())
    }
    fn read_from_address(&mut self, _address: u8) -> Option<u8> {
        Some(self.value)
    }
    fn read_from_mem_address(&mut self, _address: u16) -> Option<u8> {
        Some(self.value)        
    }
    fn write_to_address(&mut self, _memory: &mut [crate::memory::MemCell], _address: u8, _data: u8) -> Result<Option<crate::memory::dma::Dma>, super::ErrorIndicators> {
        Ok(None)
    }
    fn write_to_memory_address(&mut self, _memory: &mut [crate::memory::MemCell], _address: u16, _data: u8) -> Result<Option<crate::memory::dma::Dma>, super::ErrorIndicators> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
use crate::io::{conf_switch::ConfSwitch, memory::IoMemory};
use crate::memory::MemCell;
use crate::cpu::{CpuUi, i8080};
use crate::disassembler::i8080_opcode_consts::*;

    const BASE: u8 = 0xF0;
    const MEMORY_BASE: u16 = 0x1000;
    const CONF_DATA: u8 = 0x02;

    #[test]
    fn test_io_read() {
        let mut io_memory = IoMemory::new();
        let mut switch = Box::new(ConfSwitch::new(CONF_DATA));
        switch.set_base_address(BASE);
        let _ = io_memory.map_port(switch).unwrap();
        let cpu_memory = &mut [MemCell::Memory(0x00)]; // Empty memory as it is not used in this interface
        io_memory.write(cpu_memory, BASE, 0xff);
        let data = io_memory.read(BASE);
        assert_eq!(data, CONF_DATA);
    }
    #[test]
    fn test_memory_read() {
        let mut cpu = i8080::Cpu::new();
        let mut cpu_memory = cpu.get_memory();
        let mut switch = Box::new(ConfSwitch::new(CONF_DATA));
        switch.set_memory_base_address(MEMORY_BASE);
        let _ = cpu_memory.map_port(switch).unwrap();
        cpu_memory.write_byte(MEMORY_BASE, 0xff);
        let data = cpu_memory.read_byte(MEMORY_BASE);
        assert_eq!(data, CONF_DATA);
    }
    #[test]
    fn read_switch_io_mapped() {
        // Let's read from cpu
        let mut cpu = i8080::Cpu::new();
        let mut switch = Box::new(ConfSwitch::new(CONF_DATA));
        switch.set_base_address(BASE);
        let io_memory = cpu.get_io_memory().unwrap();
        let res = io_memory.map_port(switch);
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(false);
        let program_address = 0x100;
        // let read data 
        let program: &[u8] = &[
            MVI_A, 0xff,            
            OUT, BASE,
            IN, BASE,
            CPI, CONF_DATA,
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
        assert_eq!(cpu.a, CONF_DATA);
    }
    #[test]
    fn read_switch_mem_mapped() {
        // Let's read from cpu
        let mut cpu = i8080::Cpu::new();
        let mut switch = Box::new(ConfSwitch::new(CONF_DATA));
        switch.set_memory_base_address(MEMORY_BASE);
        let res = cpu.get_memory().map_port(switch);
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(false);
        let program_address = 0x100;
        // let read data 
        let program: &[u8] = &[
            MVI_A, 0xff,            
            STA, (MEMORY_BASE & 0xff) as u8, ((MEMORY_BASE & 0xff00) >> 8) as u8,
            LDA, (MEMORY_BASE & 0xff) as u8, ((MEMORY_BASE & 0xff00) >> 8) as u8,
            CPI, CONF_DATA,
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
        assert_eq!(cpu.a, CONF_DATA);
    }

}