use crate::disk::sssd8fd::{ErrorIndicators, Floppy};
use crate::memory::{self, MemCell, Memory};
use crate::io::IoPort;
use crate::disk::sssd8fd::Result;
use std::cell::RefCell;
use std::rc::Rc;

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
    lock_override: bool, // bit (7).  1 - "wait" bit is not set, this prevents IOPB being overwritten by the controller
    random_format_sequence: bool, // bit (6). 0 - sector addresses are assigned in sequential order, 1 - sectr addresses are assigned randomly based on pattern listed in 52 byte memory buffer.
    interrupt_control: u8, // bits (4 and 5). Enable or disable Diskette Channel interrupts 
    data_word_length: bool,       // bit (3). 0 - for 8 bit systems, 1 - for 16 bit systems
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
            successor_bit: value & 0b0000_0100 != 0,
            branch_on_wait: value & 0b0000_0010 != 0,
            wait: value & 0b0000_0001 != 0,
        }
    }
    pub fn get_value(&self) -> u8 {
        self.value
    }
}

/// Drives status
enum Dstat {
    Drive0Ready = 0b0001, // Drive 0 ready
    Drive1Ready = 0b0010, // Drive 0 ready
    InterruptPending = 0b0100, // Interrupt flip-flop status
    ControllerPresent = 0b1000, // Controller presence indicator
}
pub struct Sssd8fdc {
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
    rbyte_address: u8,
    rbyte_memory_address: u16,
    reset_address: u8,
    reset_memory_address: u16,
    iopb_address: u16,
    base_address: Option<u8>,
    memory_base_address: Option<u16>,
    port_offsets: [u8; 5], // base_address + (0 = dstat, 1 = ilow/rtype, 2 = ihigh, 3 = rbyte, 7 = reset)
    name: Option<String>,
    floppy: [Option<Floppy>; 2],
    active_floppy: u8,
}

const BASE: u8 = 0x78;
const MEMORY_BASE: u16 = 0x1000;
impl Default for Sssd8fdc {
    fn default() -> Self {
        Sssd8fdc {
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
            rbyte_address: BASE + 3,
            rbyte_memory_address: MEMORY_BASE + 3,
            reset_address: BASE + 7,
            reset_memory_address: MEMORY_BASE + 7,
            iopb_address: 0u16,
            base_address: None,
            memory_base_address: None,
            port_offsets: [0, 1, 2, 3, 7], // 0 - dstat, 1 - ilow/rtype, 2 - ihigh, 3 - rbyte, 7 - reset
            name: Some("iSBC-201".to_string()),
            floppy: [const { None }; 2],
            active_floppy: 0,
        }
    }
}

impl Sssd8fdc {
    pub fn new() -> Self {
        Sssd8fdc {
            dstat: Dstat::ControllerPresent as u8,
            ..Default::default()
        }
    }
    /// Sets base address of serial port (8251).
    ///
    /// Base address is data port for 8251
    /// Base address + 1 is address of controll and status port of 8251
    pub fn set_base_address(&mut self, address: u8) {
        self.memory_base_address = None;
        self.base_address = Some(address);
        self.dstat_address = address;
        self.ilow_address = address.wrapping_add(1);
        self.ihigh_address = address.wrapping_add(2);
        self.rtype_address = address.wrapping_add(1);
        self.rbyte_address = address.wrapping_add(2);
        self.reset_address = address.wrapping_add(7);
    }
    /// Sets base address of serial port (8251) mapped to memory (16 bits).
    ///
    /// Base address is data port for 8251
    /// Base address + 1 is address of controll and status port of 8251    
    pub fn set_memory_base_address(&mut self, address: u16) {
        self.base_address = None;
        self.memory_base_address = Some(address);
        self.dstat_memory_address = address;
        self.ilow_memory_address = address.wrapping_add(1);
        self.ihigh_memory_address = address.wrapping_add(2);
        self.rtype_memory_address = address.wrapping_add(1);
        self.rbyte_memory_address = address.wrapping_add(2);
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
            channel_command:  Self::get_memory_cell_data(memory[address]),
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
        self.dstat = 0;
    }
    /// Assign floppy disk
    /// 
    /// Assignes floppy disk to bay 1 or bay 2
    pub fn set_floppy(&mut self, floppy: Floppy, number: u8) {
        if number <= 3 {
            self.floppy[number as usize] = Some(floppy);
            if number == 0 {
                self.dstat |= Dstat::Drive0Ready as u8;
            } else {
                self.dstat |= Dstat::Drive1Ready as u8;

            }
        }
    }
    /// Remove floppy drive
    /// 
    /// Removes floppy drive[number] from the controller
    pub fn remove_floppy(&mut self, number: u8) {
           if number == 0 {
                self.floppy[0] = None;
                self.dstat &= !(Dstat::Drive0Ready as u8);
            } else {
                self.floppy[1] = None;
                self.dstat &= !(Dstat::Drive1Ready as u8);

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
    fn process_iopb(&mut self, memory: &mut [MemCell], iopb: &Iopb) -> Result<()> {
        let di = DisketteInstruction::new(iopb.diskette_instruction);
        let opcode = di.get_opcode();
        let unit = di.get_unit_select();
        self.set_active_floppy(unit);
        match opcode {
            opcode if opcode == Opcode::ReadData as u8 => {
                let data = self.read_data(iopb)?;
                let address: u16 = (iopb.buffer_address_high as u16) << 8 | iopb.buffer_address_low as u16;
                for i in 0 .. data.len() {
                    memory[address as usize + i] = memory::MemCell::Memory(data[i]);
                }
                self.set_interrupt_pending();

            }
            _ => {
                // Do nothing, for now.
            }
        }
        Ok(())
    }
    /// Read data from floppy disk
    /// 
    /// Reads data from floppy disk based on IOPB.
    fn read_data(&mut self, iopb: &Iopb) -> Result<Vec<u8>> {
        self.reset_interrupt_pending();
        match &self.floppy[self.active_floppy as usize] {
            Some(floppy) => {
                let sector = floppy.read_sector(iopb.track_address, iopb.sector_address)?;
                let data = sector.get_data();
                self.set_interrupt_pending();
                Ok(data.to_vec())
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

impl IoPort for Sssd8fdc {
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
        format!(
            "Floppy disk controller (single sided single density 8 inch): base address[{base_address}], name[{name}]",
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
        None
    }
    fn read_from_mem_address(&mut self, address: u16) -> Option<u8> {
        if address == self.dstat_memory_address {
            return Some(self.dstat);
        }
        None
    }
    fn write_to_address(&mut self, memory: &mut [MemCell], address: u8, data: u8) {
        if address == self.ilow_address {
            // Write mamory address lower
            self.iopb_address = data as u16;
            return;
        }
        if address == self.ihigh_address {
            // Write memory address upper and start disk operation
            self.iopb_address |= (data as u16) << 8;
            let iopb = self.get_iopb(memory);
            let _ = self.process_iopb(memory, &iopb);
            return;
        }
        if address == self.reset_address {
            // Write memory address upper and start disk operation
            self.reset();
        }
    }
    fn write_to_memory_address(&mut self, memory: &mut [MemCell], address: u16, data: u8) {
        if address == self.ilow_memory_address {
            // Write memory address lower
            self.iopb_address = data as u16;
            return;
        }
        if address == self.ihigh_memory_address {
            // Write memory address upper and start disk operation
            self.iopb_address |= (data as u16) << 8;

            let iopb = self.get_iopb(memory);
            let _ = self.process_iopb(memory, &iopb);
            return;
        }
        if address == self.reset_memory_address {
            // Write memory address upper and start disk operation
            self.reset();
        }
    }
}
#[derive(Default, Debug)]
struct Iopb {
    channel_command: u8,
    diskette_instruction: u8,
    number_of_records: u8,
    track_address: u8,
    sector_address: u8,
    buffer_address_low: u8,
    buffer_address_high: u8,
    block_number: u8,
    next_iopb_address_lower: u8,
    next_iopb_address_upper: u8,
}
#[cfg(test)]
mod tests {
    use crate::cpu::CpuUi;
    use crate::cpu::i8080::{self};
    use crate::disassembler::i8080_opcode_consts::*;
    use crate::disk::sssd8fd::Floppy;
    use crate::disk::sssd8fd::Sector;
    use crate::io::memory::IoMemory;
    use crate::io::{IoPort, sssd8fdc::Sssd8fdc};
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
        let cpu = i8080::Cpu::new();
        let memory = Rc::clone(&cpu.memory);
        memory.borrow_mut().write_byte(0x55aa, 0x55);
        let mut fdc = Sssd8fdc::new();
        fdc.set_base_address(0x78);
        println!("Name: {:?}", fdc.get_name());
        println!("{}", fdc.get_io_port_info());
        let mut io_memory = IoMemory::new();
        let res = io_memory.map_port(Box::new(fdc));
        assert_eq!(res, Ok(()));
    }
    #[test]
    fn test_memory_mapped() {
        let cpu = i8080::Cpu::new();
        let memory = Rc::clone(&cpu.memory);
        memory.borrow_mut().write_byte(0x55aa, 0x55);
        let mut fdc = Sssd8fdc::new();
        fdc.set_memory_base_address(0x1000);
        println!("Name: {:?}", fdc.get_name());
        println!("{}", fdc.get_io_port_info());
        let res = memory.borrow_mut().map_port(Box::new(fdc));
        assert_eq!(res, Ok(()));
    }
    #[test]
    fn test_iopb_read_sector() {
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
        let mut fdc = Box::new(Sssd8fdc::new()); // Base address 0x78
        // Let's assign the floppy as floppy[0] to the controller
        fdc.set_floppy(floppy, 0);
        fdc.set_base_address(0x78);
        let io_memory = cpu.get_io_memory().unwrap();
        let res = io_memory.map_port(fdc);
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(true);

        let iopb: &[u8] = &[
            // Iopb starts at 0x2000
            0x80, // Cannel word 
            0b0000_0100, // Diskette operation (read data)
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
    fn test_iopb_read_sector_mem_mapped() {
        let file_name = "iopb_test1.dsk";
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
        let mut fdc = Box::new(Sssd8fdc::new()); // Base address 0x78
        // Let's assign the floppy as floppy[0] to the controller
        fdc.set_floppy(floppy, 0);
        fdc.set_memory_base_address(base_addr);
        let res = cpu.get_memory().map_port(fdc); 
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(true);
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
        // Print source code
        let _ = cpu.get_memory().load_data(program, program_address);
        let _ = cpu.get_memory().load_data(iopb, iopb_address);
        let _ = cpu.print_disasm(program_address, program_address + 0x0a);
        println!("Let's print IOPB from memory");
        let _ = cpu
            .get_memory()
            .print_hex_dump(iopb_address,  iopb_address + 0xf);
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            cpu.one_step();
            if opcode == HLT {
                break;
            }
        }
        println!("Content of sector data that have just been read from floppy");
        let _ = cpu
            .get_memory()
            .print_hex_dump(0x4000,  0x407f);
        let acc= cpu.a;
        assert_eq!(0x0d, acc);
        remove_disk(file_name);
    }
}
