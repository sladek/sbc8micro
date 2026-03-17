pub mod i8251a;
pub mod memory;
pub mod serial;
pub mod isbc202;
pub mod rdr_pun;
use crate::memory::{MemCell};
use crate::memory::dma::Dma;

pub trait IoPort {
    // memory: &mut [MemCell] is sometimes required if there is a need to access CPU memory via DMA 
    fn write_to_address(&mut self, memory: &mut [MemCell], address: u8, data: u8) -> Result<Option<Dma>, ErrorIndicators>;
    fn write_to_memory_address(&mut self, memory: &mut [MemCell], address: u16, data: u8) -> Result<Option<Dma>, ErrorIndicators>;
    fn read_from_address(&mut self, address: u8) -> Option<u8>;
    fn read_from_mem_address(&mut self, address: u16) -> Option<u8>;
    fn get_base_address(&self) -> Option<u8>;
    fn get_memory_base_address(&self) -> Option<u16>;
    fn get_ports_offset(&self) -> &[u8];
    fn get_io_port_info(&self) -> String;
}
#[derive(Debug, PartialEq, Clone)]
pub enum ErrorIndicators {
    NotReady = 0x80,                           // Disk is not ready
    WriteError = 0x40,                         // Error occured during writing to the disk
    WriteProtect = 0x20,                       // Disk is write protected
    OverUnderRun = 0x10, // Controler couldn't transfer the data before next request. Data is lost
    AddressError = 0x08, // Invalid sector or track is requested by CPU
    SeekError = 0x04,    // Head is not positioned over expected track
    CrcError = 0x02,     // CRCs calculated are not the same as specified in sector.
    DeletedRecord = 0x01, // Sector has deleted data address mark
    IdCrcError = 0x08 | 0x02, // AddressError | CrcError. Indicates that the CRC of ID field doesn't match
    NoAddressMark = 0x08 | 0x04 | 0x02, // AddressError | SeekError | CrcError. No address mark was encountered for a full revolution of the diskette.
    DataMarkError = 0x08 | 0x04 | 0x02 | 0x01, // // AddressError | SeekError | CrcError | Deleted Record. It indicates that the data field wos not preceded by eitherdata mark or a delete data mark.
}
