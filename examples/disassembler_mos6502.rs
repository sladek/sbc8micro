use sbc8micro::disassembler::mos6502::{disassemble, load_opcodes_table};
use sbc8micro::disassembler::mos6502_opcode_consts::*;
use sbc8micro::memory;

fn main() {
    let opcodes = load_opcodes_table();
    let start = 0x0600;
    let mut memory = memory::Memory::new();
    let program = vec![
        ADC_IMM, 0x01, // ADC #$01
        ADC_ZP, 0x02, // ADC $02
        ADC_ZP_X, 0x03, // ADC $03,X
        ADC_ABS, 0x34, 0x12, // ADC $1234
        ADC_ABS_X, 0x78, 0x56, // ADC $5678,X
        ADC_ABS_Y, 0xbc, 0x9a, // ADC $9ABC,Y
        ADC_IND_X, 0x55, // ADC ($55,X)
        ADC_IND_Y, 0xaa, // ADC ($AA),Y
        AND_IMM, 0x01, // AND #$01
        AND_ZP, 0x02, // AND $02
        AND_ZP_X, 0x03, // AND $03,X
        AND_ABS, 0x34, 0x12, // AND $1234
        AND_ABS_X, 0x78, 0x56, // AND $5678,X
        AND_ABS_Y, 0xbc, 0x9a, // AND $9ABC,Y
        AND_IND_X, 0x55, // AND ($55,X)
        AND_IND_Y, 0xaa,  // AND ($AA),Y
        ASL_A, // ASL A
        ASL_ZP, 0x02, // ASL $02
        ASL_ZP_X, 0x03, // ASL $03,X
        ASL_ABS, 0x34, 0x12, // ASL $1234
        ASL_ABS_X, 0x78, 0x56, // ASL $5678,X
        BCC, 0x12, // BCC $0645
        BCS, 0x12, // BCS $0647
        BEQ, 0x12, // BEQ $0649
        BIT_ZP, 0x34, // BIT $34
        BIT_ABS, 0x34, 0x12, // BIT $1234
        BMI, 0x12, // BMI $0650
        BNE, 0x12, // BNE $0652
        BPL, 0x12, // BPL $0654
        BVC, 0x12, // BVC $0656
        BVS, 0x12, // BVS $0658
        CLC,  // CLC
        CLD,  // CLD
        CLI,  // CLI
        CLV,  // CLV
        CMP_IMM, 0x01, // CMP #$01
        CMP_ZP, 0x02, // CMP $02
        CMP_ZP_X, 0x03, // CMP $03,X
        CMP_ABS, 0x34, 0x12, // CMP $1234
        CMP_ABS_X, 0x78, 0x56, // CMP $5678,X
        CMP_ABS_Y, 0xbc, 0x9a, // CMP $9ABC,X
        CMP_IND_X, 0x55, // CMP ($55,X)
        CMP_IND_Y, 0xaa, // CMP ($AA),Y
        CPX_IMM, 0x01, // CPX #$01
        CPX_ZP, 0x02, // CPX $02
        CPX_ABS, 0x34, 0x12, // CPX $1234
        CPY_IMM, 0x01, // CPY #$01
        CPY_ZP, 0x02, // CPY $02
        CPY_ABS, 0x34, 0x12, // CPY $1234
        DEC_ZP, 0x02, // DEC $02
        DEC_ZP_X, 0x03, // DEC $03,X
        DEC_ABS, 0x34, 0x12, // DEC $123
        DEC_ABS_X, 0x78, 0x56, // DEC $5678,X
        DEX,  // DEX
        DEY,  // DEY
        EOR_IMM, 0x01, // EOR #$01
        EOR_ZP, 0x02, // EOR $02
        EOR_ZP_X, 0x03, // EOR $03,X
        EOR_ABS, 0x34, 0x12, // EOR $1234
        EOR_ABS_X, 0x78, 0x56, // EOR $5678,X
        EOR_ABS_Y, 0xbc, 0x9a, // EOR $9ABC,X
        EOR_IND_X, 0x55, // EOR ($55,X)
        EOR_IND_Y, 0xaa, // EOR ($AA),Y
        INC_ZP, 0x02, // INC $02
        INC_ZP_X, 0x03, // INC $03,X
        INC_ABS, 0x34, 0x12, // INC $123
        INC_ABS_X, 0x78, 0x56, // INC $5678,X
        INX,  // INX
        INY,  // INY
        JMP, 0x34, 0x12, // JMP $1234
        JMP_IND, 0x34, 0x12, // JMP ($1234)
        JSR, 0x34, 0x12, // JSR $1234
        LDA_IMM, 0x01, // LDA #$02
        LDA_ZP, 0x02, // LDA $02
        LDA_ZP_X, 0x03, // LDA $03,X
        LDA_ABS, 0x34, 0x12, // LDA $1234
        LDA_ABS_X, 0x78, 0x56, // LDA $5678,X
        LDA_ABS_Y, 0xbc, 0x9a, // LDA $9ABC,X
        LDA_IND_X, 0x55, // LDA ($55,X)
        LDA_IND_Y, 0xaa, // LDA ($AA),Y
        LDX_IMM, 0x01, // LDX #$01
        LDX_ZP, 0x02, // LDX $02
        LDX_ZP_Y, 0x03, // LDX $03,Y
        LDX_ABS, 0x34, 0x12, // LDX $1234
        LDX_ABS_Y, 0x34, 0x12, // LDX $1234,Y
        LDY_IMM, 0x01, // LDY #$01
        LDY_ZP, 0x02, // LDY $02
        LDY_ZP_X, 0x03, // LDY $03,X
        LDY_ABS, 0x34, 0x12, // LDY $1234
        LDY_ABS_X, 0x34, 0x12,  // LDY $1234,Y
        LSR_A, // LSR A
        LSR_ZP, 0x02, // LSR, $02
        LSR_ZP_X, 0x03, // LSR $03,X
        LSR_ABS, 0x34, 0x12, // LSR $1234
        LSR_ABS_X, 0x34, 0x12, // LSR $1234,X
        NOP,  // NOP
        ORA_IMM, 0x01, // ORA #$02
        ORA_ZP, 0x02, // ORA $02
        ORA_ZP_X, 0x03, // ORA $03,X
        ORA_ABS, 0x34, 0x12, // ORA $1234
        ORA_ABS_X, 0x78, 0x56, // ORA $5678,X
        ORA_ABS_Y, 0xbc, 0x9a, // ORA $9ABC,X
        ORA_IND_X, 0x55, // ORA ($55,X)
        ORA_IND_Y, 0xaa,  // ORA ($AA),Y
        PHA,   // PHA
        PHP,   // PHP
        PLA,   // PLA
        PLP,   // PLP
        ROL_A, // ROL A
        ROL_ZP, 0x02, // ROL $02
        ROL_ZP_X, 0x03, // ROL $03,X
        ROL_ABS, 0x34, 0x12, // ROL $1234
        ROL_ABS_X, 0x78, 0x56,  // ROL $5678,X
        ROR_A, // ROR A
        ROR_ZP, 0x02, // ROR $02
        ROR_ZP_X, 0x03, // ROR $03,X
        ROR_ABS, 0x34, 0x12, // ROR $1234
        ROR_ABS_X, 0x78, 0x56, // ROR $5678,X
        RTI,  // RTI
        RTS,  // RTS
        SBC_IMM, 0x01, // SBC #$02
        SBC_ZP, 0x02, // SBC $02
        SBC_ZP_X, 0x03, // SBC $03,X
        SBC_ABS, 0x34, 0x12, // SBC $1234
        SBC_ABS_X, 0x78, 0x56, // SBC $5678,X
        SBC_ABS_Y, 0xbc, 0x9a, // SBC $9ABC,X
        SBC_IND_X, 0x55, // SBC ($55,X)
        SBC_IND_Y, 0xaa, // SBC ($AA),Y
        SEC,  // SEC
        SED,  // SED
        SEI,  // SEI
        STA_ZP, 0x02, // STA $02
        STA_ZP_X, 0x03, // STA $03,X
        STA_ABS, 0x34, 0x12, // STA $1234
        STA_ABS_X, 0x78, 0x56, // STA $5678,X
        STA_ABS_Y, 0xbc, 0x9a, // STA $9ABC,X
        STA_IND_X, 0x55, // STA ($55,X)
        STA_IND_Y, 0xaa, // STA ($AA),Y
        STX_ZP, 0x02, // STX $02
        STX_ZP_Y, 0x03, // STX $03,X
        STX_ABS, 0x34, 0x12, // STX $1234
        STY_ZP, 0x02, // STY $02
        STY_ZP_X, 0x03, // STY $03,X
        STY_ABS, 0x34, 0x12, // STY $1234
        TAX,  // TAX
        TAY,  // TAY
        TSX,  // TSX
        TXA,  // TXA
        TXS,  // TXS
        TYA,  // TYA
        BRK,  // BRK
    ];
    let _ = memory.load_program(&program, start);
    let disassembly = disassemble(&memory, start, start + program.len() as u16, &opcodes);

    for line in disassembly {
        println!("{}", line);
    }
}
