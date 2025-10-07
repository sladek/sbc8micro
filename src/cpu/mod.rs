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
}
