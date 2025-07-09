pub mod i8080_opcodes;
pub mod mos6502;
pub mod mos6502_opcodes;
pub mod opcode_viewer;
use crate::disassembler::i8080_opcodes::Opcode;
use crate::disassembler::i8080_opcodes::OpcodeView as i8080;
use crate::disassembler::opcode_viewer::OpcodeViewer;
use ratatui::Frame;

#[derive(Debug)]
pub struct OpcodeView<T> {
    //    opcode: T,
    opcodes: Vec<T>,
}

impl<T> OpcodeView<T> {
    pub fn new(opcodes: Vec<T>) -> Self {
        Self { opcodes }
    }

    pub fn opcodes(&self) -> &Vec<T> {
        &self.opcodes
    }
}

/*
impl<T> Draw<T> for OpcodeView<T> {
    fn draw(&self, _viewer: OpcodeViewer<T>, _frame: &mut Frame ){
    }
}

 */

pub trait Draw<T> {
    fn draw(&self, viewer: &OpcodeViewer<T>, frame: &mut Frame);
    fn opcodes(&self) -> &Vec<T>;
}
