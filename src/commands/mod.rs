//! Command processor
pub mod command;
pub mod directory;
pub mod disasm;
pub mod help;
pub mod load;
pub mod memory;
pub mod opcodes;
pub mod registers;
pub mod set;

pub const CPU_LIST: &str = "i8080 | 8080 | mos6502 | 6502";
