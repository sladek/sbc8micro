//! Read data from sector
//! 
//! It will create a new image file, write some dta to track(0) sector(1) and then run a i8080 program to read data from that sector
//! First the empty disk is created and formated, then data array with content [0, 1, 2 .. 0x7e, 0x7f] is stored in the track(0) sector(1)
//! and then the data from that sector is read back and stored in memory
use sbc8micro::disk::sssd8fd::{Floppy, Sector};
    use sbc8micro::io::isbc201::Isbc201;
    use sbc8micro::cpu::{i8080, CpuUi};
    use std::rc::Rc;
    use sbc8micro::disassembler::i8080_opcode_consts::*;
    use std::fs;

    fn main() {
        let file_name = "iopb_test.dsk";
        _ = fs::remove_file(file_name);
        // Let's create new floppy image and format it
        _  = Floppy::new(file_name, false).unwrap().format();
        // let's use that freshly created disk image and write some data
        let mut floppy = Floppy::new(file_name, false).unwrap();
        // Create a sector data with values [0x00 - 0xff]
        let mut data = [0; 128];
        for i in 0..data.len() {
            data[i] = i as u8;
        };
        // Let's write one sector
        let sector = Sector::new(0, 1, &data);
        _ = floppy.seek_write_sector(sector);
        let mut cpu = i8080::Cpu::new();
        let memory = Rc::clone(&cpu.memory);
        let iopb_address = 0x2000;
        let program_address = 0x1000;
        let ilow = 0x79u8;
        let ihigh = 0x7au8;
        let mut fdc = Box::new(Isbc201::new()); // Base address 0x78
        // Let's assign the floppy as floppy[0] to the controller
        fdc.set_floppy(floppy, 0);
        fdc.set_base_address(0x78);
        let io_memory = cpu.get_io_memory().unwrap();
        let res = io_memory.map_port(fdc);
        assert_eq!(Ok(()), res);
        cpu.set_debug_flag(true);
        // Prepare IOPB
        let iopb: &[u8] = &[
            // Iopb starts at 0x2000
            0x80, // Cannel word 
            0b0000_0100, // Diskette operation (read data)
            0x01, // Number of records  
            0x00, // Track address
            0x01, // Sector address 
            0x00, // buffer address Lower. 0x3000 buffer address for read dtata from fdc
            0x30, // buffer address Upper.
        ];
        let program: &[u8] = &[
            MVI_A, (iopb_address & 0x0ff) as u8,
            OUT, ilow,
            MVI_A, (iopb_address >> 8) as u8,
            OUT, ihigh,
            IN, 0x78, // Read drive status
            STA, 0x10, 0x20, // and store it at 0x2010
            HLT,
        ];
        // Load programm
        let _ = cpu.get_memory().load_data(program, program_address);
        // Load IOPB
        let _ = cpu.get_memory().load_data(iopb, iopb_address);
        // Print source code
        let _ = cpu.print_disasm(program_address, program_address + 0x0a);
        // Set pc and start programm
        let _ = cpu.set_pc(program_address);
        loop {
            let pc = cpu.pc;
            let opcode = cpu.get_memory().read_byte(pc);
            cpu.one_step();
            if opcode == HLT {
                break;
            }
        }
        // Dump content of IOPB
        println!("Content of IOPB");
        let _ = cpu
            .get_memory()
            .print_hex_dump(iopb_address, iopb_address + 0x0f);
        // Dump contetnt of the memory where the sector is stored
        println!("Content of sector data that have just been read from floppy");
        let _ = cpu
            .get_memory()
            .print_hex_dump(0x3000,  0x307f);
        let dstat = cpu
            .get_memory()
            .read_byte(0x2010);
        // Status should be: fdc - present, disk 0 - ready, interrupt pending flag is set.
        assert_eq!(0x0d, dstat);
        // Let's assert the contant of accumulator.
        // The following bits should be set: bit(3) - controller present, bit(2) - interrupt pending, bit(0) - drive 0 ready
        let acc= cpu.a;
        assert_eq!(0x0d, acc);
        _ = fs::remove_file(file_name);
    }
