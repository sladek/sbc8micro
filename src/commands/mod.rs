//! Command processor

use crate::ui::app::AppState;
pub mod breakpoints;
pub mod command;
pub mod cpu;
pub mod dev;
pub mod directory;
pub mod disasm;
pub mod help;
pub mod load;
pub mod memory;
pub mod opcodes;
pub mod registers;
pub mod serial;
pub mod fdc;
pub mod fdhdc;
pub mod script;
pub mod io;
pub mod rdr_pun;
pub mod ihex;
pub mod bootloader;
pub mod hex_bin;

pub const CPU_LIST: &str = "i8080 | 8080 | mos6502 | 6502";
pub const MIN_MEMORY_RANGE: u16 = 1;
pub const MIN_DISASM_RANGE: u16 = 1;
pub const DEFAULT_DISASM_RANGE: u16 = 10;
pub const MIN_OUTPUT_HISTORY_LENGTH: u16 = 64;
pub const MIN_COMMAND_HISTORY_LENGTH: u16 = 5;

/// Error helper function. Just returns an error for non defined cpu.
pub fn cpu_not_set_error() -> std::result::Result<AppState, String> {
    Err(format!(
        "ERROR - Cpu is not defined. Use cpu <{CPU_LIST}> to set cpu."
    ))
}
