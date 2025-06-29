/////////////////////////////
/// ```
/// mod disassembler;
/// mod memory;
///
/// use disassembler::mos6502::disassemble;
///
/// fn disassemble() {
///     let mut memory = memory::Memory::new();
///
///    let program = vec![
///         0xA9, 0x01, // LDA #$01
///         0x8D, 0x00, 0x02, // STA $0200
///         0xE8, // INX
///         0xF0, 0xFC, // BEQ $0600 (loop)
///         0x00, // BRK
///    ];
///     let start = 0x0600;
///     let end = start + program.len() as u16;
///     memory.load_program(&program, start);
///  
///     let disassembly = disassemble(&memory, start, end);
///     for line in disassembly {
///         println!("{}", line);
///     }
/// }
/// ```
///
/// The result should be:
/// 0600  A9 01       LDA #$01
/// 0602  8D 00       STA $0200
/// 0605  E8 F0       INX
/// 0606  F0 FC       BEQ $0604
/// 0608  00 00       BRK
///
/////////////////////////////
use serde::Deserialize;
use std::collections::HashMap;

use crate::disassembler::mos6502_opcodes;
use crate::memory::Memory;

#[derive(Debug, Deserialize)]
pub struct OpcodeDef {
    opcode: String,
    mnemonic: String,
    mode: String,
    bytes: u8,
}

pub fn load_opcodes_table() -> HashMap<u8, OpcodeDef> {
    let defs: Vec<OpcodeDef> =
        serde_json::from_str(mos6502_opcodes::OPCODES).expect("Failed to parse JSON");
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
        if let Some(def) = opcodes.get(&opcode_byte) {
            let args = &memory_data[(pc + 1) as usize..];
            let operand_str = match def.mode.as_str() {
                "accumulator" => "A".to_string(),
                "immediate" => format!("#${:02X}", args[0]),
                "zeropage" => format!("${:02X}", args[0]),
                "zeropage,X" => format!("${:02X},X", args[0]),
                "zeropage,Y" => format!("${:02X},Y", args[0]),
                "absolute" => format!("${:04X}", u16::from_le_bytes([args[0], args[1]])),
                "absolute,X" => format!("${:04X},X", u16::from_le_bytes([args[0], args[1]])),
                "absolute,Y" => format!("${:04X},Y", u16::from_le_bytes([args[0], args[1]])),
                "indirect" => format!("(${:04X})", u16::from_le_bytes([args[0], args[1]])),
                "relative" => {
                    let offset = args[0] as i8;
                    let target = (pc as i16 + 2 + offset as i16) as u16;
                    format!("${:04X}", target)
                }
                "implied" => "".to_string(),
                "(indirect,X)" => format!("(${:02X},X)", args[0]),
                "(indirect),Y" => format!("(${:02X}),Y", args[0]),
                _ => format!("?? {}", def.mode),
            };
            let operand_bytes = match def.mode.as_str() {
                "immediate" | "zeropage" | "zeropage,X" | "zeropage,Y" | "relative"
                | "(indirect,X)" | "(indirect),Y" => {
                    format!("{:02X}", args[0])
                }
                "absolute" | "absolute,X" | "absolute,Y" => {
                    format!("{:02X} {:02X}", args[0], args[1])
                }
                "implied" => "".to_string(),
                _ => format!(""),
            };
            output.push(format!(
                "{:04X}  {:02X} {:<8} {} {}",
                pc,
                opcode_byte,
                operand_bytes,
                &def.mnemonic[..3],
                operand_str
            ));
            pc += def.bytes as u16;
        } else {
            output.push(format!(
                "{:04X}  {:02X}          !byte {:02X}",
                pc, opcode_byte, opcode_byte
            ));
            pc += 1;
        }
    }
    output
}
