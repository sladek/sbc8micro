//! Provides a code for specific CPU

use crate::memory::Memory;
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
    fn get_memory(&mut self) -> &mut Memory;
    fn disasm(&mut self, start: u16, end: u16) -> Vec<String>;
    fn show_registers(&mut self) -> Vec<String>;
    fn set_register_by_name(&mut self, reg: &str, value: u16) -> Result<(), String>;
    fn get_register_by_name(&mut self, reg: &str) -> Result<String, String>;
    fn get_breakpoints(&self) -> Result<Vec<u16>, String>;
    fn set_breakpoints(&mut self, address: u16) -> Result<(), String>;
    fn clear_breakpoints(&mut self) -> Result<(), String>;
    fn get_cpu_name(&self) -> Option<&str>;
    fn one_step(&mut self) -> Option<String>;
}

pub enum Reg {
    R8(u8),
    R16(u16),
}
