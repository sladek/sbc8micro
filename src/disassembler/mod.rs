pub mod i8080;
pub mod i8080_opcodes;
pub mod mos6502;
pub mod mos6502_opcodes;
pub mod opcode_viewer;
use crate::disassembler::opcode_viewer::OpcodeViewer;
use ratatui::Frame;

pub trait DrawOpcode<T> {
    fn draw(&self, viewer: &OpcodeViewer<T>, frame: &mut Frame);
    fn opcodes(&self) -> &Vec<T>;
}
