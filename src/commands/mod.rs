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
pub mod breakpoints;

use crate::ui::app::App;

pub const CPU_LIST: &str = "i8080 | 8080 | mos6502 | 6502";
pub fn push_cpu_not_set(app: &mut App) {
    app.messages.push(format!("Error: Cpu is not defined. Use set cpu <{CPU_LIST}> to set default cpu first or use opcodes <{CPU_LIST}>"));
}

