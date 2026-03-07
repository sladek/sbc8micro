use crate::disk::sssd8fd::{DataDeletedData, ErrorIndicators, Floppy, Sector};
use crate::memory::{self, MemCell};
use crate::io::IoPort;
use crate::disk::sssd8fd::Result;
use crate::memory::dma::DmaRequest;
use memory::dma::Dma;
use std::cell::RefCell;
use std::rc::Rc;

const NUMBER_OF_DISKS: usize = 4;
const BASE: u8 = 0x78;
const MEMORY_BASE: u16 = 0x1000;

enum Opcode {
    NoOperation = 0b000,
    Seek = 0b001,
    FormatTrack = 0b010,
    Recalibrate = 0b011,
    ReadData = 0b100,
    VerifyCrc = 0b101,
    WriteData = 0b110,
    WriteDeletedData = 0b111,
}

enum UnitRedy {
    Unit0 = 0b0100_0000,
    Unit1 = 0b1000_0000,
    Unit2 = 0b0001_0000,
    Unit3 = 0b0010_0000,
}

struct DisketteInstruction {
    value: u8, // binary value of Diskette Instruction
    unit_select: u8, // bits 4,5. 0b00 - drive 0, 0b11 - drive 1 NOTE: only 2 floppy disks are supported by this controler (SBC 201)
    data_word_length: u8, // 0 - if used in 8 - bit systems, 1 - if used in 16 bit systems
    opcode: u8, // 0b000 - no operation, 0b001 - seek, 0b010 - format trackk, 0b011 - recalibrate, 0b100 - read data, 0b101 - Verify CRC, 0b110 - write data, 0b111 - write 'deleted' data
}

impl DisketteInstruction {
    pub fn new(value: u8) -> Self {
        Self {
            value,
            unit_select: (value & 0b11_0000) >> 4,
            data_word_length: (value & 0b1000) >> 3,
            opcode: value & 0b0111,
        }
    } 
    pub fn get_value(&self) -> u8 {
        self.value
    }
    pub fn get_opcode(&self) -> u8 {
        self.opcode
    }
    pub fn get_unit_select(&self) -> u8 {
        self.unit_select
    }
}

struct ChannelWord {
    value: u8, // binary content of Channel Word
// lock_override is not implemented in 8 bit CP/M
    lock_override: bool, // bit (7).  1 - "wait" bit is not set, this prevents IOPB being overwritten by the controller
    random_format_sequence: bool, // bit (6). 0 - sector addresses are assigned in sequential order, 1 - sectr addresses are assigned randomly based on pattern listed in 52 byte memory buffer.
    interrupt_control: u8, // bits (4 and 5). Enable or disable Diskette Channel interrupts 
    data_word_length: bool,       // bit (3). 0 - for 8 bit systems, 1 - for 16 bit systems
// Following field are not implemented in 8 bit CP/M
    successor_bit: bool, // bit (2). will be reset (logical 0) if the current IOPB is the last (or only) IOPB to be executed.
    branch_on_wait: bool, // bit (1). It is interconnected with wait bit. check documentation (9800349B.pdf) for more details
    wait: bool, // bit (0). 
}
impl ChannelWord {
    pub fn new(value: u8) -> Self {
        Self {
            value,
            lock_override: value & 0b1000_0000 != 0,
            random_format_sequence: value & 0b0100_0000 != 0,
            interrupt_control: (value & 0b0011_0000) >> 4,
            data_word_length: value & 0b0000_1000 != 0,
            successor_bit: value & 0b0000_0100 != 0, // Not implemented in 8 bit CP/M
            branch_on_wait: value & 0b0000_0010 != 0, // Not implemented in 8 bit CP/M
            wait: value & 0b0000_0001 != 0, // Not implemented in 8 bit CP/M
        }
    }
    pub fn get_value(&self) -> u8 {
        self.value
    }
}

/// Drives status
enum Dstat {
    Drive0Ready = 0b0000_0001, // Drive 0 ready
    Drive1Ready = 0b0000_0010, // Drive 1 ready
    InterruptPending = 0b0000_0100, // Interrupt flip-flop status
    ControllerPresent = 0b0000_1000, // Controller presence indicator
    _DoubleDensityPresent = 0b0001_0000, // Double density controller present
    Drive2Ready = 0b0010_0000, // Drive 2 ready
    Drive3Ready = 0b0100_0000, // Drive 3 ready
}
pub struct Isbc201 {
    dstat: u8,
    dstat_address: u8,
    dstat_memory_address: u16,
    ilow_address: u8,
    ilow_memory_address: u16,
    ihigh_address: u8,
    ihigh_memory_address: u16,
    rtype: u8,
    rtype_address: u8,
    rtype_memory_address: u16,
    rbyte_00: u8,  // Returned if rtype = 0b0000_0000
    rbyte_10: u8,  // Returned if rtype = 0b0000_0010
    rbyte_address: u8,
    rbyte_memory_address: u16,
    reset_address: u8,
    reset_memory_address: u16,
    iopb_address: u16,
    base_address: Option<u8>,
    memory_base_address: Option<u16>,
    port_offsets: [u8; 5], // base_address + (0 = dstat, 1 = ilow/rtype, 2 = ihigh, 3 = rbyte, 7 = reset)
    name: Option<String>,
    floppies: [Option<Floppy>; NUMBER_OF_DISKS],
    active_floppy: u8,
    cpu_memory: Option<Rc<RefCell<memory::Memory>>>,
}

impl Default for Isbc201 {
    fn default() -> Self {
        Isbc201 {
            dstat: 0,
            dstat_address: BASE,
            dstat_memory_address: MEMORY_BASE,
            ilow_address: BASE + 1,
            ilow_memory_address: MEMORY_BASE + 1,
            ihigh_address: BASE + 2,
            ihigh_memory_address: MEMORY_BASE + 2,
            rtype: 0,
            rtype_address: BASE + 1,
            rtype_memory_address: MEMORY_BASE + 1,
            rbyte_00: 0,
            rbyte_10: 0,
            rbyte_address: BASE + 3,
            rbyte_memory_address: MEMORY_BASE + 3,
            reset_address: BASE + 7,
            reset_memory_address: MEMORY_BASE + 7,
            iopb_address: 0u16,
            base_address: None,
            memory_base_address: None,
            port_offsets: [0, 1, 2, 3, 7], // 0 - dstat, 1 - ilow/rtype, 2 - ihigh, 3 - rbyte, 7 - reset
            name: Some("iSBC-201".to_string()),
            floppies: [const { None }; NUMBER_OF_DISKS],
            active_floppy: 0,
            cpu_memory: None,
        }
    }
}

impl Isbc201 {
    pub fn new(cpu_memory: Rc<RefCell<memory::Memory>>) -> Self {
        Isbc201 {
            dstat: Dstat::ControllerPresent as u8,
            cpu_memory: Some(cpu_memory),
            ..Default::default()
        }
    }
    /// Sets base address of fdc (sbc201).
    ///
    /// Base address is data port for sbc201
    /// Base address + 1 is address of controll and status port of sbc201
    pub fn set_base_address(&mut self, address: u8) {
        self.memory_base_address = None;
        self.base_address = Some(address);
        self.dstat_address = address;
        self.ilow_address = address.wrapping_add(1);
        self.ihigh_address = address.wrapping_add(2);
        self.rtype_address = address.wrapping_add(1);
        self.rbyte_address = address.wrapping_add(3);
        self.reset_address = address.wrapping_add(7);
    }
    /// Sets base address of fdc (sbc201) mapped to memory (16 bits).
    ///
    /// Base address is data port for sbc201
    /// Base address + 1 is address of controll and status port of sbc201    
    pub fn set_memory_base_address(&mut self, address: u16) {
        self.base_address = None;
        self.memory_base_address = Some(address);
        self.dstat_memory_address = address;
        self.ilow_memory_address = address.wrapping_add(1);
        self.ihigh_memory_address = address.wrapping_add(2);
        self.rtype_memory_address = address.wrapping_add(1);
        self.rbyte_memory_address = address.wrapping_add(3);
        self.reset_memory_address = address.wrapping_add(7);
    }
    /// Gets base address of fdc
    pub fn get_base_address(&self) -> Option<u8> {
        self.base_address
    }
    /// Gets base address of fdc mapped to memory (16 bits)
    pub fn get_memory_base_address(&self) -> Option<u16> {
        self.memory_base_address
    }
    /// Get name of the floppy disk controler.
    ///
    /// Gets name of the floppy disk controler
    pub fn get_name(&self) -> String {
        if let Some(name) = self.name.clone() {
            return name;
        }
        "Undefined".to_string()
    }
    /// Get I/O parameter block from memory
    ///
    /// Reads I/O parameter block from CPU's memory and stores it in Iopb fields
    fn get_iopb(&mut self, memory: &mut [MemCell]) -> Iopb {
        let address = self.iopb_address as usize;
        Iopb {
            channel_word:  Self::get_memory_cell_data(memory[address]),
            diskette_instruction: Self::get_memory_cell_data(memory[address + 1]),
            number_of_records: Self::get_memory_cell_data(memory[address + 2]),
            track_address: Self::get_memory_cell_data(memory[address + 3]),
            sector_address: Self::get_memory_cell_data(memory[address + 4]),
            buffer_address_low: Self::get_memory_cell_data(memory[address + 5]),
            buffer_address_high: Self::get_memory_cell_data(memory[address + 6]),
            ..Default::default()
        }
    }
    /// Get interrupt pending flag
    /// 
    /// Gets a status of interrupt pending flag
    fn is_interrupt_pending(&self) -> bool {
        if self.dstat & Dstat::InterruptPending as u8 != 0 {
            return true;
        }
        false
    }
    /// Reset the controller
    ///
    /// Sets the controller to its initial state
    fn reset(&mut self) {
        self.active_floppy = 0;
        self.dstat = Dstat::ControllerPresent as u8;
    }
    /// Assign floppy disk
    /// 
    /// Assignes floppy disk to bay 1 or bay 2
    pub fn set_floppy(&mut self, floppy: Floppy, number: u8) -> Result<()> {
        if number <= NUMBER_OF_DISKS as u8 {
            let file_name = floppy.get_name();
            for name in self.floppies.iter_mut() {
                if name.is_none() {
                    break
                }
                let floppy_name = name.as_mut().unwrap().get_name();
                if floppy_name == file_name {
                    return Err(ErrorIndicators::AddressError)
                }
            }
            self.floppies[number as usize] = Some(floppy);
            self.rtype = 0b000_0010;
            match number {
                0 => {
                    self.dstat |= Dstat::Drive0Ready as u8;
                    self.rbyte_10 |= UnitRedy::Unit0 as u8; 
                },
                1 => {
                    self.dstat |= Dstat::Drive1Ready as u8;
                    self.rbyte_10 |= UnitRedy::Unit1 as u8; 
                },
                2 => {
                    self.dstat |= Dstat::Drive2Ready as u8;
                    self.rbyte_10 |= UnitRedy::Unit2 as u8 
                },
                3 => {
                    self.dstat |= Dstat::Drive3Ready as u8;
                    self.rbyte_10 |= UnitRedy::Unit3 as u8 
                },
                _ => {}
            }
        }
        Ok(())
    }
    /// Remove floppy drive
    /// 
    /// Removes floppy drive[number] from the controller
    pub fn remove_floppy(&mut self, number: u8) {
            self.rtype = 0b000_0010;
            match number {
                0 => {
                    self.floppies[0] = None;
                    self.dstat &= !(Dstat::Drive0Ready as u8);
                    self.rbyte_10 &= !(UnitRedy::Unit0 as u8); 
                },
                1 => {
                    self.floppies[1] = None;
                    self.dstat &= !(Dstat::Drive1Ready as u8);
                    self.rbyte_10 &= !(UnitRedy::Unit1 as u8); 
                },
                2 => {
                    self.floppies[2] = None;
                    self.dstat &= !(Dstat::Drive2Ready as u8);
                    self.rbyte_10 &= !(UnitRedy::Unit2 as u8); 
                },
                3 => {
                    self.floppies[3] = None;
                    self.dstat &= !(Dstat::Drive3Ready as u8);
                    self.rbyte_10 &= !(UnitRedy::Unit3 as u8); 
                },
                _ => {}
            }
    }
    /// Set active floppy
    /// 
    /// Sets active floppy drive 0 or 1
    pub fn set_active_floppy(&mut self, number: u8) {
        self.active_floppy = number;
    }
    /// Set interrupt
    /// 
    /// Sets pending interrupt flag
    fn set_interrupt_pending(&mut self) {
        self.dstat |= Dstat::InterruptPending as u8;
    }
    /// Reset interrupt
    /// 
    /// Resets pending interrupt flag
    fn reset_interrupt_pending(&mut self) {
        self.dstat &= !(Dstat::InterruptPending as u8);
    }
    /// Process I/O parameter block
    ///
    /// Processes I/O parameter block. Reference to mutable CPU memory is also a parameter as it is needed for DMA access from fdc.
    /// It cannot be simply borrowed muttably as it is already borrowed muttably during processing of the instruction itself so we rather pass the mutable reference 
    /// to memory array to make it available if needed for example to read IOPB or write sector data back to memory.
    fn process_iopb(&mut self, iopb: &Iopb) -> Result<Option<Dma>> {
        let _channel_word = iopb.channel_word;
        let discette_instruction = DisketteInstruction::new(iopb.diskette_instruction);
        let opcode = discette_instruction.get_opcode();
        let unit = discette_instruction.get_unit_select();
        self.reset_interrupt_pending();
        match opcode {
            opcode if opcode == Opcode::NoOperation as u8 => {
                return Ok(None);
            }
            opcode if opcode == Opcode::Seek as u8 => {
                let track = iopb.track_address;
                match &mut self.floppies[unit as usize] {
                    Some(floppy) => {
                        floppy.seek(track)?;
                    }
                    None => {
                        return Err(ErrorIndicators::SeekError)
                    }
                }
//                self.set_interrupt_pending();
                return Ok(None);
            }
            opcode if opcode == Opcode::FormatTrack as u8 => {
                let track = iopb.track_address;
                match &mut self.floppies[unit as usize] {
                    Some(floppy) => {
                        floppy.format_track(track)?;
                    }
                    None => {
                        return Err(ErrorIndicators::SeekError)
                    }
                }
//                self.set_interrupt_pending();
                return Ok(None);
            }
            opcode if opcode == Opcode::Recalibrate as u8 => {
                match &mut self.floppies[unit as usize] {
                    Some(floppy) => {
                        floppy.seek(0)?;
                    }
                    None => {
                        return Err(ErrorIndicators::SeekError)
                    }
                }
//                self.set_interrupt_pending();
                return Ok(None);
            }
            opcode if opcode == Opcode::ReadData as u8 => {
                let data = self.read_data(iopb)?;
                let address: u16 = (iopb.buffer_address_high as u16) << 8 | iopb.buffer_address_low as u16;
                let dma = DmaRequest::new(address, data);
//                self.set_interrupt_pending();
                return Ok(Some(Dma::new(dma)));
            }
            opcode if opcode == Opcode::VerifyCrc as u8 => {
                let _data = self.read_data(iopb)?; // Just read data. But dont transfer ot to CPU's memory.
            }
            opcode if opcode == Opcode::WriteData as u8 => {
                self.write_data(iopb, DataDeletedData::Data)?; // Write data based on iopb.
            }
            opcode if opcode == Opcode::WriteDeletedData as u8 => {
                self.write_data(iopb, DataDeletedData::DeletedData)?; // Write deleted data based on iopb.
            }
            _ => {
                // Do nothing, for now.
            }
        }
//        self.set_interrupt_pending();
        Ok(None)
    }
    /// Read data from floppy disk
    /// 
    /// Reads data from floppy disk based on IOPB.
    fn read_data(&mut self, iopb: &Iopb) -> Result<Vec<u8>> {
        let discette_instruction = DisketteInstruction::new(iopb.diskette_instruction);
        let unit = discette_instruction.get_unit_select();
        match &self.floppies[unit as usize] {
            Some(floppy) => {
                let mut result_data: Vec<u8> = Vec::new();
                let mut sectors_to_read = iopb.number_of_records;
                let mut sector_num = iopb.sector_address;
                while sectors_to_read != 0 {
                    let sector = floppy.read_sector(iopb.track_address, sector_num)?;
                    let data = sector.get_data();
                    result_data.append(&mut data.to_vec());
                    sector_num += 1;
                    sectors_to_read -= 1;
                };
                Ok(result_data)
            }
            None => {
                Err(ErrorIndicators::NotReady)
            }
        }
    }
    /// Write data to floppy disk
    /// 
    /// Write data to floppy disk based on IOPB.
    fn write_data(&mut self, iopb: &Iopb, data_deleted_data: DataDeletedData) -> Result<()> {
        let discette_instruction = DisketteInstruction::new(iopb.diskette_instruction);
        let unit = discette_instruction.get_unit_select();
        match &mut self.floppies[unit as usize] {
            Some(floppy) => {
                let mut sectors_to_read = iopb.number_of_records;
                let mut sector_num = iopb.sector_address;
                let mut sector_data: [u8; 128] = [0; 128];
                let mut buffer_address = iopb.get_buffer_address();
                match &self.cpu_memory {
                    Some(memory) => {
                        let memory = memory.borrow_mut().get_data();
                        while sectors_to_read != 0 {
                            // Get sector data from memory
                            let mut i: u8 = 0;
                            while i != 128 {
                                sector_data[i as usize] = Self::get_memory_cell_data(memory[buffer_address as usize]);
                                buffer_address = buffer_address.wrapping_add(1);
                                i += 1;                                
                            }
                            let mut sector = Sector::new(iopb.track_address, sector_num, &sector_data);
                            sector.set_data_deleted_data(data_deleted_data.clone());
                            floppy.seek_write_sector(sector)?;
                            sector_num += 1;
                            sectors_to_read -= 1;
                        };                        
                    },
                    None => {
                        return Err(ErrorIndicators::AddressError)
                    }
                }
                Ok(())
            }
            None => {
                Err(ErrorIndicators::NotReady)
            }
        }
    } 
    /// Read result type
    /// 
    /// Reads result type and clears interrupt pending flag.
    fn read_result_type(&mut self) -> u8 {
        self.reset_interrupt_pending();
        self.rtype
    }
    
    fn get_memory_cell_data(cell: MemCell) -> u8 {
        match cell {
            MemCell::Memory(data) => {
                data
            },
            _ => {
                0xff
            }
        }
    }
}

impl IoPort for Isbc201 {
    fn get_base_address(&self) -> Option<u8> {
        self.base_address
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
        let name = self.get_name();
        let floppies = &self.floppies;
        let mut char_idx = 0;
        let floppy_char = 'A';
        let mut floppy_drives = String::new();
        for floppy in floppies {
            match floppy {
                Some(floppy) => {
                    let name = floppy.get_name();
                    let is_ro = floppy.is_read_only();
                    let floppy_char = char::from_u32(floppy_char as u32 + char_idx).unwrap();
                    let mut assigned_drive = format!(" {floppy_char}:{name}");
                    char_idx += 1;
                    if is_ro {
                        assigned_drive.push_str("[RO]");
                    }
                    floppy_drives.push_str(&assigned_drive);
                }
                None => {
                    break
                }
            }
        }
        format!(
            "Floppy disk controller (single sided single density 8 inch): base address[{base_address}], name[{name}], floppy drives:{floppy_drives}",
        )
    }
    fn get_memory_base_address(&self) -> Option<u16> {
        self.memory_base_address
    }
    fn get_ports_offset(&self) -> &[u8] {
        &self.port_offsets
    }
    fn read_from_address(&mut self, address: u8) -> Option<u8> {
        if address == self.dstat_address {
            return Some(self.dstat);
        }
        if address == self.rtype_address {
            return Some(self.read_result_type());
        }
        if address == self.rbyte_address {
            if self.rtype & 0x03 == 0 {
                return Some(self.rbyte_00);
            }
            if self.rtype & 0x03 == 0b0000_0010{
                return Some(self.rbyte_10);
            }
        }
        None
    }
    fn read_from_mem_address(&mut self, address: u16) -> Option<u8> {
        if address == self.dstat_memory_address {
            return Some(self.dstat);
        }
        if address == self.rtype_memory_address {
            return Some(self.read_result_type());
        }
        if address == self.rbyte_memory_address {
            if self.rtype & 0x03 == 0 {
                return Some(self.rbyte_00);
            }
            if self.rtype & 0x03 == 0b0000_0010{
                return Some(self.rbyte_10);
            }
        }
        None
    }
    fn write_to_address(&mut self, memory: &mut [MemCell], address: u8, data: u8) -> std::result::Result<Option<Dma>, ErrorIndicators>{
        if address == self.ilow_address {
            // Write mamory address lower
            self.iopb_address = data as u16;
            return Ok(None);
        }
        if address == self.ihigh_address {
            // Write memory address upper and start disk operation
            self.iopb_address |= (data as u16) << 8;
            let iopb = self.get_iopb(memory);
            match self.process_iopb(&iopb) {
                Ok(dma) => {
                    self.rtype = 0b0000_0000;
                    self.rbyte_00 = 0b0000_0000;
                    self.set_interrupt_pending();
                    return Ok(dma)
                }
                Err(err) => {
                    self.rtype = 0b0000_0000;
                    self.rbyte_00 = err.clone() as u8;
                    self.set_interrupt_pending();
                    return Err(err)
                }
            }
        }
        if address == self.reset_address {
            // Reset
            self.reset_interrupt_pending();
            self.reset();
        }
        Ok(None)
    }
    fn write_to_memory_address(&mut self, memory: &mut [MemCell], address: u16, data: u8) -> std::result::Result<Option<Dma>, ErrorIndicators> {
        if address == self.ilow_memory_address {
            // Write memory address lower
            self.iopb_address = data as u16;
            return Ok(None);
        }
        if address == self.ihigh_memory_address {
            // Write memory address upper and start disk operation
            self.iopb_address |= (data as u16) << 8;

            let iopb = self.get_iopb(memory);
            match self.process_iopb(&iopb) {
                Ok(dma) => {
                    self.rtype = 0b0000_0000;
                    self.rbyte_00 = 0b0000_0000;
                    self.set_interrupt_pending();
                    return Ok(dma)
                }
                Err(err) => {
                    self.rtype = 0b0000_0010;
                    self.rbyte_10 = err.clone() as u8;
                    self.set_interrupt_pending();
                    return Err(err)
                }
            }
        }
        if address == self.reset_memory_address {
            // Reset
            self.reset_interrupt_pending();
            self.reset();
        }
        Ok(None)
    }
}
#[derive(Default, Debug)]
struct Iopb {
    channel_word: u8,
    diskette_instruction: u8,
    number_of_records: u8,
    track_address: u8,
    sector_address: u8,
    buffer_address_low: u8,
    buffer_address_high: u8,
    _block_number: u8,
    _next_iopb_address_lower: u8,
    _next_iopb_address_upper: u8,
}
impl Iopb {
    pub fn get_buffer_address(&self) -> u16 {
        ((self.buffer_address_high as u16) << 8) + self.buffer_address_low as u16
    }
}
#[cfg(test)]
mod tests {
    use crate::cpu::{CpuUi, mos6502};
    use crate::cpu::i8080::{self};
    use crate::disassembler::i8080_opcode_consts::*;
    use crate::disassembler::mos6502_opcode_consts::{BRK, LDA_ABS, LDA_IMM, STA_ABS};
    use crate::disk::sssd8fd::{DataDeletedData, Floppy};
    use crate::disk::sssd8fd::Sector;
    use crate::io::memory::IoMemory;
    use crate::io::{IoPort, isbc201::Isbc201};
    use std::fs;
    use std::rc::Rc;

    fn init_disk(name: &str) {
        _ = fs::remove_file(name);
        // Let's create new floppy image and format
        _ = Floppy::new(name, false).unwrap().format();
    }    
    fn remove_disk(name: &str) {
        _ = fs::remove_file(name);
    }
    #[test]
    fn test_io_mapped() {
        let mut cpu = i8080::Cpu::new();
        let memory = Rc::clone(&cpu.memory);
        let _ = memory.borrow_mut().write_byte(0x55aa, 0x55);
        let mut fdc = Isbc201::new(cpu.get_memory_ref()); // Base address 0x78
        fdc.set_base_address(0x78);
        println!("Name: {:?}", fdc.get_name());
        println!("{}", fdc.get_io_port_info());
        let mut io_memory = IoMemory::new();
        let res = io_memory.map_port(Box::new(fdc));
        assert_eq!(res, Ok(()));
    }
    #[test]
    fn test_memory_mapped() {
        let mut cpu = i8080::Cpu::new();
        let memory = Rc::clone(&cpu.memory);
        let _ = memory.borrow_mut().write_byte(0x55aa, 0x55);
        let mut fdc = Isbc201::new(cpu.get_memory_ref()); // Base address 0x78
        fdc.set_memory_base_address(0x1000);
        println!("Name: {:?}", fdc.get_name());
        println!("{}", fdc.get_io_port_info());
        let res = memory.borrow_mut().map_port(Box::new(fdc));
        assert_eq!(res, Ok(()));
    }
    #[test]
    fn test_iopb_read_sector() {
        let file_name = "iopb_test_read_sector.dsk";
        init_disk(file_name);
        // let's use that freshly created disk image and write some data
        let mut floppy = Floppy::new(file_name, false).unwrap();
        let mut data = [0; 128];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        // Let's write one sector
        let sector = Sector::new(0, 1, &data);
        _ = floppy.seek_write_sector(sector);
        let mut cpu = i8080::Cpu::new();
        let iopb_address = 0x2000;
        let program_address = 0x1000;
        let ilow = 0x79u8;
        let ihigh = 0x7au8;
        let mut fdc = Box::new(Isbc201::new(cpu.get_memory_ref())); // Base address 0x78
        // Let's assign the floppy as floppy[0] to the controller
        let _ = fdc.set_floppy(floppy, 0);
        fdc.set_base_address(0x78);
        let io_memory = cpu.get_io_memory().unwrap();
        let res = io_memory.map_port(fdc);
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(false);

        let iopb: &[u8] = &[
            // Iopb starts at 0x2000
            0x80, // Channel word 
            0b0000_0100, // Diskette operation (read data)
            0x01, // Number of records  
            0x00, // Track address
            0x01, // Sector address 
            0x00, // buffer address Lower. 0x3000 buffer address for read data from fdc
            0x30, // buffer address Upper.
        ];
        let program: &[u8] = &[
            MVI_A, (iopb_address & 0x0ff) as u8,
            OUT, ilow,
            MVI_A, (iopb_address >> 8) as u8,
            OUT, ihigh,
            IN, 0x78, // Read dstat
            HLT,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.get_memory().load_data(iopb, iopb_address);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            cpu.one_step();
            if opcode == HLT {
                break;
            }
        }
        // Lets verify some data read from floppy to memory
        assert_eq!(cpu.get_memory().read_byte(0x3000), 0x00);
        assert_eq!(cpu.get_memory().read_byte(0x3010), 0x10);
        assert_eq!(cpu.get_memory().read_byte(0x3020), 0x20);
        assert_eq!(cpu.get_memory().read_byte(0x3030), 0x30);
        assert_eq!(cpu.get_memory().read_byte(0x3040), 0x40);
        assert_eq!(cpu.get_memory().read_byte(0x3050), 0x50);
        assert_eq!(cpu.get_memory().read_byte(0x3060), 0x60);
        assert_eq!(cpu.get_memory().read_byte(0x3070), 0x70);
        remove_disk(file_name);
        let acc= cpu.a;
        assert_eq!(0x0d, acc);
    }
    #[test]
    fn test_iopb_read_multiple_sectors() {
        let file_name = "iopb_test_read_multisector.dsk";
        init_disk(file_name);
        // let's use that freshly created disk image and write some data
        let mut floppy = Floppy::new(file_name, false).unwrap();
        let mut data = [0; 128];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        // Let's write two sector
        let sector = Sector::new(0, 1, &data);
        _ = floppy.seek_write_sector(sector);
        let sector = Sector::new(0, 2, &data);
        _ = floppy.seek_write_sector(sector);
        let mut cpu = i8080::Cpu::new();
        let iopb_address = 0x2000;
        let program_address = 0x1000;
        let ilow = 0x79u8;
        let ihigh = 0x7au8;
        let mut fdc = Box::new(Isbc201::new(cpu.get_memory_ref())); // Base address 0x78
        // Let's assign the floppy as floppy[0] to the controller
        let _ = fdc.set_floppy(floppy, 0);
        fdc.set_base_address(0x78);
        let io_memory = cpu.get_io_memory().unwrap();
        let res = io_memory.map_port(fdc);
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(false);

        let iopb: &[u8] = &[
            // Iopb starts at 0x2000
            0x80, // Channel word 
            0b0000_0100, // Diskette operation (read data)
            0x02, // Number of records. Read 2 sectors
            0x00, // Track address
            0x01, // Sector address 
            0x00, // buffer address Lower. 0x3000 buffer address for read data from fdc
            0x30, // buffer address Upper.
        ];
        let program: &[u8] = &[
            MVI_A, (iopb_address & 0x0ff) as u8,
            OUT, ilow,
            MVI_A, (iopb_address >> 8) as u8,
            OUT, ihigh,
            IN, 0x78, // Read dstat
            HLT,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.get_memory().load_data(iopb, iopb_address);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            cpu.one_step();
            if opcode == HLT {
                break;
            }
        }
        let dump = cpu.get_memory().hex_dump(0x3000, 0x30ff);
        for line in dump {
            println!("{line}");
        }
        // Lets verify some data read from floppy to memory
        assert_eq!(cpu.get_memory().read_byte(0x3000), 0x00);
        assert_eq!(cpu.get_memory().read_byte(0x3010), 0x10);
        assert_eq!(cpu.get_memory().read_byte(0x3020), 0x20);
        assert_eq!(cpu.get_memory().read_byte(0x3030), 0x30);
        assert_eq!(cpu.get_memory().read_byte(0x3040), 0x40);
        assert_eq!(cpu.get_memory().read_byte(0x3050), 0x50);
        assert_eq!(cpu.get_memory().read_byte(0x3060), 0x60);
        assert_eq!(cpu.get_memory().read_byte(0x3070), 0x70);
        assert_eq!(cpu.get_memory().read_byte(0x3080), 0x00);
        assert_eq!(cpu.get_memory().read_byte(0x3090), 0x10);
        assert_eq!(cpu.get_memory().read_byte(0x30a0), 0x20);
        assert_eq!(cpu.get_memory().read_byte(0x30b0), 0x30);
        assert_eq!(cpu.get_memory().read_byte(0x30c0), 0x40);
        assert_eq!(cpu.get_memory().read_byte(0x30d0), 0x50);
        assert_eq!(cpu.get_memory().read_byte(0x30e0), 0x60);
        assert_eq!(cpu.get_memory().read_byte(0x30f0), 0x70);
        remove_disk(file_name);
        let acc= cpu.a;
        assert_eq!(0x0d, acc);
    }
    #[test]
    // Reads 2 sectors where last sector is out of sector range. The last requested sector is 27 which is out of range.
    // Number of sectors for that floppy disk is 26
    fn test_iopb_read_multiple_sectors_error() {
        let file_name = "iopb_test_read_multisector_error.dsk";
        init_disk(file_name);
        let sector_num = 25;
        // let's use that freshly created disk image and write some data
        let mut floppy = Floppy::new(file_name, false).unwrap();
        let mut data = [0; 128];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        // Let's write two sector
        let sector = Sector::new(0, sector_num, &data);
        let res1 = floppy.seek_write_sector(sector);
        println!("{:?}", res1);
        let sector = Sector::new(0, sector_num + 1, &data);
        let res2 = floppy.seek_write_sector(sector);
        println!("{:?}", res2);
        let mut cpu = i8080::Cpu::new();
        let iopb_address = 0x2000;
        let program_address = 0x1000;
        let ilow = 0x79u8;
        let ihigh = 0x7au8;
        let mut fdc = Box::new(Isbc201::new(cpu.get_memory_ref())); // Base address 0x78
        // Let's assign the floppy as floppy[0] to the controller
        let _ = fdc.set_floppy(floppy, 0);
        fdc.set_base_address(0x78);
        let io_memory = cpu.get_io_memory().unwrap();
        let res = io_memory.map_port(fdc);
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(false);

        let iopb: &[u8] = &[
            // Iopb starts at 0x2000
            0x80, // Channel word 
            0b0000_0100, // Diskette operation (read data)
            0x02, // Number of records. Read 2 sectors
            0x00, // Track address
            26, // Sector address 
            0x00, // buffer address Lower. 0x3000 buffer address for read data from fdc
            0x30, // buffer address Upper.
        ];
        let program: &[u8] = &[
            MVI_A, (iopb_address & 0x0ff) as u8,
            OUT, ilow,
            MVI_A, (iopb_address >> 8) as u8,
            OUT, ihigh,
            IN, 0x78, // Read dstat
            MOV_C_A,
            IN, 0x79, // Read rtype
            MOV_B_A,
            IN, 0x7B, // Read rbyte
            HLT,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.get_memory().load_data(iopb, iopb_address);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            cpu.one_step();
            if opcode == HLT {
                break;
            }
        }
        let regc = cpu.c;
        assert_eq!(0x0d, regc); // assert dstat
        let regb = cpu.b;
        assert_eq!(0x00, regb); // assert rtype
        let acc= cpu.a;
        assert_eq!(0x04, acc);  // assert rbyte
        remove_disk(file_name);
    }
    #[test]
    fn test_iopb_verify_crc() {
        let file_name = "iopb_test_verify.dsk";
        init_disk(file_name);
        // let's use that freshly created disk image and write some data
        let mut floppy = Floppy::new(file_name, false).unwrap();
        let mut data = [0; 128];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        // Let's write one sector
        let sector = Sector::new(0, 1, &data);
        _ = floppy.seek_write_sector(sector);
        let mut cpu = i8080::Cpu::new();
        let iopb_address = 0x2000;
        let program_address = 0x1000;
        let ilow = 0x79u8;
        let ihigh = 0x7au8;
        let mut fdc = Box::new(Isbc201::new(cpu.get_memory_ref())); // Base address 0x78
        // Let's assign the floppy as floppy[0] to the controller
        let _ = fdc.set_floppy(floppy, 0);
        fdc.set_base_address(0x78);
        let io_memory = cpu.get_io_memory().unwrap();
        let res = io_memory.map_port(fdc);
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(false);

        let iopb: &[u8] = &[
            // Iopb starts at 0x2000
            0x80, // Channel word 
            0b0000_0101, // Diskette operation (verify crc)
            0x01, // Number of records  
            0x00, // Track address
            0x01, // Sector address 
            0x00, // buffer address Lower. 0x4000 buffer address for read dtata from fdc
            0x30, // buffer address Upper.
        ];
        let program: &[u8] = &[
            MVI_A, (iopb_address & 0x0ff) as u8,
            OUT, ilow,
            MVI_A, (iopb_address >> 8) as u8,
            OUT, ihigh,
            IN, 0x78, // Read dstat
            HLT,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.get_memory().load_data(iopb, iopb_address);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            cpu.one_step();
            if opcode == HLT {
                break;
            }
        }
        remove_disk(file_name);
        let acc= cpu.a;
        assert_eq!(0x0d, acc);
    }
    #[test]
    fn test_iopb_seek() {
        let file_name = "iopb_test_seek.dsk";
        init_disk(file_name);
        // let's use that freshly created disk image and write some data
        let mut floppy = Floppy::new(file_name, false).unwrap();
        let mut data = [0; 128];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        // Let's write one sector
        let sector = Sector::new(0, 1, &data);
        _ = floppy.seek_write_sector(sector);
        let mut cpu = i8080::Cpu::new();
        let iopb_address = 0x2000;
        let program_address = 0x1000;
        let ilow = 0x79u8;
        let ihigh = 0x7au8;
        let mut fdc = Box::new(Isbc201::new(cpu.get_memory_ref())); // Base address 0x78
        // Let's assign the floppy as floppy[0] to the controller
        let _ = fdc.set_floppy(floppy, 0);
        fdc.set_base_address(0x78);
        let io_memory = cpu.get_io_memory().unwrap();
        let res = io_memory.map_port(fdc);
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(false); // Set to true if you want to see ASM cod which is executed

        let iopb: &[u8] = &[
            // Iopb starts at 0x2000
            0x80, // Channel word 
            0b0000_0001, // Diskette operation (seek)
            0x01, // Number of records  
            0x00, // Track address
            0x01, // Sector address 
            0x00, // buffer address Lower. 0x4000 buffer address for read dtata from fdc
            0x30, // buffer address Upper.
        ];
        let program: &[u8] = &[
            MVI_A, (iopb_address & 0x0ff) as u8,
            OUT, ilow,
            MVI_A, (iopb_address >> 8) as u8,
            OUT, ihigh,
            IN, 0x78, // Read dstat
            HLT,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.get_memory().load_data(iopb, iopb_address);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            if let Some(disasm) = cpu.one_step() {
                println!("{disasm}");
            };
            if opcode == HLT {
                break;
            }
        }
        remove_disk(file_name);
        let acc= cpu.a;
        assert_eq!(0x0d, acc);
    }
    #[test]
    fn test_iopb_recalibrate() {
        let file_name = "iopb_test.dsk";
        init_disk(file_name);
        // let's use that freshly created disk image and write some data
        let mut floppy = Floppy::new(file_name, false).unwrap();
        let mut data = [0; 128];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        // Let's write one sector
        let sector = Sector::new(0, 1, &data);
        _ = floppy.seek_write_sector(sector);
        let mut cpu = i8080::Cpu::new();
        let iopb_address = 0x2000;
        let program_address = 0x1000;
        let ilow = 0x79u8;
        let ihigh = 0x7au8;
        let mut fdc = Box::new(Isbc201::new(cpu.get_memory_ref())); // Base address 0x78
        // Let's assign the floppy as floppy[0] to the controller
        let _ = fdc.set_floppy(floppy, 0);
        fdc.set_base_address(0x78);
        let io_memory = cpu.get_io_memory().unwrap();
        let res = io_memory.map_port(fdc);
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(false); // Set to true if you want to see ASM cod which is executed

        let iopb: &[u8] = &[
            // Iopb starts at 0x2000
            0x80, // Channel word 
            0b0000_0011, // Diskette operation (recalibrate)
            0x01, // Number of records  
            0x00, // Track address
            0x01, // Sector address 
            0x00, // buffer address Lower. 0x4000 buffer address for read dtata from fdc
            0x30, // buffer address Upper.
        ];
        let program: &[u8] = &[
            MVI_A, (iopb_address & 0x0ff) as u8,
            OUT, ilow,
            MVI_A, (iopb_address >> 8) as u8,
            OUT, ihigh,
            IN, 0x78, // Read dstat
            HLT,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.get_memory().load_data(iopb, iopb_address);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            if let Some(disasm) = cpu.one_step() {
                println!("{disasm}");
            };
            if opcode == HLT {
                break;
            }
        }
        remove_disk(file_name);
        let acc= cpu.a;
        assert_eq!(0x0d, acc); // Check the status of operation
    }
    #[test]
    fn test_iopb_format_track() {
        let file_name = "iopb_test.dsk";
        init_disk(file_name);
        // let's use that freshly created disk image and write some data
        let mut floppy = Floppy::new(file_name, false).unwrap();
        let mut data = [0; 128];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        // Let's write one sector
        let sector = Sector::new(0, 1, &data);
        _ = floppy.seek_write_sector(sector);
        let mut cpu = i8080::Cpu::new();
        let iopb_address = 0x2000;
        let program_address = 0x1000;
        let ilow = 0x79u8;
        let ihigh = 0x7au8;
        let mut fdc = Box::new(Isbc201::new(cpu.get_memory_ref())); // Base address 0x78
        // Let's assign the floppy as floppy[0] to the controller
        let _ = fdc.set_floppy(floppy, 0);
        fdc.set_base_address(0x78);
        let io_memory = cpu.get_io_memory().unwrap();
        let res = io_memory.map_port(fdc);
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(false); // Set to true to see ASM code which is executed

        let iopb: &[u8] = &[
            // Iopb starts at 0x2000
            0x80, // Channel word 
            0b0000_0010, // Diskette operation (format track)
            0x01, // Number of records  
            0x02, // Track address
            0x01, // Sector address 
            0x00, // buffer address Lower. 0x4000 buffer address for read dtata from fdc
            0x30, // buffer address Upper.
        ];
        let program: &[u8] = &[
            MVI_A, (iopb_address & 0x0ff) as u8,
            OUT, ilow,
            MVI_A, (iopb_address >> 8) as u8,
            OUT, ihigh,
            IN, 0x78, // Read dstat
            HLT,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.get_memory().load_data(iopb, iopb_address);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            if let Some(disasm) = cpu.one_step() {
                println!("{disasm}");
            };
            if opcode == HLT {
                break;
            }
        }
        remove_disk(file_name);
        let acc= cpu.a;
        assert_eq!(0x0d, acc); // Check the status of fdc
    }
    #[test]
    fn test_iopb_read_sector_mem_mapped() {
        let file_name = "iopb_test_read_sec_mem.dsk";
        init_disk(file_name);
        // let's use that freshly created disk image and write some data
        let mut floppy = Floppy::new(file_name, false).unwrap();
        let mut data = [0; 128];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        // Let's write one sector
        let sector = Sector::new(0, 1, &data);
        _ = floppy.seek_write_sector(sector);
        let mut cpu = i8080::Cpu::new();
        let iopb_address = 0x3000u16;
        let program_address = 0x2000u16;
        let base_addr = 0x1000u16; // base address of fdc = 0x1000;
        let ilow = base_addr + 1 ; 
        let ihigh = base_addr + 2;
        let mut fdc = Box::new(Isbc201::new(cpu.get_memory_ref())); // Base address 0x78
        // Let's assign the floppy as floppy[0] to the controller
        let _ = fdc.set_floppy(floppy, 0);
        fdc.set_memory_base_address(base_addr);
        let res = cpu.get_memory().map_port(fdc); 
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(false); // Set to true if you want to generate ASM code
        let iopb: &[u8] = &[
            // Iopb starts at 0x2000
            0x80, // Channel word 
            0b0000_0100, // Diskette operation (read data)
            0x01, // Number of records  
            0x00, // Track address
            0x01, // Sector address 
            0x00, // buffer address Lower. 0x4000 buffer address for read dtata from fdc
            0x40, // buffer address Upper.
        ];
        let program: &[u8] = &[
            MVI_A, (iopb_address & 0x0ff) as u8,
            STA, (ilow & 0xff) as u8, ((ilow & 0xff00) >> 8) as u8, 
            MVI_A, (iopb_address >> 8) as u8,
            STA, (ihigh & 0xff) as u8, ((ihigh & 0xff00) >> 8) as u8,
            LDA, (base_addr & 0xff) as u8, ((base_addr & 0xff00) >> 8) as u8, // Read dstat 
            HLT,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.get_memory().load_data(iopb, iopb_address);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            if let Some(disasm) = cpu.one_step() {
                println!("{disasm}");
            };
            if opcode == HLT {
                break;
            }
        }
        // Lets verify some data read from floppy to memory
        assert_eq!(cpu.get_memory().read_byte(0x4000), 0x00);
        assert_eq!(cpu.get_memory().read_byte(0x4010), 0x10);
        assert_eq!(cpu.get_memory().read_byte(0x4020), 0x20);
        assert_eq!(cpu.get_memory().read_byte(0x4030), 0x30);
        assert_eq!(cpu.get_memory().read_byte(0x4040), 0x40);
        assert_eq!(cpu.get_memory().read_byte(0x4050), 0x50);
        assert_eq!(cpu.get_memory().read_byte(0x4060), 0x60);
        assert_eq!(cpu.get_memory().read_byte(0x4070), 0x70);
        let acc= cpu.a;
        assert_eq!(0x0d, acc);
        remove_disk(file_name);
    }
    #[test]
    fn test_iopb_read_sector_mem_mapped_6502() {
        let file_name = "iopb_test_read_sec_mem_6502.dsk";
        init_disk(file_name);
        // let's use that freshly created disk image and write some data
        let mut floppy = Floppy::new(file_name, false).unwrap();
        let mut data = [0; 128];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        // Let's write one sector
        let sector = Sector::new(0, 1, &data);
        _ = floppy.seek_write_sector(sector);
        let mut cpu = mos6502::Cpu::new();
        let iopb_address = 0x3000u16;
        let program_address = 0x2000u16;
        let base_addr = 0x1000u16; // base address of fdc = 0x1000;
        let ilow = base_addr + 1 ; 
        let ihigh = base_addr + 2;
        let mut fdc = Box::new(Isbc201::new(cpu.get_memory_ref())); // Base address 0x78
        // Let's assign the floppy as floppy[0] to the controller
        let _ = fdc.set_floppy(floppy, 0);
        fdc.set_memory_base_address(base_addr);
        let res = cpu.get_memory().map_port(fdc); 
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(false); // Set to true if you want to generate ASM code
        let iopb: &[u8] = &[
            // Iopb starts at 0x2000
            0x80, // Channel word 
            0b0000_0100, // Diskette operation (read data)
            0x01, // Number of records  
            0x00, // Track address
            0x01, // Sector address 
            0x00, // buffer address Lower. 0x4000 buffer address for read dtata from fdc
            0x40, // buffer address Upper.
        ];
        let program: &[u8] = &[
            LDA_IMM, (iopb_address & 0x0ff) as u8,
            STA_ABS, (ilow & 0xff) as u8, ((ilow & 0xff00) >> 8) as u8, 
            LDA_IMM, (iopb_address >> 8) as u8,
            STA_ABS, (ihigh & 0xff) as u8, ((ihigh & 0xff00) >> 8) as u8,
            LDA_ABS, (base_addr & 0xff) as u8, ((base_addr & 0xff00) >> 8) as u8, // Read dstat 
            BRK,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.get_memory().load_data(iopb, iopb_address);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            if let Some(disasm) = cpu.one_step() {
                println!("{disasm}");
            };
            if opcode == BRK {
                break;
            }
        }
        let disassembly = cpu.disasm(0x2000, 0x200f);
        for line in disassembly {
            println!("{}", line);
        }
        cpu.get_memory().print_hex_dump(0x4000, 0x407f);
        // Lets verify some data read from floppy to memory
        assert_eq!(cpu.get_memory().read_byte(0x4000), 0x00);
        assert_eq!(cpu.get_memory().read_byte(0x4010), 0x10);
        assert_eq!(cpu.get_memory().read_byte(0x4020), 0x20);
        assert_eq!(cpu.get_memory().read_byte(0x4030), 0x30);
        assert_eq!(cpu.get_memory().read_byte(0x4040), 0x40);
        assert_eq!(cpu.get_memory().read_byte(0x4050), 0x50);
        assert_eq!(cpu.get_memory().read_byte(0x4060), 0x60);
        assert_eq!(cpu.get_memory().read_byte(0x4070), 0x70);
        let acc= cpu.a;
        assert_eq!(0x0d, acc);
        remove_disk(file_name);
    }
    #[test]
    fn test_iopb_write_sector() {
        let file_name = "iopb_test_write_sector.dsk";
        init_disk(file_name);
        // let's use that freshly created disk image and write some data
        let floppy = Floppy::new(file_name, false).unwrap();
        let mut data = [0; 128];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        // Let's write one sector
        let mut cpu = i8080::Cpu::new();
        let iopb_address = 0x2000;
        let program_address = 0x1000;
        let ilow = 0x79u8;
        let ihigh = 0x7au8;
        let mut fdc = Box::new(Isbc201::new(cpu.get_memory_ref())); // Base address 0x78
        // Let's assign the floppy as floppy[0] to the controller
        let _ = fdc.set_floppy(floppy, 0);
        fdc.set_base_address(0x78);
        let io_memory = cpu.get_io_memory().unwrap();
        let res = io_memory.map_port(fdc);
        assert_eq!(Ok(()), res);
        let mut data = [0; 256];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        cpu.set_debug_flag(false);
        let iopb: &[u8] = &[
            // Iopb starts at 0x2000
            0x80, // Channel word 
            0b0000_0110, // Diskette operation (write data)
            0x01, // Number of records  
            0x00, // Track address
            0x01, // Sector address 
            0x00, // buffer address Lower. 0x3000 buffer address for read data from fdc
            0x30, // buffer address Upper.
        ];
        let program: &[u8] = &[
            MVI_A, (iopb_address & 0x0ff) as u8,
            OUT, ilow,
            MVI_A, (iopb_address >> 8) as u8,
            OUT, ihigh,
            IN, 0x78, // Read dstat
            MOV_C_A,
            IN, 0x79, // Read rtype
            MOV_B_A,
            IN, 0x7B, // Read rbyte
            HLT,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.get_memory().load_data(iopb, iopb_address);
        let _ = cpu.get_memory().load_data(&data, 0x3000);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            cpu.one_step();
            if opcode == HLT {
                break;
            }
        }
        // Read back sector 1
        let floppy = Floppy::new(file_name, true).unwrap();
        match floppy.read_sector(0, 1) {
            Ok(mut sector) => {
                let data = sector.get_data();
                let data_deletd_data = sector.get_data_deleted_data();
                assert_eq!(DataDeletedData::Data as u8, data_deletd_data);
                assert_eq!(0x00, data[0]);
                assert_eq!(0x10, data[0x10]);
                assert_eq!(0x20, data[0x20]);
                assert_eq!(0x30, data[0x30]);
                assert_eq!(0x40, data[0x40]);
                assert_eq!(0x50, data[0x50]);
                assert_eq!(0x60, data[0x60]);
                assert_eq!(0x70, data[0x70]);
            },
            Err(_) => {
                panic!()
            }
        };
        remove_disk(file_name);
        assert_eq!(0x0d, cpu.c); // assert dstat
        assert_eq!(0x00, cpu.b); // assert rtype - IO complete
        assert_eq!(0x00, cpu.a);  // assert rbyte - No errors
    }
    #[test]
    fn test_iopb_write_2_sectors() {
        let file_name = "iopb_test_write_2_sectors.dsk";
        init_disk(file_name);
        // let's use that freshly created disk image and write some data
        let floppy = Floppy::new(file_name, false).unwrap();
        let mut data = [0; 128];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        // Let's write one sector
        let mut cpu = i8080::Cpu::new();
        let iopb_address = 0x2000;
        let program_address = 0x1000;
        let ilow = 0x79u8;
        let ihigh = 0x7au8;
        let mut fdc = Box::new(Isbc201::new(cpu.get_memory_ref())); // Base address 0x78
        // Let's assign the floppy as floppy[0] to the controller
        let _ = fdc.set_floppy(floppy, 0);
        fdc.set_base_address(0x78);
        let io_memory = cpu.get_io_memory().unwrap();
        let res = io_memory.map_port(fdc);
        assert_eq!(Ok(()), res);
        let mut data = [0; 256];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        cpu.set_debug_flag(false);
        let iopb: &[u8] = &[
            // Iopb starts at 0x2000
            0x80, // Channel word 
            0b0000_0110, // Diskette operation (write data)
            0x02, // Number of records  
            0x00, // Track address
            0x01, // Sector address 
            0x00, // buffer address Lower. 0x3000 buffer address for read data from fdc
            0x30, // buffer address Upper.
        ];
        let program: &[u8] = &[
            MVI_A, (iopb_address & 0x0ff) as u8,
            OUT, ilow,
            MVI_A, (iopb_address >> 8) as u8,
            OUT, ihigh,
            IN, 0x78, // Read dstat
            MOV_C_A,
            IN, 0x79, // Read rtype
            MOV_B_A,
            IN, 0x7B, // Read rbyte
            HLT,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.get_memory().load_data(iopb, iopb_address);
        let _ = cpu.get_memory().load_data(&data, 0x3000);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            cpu.one_step();
            if opcode == HLT {
                break;
            }
        }
        // Read back sector 1
        let floppy = Floppy::new(file_name, true).unwrap();
        match floppy.read_sector(0, 1) {
            Ok(sector) => {
                let data = sector.get_data();
                assert_eq!(0x00, data[0]);
                assert_eq!(0x10, data[0x10]);
                assert_eq!(0x20, data[0x20]);
                assert_eq!(0x30, data[0x30]);
                assert_eq!(0x40, data[0x40]);
                assert_eq!(0x50, data[0x50]);
                assert_eq!(0x60, data[0x60]);
                assert_eq!(0x070, data[0x70]);
            },
            Err(_) => {
                panic!()
            }
        };
        // Read back sector 2
        match floppy.read_sector(0, 2) {
            Ok(sector) => {
                let data = sector.get_data();
                assert_eq!(0x80, data[0]);
                assert_eq!(0x90, data[0x10]);
                assert_eq!(0xa0, data[0x20]);
                assert_eq!(0xb0, data[0x30]);
                assert_eq!(0xc0, data[0x40]);
                assert_eq!(0xd0, data[0x50]);
                assert_eq!(0xe0, data[0x60]);
                assert_eq!(0xf0, data[0x70]);
            },
            Err(_) => {
                panic!()
            }
        };
        remove_disk(file_name);
        assert_eq!(0x0d, cpu.c); // assert dstat
        assert_eq!(0x00, cpu.b); // assert rtype - IO complete
        assert_eq!(0x00, cpu.a);  // assert rbyte - No errors
    }
    // Tries to write to sector 26, 27
    // It should generate seek error as sector 27 doesn't exist
    #[test]
    fn test_iopb_write_2_sectors_error() {
        let file_name = "iopb_test_write_2_sectors_error.dsk";
        init_disk(file_name);
        // let's use that freshly created disk image and write some data
        let floppy = Floppy::new(file_name, false).unwrap();
        let mut data = [0; 128];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        // Let's write one sector
        let mut cpu = i8080::Cpu::new();
        let iopb_address = 0x2000;
        let program_address = 0x1000;
        let ilow = 0x79u8;
        let ihigh = 0x7au8;
        let mut fdc = Box::new(Isbc201::new(cpu.get_memory_ref())); // Base address 0x78
        // Let's assign the floppy as floppy[0] to the controller
        let _ = fdc.set_floppy(floppy, 0);
        fdc.set_base_address(0x78);
        let io_memory = cpu.get_io_memory().unwrap();
        let res = io_memory.map_port(fdc);
        assert_eq!(Ok(()), res);
        let mut data = [0; 256];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        cpu.set_debug_flag(false);
        let iopb: &[u8] = &[
            // Iopb starts at 0x2000
            0x80, // Channel word 
            0b0000_0110, // Diskette operation (write data)
            0x02, // Number of records  
            0x00, // Track address
            26, // Sector address 
            0x00, // buffer address Lower. 0x3000 buffer address for read data from fdc
            0x30, // buffer address Upper.
        ];
        let program: &[u8] = &[
            MVI_A, (iopb_address & 0x0ff) as u8,
            OUT, ilow,
            MVI_A, (iopb_address >> 8) as u8,
            OUT, ihigh,
            IN, 0x78, // Read dstat
            MOV_C_A,
            IN, 0x79, // Read rtype
            MOV_B_A,
            IN, 0x7B, // Read rbyte
            HLT,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.get_memory().load_data(iopb, iopb_address);
        let _ = cpu.get_memory().load_data(&data, 0x3000);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            cpu.one_step();
            if opcode == HLT {
                break;
            }
        }
        remove_disk(file_name);
        assert_eq!(0x0D, cpu.c); // assert dstat
        assert_eq!(0x00, cpu.b); // assert rtype - IO complete
        assert_eq!(0x04, cpu.a);  // assert rbyte - Seek errors
    }
    #[test]
    fn test_iopb_write_sector_deleted_data() {
        let file_name = "iopb_test_write_sector_deleted_data.dsk";
        init_disk(file_name);
        // let's use that freshly created disk image and write some data
        let floppy = Floppy::new(file_name, false).unwrap();
        let mut data = [0; 128];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        // Let's write one sector
        let mut cpu = i8080::Cpu::new();
        let iopb_address = 0x2000;
        let program_address = 0x1000;
        let ilow = 0x79u8;
        let ihigh = 0x7au8;
        let mut fdc = Box::new(Isbc201::new(cpu.get_memory_ref())); // Base address 0x78
        // Let's assign the floppy as floppy[0] to the controller
        let _ = fdc.set_floppy(floppy, 0);
        fdc.set_base_address(0x78);
        let io_memory = cpu.get_io_memory().unwrap();
        let res = io_memory.map_port(fdc);
        assert_eq!(Ok(()), res);
        let mut data = [0; 256];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        cpu.set_debug_flag(false);
        let iopb: &[u8] = &[
            // Iopb starts at 0x2000
            0x80, // Channel word 
            0b0000_0111, // Diskette operation (write deleted data)
            0x01, // Number of records  
            0x00, // Track address
            0x01, // Sector address 
            0x00, // buffer address Lower. 0x3000 buffer address for read data from fdc
            0x30, // buffer address Upper.
        ];
        let program: &[u8] = &[
            MVI_A, (iopb_address & 0x0ff) as u8,
            OUT, ilow,
            MVI_A, (iopb_address >> 8) as u8,
            OUT, ihigh,
            IN, 0x78, // Read dstat
            MOV_C_A,
            IN, 0x79, // Read rtype
            MOV_B_A,
            IN, 0x7B, // Read rbyte
            HLT,
        ];
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.get_memory().load_data(iopb, iopb_address);
        let _ = cpu.get_memory().load_data(&data, 0x3000);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            cpu.one_step();
            if opcode == HLT {
                break;
            }
        }
        // Read back sector 1
        let floppy = Floppy::new(file_name, true).unwrap();
        match floppy.read_sector(0, 1) {
            Ok(mut sector) => {
                let data = sector.get_data();
                let data_deletd_data = sector.get_data_deleted_data();
                assert_eq!(DataDeletedData::DeletedData as u8, data_deletd_data);
                assert_eq!(0x00, data[0]);
                assert_eq!(0x10, data[0x10]);
                assert_eq!(0x20, data[0x20]);
                assert_eq!(0x30, data[0x30]);
                assert_eq!(0x40, data[0x40]);
                assert_eq!(0x50, data[0x50]);
                assert_eq!(0x60, data[0x60]);
                assert_eq!(0x70, data[0x70]);
            },
            Err(_) => {
                panic!()
            }
        };
        remove_disk(file_name);
        assert_eq!(0x0d, cpu.c); // assert dstat
        assert_eq!(0x00, cpu.b); // assert rtype - IO complete
        assert_eq!(0x00, cpu.a);  // assert rbyte - No errors
    }
}
