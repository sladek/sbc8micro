pub mod mos6502;
pub mod mos6502_opcodes;
pub mod i8080_opcodes;
pub mod opcode_viewer;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Opcode {
    opcode: String,
    mnemonic: String,
    mode: String,
    bytes: u8,
    cycles: u8,
    description: Option<String>,
}
fn load_opcodes(opcodes: &str) -> Vec<Opcode> {
    serde_json::from_str(opcodes).expect("Failed to parse JSON")
}
