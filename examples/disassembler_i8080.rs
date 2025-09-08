use sbc8micro::disassembler::i8080::{disassemble, load_opcodes_table};
use sbc8micro::disassembler::i8080_opcode_consts::*;
use sbc8micro::memory;

fn main() {
    let opcodes = load_opcodes_table();

    let mut memory = memory::Memory::new();
    let program = vec![
        ADC_B, ADC_C, ADC_D, ADC_E, ADC_H, ADC_L, ADC_M, ADC_A, // ADC
        ACI, 0xAF, // ACI 0AFH
        ADD_B, ADD_C, ADD_D, ADD_E, ADD_H, ADD_L, ADD_M, ADD_A, // ADD
        ADI, 0x18, // ADI
        ANA_B, ANA_C, ANA_D, ANA_E, ANA_H, ANA_L, ANA_M, ANA_A, // ANA
        ANI, 0xAB, // ANI
        CMA, // CMA
        CMC, // CMC
        CMP_B, CMP_C, CMP_D, CMP_E, CMP_H, CMP_L, CMP_M, CMP_A, // CMP
        CPI, 0x12,          // CPI
        CNZ, 0x12, 0xA4,    // CNZ
        CZ, 0x12, 0xA4,     // CZ
        CNC, 0x12, 0xA4,    // CNC
        CC, 0x12, 0xA4,     // CC
        CPO, 0x12, 0xA4,    // CPO
        CPE, 0x12, 0xA4,    // CPE
        CP, 0x12, 0xA4,     // CP
        CM, 0x12, 0xA4,     // CM
        CALL, 0x12, 0x9F,   // CALL
        DAA,    // DAA
        DAD_B,  // DAD B
        DAD_D,  // DAD D
        DAD_H,  // DAD H
        DAD_SP, // DAD SP
        DCR_B, DCR_C, DCR_D, DCR_E, DCR_H, DCR_L, DCR_M, DCR_A, // DCR
        DCX_B, DCX_D, DCX_H, DCX_SP, // DCX
        DI, // DI
        EI, // EI
        HLT, // HLT
        IN, 0xAA, // IN
        INR_B, INR_C, INR_D, INR_E, INR_H, INR_L, INR_M, INR_A, // INR
        INX_B, INX_D, INX_H, INX_SP, // INX
        JNZ, 0x34, 0x12,    // JNZ
        JZ, 0x34, 0x12,     // JZ
        JNC, 0x34, 0x12,    // JNC
        JC, 0x34, 0x12,     // JC
        JPO, 0x34, 0x12,    // JPO
        JPE, 0xAA, 0xAA,    // JPE
        JP, 0x34, 0x12,     // JP
        JM, 0xAA, 0xAA,     // JM
        JMP, 0xAA, 0xAA,    // JMP
        LDA, 0x34, 0x12,    // LDA
        LDAX_B, LDAX_D,     // LDAX
        LHLD, 0x34, 0x12,   // LHLD
        LXI_B, 0x34, 0x12,  // LXI B
        LXI_D, 0x34, 0x12,  // LXI B
        LXI_H, 0x34, 0x12,  // LXI B
        LXI_SP, 0x34, 0x12,     // LXI B
        MVI_B, 0x33, // MVI B
        MVI_C, 0x55, // MVI C
        MVI_D, 0xAA, // MVI D
        MVI_E, 0x33, // MVI E
        MVI_H, 0x55, // MVI H
        MVI_L, 0xAA, // MVI L
        MVI_M, 0xAA, // MVI M
        MVI_A, 0x55, // MVI A
        MOV_A_B, MOV_A_C, MOV_A_D, MOV_A_E, MOV_A_H, MOV_A_L, MOV_A_M, MOV_A_A, // MOV A
        MOV_B_B, MOV_B_C, MOV_B_D, MOV_B_E, MOV_B_H, MOV_B_L, MOV_B_M, MOV_B_A, // MOV B
        MOV_C_B, MOV_C_C, MOV_C_D, MOV_C_E, MOV_C_H, MOV_C_L, MOV_C_M, MOV_C_A, // MOV C
        MOV_D_B, MOV_D_C, MOV_D_D, MOV_D_E, MOV_D_H, MOV_D_L, MOV_D_M, MOV_D_A, // MOV D
        MOV_E_B, MOV_E_C, MOV_E_D, MOV_E_E, MOV_E_H, MOV_E_L, MOV_E_M, MOV_E_A, // MOV E
        MOV_H_B, MOV_H_C, MOV_H_D, MOV_H_E, MOV_H_H, MOV_H_L, MOV_H_M, MOV_H_A, // MOV H
        MOV_L_B, MOV_L_C, MOV_L_D, MOV_L_E, MOV_L_H, MOV_L_L, MOV_L_M, MOV_L_A, // MOV L
        MOV_M_B, MOV_M_C, MOV_M_D, MOV_M_E, MOV_M_H, MOV_M_L, MOV_M_A, // MOV M
        NOP, // NOP
        ORA_B, ORA_C, ORA_D, ORA_E, ORA_H, ORA_L, ORA_M, ORA_A, // ORA
        ORI, 0x55,  // ORI
        PCHL, // PCHL
        POP_B, POP_D, POP_H, POP_PSW, // POP
        PUSH_B, PUSH_D, PUSH_H, PUSH_PSW, // PUSH
        RAL, // RAL,
        RAR, // RAR
        RLC, // RLC
        RRC, // RRC
        RNZ, // RNZ
        RZ,  // RZ
        RNC, // RNC
        RC,  // RC
        RPO, // RPO
        RPE, // RPE
        RP,  // RP
        RM,  // RM
        RET, // RET
        RST_0, RST_1, RST_2, RST_3, RST_4, RST_5, RST_6, RST_7, // RST
        SBB_B, SBB_C, SBB_D, SBB_E, SBB_H, SBB_L, SBB_M, SBB_A, // SBB
        SBI, 0xAA, // SBI
        SUB_B, SUB_C, SUB_D, SUB_E, SUB_H, SUB_L, SUB_M, SUB_A, // SUB
        SUI, 0xAA, // SUI
        SHLD, 0x98, 0xBA, // SHLD
        STA, 0x34, 0x12, // STA
        STAX_B, STAX_D, //STAX
        SPHL, // SPHL
        XCHG, // XCHG
        XRA_B, XRA_C, XRA_D, XRA_E, XRA_H, XRA_L, XRA_M, XRA_A, // XRA
        XRI, 0xAA, // XRI
        XTHL, // XTHL
    ];
    let start = 0x0600;
    memory.load_program(&program, start);
    let disassembly = disassemble(&memory, start, start + program.len() as u16, &opcodes);

    for line in disassembly {
        println!("{}", line);
    }
}
