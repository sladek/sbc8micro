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

pub const CPU_LIST: &str = "i8080 | 8080 | mos6502 | 6502";
pub const MIN_MEMORY_RANGE: u16 = 16;
pub const MIN_DISASM_RANGE: u16 = 16;
pub const MIN_OUTPUT_HISTORY_LENGTH: u16 = 64;
pub const MIN_COMMAND_HISTORY_LENGTH: u16 = 5;

/// Error helper function. Just returns an error for non defined cpu.
pub fn cpu_not_set_error() -> std::result::Result<AppState, String> {
    Err(format!(
        "Error: Cpu is not defined. Use cpu <{CPU_LIST}> to set cpu."
    ))
}
