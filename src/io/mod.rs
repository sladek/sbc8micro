pub mod i8251a;
pub mod memory;
pub mod serial;
pub mod sssd8fdc;
use crate::memory::{MemCell};

pub trait IoPort {
    fn write_to_address(&mut self, memory: &mut [MemCell], address: u8, data: u8);
    fn write_to_memory_address(&mut self, memory: &mut [MemCell], address: u16, data: u8);
    fn read_from_address(&mut self, address: u8) -> Option<u8>;
    fn read_from_mem_address(&mut self, address: u16) -> Option<u8>;
    fn get_base_address(&self) -> Option<u8>;
    fn get_memory_base_address(&self) -> Option<u16>;
    fn get_ports_offset(&self) -> &[u8];
    fn get_io_port_info(&self) -> String;
}
