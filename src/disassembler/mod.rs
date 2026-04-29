//! Disassembler for specific CPU
pub mod i8080;
pub mod i8080_opcode;
pub mod i8080_opcode_consts;
pub mod i8080_opcodes;
pub mod mos6502;
pub mod mos6502_opcode;
pub mod mos6502_opcode_consts;
pub mod mos6502_opcodes;
pub mod opcode_viewer;

use opcode_viewer::OpcodeViewer;
use ratatui::Frame;

/// Draws description of opcode in terminal
pub trait DrawOpcode<T> {
    /// Draws opcode descriptions on terminal screen
    fn draw(&self, viewer: &OpcodeViewer<T>, frame: &mut Frame);
    /// List of opcode's descriptions to be drawn on terminal
    fn opcodes(&self) -> &Vec<T>;
    /// Find index of the opcode where mnemonic starts with character ch.
    fn find_index_by_char(&self, ch: char) -> Option<usize>;
}

pub struct AsciiDump {
    codes: Vec<u8>,
}

impl AsciiDump {
    pub fn new() -> Self {
        Self {
            codes: Vec::new(),
        }
    }
    pub fn push(&mut self, value: u8) {
        self.codes.push(value);
    }
    pub fn translate(&mut self) -> String {
        let mut result = String::new();
        result.push_str("; ");
        for val in self.codes.clone() {
            let ch: char = match val {
                0x20..=0x7e => {
                    val as char
                } 
                _ => {
                    '.'
                }
            };
            result.push(ch);
        }
        result
    }
}
impl Default for AsciiDump {
    fn default() -> Self {
        Self::new()
    }
}