//! Provides a code for specific CPU
use crate::io::memory;
use crate::memory::Memory;
use std::cell::RefMut;

pub mod i8080;
pub mod i8080_tests;
pub mod mos6502;
pub mod mos6502_tests;

#[derive(PartialEq, Clone)]
pub enum Cpu {
    None,
    I8080,
    Mos6502,
}

pub trait CpuUi {
    fn memory_dump(&mut self, start: u16, end: u16) -> Vec<String>;
    //    fn get_memory(&mut self) -> &mut Memory;
    fn get_memory(&mut self) -> RefMut<'_, Memory>;
    fn get_io_memory(&mut self) -> Option<&mut memory::IoMemory>;
    fn get_pc(&mut self) -> u16;
    fn set_pc(&mut self, pc: u16);
    fn disasm(&mut self, start: u16, end: u16) -> Vec<String>;
    fn print_disasm(&mut self, start: u16, end: u16) {
        let hex_dump = self.disasm(start, end);
        for line in hex_dump {
            println!("{line}");
        }
    }
    fn show_registers(&mut self) -> Vec<String>;
    fn set_register_by_name(&mut self, reg: &str, value: u16) -> Result<(), String>;
    fn get_register_by_name(&mut self, reg: &str) -> Result<String, String>;
    fn get_breakpoints(&self) -> Result<Vec<u16>, String>;
    fn set_breakpoints(&mut self, address: u16) -> Result<(), String>;
    fn clear_breakpoints(&mut self) -> Result<(), String>;
    fn get_cpu_name(&self) -> Option<&str>;
    fn one_step(&mut self) -> Option<String>;
    fn get_debug_flag(&self) -> bool;
    ///
    /// Sets debug flag
    ///
    /// If debug flag is set to true, then when stepping through instructions
    /// also mnemonic code of instruction is printed, which is very convenient
    /// during debugging of the programm
    ///
    fn set_debug_flag(&mut self, debug: bool);
}

pub enum Reg {
    R8(u8),
    R16(u16),
}
