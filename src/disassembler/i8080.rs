use serde::Deserialize;
use std::collections::HashMap;

use crate::disassembler::i8080_opcodes;
use crate::memory::Memory;

#[derive(Debug, Deserialize)]
pub struct OpcodeDef {
    opcode: String,
    mnemonic: String,
    mode: String,
    bytes: u8,
    //    cycles: String,
}
pub fn load_opcodes_table() -> HashMap<u8, OpcodeDef> {
    let defs: Vec<OpcodeDef> =
        serde_json::from_str(i8080_opcodes::OPCODES).expect("Failed to parse JSON");
    defs.into_iter()
        .map(|def| (u8::from_str_radix(&def.opcode, 16).unwrap(), def))
        .collect()
}

pub fn disassemble(
    memory: &Memory,
    start: u16,
    end: u16,
    opcodes: &HashMap<u8, OpcodeDef>,
) -> Vec<String> {
    let mut output = Vec::new();
    let mut pc = start;

    while pc < end {
        let memory_data = memory.get_data();
        let opcode_byte = memory_data[pc as usize];
        let mut mnemonic = "";
        if let Some(def) = opcodes.get(&opcode_byte) {
            let args = &memory_data[(pc + 1) as usize..];
            let operand_str = match def.mode.as_str() {
                "immediate8" | "direct port" => {
                    mnemonic = &def.mnemonic;
                    mnemonic = mnemonic.trim_end_matches("data");
                    mnemonic = mnemonic.trim_end_matches("port");
                    if args[0] > 0x9F {
                        format!("0{:02X}H", args[0])
                    } else {
                        format!("{:02X}H", args[0])
                    }
                }
                "immediate16" | "direct" => {
                    mnemonic = &def.mnemonic;
                    mnemonic = mnemonic.trim_end_matches("address").trim();
                    let data = u16::from_le_bytes([args[0], args[1]]);
                    if args[1] > 0x9F {
                        format!(" 0{:04X}H", data)
                    } else {
                        format!(" {:04X}H", data)
                    }
                }
                "register" | "none" => {
                    mnemonic = &def.mnemonic;
                    "".to_string()
                },
                "register indirect" => {
                    mnemonic = &def.mnemonic;
                    if mnemonic.contains(",data") {
                        mnemonic = mnemonic.trim_end_matches("data");
                        if args[0] > 0x9F {
                            format!("0{:02X}H", args[0])
                        } else {
                            format!("{:02X}H", args[0])
                        }
                    } else {
                        "".to_string()
                    }
                }
                _ => format!("?? {}", def.mode),
            };
            let operand_bytes = match def.mode.as_str() {
                "immediate8" | "direct port" => {
                    format!("{:02X}", args[0])
                },
                "register indirect"=> {
                    mnemonic = &def.mnemonic.trim_end_matches("data");
                    if mnemonic.ends_with(","){
                        format!("{:02X}", args[0])
                    }
                    else {
                        "".to_string()
                    }
                },
                "immediate16" | "direct" => {
                    format!("{:02X} {:02X}", args[0], args[1])
                }
                _ => "".to_string(),
            };
            output.push(
                format!(
                    "{:04X}  {:02X} {:<8} {}{}",
                    pc, opcode_byte, operand_bytes, mnemonic, operand_str
                )
                .trim_end()
                .to_string()
                .replace(", ", ","),
            );
            pc += def.bytes as u16;
        } else {
            output.push(
                format!(
                    "{:04X}  {:02X}          !byte {:02X}",
                    pc, opcode_byte, opcode_byte
                )
                .trim_end()
                .to_string()
                .replace(", ", ","),
            );
            pc += 1;
        }
    }
    output
}
