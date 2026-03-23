//! Intel I8080 CPU
//!
//! Emulates Intel 8080 CPU at register level. No timing is emulated
//!
//! Below is an example of its usage
//! ```
//! use sbc8micro::memory;
//! use sbc8micro::cpu::i8080::Cpu;
//! use sbc8micro::status;
//! use sbc8micro::disassembler::i8080_opcode_consts::*;
//!
//! let mut cpu = Cpu::new();
//! cpu.memory.borrow_mut().write_byte(0x1234, 0x12);
//! cpu.status.set_carry(true);
//! let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0x34, ADC_M, HLT];
//! cpu.load_program(&program, 0x0600);
//! loop {
//!     let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
//!     cpu.step();
//!     if opcode == HLT {
//!         break;
//!     }
//! }
//! assert_eq!(cpu.a, 0x47u8);
//! assert_eq!(cpu.status.value, 0x06);
//!```

use crate::cpu::{CpuUi, Reg};
use crate::debugger::Breakpoints;
use crate::disassembler::i8080::disassemble;
use crate::disassembler::i8080_opcode_consts::*;
use crate::io;
use crate::memory::Memory;
use crate::status::i8080::*;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use crate::disassembler::i8080::load_opcodes_table;
use crate::bootloader::Bootloader;

/// CPU registers, flags, counters and memory
#[derive(Default)]
pub struct Cpu {
    /// Accumulater
    pub a: u8,
    /// B register
    pub b: u8,
    /// C register
    pub c: u8,
    /// D register
    pub d: u8,
    /// E register
    pub e: u8,
    /// H register
    pub h: u8,
    /// L register
    pub l: u8,
    /// Status register
    pub status: Status,
    /// Programm counter
    pub pc: u16,
    /// Stack pointer
    pub sp: u16,
    /// Interrupt enable
    pub inte: bool,
    /// Memory assigned to CPU
    pub memory: Box<Rc<RefCell<Memory>>>,
    /// Io memory
    pub io_memory: io::memory::IoMemory,
    /// Breakpoints
    pub breakpoints: Breakpoints,
    /// Debug flag
    ///
    /// If frue opcode is also outputed when the programm is executed.
    /// This can slow the execution so it should be used mainly
    /// during debuging process.
    pub debug: bool,
    /// Bootloadr
    pub bootloader: Option<Bootloader>,
    /// Specify HLT code which will end running command
    /// and returns to UI or exits the application if UI is not used
    pub hlt_code: u8,
}

impl Cpu {
    ///
    /// Returns initialised instance of CPU 8080
    ///
    pub fn new() -> Cpu {
        let memory = Box::new(Rc::new(RefCell::new(Memory::new())));
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            status: Status::new(),
            pc: 0,
            sp: 0,
            inte: false,
            memory: memory.clone(),
            io_memory: io::memory::IoMemory::new(),
            breakpoints: Breakpoints::new(),
            debug: true,
            bootloader: None,
            hlt_code: HLT, // Fo 8080 we have HLT instruction
        }
    }
    /// Gets cpu for usage in terminal UI.
    pub fn get_cpu_ui() -> Option<Box<dyn CpuUi>> {
        Some(Box::new(Self::new()))
    }
    fn set_8_bit_value(value: u16) -> Result<u8, String> {
        if value > 0xff {
            Err(format!(
                "Value {:04X}H [{value}] is too big for 8 bit register.",
                value
            )
            .to_string())
        } else {
            Ok(value as u8)
        }
    }
    /// Sets 8 bit register value by register name
    pub fn set_register_by_name(&mut self, reg: &str, value: u16) -> Result<(), String> {
        match reg.to_uppercase().as_str() {
            "A" => self.a = Self::set_8_bit_value(value)?,
            "B" => self.b = Self::set_8_bit_value(value)?,
            "C" => self.c = Self::set_8_bit_value(value)?,
            "D" => self.d = Self::set_8_bit_value(value)?,
            "E" => self.e = Self::set_8_bit_value(value)?,
            "H" => self.h = Self::set_8_bit_value(value)?,
            "L" => self.l = Self::set_8_bit_value(value)?,
            "SP" => self.sp = value,
            "PC" => self.pc = value,
            _ => {
                return Err(format!("Unknown register {reg}"));
            }
        }
        Ok(())
    }
    /// Get register value by register name
    fn get_register(&self, reg: &str) -> Result<Reg, String> {
        match reg.to_uppercase().as_str() {
            "A" => Ok(Reg::R8(self.a)),
            "B" => Ok(Reg::R8(self.b)),
            "C" => Ok(Reg::R8(self.c)),
            "D" => Ok(Reg::R8(self.d)),
            "E" => Ok(Reg::R8(self.e)),
            "H" => Ok(Reg::R8(self.h)),
            "L" => Ok(Reg::R8(self.l)),
            "SP" => Ok(Reg::R16(self.sp)),
            "PC" => Ok(Reg::R16(self.pc)),
            "PSW" => Ok(Reg::R8(self.status.value)),
            _ => Err(format!("Unknown register {reg}")),
        }
    }
    ///
    /// Loads program to the memory and set PC to start address of the programm
    ///
    pub fn load_program(&mut self, program: &[u8], start_addr: u16) {
        let memory = Rc::clone(&self.memory);
        let _ = memory.borrow_mut().load_data(program, start_addr);
        self.pc = start_addr;
    }
    ///
    /// Prints content of registers and flags
    ///
    pub fn get_registers(&self) -> String {
        format!(
"---------------------------------------------------------------------------------------------------------
|  A  |  B  |  C  |  D  |  E  |  H  |  L  |  SP   |  PC   | INTE | PSW | S | Z | 0 | AC | 0 | P | 1 | C |
|-----|-----|-----|-----|-----|-----|-----|-------|-------|------|-----|---|---|---|----|---|---|---|---|
| {:02X}H | {:02X}H | {:02X}H | {:02X}H | {:02X}H | {:02X}H | {:02X}H | {:04X}H | {:04X}H |  {}   | {:02X}H | {} | {} | 0 | {}  | 0 | {} | 1 | {} |
---------------------------------------------------------------------------------------------------------\n",
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.sp,
            self.pc,
            if self.inte {1} else {0},
            self.status.value,
            self.status.is_negative() as u8,
            self.status.is_zero() as u8,
            self.status.is_ac() as u8,
            self.status.is_parity() as u8,
            self.status.is_carry() as u8
        )
    }
    /// Read data from data bus
    ///
    /// This will be modified after all the instructions are implemented
    /// Data bus needs to be introduced here
    fn inp(&mut self, address: u8) -> u8 {
        self.io_memory.read(address)
    }
    pub fn out(&mut self, address: u8) {
        let memory = &mut self.memory.clone().borrow_mut().get_data();
        let dma = self.io_memory.write(memory, address, self.a);
        let _ = self.memory.borrow_mut().process_dma(dma);
    }
    fn read_immediate_byte(&mut self) -> u8 {
        let value = self.memory.borrow_mut().read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
    }
    fn read_immediate_word(&mut self) -> u16 {
        let w_low = self.read_immediate_byte() as u16;
        let w_high = self.read_immediate_byte() as u16;
        (w_high << 8) | w_low
    }
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
    fn get_m(&mut self) -> u8 {
        let addr = self.get_hl();
        self.memory.borrow_mut().read_byte(addr)
    }
    fn set_bc(&mut self, data: u16) {
        self.b = ((data & 0xff00) >> 8) as u8;
        self.c = (data & 0x0ff) as u8;
    }
    fn set_de(&mut self, data: u16) {
        self.d = ((data & 0xff00) >> 8) as u8;
        self.e = (data & 0x0ff) as u8;
    }
    fn set_hl(&mut self, data: u16) {
        self.h = ((data & 0xff00) >> 8) as u8;
        self.l = (data & 0x0ff) as u8;
    }
    ///
    /// This function formats HEX string from address and opcode and
    /// is used in debug mode to print address and opcode of the instruction that
    /// is currently being processed
    /// For example if you are processing the following code at address 0x200
    /// MVI A, 55H
    /// Then it looks like this in memory
    /// 0x200 0x3E, 0x55
    /// and PC points to the address of 0x0202 (just after the operand).
    /// To get the opcode you have to go back 2 bytes so neg_offset is 2.
    /// And the resulting string is '0200 3E'
    ///  
    fn code_to_str(&mut self, mut neg_offset: u8) -> String {
        let mut addr = self.pc.wrapping_sub(neg_offset as u16);
        let mut result = String::new();
        result.push_str(format!("{:04X}  ", addr).as_str());
        while neg_offset != 0 {
            result.push_str(format!("{:02X} ", self.memory.borrow_mut().read_byte(addr)).as_str());
            neg_offset -= 1;
            addr = addr.wrapping_add(1);
        }
        format!("{:<18}", result)
    }
    /// Sets the parity flag
    fn set_parity(&mut self, data: u8) {
        let mut mask = 0x01;
        let mut result: u8 = 0;
        for _i in 1..=8 {
            if (data & mask) != 0 {
                result += 1;
            };
            mask <<= 1;
        }
        self.status.set_parity(result.is_multiple_of(2))
    }
    /// Sets sign flag
    fn set_sign(&mut self, data: u8) {
        self.status.set_negative(data & 0x80 != 0);
    }
    /// Sets zero flag
    fn set_zero(&mut self, data: u8) {
        self.status.set_zero(data == 0);
    }
    fn addc(&mut self, value: u8) {
        self.add(value, true);
    }
    fn add(&mut self, value: u8, with_carry: bool) {
        let mut carry = 0x0u8;
        if with_carry {
            carry = if self.status.is_carry() { 1 } else { 0 } as u8;
        }
        //        let carry = if self.psw.is_carry() { 1 } else { 0 }
        if (self.a & 0x0f) + (value & 0x0f) + carry > 0x0f {
            self.status.set_ac(true);
        } else {
            self.status.set_ac(false)
        };
        let sum = self.a as u16 + value as u16 + carry as u16;
        self.a = sum as u8;
        self.status.set_carry(sum > 0xFF);
        self.set_zero(self.a);
        self.set_sign(self.a);
        self.set_parity(self.a);
    }
    fn sub(&mut self, value: u8, with_carry: bool) {
        let mut operand = value as u16;
        let mut operand_lower = value & 0x0f;
        if self.status.is_carry() && with_carry {
            operand = operand.wrapping_add(1);
            operand_lower = operand_lower.wrapping_add(1);
        }
        self.status.set_carry(operand > self.a as u16);
        let two_compl = (!operand).wrapping_add(1);
        let two_compl_lower = (!operand_lower).wrapping_add(1);
        let sum = self.a.wrapping_add(two_compl as u8);
        let tmp = (self.a & 0x0f).wrapping_add(two_compl_lower & 0x0f);
        if tmp > 0x0f || operand_lower == 0 {
            self.status.set_ac(true);
        } else {
            self.status.set_ac(false);
        }
        self.a = sum;
        self.status.set_zero(self.a == 0);
        self.status.set_negative(self.a & 0x80 != 0);
        self.set_parity(sum);
    }
    ///
    ///  ANA, ANI iinstructions clear CARRY but set AC based on bit 3
    /// Some documentation states that ANI clears AC, but it is not true
    /// it is set the same way as ANA. Verified on real HW. (TESLA 8080A)
    ///
    fn and(&mut self, value: u8) {
        let is_ac = (self.a | value) & 0x08 != 0;
        let result = self.a as u16 & value as u16;
        self.a = result as u8;
        self.status.set_carry(false);
        self.status.set_ac(is_ac);
        self.status.set_zero(self.a == 0);
        self.status.set_negative(self.a & 0x80 != 0);
        self.set_parity(self.a);
    }
    ///
    ///  ORA, ORI clears CARRY and AC flags
    ///
    fn or(&mut self, value: u8) {
        let result = self.a as u16 | value as u16;
        self.a = result as u8;
        self.status.set_carry(false);
        self.status.set_ac(false);
        self.status.set_zero(self.a == 0);
        self.status.set_negative(self.a & 0x80 != 0);
        self.set_parity(self.a);
    }
    ///
    ///  XRA
    ///
    fn xra(&mut self, value: u8) {
        let result = self.a ^ value;
        self.a = result;
        self.status.set_carry(false);
        self.status.set_ac(false);
        self.status.set_zero(self.a == 0);
        self.status.set_negative(self.a & 0x80 != 0);
        self.set_parity(self.a);
    }
    ///
    /// Reads a byte from the memory address
    /// which is in HL register pair
    ///
    fn read_m(&mut self) -> u8 {
        let h = self.h as u16;
        let l = self.l as u16;
        let hl = (h << 8) | l;
        self.memory.borrow_mut().read_byte(hl)
    }
    ///
    /// Stores a byte to the memory address
    /// which is in HL register pair
    ///
    fn store_m(&mut self, data: u8) {
        let h = self.h as u16;
        let l = self.l as u16;
        let hl = (h << 8) | l;
        self.memory.borrow_mut().write_byte(hl, data);
    }
    ///
    ///  PUSH
    ///
    fn push(&mut self, rph: u8, rpl: u8) {
        let mut addr = self.sp.wrapping_sub(1);
        let mut memory = self.memory.borrow_mut();
        memory.write_byte(addr, rph);
        addr = self.sp.wrapping_sub(2);
        memory.write_byte(addr, rpl);
        self.sp = addr;
    }
    ///
    /// POP
    ///
    fn pop(&mut self) -> (u8, u8) {
        let mut addr = self.sp;
        let mut memory = self.memory.borrow_mut();
        let rpl = memory.read_byte(addr);
        addr = addr.wrapping_add(1);
        let rph = memory.read_byte(addr);
        self.sp = self.sp.wrapping_add(2);
        (rph, rpl)
    }
    ///
    /// CALL
    ///
    fn call(&mut self) {
        //        let memory = &mut self.memory.borrow_mut();
        let addr = self.memory.borrow_mut().read_word(self.pc);
        self.pc = self.pc.wrapping_add(2);
        let pcl = (self.pc & 0xff) as u8;
        let pch = ((self.pc & 0xff00) >> 8) as u8;
        self.push(pch, pcl);
        self.pc = addr;
    }
    ///
    /// JMP
    ///
    fn jmp(&mut self) {
        let addr = self.memory.borrow_mut().read_word(self.pc);
        self.pc = addr;
    }
    ///
    /// RET
    ///
    fn ret(&mut self) {
        let mut memory = self.memory.borrow_mut();
        let addrl = memory.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let addrh = (memory.read_byte(self.sp) as u16) << 8;
        self.sp = self.sp.wrapping_add(1);
        self.pc = addrh | addrl;
    }
    /// DAA
    ///
    /// The eight-bit number in the accumulator is adjusted
    /// to form two four-bit Binary-Coded-Decimal digits
    ///
    fn daa(&mut self) {
        let mut accl = (self.a & 0xf) as u16;
        if (accl > 0x09) || self.status.is_ac() {
            accl += 0x06;
            self.status.set_ac(accl > 0xf);
        }
        let mut acc = (self.a & 0xf0) as u16 + accl;
        if (acc & 0xf0 > 0x90) || acc > 0xff || self.status.is_carry() {
            acc += 0x60;
            self.status.set_carry(true);
        }
        self.a = acc as u8;
        self.set_zero(self.a);
        self.set_sign(self.a);
        self.set_parity(self.a);
    }
    ///
    ///  Add data to HL pair and set CARRY if result is > 0x00ff.
    ///
    fn dad(&mut self, rp: u16) {
        let result: u32 = self.get_hl() as u32 + rp as u32;
        self.set_hl(result as u16);
        self.status.set_carry(result > 0x0ffff);
    }
    ///
    ///  DCR reg
    ///
    /// Decrements content of register and sets relevant flags
    ///
    fn dcr(&mut self, reg: u8) -> u8 {
        let res = reg.wrapping_sub(1);
        self.set_parity(res);
        self.status.set_zero(res == 0);
        self.status.set_ac((res & 0x0fu8) != 0x0fu8);
        self.status.set_negative((res & 0x80u8) != 0);
        res
    }
    ///
    ///  INR reg
    ///
    /// Increments register and sets relevant flags
    ///
    fn inr(&mut self, reg: u8) -> u8 {
        let res = reg.wrapping_add(1);
        self.set_parity(res);
        self.status.set_zero(res == 0);
        self.status.set_ac((res & 0x0fu8) == 0x00u8);
        self.status.set_negative((res & 0x80u8) != 0);
        res
    }
    ///
    /// RST x
    ///
    fn rst(&mut self, level: u8) {
        let pcl = (self.pc & 0xff) as u8;
        let pch = ((self.pc & 0xff00) >> 8) as u8;
        self.push(pch, pcl);
        self.pc = (level * 8) as u16;
    }
    ///
    ///  Steps through the instructions
    ///
    /// Read instruction from memory, executes it and set PC to point to next instruction in memory.
    /// If debug flag is set to true it will also print mnemonic code of the instruction that is executed.
    ///
    pub fn step(&mut self) -> Option<String> {
        macro_rules! dbg { ($($x:tt)*) => { if self.debug { format!($($x)*)} else { "".to_string() }}}

        let opcode = self.memory.borrow_mut().read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let mut disasm: String = String::from("");
        match opcode {
            ACI => {
                let value = self.read_immediate_byte();
                self.addc(value);
                disasm = dbg!("{}ACI {:02X}H", self.code_to_str(2), value);
            }
            ADC_B => {
                let value = self.b;
                self.addc(value);
                disasm = dbg!("{}ADC B", self.code_to_str(1));
            }
            ADC_C => {
                let value = self.c;
                self.addc(value);
                disasm = dbg!("{}ADC C", self.code_to_str(1));
            }
            ADC_D => {
                let value = self.d;
                self.addc(value);
                disasm = dbg!("{}ADC D", self.code_to_str(1));
            }
            ADC_E => {
                let value = self.e;
                self.addc(value);
                disasm = dbg!("{}ADC E", self.code_to_str(1));
            }
            ADC_H => {
                let value = self.h;
                self.addc(value);
                disasm = dbg!("{}ADC H", self.code_to_str(1));
            }
            ADC_L => {
                let value = self.l;
                self.addc(value);
                disasm = dbg!("{}ADC L", self.code_to_str(1));
            }
            ADC_M => {
                let value = self.read_m();
                self.addc(value);
                disasm = dbg!("{}ADC M", self.code_to_str(1));
            }
            ADC_A => {
                let value = self.a;
                self.addc(value);
                disasm = dbg!("{}ADC A", self.code_to_str(1));
            }
            ADD_B => {
                let value = self.b;
                self.add(value, false);
                disasm = dbg!("{}ADD B", self.code_to_str(1));
            }
            ADD_C => {
                let value = self.c;
                self.add(value, false);
                disasm = dbg!("{}ADD C", self.code_to_str(1));
            }
            ADD_D => {
                let value = self.d;
                self.add(value, false);
                disasm = dbg!("{}ADD D", self.code_to_str(1));
            }
            ADD_E => {
                let value = self.e;
                self.add(value, false);
                disasm = dbg!("{}ADD E", self.code_to_str(1));
            }
            ADD_H => {
                let value = self.h;
                self.add(value, false);
                disasm = dbg!("{}ADD H", self.code_to_str(1));
            }
            ADD_L => {
                let value = self.l;
                self.add(value, false);
                disasm = dbg!("{}ADD L", self.code_to_str(1));
            }
            ADD_M => {
                let value = self.read_m();
                self.add(value, false);
                disasm = dbg!("{}ADD M", self.code_to_str(1));
            }
            ADD_A => {
                let value = self.a;
                self.add(value, false);
                disasm = dbg!("{}ADD A", self.code_to_str(1));
            }
            ADI => {
                let value = self.read_immediate_byte();
                self.add(value, false);
                disasm = dbg!("{}ADI {:02X}H", self.code_to_str(2), value);
            }
            ANA_B => {
                let value = self.b;
                self.and(value);
                disasm = dbg!("{}ANA B", self.code_to_str(1));
            }
            ANA_C => {
                let value = self.c;
                self.and(value);
                disasm = dbg!("{}ANA C", self.code_to_str(1));
            }
            ANA_D => {
                let value = self.d;
                self.and(value);
                disasm = dbg!("{}ANA D", self.code_to_str(1));
            }
            ANA_E => {
                let value = self.e;
                self.and(value);
                disasm = dbg!("{}ANA E", self.code_to_str(1));
            }
            ANA_H => {
                let value = self.h;
                self.and(value);
                disasm = dbg!("{}ANA H", self.code_to_str(1));
            }
            ANA_L => {
                let value = self.l;
                self.and(value);
                disasm = dbg!("{}ANA L", self.code_to_str(1));
            }
            ANA_M => {
                let value = self.read_m();
                self.and(value);
                disasm = dbg!("{}ANA M", self.code_to_str(1));
            }
            ANA_A => {
                let value = self.a;
                self.and(value);
                disasm = dbg!("{}ANA A", self.code_to_str(1));
            }
            ANI => {
                let value = self.read_immediate_byte();
                self.and(value);
                disasm = dbg!("{}ANI {:02X}H", self.code_to_str(2), value);
            }
            CMA => {
                self.a = !self.a;
                disasm = dbg!("{}CMA", self.code_to_str(1));
            }
            CMC => {
                self.status.set_carry(!self.status.is_carry());
                disasm = dbg!("{}CMC", self.code_to_str(1));
            }
            CMP_B => {
                let tmp = self.a;
                self.sub(self.b, false);
                self.a = tmp;
                disasm = dbg!("{}CMP B", self.code_to_str(1));
            }
            CMP_C => {
                let tmp = self.a;
                self.sub(self.c, false);
                self.a = tmp;
                disasm = dbg!("{}CMP C", self.code_to_str(1));
            }
            CMP_D => {
                let tmp = self.a;
                self.sub(self.d, false);
                self.a = tmp;
                disasm = dbg!("{}CMP D", self.code_to_str(1));
            }
            CMP_E => {
                let tmp = self.a;
                self.sub(self.e, false);
                self.a = tmp;
                disasm = dbg!("{}CMP E", self.code_to_str(1));
            }
            CMP_H => {
                let tmp = self.a;
                self.sub(self.h, false);
                self.a = tmp;
                disasm = dbg!("{}CMP H", self.code_to_str(1));
            }
            CMP_L => {
                let tmp = self.a;
                self.sub(self.l, false);
                self.a = tmp;
                disasm = dbg!("{}CMP L", self.code_to_str(1));
            }
            CMP_M => {
                let tmp = self.a;
                let val = self.read_m();
                self.sub(val, false);
                self.a = tmp;
                disasm = dbg!("{}CMP M", self.code_to_str(1));
            }
            CMP_A => {
                let tmp = self.a;
                self.sub(self.a, false);
                self.a = tmp;
                disasm = dbg!("{}CMP A", self.code_to_str(1));
            }
            CPI => {
                let tmp = self.a;
                let data = self.read_immediate_byte();
                self.sub(data, false);
                self.a = tmp;
                disasm = dbg!("{}CPI {:02X}H", self.code_to_str(2), data);
            }
            CALL => {
                if self.debug {
                    let addr = self.memory.borrow_mut().read_word(self.pc);
                    self.pc = self.pc.wrapping_add(2);
                    let code = self.code_to_str(3);
                    self.pc = self.pc.wrapping_sub(2);
                    disasm = dbg!("{}CALL {:04X}H", code, addr);
                }
                self.call();
            }
            CNZ => {
                if !self.status.is_zero() {
                    if self.debug {
                        let addr = self.memory.borrow_mut().read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        disasm = dbg!("{}CNZ {:04X}H", code, addr);
                    }
                    self.call();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            CZ => {
                if self.status.is_zero() {
                    if self.debug {
                        let addr = self.memory.borrow_mut().read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        disasm = dbg!("{}CZ {:04X}H", code, addr);
                    }
                    self.call();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            CNC => {
                if !self.status.is_carry() {
                    if self.debug {
                        let addr = self.memory.borrow_mut().read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        disasm = dbg!("{}CNC {:04X}H", code, addr);
                    }
                    self.call();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            CC => {
                if self.status.is_carry() {
                    if self.debug {
                        let addr = self.memory.borrow_mut().read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        disasm = dbg!("{}CC {:04X}H", code, addr);
                    }
                    self.call();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            CPO => {
                if !self.status.is_parity() {
                    if self.debug {
                        let addr = self.memory.borrow_mut().read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        disasm = dbg!("{}CPO {:04X}H", code, addr);
                    }
                    self.call();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            CPE => {
                if self.status.is_parity() {
                    if self.debug {
                        let addr = self.memory.borrow_mut().read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        disasm = dbg!("{}CPE {:04X}H", code, addr);
                    }
                    self.call();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            CP => {
                if !self.status.is_negative() {
                    if self.debug {
                        let addr = self.memory.borrow_mut().read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        disasm = dbg!("{}CP {:04X}H", code, addr);
                    }
                    self.call();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            CM => {
                if self.status.is_negative() {
                    if self.debug {
                        let addr = self.memory.borrow_mut().read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        disasm = dbg!("{}CM {:04X}H", code, addr);
                    }
                    self.call();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            DAA => {
                self.daa();
                disasm = dbg!("{}DAA", self.code_to_str(1));
            }
            DAD_B => {
                self.dad(self.get_bc());
                disasm = dbg!("{}DAD B", self.code_to_str(1));
            }
            DAD_D => {
                self.dad(self.get_de());
                disasm = dbg!("{}DAD D", self.code_to_str(1));
            }
            DAD_H => {
                self.dad(self.get_hl());
                disasm = dbg!("{}DAD H", self.code_to_str(1));
            }
            DAD_SP => {
                self.dad(self.sp);
                disasm = dbg!("{}DAD SP", self.code_to_str(1));
            }
            DCR_B => {
                self.b = self.dcr(self.b);
                disasm = dbg!("{}DCR B", self.code_to_str(1));
            }
            DCR_C => {
                self.c = self.dcr(self.c);
                disasm = dbg!("{}DCR C", self.code_to_str(1));
            }
            DCR_D => {
                self.d = self.dcr(self.d);
                disasm = dbg!("{}DCR D", self.code_to_str(1));
            }
            DCR_E => {
                self.e = self.dcr(self.e);
                disasm = dbg!("{}DCR E", self.code_to_str(1));
            }
            DCR_H => {
                self.h = self.dcr(self.h);
                disasm = dbg!("{}DCR H", self.code_to_str(1));
            }
            DCR_L => {
                self.l = self.dcr(self.l);
                disasm = dbg!("{}DCR L", self.code_to_str(1));
            }
            DCR_M => {
                let mut value = self.read_m();
                value = self.dcr(value);
                self.store_m(value);
                disasm = dbg!("{}DCR M", self.code_to_str(1));
            }
            DCR_A => {
                self.a = self.dcr(self.a);
                disasm = dbg!("{}DCR A", self.code_to_str(1));
            }
            DCX_B => {
                self.set_bc(self.get_bc().wrapping_sub(1));
                disasm = dbg!("{}DCX B", self.code_to_str(1));
            }
            DCX_D => {
                self.set_de(self.get_de().wrapping_sub(1));
                disasm = dbg!("{}DCX D", self.code_to_str(1));
            }
            DCX_H => {
                self.set_hl(self.get_hl().wrapping_sub(1));
                disasm = dbg!("{}DCX H", self.code_to_str(1));
            }
            DCX_SP => {
                self.sp = self.sp.wrapping_sub(1);
                disasm = dbg!("{}DCX SP", self.code_to_str(1));
            }
            DI => {
                self.inte = false;
                disasm = dbg!("{}EI", self.code_to_str(1));
            }
            EI => {
                self.inte = true;
                disasm = dbg!("{}EI", self.code_to_str(1));
            }
            HLT => {
                disasm = dbg!("{}HLT", self.code_to_str(1));
            }
            IN => {
                let addr = self.read_immediate_byte();
                self.a = self.inp(addr);
                disasm = dbg!("{}IN {:02X}H", self.code_to_str(2), addr);
            }
            INR_B => {
                self.b = self.inr(self.b);
                disasm = dbg!("{}INR B", self.code_to_str(1));
            }
            INR_C => {
                self.c = self.inr(self.c);
                disasm = dbg!("{}INR C", self.code_to_str(1));
            }
            INR_D => {
                self.d = self.inr(self.d);
                disasm = dbg!("{}INR D", self.code_to_str(1));
            }
            INR_E => {
                self.e = self.inr(self.e);
                disasm = dbg!("{}INR D", self.code_to_str(1));
            }
            INR_H => {
                self.h = self.inr(self.h);
                disasm = dbg!("{}INR H", self.code_to_str(1));
            }
            INR_L => {
                self.l = self.inr(self.l);
                disasm = dbg!("{}INR L", self.code_to_str(1));
            }
            INR_M => {
                let mut value = self.read_m();
                value = self.inr(value);
                self.store_m(value);
                disasm = dbg!("{}INR M", self.code_to_str(1));
            }
            INR_A => {
                self.a = self.inr(self.a);
                disasm = dbg!("{}INR A", self.code_to_str(1));
            }
            INX_B => {
                self.set_bc(self.get_bc().wrapping_add(1));
                disasm = dbg!("{}INX B", self.code_to_str(1));
            }
            INX_D => {
                self.set_de(self.get_de().wrapping_add(1));
                disasm = dbg!("{}INX D", self.code_to_str(1));
            }
            INX_H => {
                self.set_hl(self.get_hl().wrapping_add(1));
                disasm = dbg!("{}INX H", self.code_to_str(1));
            }
            INX_SP => {
                self.sp = self.sp.wrapping_add(1);
                disasm = dbg!("{}INX SP", self.code_to_str(1));
            }
            JNZ => {
                if !self.status.is_zero() {
                    if self.debug {
                        let addr = self.memory.borrow_mut().read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        disasm = dbg!("{}JNZ {:04X}H", code, addr);
                    }
                    self.jmp();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            JZ => {
                if self.status.is_zero() {
                    if self.debug {
                        let addr = self.memory.borrow_mut().read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        disasm = dbg!("{}JZ {:04X}H", code, addr);
                    }
                    self.jmp();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            JNC => {
                if !self.status.is_carry() {
                    if self.debug {
                        let addr = self.memory.borrow_mut().read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        disasm = dbg!("{}JNC {:04X}H", code, addr);
                    }
                    self.jmp();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            JC => {
                if self.status.is_carry() {
                    if self.debug {
                        let addr = self.memory.borrow_mut().read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        disasm = dbg!("{}JC {:04X}H", code, addr);
                    }
                    self.jmp();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            JPO => {
                if !self.status.is_parity() {
                    if self.debug {
                        let addr = self.memory.borrow_mut().read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        disasm = dbg!("{}JPO {:04X}H", code, addr);
                    }
                    self.jmp();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            JPE => {
                if self.status.is_parity() {
                    if self.debug {
                        let addr = self.memory.borrow_mut().read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        disasm = dbg!("{}JPE {:04X}H", code, addr);
                    }
                    self.jmp();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            JP => {
                if !self.status.is_negative() {
                    if self.debug {
                        let addr = self.memory.borrow_mut().read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        disasm = dbg!("{}JP {:04X}H", code, addr);
                    }
                    self.jmp();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            JM => {
                if self.status.is_negative() {
                    if self.debug {
                        let addr = self.memory.borrow_mut().read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        disasm = dbg!("{}JM {:04X}H", code, addr);
                    }
                    self.jmp();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            JMP => {
                if self.debug {
                    let addr = self.memory.borrow_mut().read_word(self.pc);
                    self.pc = self.pc.wrapping_add(2);
                    let code = self.code_to_str(3);
                    self.pc = self.pc.wrapping_sub(2);
                    disasm = dbg!("{}JMP {:04X}H", code, addr);
                }
                self.jmp();
            }
            LDA => {
                let addr = self.read_immediate_word();
                self.a = self.memory.borrow_mut().read_byte(addr);
                disasm = dbg!("{}LDA {:04X}H", self.code_to_str(3), addr);
            }
            LDAX_B => {
                let addr = self.get_bc();
                self.a = self.memory.borrow_mut().read_byte(addr);
                disasm = dbg!("{}LDAX B", self.code_to_str(1));
            }
            LDAX_D => {
                let addr = self.get_de();
                self.a = self.memory.borrow_mut().read_byte(addr);
                disasm = dbg!("{}LDAX B", self.code_to_str(1));
            }
            LHLD => {
                let addr = self.read_immediate_word();
                self.l = self.memory.borrow_mut().read_byte(addr);
                self.h = self.memory.borrow_mut().read_byte(addr + 1);
                disasm = dbg!("{}LHLD {:04X}H", self.code_to_str(3), addr);
            }
            LXI_B => {
                let word = self.read_immediate_word();
                self.set_bc(word);
                disasm = dbg!("{}LXI B {:04X}H", self.code_to_str(3), word);
            }
            LXI_D => {
                let word = self.read_immediate_word();
                self.set_de(word);
                disasm = dbg!("{}LXI D {:04X}H", self.code_to_str(3), word);
            }
            LXI_H => {
                let word = self.read_immediate_word();
                self.set_hl(word);
                disasm = dbg!("{}LXI H {:04X}H", self.code_to_str(3), word);
            }
            LXI_SP => {
                self.sp = self.read_immediate_word();
                disasm = dbg!("{}LXI SP {:04X}H", self.code_to_str(3), self.sp);
            }
            MVI_A => {
                let value = self.read_immediate_byte();
                self.a = value;
                disasm = dbg!("{}MVI A,{:02X}H", self.code_to_str(2), value);
            }
            MVI_B => {
                let value = self.read_immediate_byte();
                self.b = value;
                disasm = dbg!("{}MVI B,{:02X}H", self.code_to_str(2), value);
            }
            MVI_C => {
                let value = self.read_immediate_byte();
                self.c = value;
                disasm = dbg!("{}MVI C,{:02X}H", self.code_to_str(2), value);
            }
            MVI_D => {
                let value = self.read_immediate_byte();
                self.d = value;
                disasm = dbg!("{}MVI D,{:02X}H", self.code_to_str(2), value);
            }
            MVI_E => {
                let value = self.read_immediate_byte();
                self.e = value;
                disasm = dbg!("{}MVI E,{:02X}H", self.code_to_str(2), value);
            }
            MVI_H => {
                let value = self.read_immediate_byte();
                self.h = value;
                disasm = dbg!("{}MVI H,{:02X}H", self.code_to_str(2), value);
            }
            MVI_L => {
                let value = self.read_immediate_byte();
                self.l = value;
                disasm = dbg!("{}MVI L,{:02X}H", self.code_to_str(2), value);
            }
            MVI_M => {
                let addr = self.get_hl();
                let value = self.read_immediate_byte();
                self.memory.borrow_mut().write_byte(addr, value);
                disasm = dbg!("{}MVI M,{:02X}H", self.code_to_str(2), value);
            }
            MOV_A_B => {
                self.a = self.b;
                disasm = dbg!("{}MOV A,B", self.code_to_str(1));
            }
            MOV_A_C => {
                self.a = self.c;
                disasm = dbg!("{}MOV A,C", self.code_to_str(1));
            }
            MOV_A_D => {
                self.a = self.d;
                disasm = dbg!("{}MOV A,D", self.code_to_str(1));
            }
            MOV_A_E => {
                self.a = self.e;
                disasm = dbg!("{}MOV A,E", self.code_to_str(1));
            }
            MOV_A_H => {
                self.a = self.h;
                disasm = dbg!("{}MOV A,H", self.code_to_str(1));
            }
            MOV_A_L => {
                self.a = self.l;
                disasm = dbg!("{}MOV A,L", self.code_to_str(1));
            }
            MOV_A_M => {
                self.a = self.memory.borrow_mut().read_byte(self.get_hl());
                disasm = dbg!("{}MOV A,M", self.code_to_str(1));
            }
            MOV_A_A => {
                disasm = dbg!("{}MOV A,A", self.code_to_str(1));
            }
            MOV_B_B => {
                disasm = dbg!("{}MOV B,B", self.code_to_str(1));
            }
            MOV_B_C => {
                self.b = self.c;
                disasm = dbg!("{}MOV B,C", self.code_to_str(1));
            }
            MOV_B_D => {
                self.b = self.d;
                disasm = dbg!("{}MOV B,D", self.code_to_str(1));
            }
            MOV_B_E => {
                self.b = self.e;
                disasm = dbg!("{}MOV B,E", self.code_to_str(1));
            }
            MOV_B_H => {
                self.b = self.h;
                disasm = dbg!("{}MOV B,H", self.code_to_str(1));
            }
            MOV_B_L => {
                self.b = self.l;
                disasm = dbg!("{}MOV B,L", self.code_to_str(1));
            }
            MOV_B_M => {
                self.b = self.memory.borrow_mut().read_byte(self.get_hl());
                disasm = dbg!("{}MOV B,M", self.code_to_str(1));
            }
            MOV_B_A => {
                self.b = self.a;
                disasm = dbg!("{}MOV B,A", self.code_to_str(1));
            }
            MOV_C_B => {
                self.c = self.b;
                disasm = dbg!("{}MOV C,B", self.code_to_str(1));
            }
            MOV_C_C => {
                disasm = dbg!("{}MOV C,C", self.code_to_str(1));
            }
            MOV_C_D => {
                self.c = self.d;
                disasm = dbg!("{}MOV C,D", self.code_to_str(1));
            }
            MOV_C_E => {
                self.c = self.e;
                disasm = dbg!("{}MOV C,E", self.code_to_str(1));
            }
            MOV_C_H => {
                self.c = self.h;
                disasm = dbg!("{}MOV C,H", self.code_to_str(1));
            }
            MOV_C_L => {
                self.c = self.l;
                disasm = dbg!("{}MOV C,L", self.code_to_str(1));
            }
            MOV_C_M => {
                self.c = self.memory.borrow_mut().read_byte(self.get_hl());
                disasm = dbg!("{}MOV C,M", self.code_to_str(1));
            }
            MOV_C_A => {
                self.c = self.a;
                disasm = dbg!("{}MOV C,A", self.code_to_str(1));
            }
            MOV_D_B => {
                self.d = self.b;
                disasm = dbg!("{}MOV D,B", self.code_to_str(1));
            }
            MOV_D_C => {
                self.d = self.c;
                disasm = dbg!("{}MOV D,C", self.code_to_str(1));
            }
            MOV_D_D => {
                disasm = dbg!("{}MOV D,D", self.code_to_str(1));
            }
            MOV_D_E => {
                self.d = self.e;
                disasm = dbg!("{}MOV D,E", self.code_to_str(1));
            }
            MOV_D_H => {
                self.d = self.h;
                disasm = dbg!("{}MOV D,H", self.code_to_str(1));
            }
            MOV_D_L => {
                self.d = self.l;
                disasm = dbg!("{}MOV D,L", self.code_to_str(1));
            }
            MOV_D_M => {
                self.d = self.memory.borrow_mut().read_byte(self.get_hl());
                disasm = dbg!("{}MOV D,M", self.code_to_str(1));
            }
            MOV_D_A => {
                self.d = self.a;
                disasm = dbg!("{}MOV D,A", self.code_to_str(1));
            }
            MOV_E_B => {
                self.e = self.b;
                disasm = dbg!("{}MOV E,B", self.code_to_str(1));
            }
            MOV_E_C => {
                self.e = self.c;
                disasm = dbg!("{}MOV E,C", self.code_to_str(1));
            }
            MOV_E_D => {
                self.e = self.d;
                disasm = dbg!("{}MOV E,D", self.code_to_str(1));
            }
            MOV_E_E => {
                disasm = dbg!("{}MOV E,E", self.code_to_str(1));
            }
            MOV_E_H => {
                self.e = self.h;
                disasm = dbg!("{}MOV E,H", self.code_to_str(1));
            }
            MOV_E_L => {
                self.e = self.l;
                disasm = dbg!("{}MOV E,L", self.code_to_str(1));
            }
            MOV_E_M => {
                self.e = self.memory.borrow_mut().read_byte(self.get_hl());
                disasm = dbg!("{}MOV E,M", self.code_to_str(1));
            }
            MOV_E_A => {
                self.e = self.a;
                disasm = dbg!("{}MOV E,A", self.code_to_str(1));
            }
            MOV_H_B => {
                self.h = self.b;
                disasm = dbg!("{}MOV H,B", self.code_to_str(1));
            }
            MOV_H_C => {
                self.h = self.c;
                disasm = dbg!("{}MOV H,C", self.code_to_str(1));
            }
            MOV_H_D => {
                self.h = self.d;
                disasm = dbg!("{}MOV H,D", self.code_to_str(1));
            }
            MOV_H_E => {
                self.h = self.e;
                disasm = dbg!("{}MOV H,E", self.code_to_str(1));
            }
            MOV_H_H => {
                disasm = dbg!("{}MOV H,H", self.code_to_str(1));
            }
            MOV_H_L => {
                self.h = self.l;
                disasm = dbg!("{}MOV H,L", self.code_to_str(1));
            }
            MOV_H_M => {
                self.h = self.memory.borrow_mut().read_byte(self.get_hl());
                disasm = dbg!("{}MOV H,M", self.code_to_str(1));
            }
            MOV_H_A => {
                self.h = self.a;
                disasm = dbg!("{}MOV H,A", self.code_to_str(1));
            }
            MOV_L_B => {
                self.l = self.b;
                disasm = dbg!("{}MOV L,B", self.code_to_str(1));
            }
            MOV_L_C => {
                self.l = self.c;
                disasm = dbg!("{}MOV L,C", self.code_to_str(1));
            }
            MOV_L_D => {
                self.l = self.d;
                disasm = dbg!("{}MOV L,D", self.code_to_str(1));
            }
            MOV_L_E => {
                self.l = self.e;
                disasm = dbg!("{}MOV L,E", self.code_to_str(1));
            }
            MOV_L_H => {
                self.l = self.h;
                disasm = dbg!("{}MOV L,H", self.code_to_str(1));
            }
            MOV_L_L => {
                disasm = dbg!("{}MOV L,L", self.code_to_str(1));
            }
            MOV_L_M => {
                self.l = self.memory.borrow_mut().read_byte(self.get_hl());
                disasm = dbg!("{}MOV L,M", self.code_to_str(1));
            }
            MOV_L_A => {
                self.l = self.a;
                disasm = dbg!("{}MOV L,A", self.code_to_str(1));
            }
            MOV_M_B => {
                let addr = self.get_hl();
                self.memory.borrow_mut().write_byte(addr, self.b);
                disasm = dbg!("{}MOV M,B", self.code_to_str(1));
            }
            MOV_M_C => {
                let addr = self.get_hl();
                self.memory.borrow_mut().write_byte(addr, self.c);
                disasm = dbg!("{}MOV M,C", self.code_to_str(1));
            }
            MOV_M_D => {
                let addr = self.get_hl();
                self.memory.borrow_mut().write_byte(addr, self.d);
                disasm = dbg!("{}MOV M,D", self.code_to_str(1));
            }
            MOV_M_E => {
                let addr = self.get_hl();
                self.memory.borrow_mut().write_byte(addr, self.e);
                disasm = dbg!("{}MOV M,E", self.code_to_str(1));
            }
            MOV_M_H => {
                let addr = self.get_hl();
                self.memory.borrow_mut().write_byte(addr, self.h);
                disasm = dbg!("{}MOV M,H", self.code_to_str(1));
            }
            MOV_M_L => {
                let addr = self.get_hl();
                self.memory.borrow_mut().write_byte(addr, self.l);
                disasm = dbg!("{}MOV M,L", self.code_to_str(1));
            }
            MOV_M_A => {
                let addr = self.get_hl();
                self.memory.borrow_mut().write_byte(addr, self.a);
                disasm = dbg!("{}MOV M,A", self.code_to_str(1));
            }
            NOP => {
                disasm = dbg!("{}NOP", self.code_to_str(1));
            }
            ORA_B => {
                let value = self.b;
                self.or(value);
                disasm = dbg!("{}ORA B", self.code_to_str(1));
            }
            ORA_C => {
                let value = self.c;
                self.or(value);
                disasm = dbg!("{}ORA C", self.code_to_str(1));
            }
            ORA_D => {
                let value = self.d;
                self.or(value);
                disasm = dbg!("{}ORA D", self.code_to_str(1));
            }
            ORA_E => {
                let value = self.e;
                self.or(value);
                disasm = dbg!("{}ORA E", self.code_to_str(1));
            }
            ORA_H => {
                let value = self.h;
                self.or(value);
                disasm = dbg!("{}ORA H", self.code_to_str(1));
            }
            ORA_L => {
                let value = self.l;
                self.or(value);
                disasm = dbg!("{}ORA L", self.code_to_str(1));
            }
            ORA_M => {
                let value = self.memory.borrow_mut().read_byte(self.get_hl());
                self.or(value);
                disasm = dbg!("{}ORA M", self.code_to_str(1));
            }
            ORA_A => {
                let value = self.a;
                self.or(value);
                disasm = dbg!("{}ORA A", self.code_to_str(1));
            }
            ORI => {
                let value = self.read_immediate_byte();
                self.or(value);
                disasm = dbg!("{}ORI {:02X}H", self.code_to_str(2), value);
            }
            OUT => {
                let addr = self.read_immediate_byte();
                self.out(addr);
                disasm = dbg!("{}OUT {:02X}H", self.code_to_str(2), addr);
            }
            PCHL => {
                let hl = self.get_hl();
                self.set_hl(self.pc);
                self.pc = hl;
                disasm = dbg!("{}PCHL", self.code_to_str(1));
            }
            POP_B => {
                (self.b, self.c) = self.pop();
                disasm = dbg!("{}POP B", self.code_to_str(1));
            }
            POP_D => {
                (self.d, self.e) = self.pop();
                disasm = dbg!("{}POP D", self.code_to_str(1));
            }
            POP_H => {
                (self.h, self.l) = self.pop();
                disasm = dbg!("{}POP H", self.code_to_str(1));
            }
            POP_PSW => {
                let mut addr = self.sp;
                let value = self.memory.borrow_mut().read_byte(addr);
                self.status.set_negative((value & SIGN) != 0);
                self.status.set_zero((value & ZERO) != 0);
                self.status.set_ac((value & AUX_CARRY) != 0);
                self.status.set_parity((value & PARITY) != 0);
                self.status.set_carry((value & CARRY) != 0);
                addr = addr.wrapping_add(1);
                self.a = self.memory.borrow_mut().read_byte(addr);
                self.sp = self.sp.wrapping_add(2);
                disasm = dbg!("{}POP PSW", self.code_to_str(1));
            }
            PUSH_B => {
                self.push(self.b, self.c);
                disasm = dbg!("{}PUSH B", self.code_to_str(1));
            }
            PUSH_D => {
                self.push(self.d, self.e);
                disasm = dbg!("{}PUSH D", self.code_to_str(1));
            }
            PUSH_H => {
                self.push(self.h, self.l);
                disasm = dbg!("{}PUSH H", self.code_to_str(1));
            }
            PUSH_PSW => {
                self.push(self.a, self.status.value);
                disasm = dbg!("{}PUSH PSW", self.code_to_str(1));
            }
            RAL => {
                let mut val = (self.a as u16) << 1;
                if self.status.is_carry() {
                    val |= 0b1u16;
                }
                self.status.set_carry((self.a & 0x80) != 0);
                self.a = val as u8;
                disasm = dbg!("{}RAL", self.code_to_str(1));
            }
            RAR => {
                let mut val = (self.a as u16) >> 1;
                if self.status.is_carry() {
                    val |= 0b1000_0000u16;
                }
                self.status.set_carry((self.a & 0x01) != 0);
                self.a = val as u8;
                disasm = dbg!("{}RAR", self.code_to_str(1));
            }
            RLC => {
                let mut val = (self.a as u16) << 1;
                if self.a & 0x80 != 0 {
                    val |= 0b1u16;
                }
                self.status.set_carry(self.a & 0x80 != 0);
                self.a = val as u8;
                disasm = dbg!("{}RLC", self.code_to_str(1));
            }
            RRC => {
                let mut val = (self.a as u16) >> 1;
                if self.a & 0x01 != 0 {
                    val |= 0b1000_0000u16;
                }
                self.status.set_carry(self.a & 0x01 != 0);
                self.a = val as u8;
                disasm = dbg!("{}RRC", self.code_to_str(1));
            }
            RET => {
                disasm = dbg!("{}RET", self.code_to_str(1));
                self.ret();
            }
            RNZ => {
                if !self.status.is_zero() {
                    disasm = dbg!("{}RNZ", self.code_to_str(1));
                    self.ret();
                }
            }
            RZ => {
                if self.status.is_zero() {
                    disasm = dbg!("{}RZ", self.code_to_str(1));
                    self.ret();
                }
            }
            RNC => {
                if !self.status.is_carry() {
                    disasm = dbg!("{}RNC", self.code_to_str(1));
                    self.ret();
                }
            }
            RC => {
                if self.status.is_carry() {
                    disasm = dbg!("{}RC", self.code_to_str(1));
                    self.ret();
                }
            }
            RPO => {
                if !self.status.is_parity() {
                    disasm = dbg!("{}RPO", self.code_to_str(1));
                    self.ret();
                }
            }
            RPE => {
                if self.status.is_parity() {
                    disasm = dbg!("{}RPE", self.code_to_str(1));
                    self.ret();
                }
            }
            RP => {
                if !self.status.is_negative() {
                    disasm = dbg!("{}RP", self.code_to_str(1));
                    self.ret();
                }
            }
            RM => {
                if self.status.is_negative() {
                    disasm = dbg!("{}RM", self.code_to_str(1));
                    self.ret();
                }
            }
            RST_0 => {
                disasm = dbg!("{}RST 0", self.code_to_str(1));
                self.rst(0);
            }
            RST_1 => {
                disasm = dbg!("{}RST 1", self.code_to_str(1));
                self.rst(1);
            }
            RST_2 => {
                disasm = dbg!("{}RST 2", self.code_to_str(1));
                self.rst(2);
            }
            RST_3 => {
                disasm = dbg!("{}RST 3", self.code_to_str(1));
                self.rst(3);
            }
            RST_4 => {
                disasm = dbg!("{}RST 4", self.code_to_str(1));
                self.rst(4);
            }
            RST_5 => {
                disasm = dbg!("{}RST 5", self.code_to_str(1));
                self.rst(5);
            }
            RST_6 => {
                disasm = dbg!("{}RST 6", self.code_to_str(1));
                self.rst(6);
            }
            RST_7 => {
                disasm = dbg!("{}RST 7", self.code_to_str(1));
                self.rst(7);
            }
            SBB_B => {
                let value = self.b;
                self.sub(value, true);
                disasm = dbg!("{}SBB B", self.code_to_str(1));
            }
            SBB_C => {
                let value = self.c;
                self.sub(value, true);
                disasm = dbg!("{}SBB C", self.code_to_str(1));
            }
            SBB_D => {
                let value = self.d;
                self.sub(value, true);
                disasm = dbg!("{}SBB D", self.code_to_str(1));
            }
            SBB_E => {
                let value = self.e;
                self.sub(value, true);
                disasm = dbg!("{}SBB E", self.code_to_str(1));
            }
            SBB_H => {
                let value = self.h;
                self.sub(value, true);
                disasm = dbg!("{}SBB H", self.code_to_str(1));
            }
            SBB_L => {
                let value = self.l;
                self.sub(value, true);
                disasm = dbg!("{}SBB L", self.code_to_str(1));
            }
            SBB_M => {
                let value = self.get_m();
                self.sub(value, true);
                disasm = dbg!("{}SBB M", self.code_to_str(1));
            }
            SBB_A => {
                let value = self.a;
                self.sub(value, true);
                disasm = dbg!("{}SBB A", self.code_to_str(1));
            }
            SBI => {
                let value = self.read_immediate_byte();
                self.sub(value, true);
                disasm = dbg!("{}SBI {:02X}H", self.code_to_str(2), value);
            }
            SUB_B => {
                let value = self.b;
                self.sub(value, false);
                disasm = dbg!("{}SUB B", self.code_to_str(1));
            }
            SUB_C => {
                let value = self.c;
                self.sub(value, false);
                disasm = dbg!("{}SUB C", self.code_to_str(1));
            }
            SUB_D => {
                let value = self.d;
                self.sub(value, false);
                disasm = dbg!("{}SUB D", self.code_to_str(1));
            }
            SUB_E => {
                let value = self.e;
                self.sub(value, false);
                disasm = dbg!("{}SUB E", self.code_to_str(1));
            }
            SUB_H => {
                let value = self.h;
                self.sub(value, false);
                disasm = dbg!("{}SUB H", self.code_to_str(1));
            }
            SUB_L => {
                let value = self.l;
                self.sub(value, false);
                disasm = dbg!("{}SUB L", self.code_to_str(1));
            }
            SUB_M => {
                let value = self.read_m();
                self.sub(value, false);
                disasm = dbg!("{}SUB M", self.code_to_str(1));
            }
            SUB_A => {
                let value = self.a;
                self.sub(value, false);
                disasm = dbg!("{}SUB A", self.code_to_str(1));
            }
            SUI => {
                let value = self.read_immediate_byte();
                self.sub(value, false);
                disasm = dbg!("{}SUI {:02X}H", self.code_to_str(2), value);
            }
            SHLD => {
                let addr = self.read_immediate_word();
                self.memory.borrow_mut().write_byte(addr, self.l);
                self.memory.borrow_mut().write_byte(addr.wrapping_add(1), self.h);
               disasm = dbg!("{}SHLD {:04X}H", self.code_to_str(3), addr);
            }
            STA => {
                let addr = self.read_immediate_word();
                self.memory.borrow_mut().write_byte(addr, self.a);
                disasm = dbg!("{}STA {:04X}H", self.code_to_str(3), addr);
            }
            STAX_B => {
                let addr = self.get_bc();
                self.memory.borrow_mut().write_byte(addr, self.a);
                disasm = dbg!("{}STAX B", self.code_to_str(1));
            }
            STAX_D => {
                let addr = self.get_de();
                self.memory.borrow_mut().write_byte(addr, self.a);
                disasm = dbg!("{}STAX B", self.code_to_str(1));
            }
            STC => {
                self.status.set_carry(true);
                disasm = dbg!("{}STC", self.code_to_str(1));
            }
            SPHL => {
                self.sp = self.get_hl();
                disasm = dbg!("{}SPHL", self.code_to_str(1));
            }
            XCHG => {
                let temp = self.get_hl();
                self.set_hl(self.get_de());
                self.set_de(temp);
                disasm = dbg!("{}XCHG", self.code_to_str(1));
            }
            XRA_B => {
                let value = self.b;
                self.xra(value);
                disasm = dbg!("{}XRA B", self.code_to_str(1));
            }
            XRA_C => {
                let value = self.c;
                self.xra(value);
                disasm = dbg!("{}XRA C", self.code_to_str(1));
            }
            XRA_D => {
                let value = self.d;
                self.xra(value);
                disasm = dbg!("{}XRA D", self.code_to_str(1));
            }
            XRA_E => {
                let value = self.e;
                self.xra(value);
                disasm = dbg!("{}XRA E", self.code_to_str(1));
            }
            XRA_H => {
                let value = self.h;
                self.xra(value);
                disasm = dbg!("{}XRA H", self.code_to_str(1));
            }
            XRA_L => {
                let value = self.l;
                self.xra(value);
                disasm = dbg!("{}XRA L", self.code_to_str(1));
            }
            XRA_M => {
                let value = self.get_m();
                self.xra(value);
                disasm = dbg!("{}XRA M", self.code_to_str(1));
            }
            XRA_A => {
                let value = self.a;
                self.xra(value);
                disasm = dbg!("{}XRA A", self.code_to_str(1));
            }
            XRI => {
                let value = self.read_immediate_byte();
                self.xra(value);
                disasm = dbg!("{}XRI {:02X}", self.code_to_str(2), value);
            }
            XTHL => {
                let addr = self.sp;
                let hl = self.get_hl();
                self.l = self.memory.borrow_mut().read_byte(addr);
                self.h = self.memory.borrow_mut().read_byte(addr + 1);
                self.memory.borrow_mut().write_word(addr, hl);
                disasm = dbg!("{}XTHL", self.code_to_str(1));
            }

            _ => {
                disasm = dbg!("{}DB {:02X}H", self.code_to_str(1), opcode);
            }
        }
        match disasm.as_str() {
            "" => None,
            _ => Some(disasm),
        }
    }
}

impl CpuUi for Cpu {
    fn memory_dump(&mut self, start: u16, end: u16) -> Vec<String> {
        self.memory.borrow_mut().hex_dump(start, end)
    }
    fn get_memory(&mut self) -> RefMut<'_, Memory> {
        self.memory.borrow_mut()
    }
    fn get_memory_ref(&mut self) -> Rc<RefCell<Memory>> {
        *self.memory.clone()
    }
    fn get_io_memory(&mut self) -> Option<&mut io::memory::IoMemory> {
        Some(&mut self.io_memory)
    }
    fn disasm(&mut self, start: u16, end: u16) -> Vec<String> {
        disassemble(&mut self.get_memory(), start, end, &load_opcodes_table())
    }
    fn show_registers(&mut self) -> Vec<String> {
        self.get_registers().lines().map(String::from).collect()
    }
    fn set_register_by_name(&mut self, reg: &str, value: u16) -> Result<(), String> {
        self.set_register_by_name(reg, value)
    }
    fn get_register_by_name(&mut self, reg: &str) -> Result<String, String> {
        match self.get_register(reg) {
            Ok(Reg::R8(val)) => Ok(format!("{reg}: {:02X}H [{val}]", val)),
            Ok(Reg::R16(val)) => Ok(format!("{reg}: {:04X}H [{val}]", val)),
            Err(err) => Err(err),
        }
    }
    fn get_breakpoints(&self) -> Result<Vec<u16>, String> {
        Ok(self.breakpoints.get_breakpoints())
    }
    fn set_breakpoints(&mut self, address: u16) -> Result<(), String> {
        let breakpoints = &mut self.breakpoints;
        breakpoints.set_breakpoint(address)
    }
    fn clear_breakpoints(&mut self) -> Result<(), String> {
        self.breakpoints.clear_breakpoints()?;
        Ok(())
    }
    fn get_cpu_name(&self) -> Option<&str> {
        Some("Intel 8080")
    }
    fn one_step(&mut self) -> Option<String> {
        self.step()
    }
    fn get_pc(&mut self) -> u16 {
        self.pc
    }
    fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }
    fn get_debug_flag(&self) -> bool {
        self.debug
    }
    fn set_debug_flag(&mut self, debug: bool) {
        self.debug = debug;
    }
    fn io_read(&mut self, address: u8) -> u8 {
        let reg_a = self.a;
        let res = self.inp(address);
        self.a = reg_a;
        res
    }
    fn io_write(&mut self, address: u8, data: u8) {
        let reg_a = self.a;
        self.a = data;
        self.out(address);
        self.a = reg_a;        
    }
    fn set_bootloader(&mut self, bootloader: Bootloader) {
        self.bootloader = Some(bootloader);
    }
    fn get_bootloader(&mut self) -> Option<Bootloader> {
        self.bootloader.clone()
    }
    fn get_hlt(&self) -> u8 {
        self.hlt_code
    }
    /// Reset the CPU
    /// 
    /// Resets the CPU and if bootloader is not specified, sets PC to 0x0000 and starts program.
    /// If bootloader is specified, load bootloader to memory and starts bootloader
    fn reset(&mut self) -> Result<(), String> {
        let pc: u16 = if self.bootloader.is_none() {
            0x0000u16
        }
        else {
            // Load bootloader and set pc to its start address
            let bootloader = self.bootloader.as_ref().unwrap();
            match self.memory.borrow_mut().load_data_from_intelhex_file(&bootloader.get_filename()){
                Ok(region) => {
                    region.start
                }
                Err(err) => {
                    return Err(err.to_string());
                }
            }
        };
        // And start the code 
        self.pc = pc;
        self.run(pc)?;
        Ok(())
    }
    /// Run from PC
    /// 
    /// Runs from PC. Breaks when it reaches predefined HLT instruction
    fn run(&mut self, pc: u16) -> Result<(), String> {
        self.pc = pc;
        loop {
            let opcode = self.memory.borrow_mut().read_byte(self.pc);
            self.step();
            if opcode == self.hlt_code {
                break;
            }
        }
        Ok(())
    }
}
