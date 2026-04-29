//! Disassembler for INTEL i8080 CPU
use crate::disassembler::AsciiDump;
use crate::disassembler::i8080_opcodes::OPCODES;
use crate::memory::Memory;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct OpcodeDef {
    opcode: String,
    mnemonic: String,
    mode: String,
    bytes: u8,
    //    cycles: String,
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
        let mut mnemonic = "";
        if let Some(def) = opcodes.get(&opcode_byte) {
            let addr_0 = pc.wrapping_add(1);
            let addr_1 = addr_0.wrapping_add(1);
            let args_0 = memory.read_byte(addr_0);
            let args_1 = memory.read_byte(addr_1);
            let mut ascii_dump = AsciiDump::new();
            ascii_dump.push(opcode_byte);
            let operand_str = match def.mode.as_str() {
                "immediate8" | "direct port" => {
                    mnemonic = &def.mnemonic;
                    mnemonic = mnemonic.trim_end_matches("data");
                    mnemonic = mnemonic.trim_end_matches("port");
                    if memory.read_byte(addr_0) > 0x9F {
                        format!("0{:02X}H", args_0)
                    } else {
                        format!("{:02X}H", args_0)
                    }
                }
                "immediate16" | "direct" => {
                    mnemonic = &def.mnemonic;
                    mnemonic = mnemonic.trim_end_matches("address").trim();
                    let data = u16::from_le_bytes([args_0, args_1]);
                    if memory.read_byte(addr_1) > 0x9F {
                        format!(" 0{:04X}H", data)
                    } else {
                        format!(" {:04X}H", data)
                    }
                }
                "register" | "none" => {
                    mnemonic = &def.mnemonic;
                    "".to_string()
                }
                "register indirect" => {
                    mnemonic = &def.mnemonic;
                    if mnemonic.contains(",data") {
                        mnemonic = mnemonic.trim_end_matches("data");
                        if args_0 > 0x9F {
                            format!("0{:02X}H", args_0)
                        } else {
                            format!("{:02X}H", args_0)
                        }
                    } else {
                        "".to_string()
                    }
                }
                _ => format!("?? {}", def.mode),
            };
            let operand_bytes = match def.mode.as_str() {
                "immediate8" | "direct port" => {
                    ascii_dump.push(args_0);
                    format!("{:02X}", args_0)
                }
                "register indirect" => {
                    mnemonic = def.mnemonic.trim_end_matches("data");
                    if mnemonic.ends_with(",") {
                        ascii_dump.push(args_0);
                        format!("{:02X}", args_0)
                    } else {
                        "".to_string()
                    }
                }
                "immediate16" | "direct" => {
                    ascii_dump.push(args_0);
                    ascii_dump.push(args_1);
                    format!("{:02X} {:02X}", args_0, args_1)
                }
                _ => "".to_string(),
            };
            let mut out = format!(
                    "{:04X}  {:02X} {:<8} {}{}",
                    pc, opcode_byte, operand_bytes, mnemonic, operand_str
                )
                .trim_end()
                .to_string()
                .replace(", ", ",");
                // Add some spaces untol the column 32
                while out.len() <= 32 {
                out.push(' ');
            }
            out.push_str(&ascii_dump.translate());
            output.push(out);
            pc = pc.wrapping_add(def.bytes as u16);
        } else {
            output.push(
                format!(
                    "{:04X}  {:02X}          DB {:02X}",
                    pc, opcode_byte, opcode_byte
                )
                .trim_end()
                .to_string()
                .replace(", ", ","),
            );
            pc = pc.wrapping_add(1);
        }
    }
    output
}
