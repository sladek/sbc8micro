//! Disassembler for MOS Technology 6502 CPU
//!
//! ```
//! use sbc8micro::disassembler::mos6502::{load_opcodes_table,disassemble};
//! use sbc8micro::memory;
//!
//! fn example() {
//!     let opcodes = load_opcodes_table();
//!
//!     let mut memory = memory::Memory::new();
//!
//!    let program = vec![
//!         0xA9, 0x01,         // LDA #$01
//!         0x8D, 0x00, 0x02,   // STA $0200
//!         0xE8,               // INX
//!         0xF0, 0xFC,         // BEQ $0600 (loop)
//!         0x00,               // BRK
//!    ];
//!     let start = 0x0600;
//!     let end = start + program.len() as u16;
//!     memory.load_program(&program, start);
//!  
//!     let disassembly = disassemble(&mut memory, start, end, &opcodes);
//!     for line in disassembly {
//!         println!("{}", line);
//!     }
//! }
//! ```
//!
//! The result should be:
//! 0600  A9 01       LDA #$01
//! 0602  8D 00 02    STA $0200
//! 0605  E8          INX
//! 0606  F0 FC       BEQ $0604
//! 0608  00 00       BRK
use serde::Deserialize;
use std::collections::HashMap;

use crate::disassembler::mos6502_opcodes::OPCODES;
use crate::memory::Memory;

#[derive(Debug, Deserialize)]
pub struct OpcodeDef {
    opcode: String,
    mnemonic: String,
    mode: String,
    bytes: u8,
}

pub fn load_opcodes_table() -> HashMap<u8, OpcodeDef> {
    let defs: Vec<OpcodeDef> = serde_json::from_str(OPCODES).expect("Failed to parse JSON");
    defs.into_iter()
        .map(|def| (u8::from_str_radix(&def.opcode, 16).unwrap(), def))
        .collect()
}

pub fn disassemble(
    memory: &mut Memory,
    start: u16,
    end: u16,
    opcodes: &HashMap<u8, OpcodeDef>,
) -> Vec<String> {
    let mut output = Vec::new();
    let mut pc = start;

    while pc <= end && pc >= start {
        //pc >= start means that pc wrapped over maximum address so we have to check also this possibility
        let opcode_byte = memory.read_byte(pc);
        if let Some(def) = opcodes.get(&opcode_byte) {
            let addr_0 = pc.wrapping_add(1);
            let addr_1 = addr_0.wrapping_add(1);
            let args_0 = memory.read_byte(addr_0);
            let args_1 = memory.read_byte(addr_1);
            let operand_str = match def.mode.as_str() {
                "accumulator" => "A".to_string(),
                "immediate" => format!("#${:02X}", args_0),
                "zeropage" => format!("${:02X}", args_0),
                "zeropage,X" => format!("${:02X},X", args_0),
                "zeropage,Y" => format!("${:02X},Y", args_0),
                "absolute" => format!("${:04X}", u16::from_le_bytes([args_0, args_1])),
                "absolute,X" => format!("${:04X},X", u16::from_le_bytes([args_0, args_1])),
                "absolute,Y" => format!("${:04X},Y", u16::from_le_bytes([args_0, args_1])),
                "indirect" => format!("(${:04X})", u16::from_le_bytes([args_0, args_1])),
                "relative" => {
                    let offset = args_0 as i8;
                    let target = (pc as i16 + 2 + offset as i16) as u16;
                    format!("${:04X}", target)
                }
                "implied" => "".to_string(),
                "(indirect,X)" => format!("(${:02X},X)", args_0),
                "(indirect),Y" => format!("(${:02X}),Y", args_0),
                _ => format!("?? {}", def.mode),
            };
            let operand_bytes = match def.mode.as_str() {
                "immediate" | "zeropage" | "zeropage,X" | "zeropage,Y" | "relative"
                | "(indirect,X)" | "(indirect),Y" => {
                    format!("{:02X}", args_0)
                }
                "absolute" | "absolute,X" | "absolute,Y" => {
                    format!("{:02X} {:02X}", args_0, args_1)
                }
                "implied" => "".to_string(),
                _ => "".to_string(),
            };
            output.push(format!(
                "{:04X}  {:02X} {:<8} {} {}",
                pc,
                opcode_byte,
                operand_bytes,
                &def.mnemonic[..3],
                operand_str
            ));
            pc = pc.wrapping_add(def.bytes as u16);
        } else {
            output.push(format!(
                "{:04X}  {:02X}          !byte {:02X}",
                pc, opcode_byte, opcode_byte
            ));
            pc = pc.wrapping_add(1);
        }
    }
    output
}
