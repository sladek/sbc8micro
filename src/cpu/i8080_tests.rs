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
    let program: Vec<u8> = vec![MVI_A, 0x66, ADI, 0xaa, // set CY and AC
    MVI_B, 0x10, ANA_B, HLT];
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
    let program: Vec<u8> = vec![MVI_A, 0x66, ADI, 0xaa, // set CY and AC
    MVI_A, 0x55, MVI_C, 0xAA, ANA_C, HLT];
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
