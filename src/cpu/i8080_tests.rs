#[cfg(test)]
use crate::cpu::i8080::Cpu;
#[cfg(test)]
use crate::disassembler::i8080_opcodes_const::*;
#[cfg(test)]
use crate::status::i8080;

#[test]
///
/// Tests immediate ACI without CARRY flag
/// Initial CARRY = 0
///
fn aci_no_c_neg_p() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x55, ACI, 0x74, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xc9u8);
    assert_eq!(cpu.psw.value, 0x86);
}
#[test]
///
/// Tests immediate ACI
///
fn aci_c_ac_p() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x66, ACI, 0xAA, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.psw.value, 0x13);
}
#[test]
///
/// Tests immediate ACI
///
fn aci_z_c_ac_p() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![
        MVI_A, 0x56, // MVI A,55H
        ACI, 0xAA, // ACI 74H
        HLT,  // HLT
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.psw.value, 0x57);
}
#[test]
///
/// Tests immediate ACI
///
fn aci_2_z_c_ac_p() {
    let mut cpu = Cpu::new();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, ACI, 0xAA, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.psw.value, 0x57);
}
#[test]
///
/// Tests ADC B
///
fn adc_b_z_ac_p() {
    let mut cpu = Cpu::new();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_B, 0xaa, ADC_B, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.psw.value, 0x57);
}
#[test]
///
/// Tests ADC C
///
fn adc_c_neg_p() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_C, 0xaa, ADC_C, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.psw.value, 0x86);
}
#[test]
///
/// Tests ADC D
///
fn adc_d_neg_p() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_D, 0xaa, ADC_D, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.psw.value, 0x86);
}
#[test]
///
/// Tests ADC E
///
fn adc_e_z_ac_p() {
    let mut cpu = Cpu::new();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_E, 0xaa, ADC_E, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.psw.value, 0x57);
}
#[test]
///
/// Tests ADC H
///
fn adc_h_z_ac_p() {
    let mut cpu = Cpu::new();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_H, 0xaa, ADC_H, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.psw.value, 0x57);
}
#[test]
///
/// Tests ADC L
///
fn adc_l_neg_p() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_L, 0xaa, ADC_L, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.psw.value, 0x86);
}
#[test]
///
/// Tests ADC A
///
fn adc_a_c_ac_neg() {
    let mut cpu = Cpu::new();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, ADC_A, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xABu8);
    assert_eq!(cpu.psw.value, 0x82);
}
#[test]
///
/// Tests ADC M
///
fn adc_m_p() {
    let mut cpu = Cpu::new();
    cpu.memory.write_byte(0x1234, 0x12);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0x34, ADC_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x47u8);
    assert_eq!(cpu.psw.value, 0x06);
}
#[test]
///
/// Tests ADC M
///
fn adc_m_neg_ac_p_c() {
    let mut cpu = Cpu::new();
    cpu.memory.write_byte(0x1234, 0xff);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0xAA, ADC_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.psw.value, 0x97);
}
#[test]
///
/// Tests ADC M
///
fn adc_m_neg_ac_p_c_2() {
    let mut cpu = Cpu::new();
    cpu.memory.write_byte(0x1234, 0xff);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0xFF, ADC_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.psw.value, 0x97);
}
#[test]
///
/// Tests ADC M
///
fn adc_m_ac_p_c() {
    let mut cpu = Cpu::new();
    cpu.memory.write_byte(0x1234, 0xaa);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0xaa, ADC_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.psw.value, 0x17);
}
#[test]
///
/// Tests ADC M
///
fn adc_m_neg_ac_c() {
    let mut cpu = Cpu::new();
    cpu.memory.write_byte(0x1234, 0xff);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0xff, ADC_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xfeu8);
    assert_eq!(cpu.psw.value, 0x93);
}
#[test]
///
/// Tests ADC M
///
fn adc_m_ac_c() {
    let mut cpu = Cpu::new();
    cpu.memory.write_byte(0x1234, 0xAA);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0xAA, ADC_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x54u8);
    assert_eq!(cpu.psw.value, 0x13);
}

#[test]
///
/// Tests ADD B
///
fn add_b_neg_p() {
    let mut cpu = Cpu::new();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_B, 0xaa, ADD_B, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.psw.value, 0x86);
}

#[test]
///
/// Tests ADD C
///
fn add_c_neg_p() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_C, 0xaa, ADD_C, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.psw.value, 0x86);
}
#[test]
///
/// Tests ADD D
///
fn add_d_neg_p() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_D, 0xaa, ADD_D, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.psw.value, 0x86);
}
#[test]
///
/// Tests ADD D
///
fn add_d_acc_c() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x66, MVI_D, 0xaa, ADD_D, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.psw.value, 0x13);
}
#[test]
///
/// Tests ADC E
///
fn add_e_z_ac_p_c() {
    let mut cpu = Cpu::new();
    //    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x56, MVI_E, 0xaa, ADD_E, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.psw.value, 0x57);
}
#[test]
///
/// Tests ADC H
///
fn add_h_z_ac_p() {
    let mut cpu = Cpu::new();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xff, MVI_H, 0xff, ADD_H, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xfeu8);
    assert_eq!(cpu.psw.value, 0x93);
}
#[test]
///
/// Tests ADC L
///
fn add_l_p() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0xaa, MVI_L, 0xaa, ADD_L, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x54u8);
    assert_eq!(cpu.psw.value, 0x13);
}
#[test]
///
/// Tests ADC A
///
fn add_a_c_ac_neg() {
    let mut cpu = Cpu::new();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, ADD_A, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xAAu8);
    assert_eq!(cpu.psw.value, 0x86);
}
#[test]
///
/// Tests ADC M
///
fn add_m_p() {
    let mut cpu = Cpu::new();
    cpu.memory.write_byte(0x1234, 0x12);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0x35, ADD_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x47u8);
    assert_eq!(cpu.psw.value, 0x06);
}
#[test]
///
/// Tests ADC M
///
fn add_m() {
    let mut cpu = Cpu::new();
    cpu.memory.write_byte(0x1234, 0x12);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0x34, ADD_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x46u8);
    assert_eq!(cpu.psw.value, 0x02);
}
#[test]
///
/// Tests ADI
///
fn adi_p() {
    let mut cpu = Cpu::new();
    cpu.memory.write_byte(0x1234, 0x12);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x34, ADI, 0x34, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x68u8);
    assert_eq!(cpu.psw.value, 0x02);
}
#[test]
///
/// Tests ANA B
///
fn ana_b() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x34, MVI_B, 0x34, ANA_B, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x34u8);
    assert_eq!(cpu.psw.value, 0x02);
}
#[test]
///
/// Tests ANA B
///
fn ana_b_2() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![
        MVI_A, 0x66, ADI, 0xaa, // set CY and AC
        MVI_B, 0x10, ANA_B, HLT,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.psw.value, 0x02);
}
#[test]
///
/// Tests ANA C with PARITY
///
fn ana_c_p() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![
        MVI_A, 0x66, ADI, 0xaa, // set CY and AC
        MVI_A, 0x55, MVI_C, 0xAA, ANA_C, HLT,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.psw.value, 0x56);
}
#[test]
///
/// Tests ANA D with PARITY and ZERO
///
fn ana_d_z_p() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_D, 0xAA, ANA_D, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.psw.value, 0x56);
}
#[test]
///
/// Tests ANA E with PARITY and ZERO
///
fn ana_e_z_p() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_E, 0xAA, ANA_E, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.psw.value, 0x56);
}
#[test]
///
/// Tests ANA H
///
fn ana_h() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x34, MVI_H, 0x34, ANA_H, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x34u8);
    assert_eq!(cpu.psw.value, 0x02);
}
#[test]
///
/// Tests ANA L
///
fn ana_l() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x12, MVI_L, 0x34, ANA_L, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.psw.value, 0x02);
}
#[test]
///
/// Tests ANA M
///
fn ana_m() {
    let mut cpu = Cpu::new();
    cpu.memory.write_byte(0x200, 0x55);
    let program: Vec<u8> = vec![MVI_A, 0x12, MVI_L, 0x34, ANA_L, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.psw.value, 0x02);
}
#[test]
///
/// Tests ANA A
///
fn ana_a() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x55, ANA_A, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.psw.value, 0x06);
}
#[test]
///
/// Tests ANI
///
fn ani_no_z() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x08, ANI, 0x08, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x08u8);
    assert_eq!(cpu.psw.value, 0x12);
}
#[test]
///
/// Tests ANI ZERO
///
fn ani_08_00() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x08, ANI, 0x00, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.psw.value, 0x56);
}
#[test]
///
/// Tests ANI
///
fn ani_ff_ff() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0xff, ANI, 0xff, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.psw.value, 0x96);
}
#[test]
///
/// Tests ANI
///
fn ani_ff_0f() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0xff, ANI, 0x0f, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x0Fu8);
    assert_eq!(cpu.psw.value, 0x16);
}
#[test]
///
/// Tests CMA
///
fn cma() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x55, CMA, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xAAu8);
    assert_eq!(cpu.psw.value, 0x02);
}
#[test]
///
/// Tests CMC
///
fn cmc() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![CMC, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.psw.value, 0x03);
}
#[test]
/// This and following tests use directly psw.value for assertions
/// This simplifies writing tests
///
/// Tests CMP B
///
fn cmp_b_ff_aa() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0xff, MVI_B, 0xAA, CMP_B, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.psw.value, 0x16);
}
#[test]
///
/// Tests CMP B
///
fn cmp_b_aa_ff() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0xaa, MVI_B, 0xff, CMP_B, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.psw.value, 0x83);
}
#[test]
///
/// Tests CMP C
///
fn cmp_c_80_70() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x80, MVI_C, 0x70, CMP_C, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x80u8);
    assert_eq!(cpu.psw.value, 0x12);
}
#[test]
///
/// Tests CMP C
///
fn cmp_c_70_80() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x70, MVI_C, 0x80, CMP_C, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x70u8);
    assert_eq!(cpu.psw.value, 0x97);
}
#[test]
///
/// Tests CMP D
///
fn cmp_c_55_aa() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_D, 0xAA, CMP_D, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.psw.value, 0x83);
}
#[test]
///
/// Tests CMP E
///
fn cmp_e_aa_55() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0xAA, MVI_E, 0x55, CMP_E, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.psw.value, 0x16);
}
#[test]
///
/// Tests CMP E
///
fn cmp_e_20_10() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x20, MVI_E, 0x10, CMP_E, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x20u8);
    assert_eq!(cpu.psw.value, 0x12);
}
#[test]
///
/// Tests CMP H
///
fn cmp_h_10_20() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x10, MVI_H, 0x20, CMP_H, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.psw.value, 0x97);
}
#[test]
///
/// Tests CMP H
///
fn cmp_h_05_0a() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x05, MVI_H, 0x0a, CMP_H, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x05u8);
    assert_eq!(cpu.psw.value, 0x83);
}
#[test]
///
/// Tests CMP H
///
fn cmp_h_0a_05() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x0a, MVI_H, 0x05, CMP_H, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x0au8);
    assert_eq!(cpu.psw.value, 0x16);
}
#[test]
///
/// Tests CMP L
///
fn cmp_l_05_05() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x05, MVI_L, 0x05, CMP_L, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x05u8);
    assert_eq!(cpu.psw.value, 0x56);
}
#[test]
///
/// Tests CMP L
///
fn cmp_l_55_55() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_L, 0x55, CMP_L, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.psw.value, 0x56);
}
#[test]
///
/// Tests CMP M
///
fn cmp_m_05_01() {
    let mut cpu = Cpu::new();
    cpu.memory.write_byte(0x1234, 0x05);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0x01, CMP_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
    assert_eq!(cpu.psw.value, 0x87);
}
#[test]
///
/// Tests CMP M
///
fn cmp_m_01_05() {
    let mut cpu = Cpu::new();
    cpu.memory.write_byte(0x1234, 0x01);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0x05, CMP_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x05u8);
    assert_eq!(cpu.psw.value, 0x12);
}
#[test]
///
/// Tests CMP A
///
fn cmp_a_aa_aa() {
    let mut cpu = Cpu::new();
    let program: Vec<u8> = vec![MVI_A, 0xaa, CMP_A, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.psw.value, 0x56);
}
#[test]
///
/// Tests CPI
///
fn cpi_55_aa() {
    let mut cpu = Cpu::new();
    cpu.a = 0x55;
    let program: Vec<u8> = vec![MVI_A, 0x55, CPI, 0xaa, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.psw.value, 0x83);
}
#[test]
///
/// Tests CALL
///
fn call() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xa5u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests CNZ
///
fn cnz_nz() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.a = 0xff;
    let program: Vec<u8> = vec![CPI, 0xf0, CNZ, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xa5u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests CNZ
///
fn cnz_z() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.a = 0xff;
    let program: Vec<u8> = vec![CPI, 0xff, CNZ, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xafu8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.read_word(addr);
    assert_eq!(val, 0x0000); // No data ahould be on stack
}
#[test]
///
/// Tests CZ
///
fn cz_z() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.a = 0xff;
    let program: Vec<u8> = vec![CPI, 0xff, CZ, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xa5u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests CZ
///
fn cz_nz() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.a = 0xff;
    let program: Vec<u8> = vec![CPI, 0xf0, CZ, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaFu8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.read_word(addr);
    assert_eq!(val, 0x0000);
}
#[test]
///
/// Tests CNC
///
fn cnc_nc() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.a = 0xff;
    let program: Vec<u8> = vec![ANI, 0xff, CNC, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xa5u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests CNC
///
fn cnc_c() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.a = 0xff;
    let program: Vec<u8> = vec![ADI, 0x01, CNC, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.read_word(addr);
    assert_eq!(val, 0x0000);
}
#[test]
///
/// Tests CC
///
fn cc_c() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.a = 0xff;
    let program: Vec<u8> = vec![ADI, 0xff, CC, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xa4u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests CC
///
fn cc_nc() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.a = 0xff;
    let program: Vec<u8> = vec![ADI, 0x00, CC, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xafu8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.read_word(addr);
    assert_eq!(val, 0x0000);
}
#[test]
///
/// Tests CPO
///
fn cpo_po() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.a = 0xff;
    let program: Vec<u8> = vec![ANI, 0x01, CPO, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests CPO
///
fn cpo_npo() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.a = 0xff;
    let program: Vec<u8> = vec![ANI, 0x03, CPO, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x03u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.read_word(addr);
    assert_eq!(val, 0x0000);
}
#[test]
///
/// Tests CPE
///
fn cpe_pe() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.a = 0xff;
    let program: Vec<u8> = vec![ANI, 0x03, CPE, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests CPE
///
fn cpe_po() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.a = 0xff;
    let program: Vec<u8> = vec![ANI, 0x01, CPE, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.read_word(addr);
    assert_eq!(val, 0x0000);
}
#[test]
///
/// Tests CP
///
fn cp_p() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.a = 0xff;
    let program: Vec<u8> = vec![ANI, 0x03, CP, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests CP
///
fn cp_m() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.a = 0xff;
    let program: Vec<u8> = vec![ANI, 0xff, CP, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xafu8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.read_word(addr);
    assert_eq!(val, 0x0000);
}
#[test]
///
/// Tests CM
///
fn cm_m() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.a = 0xff;
    let program: Vec<u8> = vec![ANI, 0xff, CM, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xa5u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests DAA
///
fn daa_00() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0x00, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.psw.value, 0x46u8);
}
#[test]
///
/// Tests DAA
///
fn daa_05() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0x05, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x05u8);
    assert_eq!(cpu.psw.value, 0x06u8);
}
#[test]
///
/// Tests DAA
///
fn daa_0a() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0x0A, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.psw.value, 0x12u8);
}
#[test]
///
/// Tests DAA
///
fn daa_0f() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0x0f, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x15u8);
    assert_eq!(cpu.psw.value, 0x12u8);
}
#[test]
///
/// Tests DAA
///
fn daa_50() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0x50, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x50u8);
    assert_eq!(cpu.psw.value, 0x06u8);
}
#[test]
///
/// Tests DAA
///
fn daa_a0() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0xa0, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.psw.value, 0x47u8);
}
#[test]
///
/// Tests DAA
///
fn daa_f0() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0xf0, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x50u8);
    assert_eq!(cpu.psw.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_55() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0x55, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.psw.value, 0x06u8);
}
#[test]
///
/// Tests DAA
///
fn daa_aa() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0xaa, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.psw.value, 0x13u8);
}
#[test]
///
/// Tests DAA
///
fn daa_ff() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0xff, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x65u8);
    assert_eq!(cpu.psw.value, 0x17u8);
}
#[test]
///
/// Tests DAA
///
fn daa_33() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0x33, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x33u8);
    assert_eq!(cpu.psw.value, 0x06u8);
}
#[test]
///
/// Tests DAA
///
fn daa_00_ac() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0x00, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x06u8);
    assert_eq!(cpu.psw.value, 0x06u8);
}
#[test]
///
/// Tests DAA
///
fn daa_05_ac() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0x05, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x0bu8);
    assert_eq!(cpu.psw.value, 0x02u8);
}
#[test]
///
/// Tests DAA
///
fn daa_0a_ac() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0x0A, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.psw.value, 0x12u8);
}
#[test]
///
/// Tests DAA
///
fn daa_0f_ac() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0x0f, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x15u8);
    assert_eq!(cpu.psw.value, 0x12u8);
}
#[test]
///
/// Tests DAA
///
fn daa_50_ac() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0x50, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x56u8);
    assert_eq!(cpu.psw.value, 0x06u8);
}
#[test]
///
/// Tests DAA
///
fn daa_a0_ac() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0xa0, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x06u8);
    assert_eq!(cpu.psw.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_f0_ac() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0xf0, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x56u8);
    assert_eq!(cpu.psw.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_55_ac() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x5bu8);
    assert_eq!(cpu.psw.value, 0x02u8);
}
#[test]
///
/// Tests DAA
///
fn daa_aa_ac() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0xaa, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.psw.value, 0x13u8);
}
#[test]
///
/// Tests DAA
///
fn daa_ff_ac() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0xff, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x65u8);
    assert_eq!(cpu.psw.value, 0x17u8);
}
#[test]
///
/// Tests DAA
///
fn daa_33_ac() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0x33, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x39u8);
    assert_eq!(cpu.psw.value, 0x06u8);
}
#[test]
///
/// Tests DAA
///
fn daa_00_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x00, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x60u8);
    assert_eq!(cpu.psw.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_05_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x05, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x65u8);
    assert_eq!(cpu.psw.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_0a_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x0A, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x70u8);
    assert_eq!(cpu.psw.value, 0x13u8);
}
#[test]
///
/// Tests DAA
///
fn daa_0f_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x0f, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x75u8);
    assert_eq!(cpu.psw.value, 0x13u8);
}
#[test]
///
/// Tests DAA
///
fn daa_50_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x50, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xb0u8);
    assert_eq!(cpu.psw.value, 0x83u8);
}
#[test]
///
/// Tests DAA
///
fn daa_a0_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xa0, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.psw.value, 0x47u8);
}
#[test]
///
/// Tests DAA
///
fn daa_f0_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xf0, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x50u8);
    assert_eq!(cpu.psw.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_55_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xb5u8);
    assert_eq!(cpu.psw.value, 0x83u8);
}
#[test]
///
/// Tests DAA
///
fn daa_aa_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xaa, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.psw.value, 0x13u8);
}
#[test]
///
/// Tests DAA
///
fn daa_ff_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xff, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x65u8);
    assert_eq!(cpu.psw.value, 0x17u8);
}
#[test]
///
/// Tests DAA
///
fn daa_00_ac_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x00, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x66u8);
    assert_eq!(cpu.psw.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_05_ac_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x05, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x6bu8);
    assert_eq!(cpu.psw.value, 0x03u8);
}
#[test]
///
/// Tests DAA
///
fn daa_0a_ac_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x0A, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x70u8);
    assert_eq!(cpu.psw.value, 0x13u8);
}
#[test]
///
/// Tests DAA
///
fn daa_0f_ac_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x0f, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x75u8);
    assert_eq!(cpu.psw.value, 0x13u8);
}
#[test]
///
/// Tests DAA
///
fn daa_50_ac_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x50, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xb6u8);
    assert_eq!(cpu.psw.value, 0x83u8);
}
#[test]
///
/// Tests DAA
///
fn daa_a0_ac_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xa0, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x06u8);
    assert_eq!(cpu.psw.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_f0_ac_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xf0, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x56u8);
    assert_eq!(cpu.psw.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_55_ac_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xbbu8);
    assert_eq!(cpu.psw.value, 0x87u8);
}
#[test]
///
/// Tests DAA
///
fn daa_aa_ac_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xaa, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.psw.value, 0x13u8);
}
#[test]
///
/// Tests DAA
///
fn daa_ff_ac_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xff, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x65u8);
    assert_eq!(cpu.psw.value, 0x17u8);
}
#[test]
///
/// Tests DAA
///
fn daa_33_ac_c() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.psw.set_ac(true);
    cpu.psw.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x33, DAA,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x99u8);
    assert_eq!(cpu.psw.value, 0x87u8);
}
#[test]
///
/// Tests DAD B
///
fn dad_b_1() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0x00;
    cpu.l = 0x00;
    cpu.b = 0x00;
    cpu.c = 0x00;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A,DAD_B,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x00u8);
    assert_eq!(cpu.l, 0x00u8);
    assert_eq!(cpu.psw.value, 0x46u8);
}
#[test]
///
/// Tests DAD B
///
fn dad_b_2() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0x00;
    cpu.l = 0x00;
    cpu.b = 0x12;
    cpu.c = 0x34;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_B,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x12u8);
    assert_eq!(cpu.l, 0x34u8);
    assert_eq!(cpu.psw.value, 0x46u8);
}
#[test]
///
/// Tests DAD B
///
fn dad_b_3() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0x00;
    cpu.l = 0x00;
    cpu.b = 0x55;
    cpu.c = 0x55;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_B,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x55u8);
    assert_eq!(cpu.l, 0x55u8);
    assert_eq!(cpu.psw.value, 0x46u8);
}
#[test]
///
/// Tests DAD B
///
fn dad_b_4() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0x00;
    cpu.l = 0x00;
    cpu.b = 0xAA;
    cpu.c = 0xAA;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_B,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xAAu8);
    assert_eq!(cpu.l, 0xAAu8);
    assert_eq!(cpu.psw.value, 0x46u8);
}
#[test]
///
/// Tests DAD B
///
/// 
fn dad_b_5() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0x00;
    cpu.l = 0x00;
    cpu.b = 0xff;
    cpu.c = 0xff;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_B,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xffu8);
    assert_eq!(cpu.l, 0xffu8);
    assert_eq!(cpu.psw.value, 0x46u8);
}
#[test]
///
/// Tests DAD D
///
fn dad_d_1() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0xf0;
    cpu.l = 0xf0;
    cpu.d = 0x00;
    cpu.e = 0x00;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A,DAD_D,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xf0u8);
    assert_eq!(cpu.l, 0xf0u8);
    assert_eq!(cpu.psw.value, 0x46u8);
}
#[test]
///
/// Tests DAD D
///
fn dad_d_2() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0xf0;
    cpu.l = 0xf0;
    cpu.d = 0x12;
    cpu.e = 0x34;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_D,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x03u8);
    assert_eq!(cpu.l, 0x24u8);
    assert_eq!(cpu.psw.value, 0x47u8);
}
#[test]
///
/// Tests DAD D
///
fn dad_d_3() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0xf0;
    cpu.l = 0xf0;
    cpu.d = 0x55;
    cpu.e = 0x55;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_D,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x46u8);
    assert_eq!(cpu.l, 0x45u8);
    assert_eq!(cpu.psw.value, 0x47u8);
}
#[test]
///
/// Tests DAD D
///
fn dad_d_4() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0xf0;
    cpu.l = 0xf0;
    cpu.d = 0xAA;
    cpu.e = 0xAA;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_D,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x9bu8);
    assert_eq!(cpu.l, 0x9au8);
    assert_eq!(cpu.psw.value, 0x47u8);
}
#[test]
///
/// Tests DAD D
///
/// 
fn dad_d_5() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0xf0;
    cpu.l = 0xf0;
    cpu.d = 0xff;
    cpu.e = 0xff;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_D,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xf0u8);
    assert_eq!(cpu.l, 0xefu8);
    assert_eq!(cpu.psw.value, 0x47u8);
}
#[test]
///
/// Tests DAD H
///
fn dad_h_1() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0x00;
    cpu.l = 0x00;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A,DAD_H,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x00u8);
    assert_eq!(cpu.l, 0x00u8);
    assert_eq!(cpu.psw.value, 0x46u8);
}
#[test]
///
/// Tests DAD H
///
fn dad_h_2() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_H,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x24u8);
    assert_eq!(cpu.l, 0x68u8);
    assert_eq!(cpu.psw.value, 0x46u8);
}
#[test]
///
/// Tests DAD H
///
fn dad_h_3() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0x55;
    cpu.l = 0x55;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_H,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xaau8);
    assert_eq!(cpu.l, 0xaau8);
    assert_eq!(cpu.psw.value, 0x46u8);
}
#[test]
///
/// Tests DAD H
///
fn dad_h_4() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0xAA;
    cpu.l = 0xAA;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_H,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x55u8);
    assert_eq!(cpu.l, 0x54u8);
    assert_eq!(cpu.psw.value, 0x47u8);
}
#[test]
///
/// Tests DAD H
///
/// 
fn dad_h_5() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0xff;
    cpu.l = 0xff;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_H,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xffu8);
    assert_eq!(cpu.l, 0xfeu8);
    assert_eq!(cpu.psw.value, 0x47u8);
}
#[test]
///
/// Tests DAD SP
///
fn dad_sp_1() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0x0f;
    cpu.l = 0x0f;
    cpu.sp = 0x0000;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A,DAD_SP,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x0fu8);
    assert_eq!(cpu.l, 0x0fu8);
    assert_eq!(cpu.psw.value, 0x46u8);
}
#[test]
///
/// Tests DAD SP
///
fn dad_sp_2() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0x0f;
    cpu.l = 0x0f;
    cpu.sp = 0x1234;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_SP,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x21u8);
    assert_eq!(cpu.l, 0x43u8);
    assert_eq!(cpu.psw.value, 0x46u8);
}
#[test]
///
/// Tests DAD SP
///
fn dad_sp_3() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0x0f;
    cpu.l = 0x0f;
    cpu.sp = 0x5555;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_SP,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x64u8);
    assert_eq!(cpu.l, 0x64u8);
    assert_eq!(cpu.psw.value, 0x46u8);
}
#[test]
///
/// Tests DAD SP
///
fn dad_sp_4() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0x0f;
    cpu.l = 0x0f;
    cpu.sp = 0xaaaa;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_SP,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xb9u8);
    assert_eq!(cpu.l, 0xb9u8);
    assert_eq!(cpu.psw.value, 0x46u8);
}
#[test]
///
/// Tests DAD SP
///
/// 
fn dad_sp_5() {
    let mut cpu = Cpu::new();
    cpu.psw.clear_flags();
    cpu.h = 0x0f;
    cpu.l = 0x0f;
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_SP,  HLT,];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x0fu8);
    assert_eq!(cpu.l, 0x0eu8);
    assert_eq!(cpu.psw.value, 0x47u8);
}
/*

*/