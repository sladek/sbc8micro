#![doc(hidden)]
#[cfg(test)]
use crate::cpu::i8080::Cpu;
#[cfg(test)]
use crate::disassembler::i8080_opcode_consts::*;

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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xc9u8);
    assert_eq!(cpu.status.value, 0x86);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.status.value, 0x13);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x57);
}
#[test]
///
/// Tests immediate ACI
///
fn aci_2_z_c_ac_p() {
    let mut cpu = Cpu::new();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, ACI, 0xAA, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x57);
}
#[test]
///
/// Tests ADC B
///
fn adc_b_z_ac_p() {
    let mut cpu = Cpu::new();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_B, 0xaa, ADC_B, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x57);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.status.value, 0x86);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.status.value, 0x86);
}
#[test]
///
/// Tests ADC E
///
fn adc_e_z_ac_p() {
    let mut cpu = Cpu::new();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_E, 0xaa, ADC_E, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x57);
}
#[test]
///
/// Tests ADC H
///
fn adc_h_z_ac_p() {
    let mut cpu = Cpu::new();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_H, 0xaa, ADC_H, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x57);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.status.value, 0x86);
}
#[test]
///
/// Tests ADC A
///
fn adc_a_c_ac_neg() {
    let mut cpu = Cpu::new();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, ADC_A, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xABu8);
    assert_eq!(cpu.status.value, 0x82);
}
#[test]
///
/// Tests ADC M
///
fn adc_m_p() {
    let mut cpu = Cpu::new();
    cpu.memory.borrow_mut().write_byte(0x1234, 0x12);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0x34, ADC_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x47u8);
    assert_eq!(cpu.status.value, 0x06);
}
#[test]
///
/// Tests ADC M
///
fn adc_m_neg_ac_p_c() {
    let mut cpu = Cpu::new();
    cpu.memory.borrow_mut().write_byte(0x1234, 0xff);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0xAA, ADC_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x97);
}
#[test]
///
/// Tests ADC M
///
fn adc_m_neg_ac_p_c_2() {
    let mut cpu = Cpu::new();
    cpu.memory.borrow_mut().write_byte(0x1234, 0xff);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0xFF, ADC_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.status.value, 0x97);
}
#[test]
///
/// Tests ADC M
///
fn adc_m_ac_p_c() {
    let mut cpu = Cpu::new();
    cpu.memory.borrow_mut().write_byte(0x1234, 0xaa);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0xaa, ADC_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x17);
}
#[test]
///
/// Tests ADC M
///
fn adc_m_neg_ac_c() {
    let mut cpu = Cpu::new();
    cpu.memory.borrow_mut().write_byte(0x1234, 0xff);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0xff, ADC_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xfeu8);
    assert_eq!(cpu.status.value, 0x93);
}
#[test]
///
/// Tests ADC M
///
fn adc_m_ac_c() {
    let mut cpu = Cpu::new();
    cpu.memory.borrow_mut().write_byte(0x1234, 0xAA);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0xAA, ADC_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x54u8);
    assert_eq!(cpu.status.value, 0x13);
}

#[test]
///
/// Tests ADD B
///
fn add_b_neg_p() {
    let mut cpu = Cpu::new();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_B, 0xaa, ADD_B, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.status.value, 0x86);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.status.value, 0x86);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.status.value, 0x86);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.status.value, 0x13);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x57);
}
#[test]
///
/// Tests ADC H
///
fn add_h_z_ac_p() {
    let mut cpu = Cpu::new();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xff, MVI_H, 0xff, ADD_H, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xfeu8);
    assert_eq!(cpu.status.value, 0x93);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x54u8);
    assert_eq!(cpu.status.value, 0x13);
}
#[test]
///
/// Tests ADC A
///
fn add_a_c_ac_neg() {
    let mut cpu = Cpu::new();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, ADD_A, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xAAu8);
    assert_eq!(cpu.status.value, 0x86);
}
#[test]
///
/// Tests ADC M
///
fn add_m_p() {
    let mut cpu = Cpu::new();
    cpu.memory.borrow_mut().write_byte(0x1234, 0x12);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0x35, ADD_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x47u8);
    assert_eq!(cpu.status.value, 0x06);
}
#[test]
///
/// Tests ADC M
///
fn add_m() {
    let mut cpu = Cpu::new();
    cpu.memory.borrow_mut().write_byte(0x1234, 0x12);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0x34, ADD_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x46u8);
    assert_eq!(cpu.status.value, 0x02);
}
#[test]
///
/// Tests ADI
///
fn adi_p() {
    let mut cpu = Cpu::new();
    cpu.memory.borrow_mut().write_byte(0x1234, 0x12);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x34, ADI, 0x34, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x68u8);
    assert_eq!(cpu.status.value, 0x02);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x34u8);
    assert_eq!(cpu.status.value, 0x02);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.status.value, 0x02);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x56);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x56);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x56);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x34u8);
    assert_eq!(cpu.status.value, 0x02);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.status.value, 0x02);
}
#[test]
///
/// Tests ANA M
///
fn ana_m() {
    let mut cpu = Cpu::new();
    cpu.memory.borrow_mut().write_byte(0x200, 0x55);
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_H, 0x02, MVI_L, 0x00, ANA_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x06);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x06);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x08u8);
    assert_eq!(cpu.status.value, 0x12);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x56);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.status.value, 0x96);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x0Fu8);
    assert_eq!(cpu.status.value, 0x16);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xAAu8);
    assert_eq!(cpu.status.value, 0x02);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x03);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.status.value, 0x16);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x83);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x80u8);
    assert_eq!(cpu.status.value, 0x12);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x70u8);
    assert_eq!(cpu.status.value, 0x97);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x83);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x16);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x20u8);
    assert_eq!(cpu.status.value, 0x12);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.status.value, 0x97);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x05u8);
    assert_eq!(cpu.status.value, 0x83);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x0au8);
    assert_eq!(cpu.status.value, 0x16);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x05u8);
    assert_eq!(cpu.status.value, 0x56);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x56);
}
#[test]
///
/// Tests CMP M
///
fn cmp_m_05_01() {
    let mut cpu = Cpu::new();
    cpu.memory.borrow_mut().write_byte(0x1234, 0x05);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0x01, CMP_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
    assert_eq!(cpu.status.value, 0x87);
}
#[test]
///
/// Tests CMP M
///
fn cmp_m_01_05() {
    let mut cpu = Cpu::new();
    cpu.memory.borrow_mut().write_byte(0x1234, 0x01);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, MVI_A, 0x05, CMP_M, HLT];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x05u8);
    assert_eq!(cpu.status.value, 0x12);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x56);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x83);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xa5u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xa5u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xafu8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xa5u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaFu8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xa5u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xa4u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xafu8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x03u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xafu8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
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
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xa5u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests DAA
///
fn daa_00() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0x00, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Tests DAA
///
fn daa_05() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0x05, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x05u8);
    assert_eq!(cpu.status.value, 0x06u8);
}
#[test]
///
/// Tests DAA
///
fn daa_0a() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0x0A, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Tests DAA
///
fn daa_0f() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0x0f, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x15u8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Tests DAA
///
fn daa_50() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0x50, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x50u8);
    assert_eq!(cpu.status.value, 0x06u8);
}
#[test]
///
/// Tests DAA
///
fn daa_a0() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0xa0, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x47u8);
}
#[test]
///
/// Tests DAA
///
fn daa_f0() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0xf0, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x50u8);
    assert_eq!(cpu.status.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_55() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0x55, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x06u8);
}
#[test]
///
/// Tests DAA
///
fn daa_aa() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0xaa, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.status.value, 0x13u8);
}
#[test]
///
/// Tests DAA
///
fn daa_ff() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0xff, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x65u8);
    assert_eq!(cpu.status.value, 0x17u8);
}
#[test]
///
/// Tests DAA
///
fn daa_33() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0x33, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x33u8);
    assert_eq!(cpu.status.value, 0x06u8);
}
#[test]
///
/// Tests DAA
///
fn daa_00_ac() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0x00, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x06u8);
    assert_eq!(cpu.status.value, 0x06u8);
}
#[test]
///
/// Tests DAA
///
fn daa_05_ac() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0x05, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x0bu8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Tests DAA
///
fn daa_0a_ac() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0x0A, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Tests DAA
///
fn daa_0f_ac() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0x0f, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x15u8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Tests DAA
///
fn daa_50_ac() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0x50, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x56u8);
    assert_eq!(cpu.status.value, 0x06u8);
}
#[test]
///
/// Tests DAA
///
fn daa_a0_ac() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0xa0, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x06u8);
    assert_eq!(cpu.status.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_f0_ac() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0xf0, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x56u8);
    assert_eq!(cpu.status.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_55_ac() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x5bu8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Tests DAA
///
fn daa_aa_ac() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0xaa, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.status.value, 0x13u8);
}
#[test]
fn daa_0a_bcd() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0x0a, ADI, 0x90, DAA, ACI, 0x40, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x41u8);
    assert_eq!(cpu.status.value, 0x06u8);
}
///
/// Test DAA
///

#[test]
///
/// Tests DAA
///
fn daa_ff_ac() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0xff, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x65u8);
    assert_eq!(cpu.status.value, 0x17u8);
}
#[test]
///
/// Tests DAA
///
fn daa_33_ac() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    let program: Vec<u8> = vec![MVI_A, 0x33, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x39u8);
    assert_eq!(cpu.status.value, 0x06u8);
}
#[test]
///
/// Tests DAA
///
fn daa_00_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x00, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x60u8);
    assert_eq!(cpu.status.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_05_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x05, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x65u8);
    assert_eq!(cpu.status.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_0a_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x0A, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x70u8);
    assert_eq!(cpu.status.value, 0x13u8);
}
#[test]
///
/// Tests DAA
///
fn daa_0f_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x0f, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x75u8);
    assert_eq!(cpu.status.value, 0x13u8);
}
#[test]
///
/// Tests DAA
///
fn daa_50_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x50, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xb0u8);
    assert_eq!(cpu.status.value, 0x83u8);
}
#[test]
///
/// Tests DAA
///
fn daa_a0_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xa0, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x47u8);
}
#[test]
///
/// Tests DAA
///
fn daa_f0_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xf0, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x50u8);
    assert_eq!(cpu.status.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_55_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xb5u8);
    assert_eq!(cpu.status.value, 0x83u8);
}
#[test]
///
/// Tests DAA
///
fn daa_aa_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xaa, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.status.value, 0x13u8);
}
#[test]
///
/// Tests DAA
///
fn daa_ff_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xff, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x65u8);
    assert_eq!(cpu.status.value, 0x17u8);
}
#[test]
///
/// Tests DAA
///
fn daa_00_ac_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x00, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x66u8);
    assert_eq!(cpu.status.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_05_ac_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x05, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x6bu8);
    assert_eq!(cpu.status.value, 0x03u8);
}
#[test]
///
/// Tests DAA
///
fn daa_0a_ac_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x0A, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x70u8);
    assert_eq!(cpu.status.value, 0x13u8);
}
#[test]
///
/// Tests DAA
///
fn daa_0f_ac_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x0f, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x75u8);
    assert_eq!(cpu.status.value, 0x13u8);
}
#[test]
///
/// Tests DAA
///
fn daa_50_ac_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x50, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xb6u8);
    assert_eq!(cpu.status.value, 0x83u8);
}
#[test]
///
/// Tests DAA
///
fn daa_a0_ac_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xa0, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x06u8);
    assert_eq!(cpu.status.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_f0_ac_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xf0, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x56u8);
    assert_eq!(cpu.status.value, 0x07u8);
}
#[test]
///
/// Tests DAA
///
fn daa_55_ac_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xbbu8);
    assert_eq!(cpu.status.value, 0x87u8);
}
#[test]
///
/// Tests DAA
///
fn daa_aa_ac_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xaa, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.status.value, 0x13u8);
}
#[test]
///
/// Tests DAA
///
fn daa_ff_ac_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xff, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x65u8);
    assert_eq!(cpu.status.value, 0x17u8);
}
#[test]
///
/// Tests DAA
///
fn daa_33_ac_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.status.set_ac(true);
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x33, DAA, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x99u8);
    assert_eq!(cpu.status.value, 0x87u8);
}
#[test]
///
/// Tests DAD B
///
fn dad_b_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0x00;
    cpu.l = 0x00;
    cpu.b = 0x00;
    cpu.c = 0x00;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x00u8);
    assert_eq!(cpu.l, 0x00u8);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Tests DAD B
///
fn dad_b_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0x00;
    cpu.l = 0x00;
    cpu.b = 0x12;
    cpu.c = 0x34;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x12u8);
    assert_eq!(cpu.l, 0x34u8);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Tests DAD B
///
fn dad_b_3() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0x00;
    cpu.l = 0x00;
    cpu.b = 0x55;
    cpu.c = 0x55;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x55u8);
    assert_eq!(cpu.l, 0x55u8);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Tests DAD B
///
fn dad_b_4() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0x00;
    cpu.l = 0x00;
    cpu.b = 0xAA;
    cpu.c = 0xAA;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xAAu8);
    assert_eq!(cpu.l, 0xAAu8);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Tests DAD B
///
///
fn dad_b_5() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0x00;
    cpu.l = 0x00;
    cpu.b = 0xff;
    cpu.c = 0xff;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xffu8);
    assert_eq!(cpu.l, 0xffu8);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Tests DAD D
///
fn dad_d_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0xf0;
    cpu.l = 0xf0;
    cpu.d = 0x00;
    cpu.e = 0x00;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xf0u8);
    assert_eq!(cpu.l, 0xf0u8);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Tests DAD D
///
fn dad_d_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0xf0;
    cpu.l = 0xf0;
    cpu.d = 0x12;
    cpu.e = 0x34;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x03u8);
    assert_eq!(cpu.l, 0x24u8);
    assert_eq!(cpu.status.value, 0x47u8);
}
#[test]
///
/// Tests DAD D
///
fn dad_d_3() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0xf0;
    cpu.l = 0xf0;
    cpu.d = 0x55;
    cpu.e = 0x55;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x46u8);
    assert_eq!(cpu.l, 0x45u8);
    assert_eq!(cpu.status.value, 0x47u8);
}
#[test]
///
/// Tests DAD D
///
fn dad_d_4() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0xf0;
    cpu.l = 0xf0;
    cpu.d = 0xAA;
    cpu.e = 0xAA;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x9bu8);
    assert_eq!(cpu.l, 0x9au8);
    assert_eq!(cpu.status.value, 0x47u8);
}
#[test]
///
/// Tests DAD D
///
///
fn dad_d_5() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0xf0;
    cpu.l = 0xf0;
    cpu.d = 0xff;
    cpu.e = 0xff;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xf0u8);
    assert_eq!(cpu.l, 0xefu8);
    assert_eq!(cpu.status.value, 0x47u8);
}
#[test]
///
/// Tests DAD H
///
fn dad_h_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0x00;
    cpu.l = 0x00;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x00u8);
    assert_eq!(cpu.l, 0x00u8);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Tests DAD H
///
fn dad_h_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x24u8);
    assert_eq!(cpu.l, 0x68u8);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Tests DAD H
///
fn dad_h_3() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0x55;
    cpu.l = 0x55;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xaau8);
    assert_eq!(cpu.l, 0xaau8);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Tests DAD H
///
fn dad_h_4() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0xAA;
    cpu.l = 0xAA;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x55u8);
    assert_eq!(cpu.l, 0x54u8);
    assert_eq!(cpu.status.value, 0x47u8);
}
#[test]
///
/// Tests DAD H
///
///
fn dad_h_5() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0xff;
    cpu.l = 0xff;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xffu8);
    assert_eq!(cpu.l, 0xfeu8);
    assert_eq!(cpu.status.value, 0x47u8);
}
#[test]
///
/// Tests DAD SP
///
fn dad_sp_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0x0f;
    cpu.l = 0x0f;
    cpu.sp = 0x0000;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_SP, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x0fu8);
    assert_eq!(cpu.l, 0x0fu8);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Tests DAD SP
///
fn dad_sp_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0x0f;
    cpu.l = 0x0f;
    cpu.sp = 0x1234;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_SP, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x21u8);
    assert_eq!(cpu.l, 0x43u8);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Tests DAD SP
///
fn dad_sp_3() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0x0f;
    cpu.l = 0x0f;
    cpu.sp = 0x5555;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_SP, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x64u8);
    assert_eq!(cpu.l, 0x64u8);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Tests DAD SP
///
fn dad_sp_4() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0x0f;
    cpu.l = 0x0f;
    cpu.sp = 0xaaaa;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_SP, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xb9u8);
    assert_eq!(cpu.l, 0xb9u8);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Tests DAD SP
///
///
fn dad_sp_5() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0x0f;
    cpu.l = 0x0f;
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, DAD_SP, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x0fu8);
    assert_eq!(cpu.l, 0x0eu8);
    assert_eq!(cpu.status.value, 0x47u8);
}
#[test]
///
/// Test DCR B
///
fn dcr_b() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_B, 0x01, DCR_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0x00u8);
    assert_eq!(cpu.status.value, 0x56u8);
}
#[test]
///
/// Test DCR C
///
fn dcr_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_C, 0x10, DCR_C, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.c, 0x0fu8);
    assert_eq!(cpu.status.value, 0x06u8);
}
#[test]
///
/// Test DCR D
///
fn dcr_d() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_D, 0x00, DCR_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0xffu8);
    assert_eq!(cpu.status.value, 0x86u8);
}
#[test]
///
/// Test DCR E
///
fn dcr_e() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_E, 0xFF, DCR_E, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.e, 0xfeu8);
    assert_eq!(cpu.status.value, 0x92u8);
}
#[test]
///
/// Test DCR H
///
fn dcr_h() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_H, 0x0F, DCR_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x0eu8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Test DCR L
///
fn dcr_l() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_L, 0x90, DCR_L, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.l, 0x8fu8);
    assert_eq!(cpu.status.value, 0x82u8);
}
#[test]
///
/// Test DCR M
///
fn dcr_m() {
    let addr = 0x210;
    let mut cpu = Cpu::new();
    cpu.memory.borrow_mut().write_byte(0x0210, 0xff);
    cpu.status.clear_flags();
    cpu.h = 0x02;
    cpu.l = 0x10;
    let program: Vec<u8> = vec![DCR_M, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let result = cpu.memory.borrow_mut().read_byte(addr);
    assert_eq!(result, 0xfeu8);
    assert_eq!(cpu.status.value, 0x92u8);
}
#[test]
///
/// Test DCR A
///
fn dcr_a() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0xfe, DCR_A, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xfdu8);
    assert_eq!(cpu.status.value, 0x92u8);
}
#[test]
///
/// Test DCX B
///
fn dcx_b_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.b = 0x12u8;
    cpu.c = 0x34u8;
    let program: Vec<u8> = vec![DCX_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0x12u8);
    assert_eq!(cpu.c, 0x33u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test DCX B
///
fn dcx_b_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.b = 0x00u8;
    cpu.c = 0x00u8;
    let program: Vec<u8> = vec![DCX_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0xffu8);
    assert_eq!(cpu.c, 0xffu8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test DCX D
///
fn dcx_d_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.d = 0x12u8;
    cpu.e = 0x34u8;
    let program: Vec<u8> = vec![DCX_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0x12u8);
    assert_eq!(cpu.e, 0x33u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test DCX D
///
fn dcx_d_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.d = 0x00u8;
    cpu.e = 0x00u8;
    let program: Vec<u8> = vec![DCX_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0xffu8);
    assert_eq!(cpu.e, 0xffu8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test DCX H
///
fn dcx_h_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0x12u8;
    cpu.l = 0x34u8;
    let program: Vec<u8> = vec![DCX_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x12u8);
    assert_eq!(cpu.l, 0x33u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test DCX H
///
fn dcx_h_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0x00u8;
    cpu.l = 0x00u8;
    let program: Vec<u8> = vec![DCX_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xffu8);
    assert_eq!(cpu.l, 0xffu8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test DCX SP
///
fn dcx_sp_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    let program: Vec<u8> = vec![DCX_SP, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.sp, 0x1233u16);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test DCX SP
///
fn dcx_sp_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x0000u16;
    let program: Vec<u8> = vec![DCX_SP, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.sp, 0xffffu16);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test DI
///
fn di() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.inte = true;
    let program: Vec<u8> = vec![DI, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.inte, false);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test EI
///
fn ei() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.inte = false;
    let program: Vec<u8> = vec![EI, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.inte, true);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test IN
///
fn inp() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.a = 0x00;
    let program: Vec<u8> = vec![IN, 0x0f, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test INR B
///
fn inr_b() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_B, 0x0f, INR_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0x10u8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Test INR C
///
fn inr_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_C, 0xef, INR_C, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.c, 0xf0u8);
    assert_eq!(cpu.status.value, 0x96u8);
}
#[test]
///
/// Test INR D
///
fn inr_d() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_D, 0x1f, INR_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0x20u8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Test INR E
///
fn inr_e() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_E, 0xf0, INR_E, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.e, 0xf1u8);
    assert_eq!(cpu.status.value, 0x82u8);
}
#[test]
///
/// Test INR H
///
fn inr_h() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_H, 0xff, INR_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x00u8);
    assert_eq!(cpu.status.value, 0x56u8);
}
#[test]
///
/// Test INR L
///
fn inr_l() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_L, 0xef, INR_L, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.l, 0xf0u8);
    assert_eq!(cpu.status.value, 0x96u8);
}
#[test]
///
/// Test INR M
///
fn inr_m() {
    let addr = 0x210;
    let mut cpu = Cpu::new();
    cpu.memory.borrow_mut().write_byte(0x0210, 0xff);
    cpu.status.clear_flags();
    cpu.h = 0x02;
    cpu.l = 0x10;
    let program: Vec<u8> = vec![INR_M, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let result = cpu.memory.borrow_mut().read_byte(addr);
    assert_eq!(result, 0x00u8);
    assert_eq!(cpu.status.value, 0x56u8);
}
#[test]
///
/// Test INR A
///
fn inr_a() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    let program: Vec<u8> = vec![MVI_A, 0xff, INR_A, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x56u8);
}

#[test]
///
/// Test INX B
///
fn inx_b_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.b = 0x12u8;
    cpu.c = 0x34u8;
    let program: Vec<u8> = vec![INX_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0x12u8);
    assert_eq!(cpu.c, 0x35u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test INX B
///
fn inx_b_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.b = 0xffu8;
    cpu.c = 0xffu8;
    let program: Vec<u8> = vec![INX_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0x00u8);
    assert_eq!(cpu.c, 0x00u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test INX D
///
fn inx_d_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.d = 0x12u8;
    cpu.e = 0x34u8;
    let program: Vec<u8> = vec![INX_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0x12u8);
    assert_eq!(cpu.e, 0x35u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test INX D
///
fn inx_d_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.d = 0xffu8;
    cpu.e = 0xffu8;
    let program: Vec<u8> = vec![INX_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0x00u8);
    assert_eq!(cpu.e, 0x00u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test INX H
///
fn inx_h_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0x12u8;
    cpu.l = 0x34u8;
    let program: Vec<u8> = vec![INX_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x12u8);
    assert_eq!(cpu.l, 0x35u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test INX H
///
fn inx_h_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.h = 0xffu8;
    cpu.l = 0xffu8;
    let program: Vec<u8> = vec![INX_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x00u8);
    assert_eq!(cpu.l, 0x00u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test INX SP
///
fn inx_sp_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    let program: Vec<u8> = vec![INX_SP, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.sp, 0x1235u16);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test INX SP
///
fn inx_sp_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![INX_SP, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.sp, 0x0000u16);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test JNZ
///
fn jnz_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0x0f, ANA_A, JNZ, 0x07, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0209u16);
    assert_eq!(cpu.status.value, 0x16u8);
}
#[test]
///
/// Test JNZ
///
fn jnz_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, JNZ, 0x07, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0207u16);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Test JZ
///
fn jz_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0x0f, ANA_A, JZ, 0x07, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0207u16);
    assert_eq!(cpu.status.value, 0x16u8);
}
#[test]
///
/// Test JZ
///
fn jz_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, JZ, 0x07, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0209u16);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Test JNC
///
fn jnc_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x0f, JNC, 0x06, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0208u16);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test JNC
///
fn jnc_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x0f, JNC, 0x06, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0206u16);
    assert_eq!(cpu.status.value, 0x03u8);
}
#[test]
///
/// Test JNC
///
fn jc_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x0f, JC, 0x06, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0208u16);
    assert_eq!(cpu.status.value, 0x03u8);
}
#[test]
///
/// Test JNC
///
fn jc_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x0f, JC, 0x06, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0206u16);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test JPO
///
fn jpo_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.status.set_parity(false);
    let program: Vec<u8> = vec![MVI_A, 0x00, JPO, 0x06, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0208u16);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test JPO
///
fn jpo_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0x00, ANA_A, JPO, 0x07, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0207u16);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Test JPE
///
fn jpe_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.status.set_parity(false);
    let program: Vec<u8> = vec![MVI_A, 0x00, JPE, 0x06, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0206u16);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test JPE
///
fn jpe_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.status.set_parity(false);
    let program: Vec<u8> = vec![MVI_A, 0x00, JPE, 0x06, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0206u16);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test JP
///
fn jp_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0x0F, ANA_A, JP, 0x07, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0209u16);
    assert_eq!(cpu.status.value, 0x16u8);
}
#[test]
///
/// Test JP
///
fn jp_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0xFF, ANA_A, JP, 0x07, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0207u16);
    assert_eq!(cpu.status.value, 0x96u8);
}
#[test]
///
/// Test JM
///
fn jm_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0x0F, ANA_A, JM, 0x07, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0207u16);
    assert_eq!(cpu.status.value, 0x16u8);
}
#[test]
///
/// Test JP
///
fn jm_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0xFF, ANA_A, JM, 0x07, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0209u16);
    assert_eq!(cpu.status.value, 0x96u8);
}
#[test]
///
/// Test JMP
///
fn jmp() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![JMP, 0x04, 0x02, HLT, 00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.pc, 0x0206u16);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test LDA
///
fn lda() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.memory.borrow_mut().write_byte(0x220, 0x55);
    let program: Vec<u8> = vec![LDA, 0x20, 0x02, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test LDAX
///
fn ldax_b() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.b = 0x12;
    cpu.c = 0x34;
    cpu.memory.borrow_mut().write_byte(0x1234, 0x55);
    let program: Vec<u8> = vec![LDAX_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test LDAX
///
fn ldax_d() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.d = 0x12;
    cpu.e = 0x34;
    cpu.memory.borrow_mut().write_byte(0x1234, 0xaa);
    let program: Vec<u8> = vec![LDAX_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test LHLD
///
fn lhld() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.d = 0x12;
    cpu.e = 0x34;
    cpu.memory.borrow_mut().write_byte(0x1234, 0x55);
    cpu.memory.borrow_mut().write_byte(0x1235, 0xaa);
    let program: Vec<u8> = vec![LHLD, 0x34, 0x12, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.l, 0x55u8);
    assert_eq!(cpu.h, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test LXI B
///
fn lxi_b() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![LXI_B, 0x34, 0x12, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0x12u8);
    assert_eq!(cpu.c, 0x34u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test LXI D
///
fn lxi_d() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![LXI_D, 0x34, 0x12, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0x12u8);
    assert_eq!(cpu.e, 0x34u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test LXI H
///
fn lxi_h() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![LXI_H, 0x34, 0x12, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x12u8);
    assert_eq!(cpu.l, 0x34u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test LXI SP
///
fn lxi_sp() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![LXI_SP, 0x34, 0x12, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.sp, 0x1234u16);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MVI B
///
fn mvi_b() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_B, 0x12, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0x12u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MVI C
///
fn mvi_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_C, 0x34, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.c, 0x34u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MVI D
///
fn mvi_d() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_D, 0x55, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0x55u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MVI E
///
fn mvi_e() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_E, 0xaa, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.e, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MVI H
///
fn mvi_h() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_H, 0x12, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x12u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MVI L
///
fn mvi_l() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_L, 0x34, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.l, 0x34u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MVI M
///
fn mvi_m() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_M, 0x55, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let val = cpu.memory.borrow_mut().read_byte(0x1234);
    assert_eq!(val, 0x55u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MVI A
///
fn mvi_a() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0xaa, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV A,B
///
fn mov_a_b() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_B, 0xaa, MOV_A_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV A,C
///
fn mov_a_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_C, 0x55, MOV_A_C, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV A,D
///
fn mov_a_d() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_D, 0xaa, MOV_A_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV A,E
///
fn mov_a_e() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_E, 0xaa, MOV_A_E, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV A,H
///
fn mov_a_h() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_H, 0xaa, MOV_A_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV A,L
///
fn mov_a_l() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_L, 0xaa, MOV_A_L, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV A,M
///
fn mov_a_m() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    cpu.memory.borrow_mut().write_byte(0x1234, 0xaa);
    let program: Vec<u8> = vec![MOV_A_M, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV A,A
///
fn mov_a_a() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0xaa, MOV_A_A, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV B,B
///
fn mov_b_b() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_B, 0xaa, MOV_B_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV B,C
///
fn mov_b_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_C, 0x55, MOV_B_C, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0x55u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV B,D
///
fn mov_b_d() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_D, 0xaa, MOV_B_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV B,E
///
fn mov_b_e() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_E, 0xaa, MOV_B_E, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV B,H
///
fn mov_b_h() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_H, 0xaa, MOV_B_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV B,L
///
fn mov_b_l() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_L, 0xaa, MOV_B_L, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV B,M
///
fn mov_b_m() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    cpu.memory.borrow_mut().write_byte(0x1234, 0xaa);
    let program: Vec<u8> = vec![MOV_B_M, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV B,A
///
fn mov_b_a() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0xaa, MOV_B_A, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV C,B
///
fn mov_c_b() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_B, 0xaa, MOV_C_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.c, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV C,C
///
fn mov_c_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_C, 0x55, MOV_C_C, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.c, 0x55u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV C,D
///
fn mov_c_d() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_D, 0xaa, MOV_C_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.c, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV C,E
///
fn mov_c_e() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_E, 0xaa, MOV_C_E, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.c, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV C,H
///
fn mov_c_h() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_H, 0xaa, MOV_C_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.c, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV C,L
///
fn mov_c_l() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_L, 0xaa, MOV_C_L, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.c, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV C,M
///
fn mov_c_m() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    cpu.memory.borrow_mut().write_byte(0x1234, 0xaa);
    let program: Vec<u8> = vec![MOV_C_M, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.c, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV C,A
///
fn mov_c_a() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0xaa, MOV_C_A, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.c, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV D,B
///
fn mov_d_b() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_B, 0xaa, MOV_D_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV D,C
///
fn mov_d_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_C, 0x55, MOV_D_C, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0x55u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV D,D
///
fn mov_d_d() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_D, 0xaa, MOV_D_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV D,E
///
fn mov_d_e() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_E, 0xaa, MOV_D_E, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV D,H
///
fn mov_d_h() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_H, 0xaa, MOV_D_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV D,L
///
fn mov_d_l() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_L, 0xaa, MOV_D_L, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV D,M
///
fn mov_d_m() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    cpu.memory.borrow_mut().write_byte(0x1234, 0xaa);
    let program: Vec<u8> = vec![MOV_D_M, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV D,A
///
fn mov_d_a() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0xaa, MOV_D_A, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV E,B
///
fn mov_e_b() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_B, 0xaa, MOV_E_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.e, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV E,C
///
fn mov_e_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_C, 0x55, MOV_E_C, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.e, 0x55u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV E,D
///
fn mov_e_d() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_D, 0xaa, MOV_E_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.e, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV E,E
///
fn mov_e_e() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_E, 0xaa, MOV_E_E, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.e, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV E,H
///
fn mov_e_h() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_H, 0xaa, MOV_E_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.e, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV E,L
///
fn mov_e_l() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_L, 0xaa, MOV_E_L, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.e, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV E,M
///
fn mov_e_m() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    cpu.memory.borrow_mut().write_byte(0x1234, 0xaa);
    let program: Vec<u8> = vec![MOV_E_M, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.e, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV E,A
///
fn mov_e_a() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0xaa, MOV_E_A, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.e, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV H,B
///
fn mov_h_b() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_B, 0xaa, MOV_H_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV H,C
///
fn mov_h_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_C, 0x55, MOV_H_C, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x55u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV H,D
///
fn mov_h_d() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_D, 0xaa, MOV_H_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV H,E
///
fn mov_h_e() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_E, 0xaa, MOV_H_E, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV H,H
///
fn mov_h_h() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_H, 0xaa, MOV_H_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV H,L
///
fn mov_h_l() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_L, 0xaa, MOV_H_L, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV H,M
///
fn mov_h_m() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    cpu.memory.borrow_mut().write_byte(0x1234, 0xaa);
    let program: Vec<u8> = vec![MOV_H_M, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV H,A
///
fn mov_h_a() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0xaa, MOV_H_A, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
//------------------------
#[test]
///
/// Test MOV L,B
///
fn mov_l_b() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_B, 0xaa, MOV_L_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.l, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV L,C
///
fn mov_l_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_C, 0x55, MOV_L_C, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.l, 0x55u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV L,D
///
fn mov_l_d() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_D, 0xaa, MOV_L_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.l, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV L,E
///
fn mov_l_e() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_E, 0xaa, MOV_L_E, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.l, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV L,H
///
fn mov_l_h() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_H, 0xaa, MOV_L_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.l, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV L,L
///
fn mov_l_l() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_L, 0xaa, MOV_L_L, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.l, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV L,M
///
fn mov_l_m() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    cpu.memory.borrow_mut().write_byte(0x1234, 0xaa);
    let program: Vec<u8> = vec![MOV_L_M, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.l, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV L,A
///
fn mov_l_a() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    let program: Vec<u8> = vec![MVI_A, 0xaa, MOV_L_A, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.l, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV M,B
///
fn mov_m_b() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_B, 0xaa, MOV_M_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let value = cpu.memory.borrow_mut().read_byte(0x1234);
    assert_eq!(value, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV M,C
///
fn mov_m_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_C, 0xaa, MOV_M_C, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let value = cpu.memory.borrow_mut().read_byte(0x1234);
    assert_eq!(value, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV M,D
///
fn mov_m_d() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_D, 0xaa, MOV_M_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let value = cpu.memory.borrow_mut().read_byte(0x1234);
    assert_eq!(value, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV M,E
///
fn mov_m_e() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_E, 0xaa, MOV_M_E, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let value = cpu.memory.borrow_mut().read_byte(0x1234);
    assert_eq!(value, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV M,H
///
fn mov_m_h() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MOV_M_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let value = cpu.memory.borrow_mut().read_byte(0x1234);
    assert_eq!(value, 0x12u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV M,L
///
fn mov_m_l() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MOV_M_L, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let value = cpu.memory.borrow_mut().read_byte(0x1234);
    assert_eq!(value, 0x34u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test MOV M,A
///
fn mov_m_a() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_A, 0xaa, MOV_M_A, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let value = cpu.memory.borrow_mut().read_byte(0x1234);
    assert_eq!(value, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test ORA B
///
fn ora_b() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_A, 0xaa, MVI_B, 0x55, ORA_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.status.value, 0x86u8);
}
#[test]
///
/// Test ORA C
///
fn ora_c() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_C, 0xaa, ORA_C, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.status.value, 0x86u8);
}
#[test]
///
/// Test ORA D
///
fn ora_d() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_D, 0x55, ORA_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x06u8);
}
#[test]
///
/// Test ORA E
///
fn ora_e() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_A, 0xaa, MVI_E, 0xaa, ORA_E, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x86u8);
}
#[test]
///
/// Test ORA H
///
fn ora_h() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_A, 0x00, MVI_H, 0x00, ORA_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x46u8);
}
#[test]
///
/// Test ORA L
///
fn ora_l() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_A, 0xf0, MVI_L, 0x0f, ORA_L, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.status.value, 0x86u8);
}
#[test]
///
/// Test ORA M
///
fn ora_m() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    cpu.memory.borrow_mut().write_byte(0x1234, 0xf0);
    let program: Vec<u8> = vec![MVI_A, 0x0f, ORA_M, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.status.value, 0x86u8);
}
#[test]
///
/// Test ORA A
///
fn ora_a() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_L, ORA_A, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test ORI data
///
fn ori() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_A, 0x55, ORI, 0xaa, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.status.value, 0x86u8);
}
#[test]
///
/// Test PCHL
///
fn pchl() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0xffffu16;
    cpu.h = 0x12;
    cpu.l = 0x34;
    let program: Vec<u8> = vec![MVI_A, 0x55, LXI_H, 0x07, 0x02, PCHL, HLT, MVI_A, 0xaa, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test POP B
///
fn pop_b() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    cpu.memory.borrow_mut().write_byte(0x1234, 0x55);
    cpu.memory.borrow_mut().write_byte(0x1235, 0xaa);
    let program: Vec<u8> = vec![POP_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.b, 0xaau8);
    assert_eq!(cpu.c, 0x55u8);
    assert_eq!(cpu.sp, 0x1236);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test POP D
///
fn pop_d() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    cpu.memory.borrow_mut().write_byte(0x1234, 0x12);
    cpu.memory.borrow_mut().write_byte(0x1235, 0x34);
    let program: Vec<u8> = vec![POP_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0x34u8);
    assert_eq!(cpu.e, 0x12u8);
    assert_eq!(cpu.sp, 0x1236);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test POP H
///
fn pop_h() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    cpu.memory.borrow_mut().write_byte(0x1234, 0xaa);
    cpu.memory.borrow_mut().write_byte(0x1235, 0x55);
    let program: Vec<u8> = vec![POP_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.d, 0x55u8);
    assert_eq!(cpu.e, 0xaau8);
    assert_eq!(cpu.sp, 0x1236);
    assert_eq!(cpu.status.value, 0x02u8);
}
#[test]
///
/// Test POP PSW
///
fn pop_psw() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    cpu.memory.borrow_mut().write_byte(0x1234, 0x56);
    cpu.memory.borrow_mut().write_byte(0x1235, 0xaa);
    let program: Vec<u8> = vec![POP_PSW, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x56u8);
    assert_eq!(cpu.sp, 0x1236);
}
#[test]
///
/// Test PUSH B
///
fn push_b() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    let program: Vec<u8> = vec![LXI_B, 0x12, 0x34, PUSH_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let mut val = cpu.memory.borrow_mut().read_byte(0x1232);
    assert_eq!(val, 0x12u8);
    val = cpu.memory.borrow_mut().read_byte(0x1233);
    assert_eq!(val, 0x34u8);
    assert_eq!(cpu.sp, 0x1232);
}
#[test]
///
/// Test PUSH D
///
fn push_d() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    let program: Vec<u8> = vec![LXI_D, 0x55, 0xaa, PUSH_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let mut val = cpu.memory.borrow_mut().read_byte(0x1232);
    assert_eq!(val, 0x55u8);
    val = cpu.memory.borrow_mut().read_byte(0x1233);
    assert_eq!(val, 0xaau8);
    assert_eq!(cpu.sp, 0x1232);
}
#[test]
///
/// Test PUSH H
///
fn push_h() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    let program: Vec<u8> = vec![LXI_H, 0x34, 0x12, PUSH_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let mut val = cpu.memory.borrow_mut().read_byte(0x1232);
    assert_eq!(val, 0x34u8);
    val = cpu.memory.borrow_mut().read_byte(0x1233);
    assert_eq!(val, 0x12u8);
    assert_eq!(cpu.sp, 0x1232);
}
#[test]
///
/// Test PUSH PSW
///
fn push_psw() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    let program: Vec<u8> = vec![MVI_A, 0x55, PUSH_PSW, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let mut val = cpu.memory.borrow_mut().read_byte(0x1232);
    assert_eq!(val, 0x02u8);
    val = cpu.memory.borrow_mut().read_byte(0x1233);
    assert_eq!(val, 0x55u8);
    assert_eq!(cpu.sp, 0x1232);
}
#[test]
///
/// Test RAL
///
fn ral_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    let program: Vec<u8> = vec![MVI_A, 0x55, RAL, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x02);
}
#[test]
///
/// Test RAL
///
fn ral_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, RAL, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xabu8);
    assert_eq!(cpu.status.value, 0x02);
}
#[test]
///
/// Test RAL
///
fn ral_3() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xaa, RAL, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x03);
}
#[test]
///
/// Test RAR
///
fn rar_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    let program: Vec<u8> = vec![MVI_A, 0xaa, RAR, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x02);
}
#[test]
///
/// Test RAR
///
fn rar_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0xab, RAR, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xd5u8);
    assert_eq!(cpu.status.value, 0x03);
}
#[test]
///
/// Test RLC
///
fn rlc_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0xaa, RLC, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x03);
}
#[test]
///
/// Test RLC
///
fn rlc_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x55, RLC, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x02);
}
#[test]
///
/// Test RRC
///
fn rrc_1() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0xaa, RRC, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x02);
}
#[test]
///
/// Test RRC
///
fn rrc_2() {
    let mut cpu = Cpu::new();
    cpu.status.clear_flags();
    cpu.sp = 0x1234u16;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0xab, RRC, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xd5u8);
    assert_eq!(cpu.status.value, 0x03);
}
#[test]
///
/// Tests RET
///
fn ret() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xa5u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RNZ
///
fn rnz_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RNZ, ANI, 0x0f, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xa5u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RNZ
///
fn rnz_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0x00, RNZ, ANI, 0x0f, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RZ
///
fn rz_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0xf5, RZ, ANI, 0x0f, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x05u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RZ
///
fn rz_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0x00, RZ, ANI, 0x0f, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RNC
///
fn rnc_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0x00, RNC, ANI, 0x0f, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RNC
///
fn rnc_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaf, HLT, ADI, 0x01, RNC, ANI, 0x0f, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RC
///
fn rc_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0x5f, RC, ANI, 0xfa, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x0au8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RC
///
fn rc_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaf, HLT, ADI, 0x01, RC, ANI, 0x0f, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RPO
///
fn rpo_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0x00, RPO, ANI, 0xfa, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RPO
///
fn rpo_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0x01, RPO, ANI, 0x0f, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RPE
///
fn rpe_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0x05, RPE, ORI, 0xff, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x05u8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RPE
///
fn rpe_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaf, HLT, ANI, 0x04, RPE, ORI, 0xff, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xafu8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RP
///
fn rp_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaa, HLT, ANI, 0x7f, RP, ORI, 0xff, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x2au8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RP
///
fn rp_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaa, HLT, ANI, 0xf0, RP, ORI, 0xff, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RM
///
fn rm_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaa, HLT, ANI, 0x8f, RM, ORI, 0xff, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x8au8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RM
///
fn rm_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let program: Vec<u8> = vec![
        MVI_A, 0xff, CALL, 0x08, 0x02, ANI, 0xaa, HLT, ANI, 0x7f, RM, ORI, 0xff, RET,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.sp, 0xffff);
    let addr = 0xfffd;
    let val = cpu.memory.borrow_mut().read_word(addr);
    assert_eq!(val, 0x0205);
}
#[test]
///
/// Tests RST 0
///
fn rst_0() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let rst = vec![MVI_A, 0x55, ANI, 0xff, RET];
    let program: Vec<u8> = vec![MVI_A, 0xff, RST_0, HLT];
    cpu.load_program(&rst, 0x0000);
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
}
#[test]
///
/// Tests RST 1
///
fn rst_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let rst = vec![MVI_A, 0x55, ANI, 0xaa, RZ, MVI_A, 0xaa, RET];
    let program: Vec<u8> = vec![MVI_A, 0xff, RST_1, NOP, HLT];
    cpu.load_program(&rst, 0x0008);
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
}
#[test]
///
/// Tests RST 2
///
fn rst_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let rst = vec![MVI_A, 0x55, ANI, 0x55, RZ, MVI_A, 0xaa, RET];
    let program: Vec<u8> = vec![MVI_A, 0xff, RST_2, NOP, HLT];
    cpu.load_program(&rst, 0x0010);
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
}
#[test]
///
/// Tests RST 3
///
fn rst_3() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let rst = vec![MVI_A, 0xff, ADI, 0x01, RC, MVI_A, 0xaa, RET];
    let program: Vec<u8> = vec![MVI_A, 0xff, RST_3, NOP, HLT];
    cpu.load_program(&rst, 0x018);
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x0u8);
}
#[test]
///
/// Tests RST 4
///
fn rst_4() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let rst = vec![MVI_A, 0xff, ADI, 0x00, RNC, MVI_A, 0xaa, RET];
    let program: Vec<u8> = vec![MVI_A, 0xff, RST_4, NOP, HLT];
    cpu.load_program(&rst, 0x0020);
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
}
#[test]
///
/// Tests RST 5
///
fn rst_5() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let rst = vec![MVI_A, 0xff, ADI, 0x02, RPO, MVI_A, 0xaa, RET];
    let program: Vec<u8> = vec![MVI_A, 0xff, RST_5, NOP, HLT];
    cpu.load_program(&rst, 0x0028);
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
}
#[test]
///
/// Tests RST 6
///
fn rst_6() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let rst = vec![MVI_A, 0xff, ADI, 0x02, RPE, MVI_A, 0xaa, RET];
    let program: Vec<u8> = vec![MVI_A, 0xff, RST_6, NOP, HLT];
    cpu.load_program(&rst, 0x0030);
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
}
#[test]
///
/// Tests RST 7
///
fn rst_7() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    let rst = vec![MVI_A, 0xff, ADI, 0x02, RP, MVI_A, 0xaa, RET];
    let program: Vec<u8> = vec![MVI_A, 0xff, RST_7, NOP, HLT];
    cpu.load_program(&rst, 0x0038);
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
}
#[test]
///
/// Tests SBB B
///
fn sbb_b_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x0f, MVI_B, 0x0f, SBB_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x0u8);
    assert_eq!(cpu.status.value, 0x56u8);
}
#[test]
///
/// Tests SBB B
///
fn sbb_b_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x0f, MVI_B, 0x0f, SBB_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.status.value, 0x87u8);
}
#[test]
///
/// Tests SBB B
///
fn sbb_b_3() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x00, MVI_B, 0x0f, SBB_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xf1u8);
    assert_eq!(cpu.status.value, 0x83u8);
}
#[test]
///
/// Tests SBB B
///
fn sbb_b_4() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x00, MVI_B, 0x0f, SBB_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xf0u8);
    assert_eq!(cpu.status.value, 0x87u8);
}
#[test]
///
/// Tests SBB B
///
fn sbb_b_5() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x0f, MVI_B, 0x00, SBB_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x0fu8);
    assert_eq!(cpu.status.value, 0x16u8);
}
#[test]
///
/// Tests SBB B
///
fn sbb_b_6() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x0f, MVI_B, 0x00, SBB_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x0eu8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Tests SBB B
///
fn sbb_b_7() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x00, MVI_B, 0xff, SBB_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
    assert_eq!(cpu.status.value, 0x03u8);
}
#[test]
///
/// Tests SBB B
///
fn sbb_b_8() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x00, MVI_B, 0xff, SBB_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x47u8);
}
#[test]
///
/// Tests SBB B
///
fn sbb_b_9() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x20, MVI_B, 0x10, SBB_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Tests SBB C
///
fn sbb_c_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x0f, MVI_C, 0x0ff, SBB_C, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.status.value, 0x13u8);
}
#[test]
///
/// Tests SBB C
///
fn sbb_c_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x0f, MVI_C, 0x0ff, SBB_C, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x0fu8);
    assert_eq!(cpu.status.value, 0x07u8);
}
#[test]
///
/// Tests SBB D
///
fn sbb_d_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x0ff, MVI_D, 0x0f, SBB_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xf0u8);
    assert_eq!(cpu.status.value, 0x96u8);
}
#[test]
///
/// Tests SBB D
///
fn sbb_d_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x0ff, MVI_D, 0x0f, SBB_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xefu8);
    assert_eq!(cpu.status.value, 0x82u8);
}
#[test]
///
/// Tests SBB E
///
fn sbb_e_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x04, MVI_E, 0x02, SBB_E, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x02u8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Tests SBB E
///
fn sbb_e_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x04, MVI_E, 0x02, SBB_E, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Tests SBB H
///
fn sbb_h_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x02, MVI_H, 0x04, SBB_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xfeu8);
    assert_eq!(cpu.status.value, 0x83u8);
}
#[test]
///
/// Tests SBB H
///
fn sbb_h_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x02, MVI_H, 0x04, SBB_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xfdu8);
    assert_eq!(cpu.status.value, 0x83u8);
}
#[test]
///
/// Tests SBB L
///
fn sbb_l_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_L, 0xaa, SBB_L, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xabu8);
    assert_eq!(cpu.status.value, 0x83u8);
}
#[test]
///
/// Tests SBB L
///
fn sbb_l_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_L, 0xaa, SBB_L, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.status.value, 0x87u8);
}
#[test]
///
/// Tests SBB M
///
fn sbb_m_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    cpu.h = 0x12;
    cpu.l = 0x34;
    cpu.memory.borrow_mut().write_byte(0x1234, 0x55);
    let program: Vec<u8> = vec![MVI_A, 0xaa, SBB_M, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x16u8);
}
#[test]
///
/// Tests SBB M
///
fn sbb_m_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(true);
    cpu.h = 0x12;
    cpu.l = 0x34;
    cpu.memory.borrow_mut().write_byte(0x1234, 0x55);
    let program: Vec<u8> = vec![MVI_A, 0xaa, SBB_M, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x54u8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Tests SBB A
///
fn sbb_a_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x00, SBB_A, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x56u8);
}
#[test]
///
/// Tests SBB A
///
fn sbb_a_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x00, SBB_A, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.status.value, 0x87u8);
}
#[test]
///
/// Tests SBI data
///
fn sbi_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x04, SBI, 0x02, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x02u8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Tests SBI data
///
fn sbi_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x04, SBI, 0x02, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Tests SBI data
///
fn sbi_3() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x00, SBI, 0x00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x56u8);
}
#[test]
///
/// Tests SBI data
///
fn sbi_4() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![MVI_A, 0x00, SBI, 0x00, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xffu8);
    assert_eq!(cpu.status.value, 0x87u8);
}
#[test]
///
/// Tests SUB B
///
fn sub_b_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x0f, MVI_B, 0x0f, SUB_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x0u8);
    assert_eq!(cpu.status.value, 0x56u8);
}
#[test]
///
/// Tests SUB B
///
fn sub_b_3() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x00, MVI_B, 0x0f, SUB_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xf1u8);
    assert_eq!(cpu.status.value, 0x83u8);
}
#[test]
///
/// Tests SUB B
///
fn sub_b_5() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x0f, MVI_B, 0x00, SUB_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x0fu8);
    assert_eq!(cpu.status.value, 0x16u8);
}
#[test]
///
/// Tests SUB B
///
fn sub_b_7() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x00, MVI_B, 0xff, SUB_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
    assert_eq!(cpu.status.value, 0x03u8);
}
#[test]
///
/// Tests SUB B
///
fn sub_b_9() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x20, MVI_B, 0x10, SUB_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Tests SUB C
///
fn sub_c_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x0f, MVI_C, 0x0ff, SUB_C, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.status.value, 0x13u8);
}
#[test]
///
/// Tests SUB D
///
fn sub_d_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x0ff, MVI_D, 0x0f, SUB_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xf0u8);
    assert_eq!(cpu.status.value, 0x96u8);
}
#[test]
///
/// Tests SUB E
///
fn sub_e_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x04, MVI_E, 0x02, SUB_E, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x02u8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Tests SUB H
///
fn sub_h_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x02, MVI_H, 0x04, SUB_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xfeu8);
    assert_eq!(cpu.status.value, 0x83u8);
}
#[test]
///
/// Tests SUB L
///
fn sub_l_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_L, 0xaa, SUB_L, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xabu8);
    assert_eq!(cpu.status.value, 0x83u8);
}
#[test]
///
/// Tests SUB M
///
fn sub_m_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    cpu.h = 0x12;
    cpu.l = 0x34;
    cpu.memory.borrow_mut().write_byte(0x1234, 0x55);
    let program: Vec<u8> = vec![MVI_A, 0xaa, SUB_M, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.status.value, 0x16u8);
}
#[test]
///
/// Tests SUB A
///
fn sub_a_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x00, SUB_A, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.status.value, 0x56u8);
}
#[test]
///
/// Tests SUI data
///
fn sui_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x04, SUI, 0x02, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x02u8);
    assert_eq!(cpu.status.value, 0x12u8);
}
#[test]
///
/// Tests SHLD address
///
fn shld_1() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, SHLD, 0x10, 0x02, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let addr = 0x0210;
    assert_eq!(cpu.memory.borrow_mut().read_byte(addr), 0x34);
    assert_eq!(cpu.memory.borrow_mut().read_byte(addr + 1), 0x12);
}
#[test]
///
/// Tests SHLD address
///
fn shld_2() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_H, 0x12, MVI_L, 0x34, SHLD, 0xff, 0xff, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let addr = 0xffff;
    assert_eq!(cpu.memory.borrow_mut().read_byte(addr), 0x34);
    assert_eq!(cpu.memory.borrow_mut().read_byte(0), 0x12);
}
#[test]
///
/// Tests STA address
///
fn sta() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x55, STA, 0x10, 0x02, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let addr = 0x0210;
    assert_eq!(cpu.memory.borrow_mut().read_byte(addr), 0x55);
}
#[test]
///
/// Tests STAX D
///
fn stax_b() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_B, 0x02, MVI_C, 0x10, MVI_A, 0x55, STAX_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let addr = 0x0210;
    assert_eq!(cpu.memory.borrow_mut().read_byte(addr), 0x55);
}
#[test]
///
/// Tests STAX D
///
fn stax_d() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_D, 0x02, MVI_E, 0x10, MVI_A, 0xaa, STAX_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let addr = 0x0210;
    assert_eq!(cpu.memory.borrow_mut().read_byte(addr), 0xaa);
}
#[test]
///
/// Tests SPHL
///
fn sphl() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_H, 0x02, MVI_L, 0x10, SPHL, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.sp, 0x210);
}
#[test]
///
/// Tests XCHG
///
fn xchg() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![
        MVI_H, 0x12, MVI_L, 0x34, MVI_D, 0x34, MVI_E, 0x56, XCHG, HLT,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.h, 0x34);
    assert_eq!(cpu.l, 0x56);
    assert_eq!(cpu.d, 0x12);
    assert_eq!(cpu.e, 0x34);
}
#[test]
///
/// Tests XRA B
///
fn xra_b() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x55, MVI_B, 0xaa, XRA_B, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFF);
    assert_eq!(cpu.status.value, 0x86);
}
#[test]
///
/// Tests XRA C
///
fn xra_c() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x00, MVI_C, 0x00, XRA_C, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.status.value, 0x46);
}
#[test]
///
/// Tests XRA D
///
fn xra_d() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x0f, MVI_D, 0xf0, XRA_D, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0xff);
    assert_eq!(cpu.status.value, 0x86);
}
#[test]
///
/// Tests XRA E
///
fn xra_e() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x12, MVI_E, 0x34, XRA_E, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x26);
    assert_eq!(cpu.status.value, 0x02);
}
#[test]
///
/// Tests XRA H
///
fn xra_h() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x34, MVI_H, 0x12, XRA_H, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x26);
    assert_eq!(cpu.status.value, 0x02);
}
#[test]
///
/// Tests XRA L
///
fn xra_l() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x80, MVI_L, 0x08, XRA_L, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x88);
    assert_eq!(cpu.status.value, 0x86);
}
#[test]
///
/// Tests XRA M
///
fn xra_m() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    cpu.memory.borrow_mut().write_byte(0x1234, 0x08);
    let program: Vec<u8> = vec![MVI_A, 0x80, MVI_H, 0x12, MVI_L, 0x34, XRA_M, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x88);
    assert_eq!(cpu.status.value, 0x86);
}
#[test]
///
/// Tests XRA A
///
fn xra_a() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x0F, XRA_A, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.status.value, 0x46);
}
#[test]
///
/// Tests XRA A
///
fn xri() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    let program: Vec<u8> = vec![MVI_A, 0x0f, XRI, 0x0f, HLT];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.status.value, 0x46);
}
#[test]
///
/// Tests XTHL
///
fn xthl() {
    let mut cpu = Cpu::new();
    cpu.sp = 0xffff;
    cpu.status.set_carry(false);
    cpu.sp = 0x210;
    let program: Vec<u8> = vec![
        LXI_D, 0x34, 0x12, LXI_H, 0x78, 0x56, PUSH_D, XTHL, PUSH_H, HLT,
    ];
    cpu.load_program(&program, 0x0200);
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        if opcode == HLT {
            break;
        }
    }
    let addr = 0x020c;
    assert_eq!(cpu.memory.borrow_mut().read_byte(addr), 0x34);
    assert_eq!(cpu.memory.borrow_mut().read_byte(addr + 1), 0x12);
    assert_eq!(cpu.memory.borrow_mut().read_byte(addr + 2), 0x78);
    assert_eq!(cpu.memory.borrow_mut().read_byte(addr + 3), 0x56);
}
