pub mod memory;
pub mod serial;

pub trait IoPort {
    fn write_to_address(&mut self, address: u8, data: u8);
    fn read_from_address(&mut self, address: u8) -> Option<u8>;
    fn get_base_address(& self) -> Option<u8>;
    fn get_ports_offset (& self) -> &[u8];
    fn get_io_port_info(&self) -> String;
}
