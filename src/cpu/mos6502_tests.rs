#[cfg(test)]
use crate::cpu::mos6502;
#[cfg(test)]
use crate::status;

#[test]
///
/// Tests immediate ADC without CARRY flag
///
fn adc_direct_no_c() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x69, 0x41, // ADC #$41
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x83u8);
    assert_eq!(cpu.p.is_carry(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests ADC immediate with CARRY flag
///
fn adc_direct_c() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0xff, // LDA #$ff
        0x69, 0x42, // ADC #$42
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x41u8);
    assert_eq!(cpu.p.is_carry(), true);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_overflow(), false);
}
#[test]
///
/// Tests ADC zero page
///
fn adc_zero_page() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte_zero_page(0x10, 0x55);
    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x65, 0x10, // ADC $10
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x97u8);
    assert_eq!(cpu.p.is_carry(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests ADC zero page,X
///
fn adc_zero_page_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1Fu16;
    let value = 0x55u8;
    cpu.x = 0x0Fu8;
    cpu.memory.write_byte(addr, value);
    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x75, 0x10, // ADC $10,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x97u8);
    assert_eq!(cpu.p.is_carry(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests ADC zero page,X wrap arround zero page
///
fn adc_zero_page_x_wrap_around() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x0Fu16;
    let value = 0x55u8;
    cpu.x = 0xFFu8;
    cpu.memory.write_byte(addr, value);
    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x75, 0x10, // ADC $10,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x97u8);
    assert_eq!(cpu.p.is_carry(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests ADC absolute
///
fn adc_absolute() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    let value = 0x55u8;
    cpu.memory.write_byte(addr, value);

    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x6D, 0x34u8, 0x12u8, // ADC $1234
        0x00,   // BRK
    ];

    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x97u8);
    assert_eq!(cpu.p.is_carry(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests ADC absolute X
///
fn adc_absolute_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x100fu16;
    let value = 0x55u8;
    cpu.x = 0x0F;
    cpu.memory.write_byte(addr, value);

    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x7D, 0x00u8, 0x10u8, // ADC $1000,X ;X=0x0f
        0x00,   // BRK
    ];

    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x97u8);
    assert_eq!(cpu.p.is_carry(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests ADC absolute Y
///
fn adc_absolute_y() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x100fu16;
    let value = 0x55u8;
    cpu.y = 0x0F;
    cpu.memory.write_byte(addr, value);

    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x79, 0x00u8, 0x10u8, // ADC $1000,Y ;Y=0x0f
        0x00,   // BRK
    ];

    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x97u8);
    assert_eq!(cpu.p.is_carry(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests ADC indexed indirect
///
fn adc_indirect_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x001Fu16;
    let value = 0x55u8;
    cpu.x = 0x0F;
    cpu.memory.write_word(addr, 0x1234);
    cpu.memory.write_byte(0x1234 as u16, value);

    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x61, 0x10u8, // ADC ($10,X) ;X=0x0f
        0x00,   // BRK
    ];

    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x97u8);
    assert_eq!(cpu.p.is_carry(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests ADC indirect indexed
///
fn adc_indirect_y() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x0010u16;
    let value = 0x55u8;
    cpu.y = 0x0F;
    cpu.memory.write_word(addr, 0x1234);
    cpu.memory.write_byte(0x1234 + cpu.y as u16, value);

    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x71, 0x10u8, // ADC ($10),Y ;Y=0x0f
        0x00,   // BRK
    ];

    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x97u8);
    assert_eq!(cpu.p.is_carry(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///////////////////////////////////////////////
/// Tests AND immediate
///
fn and_direct() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x42u8, // LDA #$42
        0x29, 0x55u8, // AND #$55
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x40u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests AND immediate with Zero flag
///
fn and_direct_zero() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x00u8, // LDA #$00
        0x29, 0x55u8, // AND #$55
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.p.is_zero(), true);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests AND immediate with Negative flag
///
fn and_direct_negative() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x80u8, // LDA #$80
        0x29, 0x85u8, // AND #$85
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x80u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
}
#[test]
///
/// Tests AND zero page
///
fn and_zero_page() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte_zero_page(0x10, 0x55);
    let program = vec![
        0xA9, 0x42u8, // LDA #$42
        0x25, 0x10u8, // AND $10
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x40u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests AND zp,X
///
fn and_zero_page_x() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0x0Fu8;
    cpu.memory.write_byte_zero_page(0x10 + cpu.x, 0x55);
    let program = vec![
        0xA9, 0x42u8, // LDA #$42
        0x35, 0x10u8, // AND $10
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x40u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests AND absolute
///
fn and_absolute() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte(0x1234, 0x55);
    let program = vec![
        0xA9, 0x42u8, // LDA #$42
        0x2D, 0x34u8, 0x12, // AND $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x40u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests AND absolute,X
///
fn and_absolute_x() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0xF;
    cpu.memory.write_byte(0x1234 + cpu.x as u16, 0x55);
    let program = vec![
        0xA9, 0x42u8, // LDA #$42
        0x3D, 0x34u8, 0x12, // AND $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x40u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests AND absolute,Y
///
fn and_absolute_y() {
    let mut cpu = mos6502::Cpu::new();
    cpu.y = 0xF;
    cpu.memory.write_byte(0x1234 + cpu.y as u16, 0x55);
    let program = vec![
        0xA9, 0x42u8, // LDA #$42
        0x39, 0x34u8, 0x12, // AND $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x40u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests AND indexed indirect, X
///
fn and_indirect_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x001Fu16;
    let value = 0x55u8;
    cpu.x = 0x0F;
    cpu.memory.write_word(addr, 0x1234);
    cpu.memory.write_byte(0x1234 as u16, value);

    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x21, 0x10u8, // AND ($10,X) ;X=0x0f
        0x00,   // BRK
    ];

    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x40u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests ND indirect indexed,Y
///
fn and_indirect_y() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x0010u16;
    let value = 0x55u8;
    cpu.y = 0x0F;
    cpu.memory.write_word(addr, 0x1234);
    cpu.memory.write_byte(0x1234 + cpu.y as u16, value);
    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x31, 0x10u8, // ADC ($10),Y ;Y=0x0f
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x40u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests ASL accumulator with cary flag
///
fn asl_accumulator() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x88u8, // LDA #$00
        0x38,   //SEC
        0x0a,   // ASL A
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x10u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_carry(), true);
}
#[test]
///
/// Tests ASL accumulator with zero flag
///
fn asl_accumulator_z() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x00u8, // LDA #$00
        0x38,   //SEC
        0x0a,   // ASL A
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.p.is_zero(), true);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests ASL zero page
///
fn asl_zerro_page() {
    let mut cpu = mos6502::Cpu::new();
    let z_addr = 0x10;
    cpu.memory.write_byte_zero_page(z_addr, 0x42);
    let program = vec![
        0x06, 0x10, // ASL $10
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let result = cpu.memory.read_byte_zero_page(z_addr);
    assert_eq!(result, 0x84u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests ASL zero page,X
///
fn asl_zerro_page_x() {
    let mut cpu = mos6502::Cpu::new();
    let z_addr = 0x1F;
    cpu.memory.write_byte_zero_page(z_addr, 0x42);
    cpu.x = 0x0Fu8;
    let program = vec![
        0x16, 0x10, // ASL $10,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.memory.read_byte_zero_page(z_addr), 0x84u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests ASL absolute
///
fn asl_absolute() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    cpu.memory.write_byte(addr, 0x42);
    let program = vec![
        0x0E, 0x34, 0x12, // ASL $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.memory.read_byte(addr), 0x84u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests ASL absolute,X
///
fn asl_absolute_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    cpu.x = 0x0Fu8;
    cpu.memory.write_byte(addr.wrapping_add(cpu.x as u16), 0x42);
    let program = vec![
        0x1E, 0x34, 0x12, // ASL $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(
        cpu.memory.read_byte(addr.wrapping_add(cpu.x as u16)),
        0x84u8
    );
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests BCC no C
///
fn bcc_no_c() {
    let mut cpu = mos6502::Cpu::new();
    cpu.p.set_carry(false);
    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x90, 0x01, // BCC label
        0x00, // BRK
        // label:
        0xA9, 0x24, // LDA #$24
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x24u8);
}
#[test]
///
/// Tests BCC with C
///
fn bcc_c() {
    let mut cpu = mos6502::Cpu::new();
    cpu.p.set_carry(true);
    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x90, 0x01, // // BCC label
        0x00, // BRK
        // label:
        0xA9, 0x24, // LDA #$24
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x42u8);
}
#[test]
///////////////////////////////////////////////
/// Tests BCS no C
///
fn bcs_no_c() {
    let mut cpu = mos6502::Cpu::new();
    cpu.p.set_carry(false);
    let program = vec![
        0xA9, 0x42, // LDA #$42
        0xB0, 0x01, // BCS label
        0x00, // BRK
        // label:
        0xA9, 0x24, // LDA #$24
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x42u8);
}
#[test]
///
/// Tests BCS with C
///
fn bcs_c() {
    let mut cpu = mos6502::Cpu::new();
    cpu.p.set_carry(true);
    let program = vec![
        0xA9, 0x42, // LDA #$42
        0xB0, 0x01, // // BCS label
        0x00, // BRK
        // label:
        0xA9, 0x24, // LDA #$24
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x24u8);
}
#[test]
///////////////////////////////////////////////
/// Tests BEQ no C
///
fn beq_no_z() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x42, // LDA #$42
        0xF0, 0x01, // BEQ label
        0x00, // BRK
        // label:
        0xA9, 0x24, // LDA #$24
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x42u8);
}
#[test]
///
/// Tests BEQ with C
///
fn beq_z() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x00, // LDA #$00
        0xF0, 0x01, // // BEQ label
        0x00, // BRK
        // label:
        0xA9, 0x24, // LDA #$24
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x24u8);
}
#[test]
///
/// Tests BIT zero page
///
fn bit_zero_page() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte(0x0010, 0xc0);
    let program = vec![
        0xA9, 0xc0, // LDA #$00
        0x24, 0x10, // // BIT $10
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0xC0u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}

#[test]
///
/// Tests BIT zero page  set z flag
///
fn bit_zero_page_z() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte(0x0010, 0x55);
    let program = vec![
        0xA9, 0x00, // LDA #$00
        0x24, 0x10, // // BIT $10
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.p.is_zero(), true);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests BIT zero page set status bits V and N
///
fn bit_zero_page_v_n() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte(0x0010, 0xC5);
    let program = vec![
        0xA9, 0x00, // LDA #$00
        0x24, 0x10, // // BIT $10
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.p.is_zero(), true);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests BIT absolute
///
fn bit_absolute() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte(0x1234, 0xc0);
    let program = vec![
        0xA9, 0xc0, // LDA #$00
        0x2C, 0x34, 0x12, // // BIT $1234
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0xC0u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests BIT absolute set zero flag
///
fn bit_absolute_z() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte(0x1234, 0xc0);
    let program = vec![
        0xA9, 0x00, // LDA #$00
        0x2C, 0x34, 0x12, // // BIT $1234
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.p.is_zero(), true);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///////////////////////////////////////////////
/// Tests BMI N=1
///
fn bmi_mi() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x82, // LDA #$82
        0x30, 0x01, // BMI label
        0x00, // BRK
        // label:
        0xA9, 0x24, // LDA #$24
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x24u8);
}
#[test]
///
/// Tests BMI N=0
///
fn bmi_no_mi() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x30, 0x01, // BMI label
        0x00, // BRK
        // label:
        0xA9, 0x24, // LDA #$24
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x42u8);
}
#[test]
///////////////////////////////////////////////
/// Tests BNE Z=0
///
fn bmi_z_0() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x42, // LDA #$42
        0xD0, 0x01, // BNE label
        0x00, // BRK
        // label:
        0xA9, 0x24, // LDA #$24
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x24u8);
}
#[test]
///
/// Tests BNE N=1
///
fn bmi_z_1() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x00, // LDA #$00
        0xD0, 0x01, // BMI label
        0x00, // BRK
        // label:
        0xA9, 0x24, // LDA #$24
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
}
#[test]
///////////////////////////////////////////////
/// Tests BPL N=0
///
fn bpl_n_0() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x10, 0x01, // BNE label
        0x00, // BRK
        // label:
        0xA9, 0x24, // LDA #$24
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x24u8);
    assert_ne!(cpu.p.is_negative(), true);
}
#[test]
///
/// Tests BPL N=1
///
fn bpl_n_1() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x82, // LDA #$42
        0x10, 0x01, // BNE label
        0x00, // BRK
        // label:
        0xA9, 0x24, // LDA #$24
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x82u8);
    assert_eq!(cpu.p.is_negative(), true);
}
#[test]
///////////////////////////////////////////////
/// Tests BVC V=1
///
fn bvc_v_1() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x40, // LDA #$40
        0x69, 0x40, // ADC #$40
        0x50, 0x01, // BVC label
        0x00, // BRK
        // label:
        0xA9, 0x24, // LDA #$24
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x80u8);
}
#[test]
///
/// Tests BVC V=1
///
fn bvc_v_0() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x20, // LDA #$40
        0x69, 0x20, // ADC #$40
        0x50, 0x01, // BVC label
        0x00, // BRK
        // label:
        0xA9, 0x24, // LDA #$24
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x24u8);
}
#[test]
///////////////////////////////////////////////
/// Tests BVS V=1
///
fn bvs_v_1() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x40, // LDA #$40
        0x69, 0x40, // ADC #$40
        0x70, 0x01, // BVC label
        0x00, // BRK
        // label:
        0xA9, 0x24, // LDA #$24
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x24u8);
}
#[test]
///
/// Tests BVC V=1
///
fn bvs_v_0() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x20, // LDA #$40
        0x69, 0x20, // ADC #$40
        0x70, 0x01, // BVC label
        0x00, // BRK
        // label:
        0xA9, 0x24, // LDA #$24
        0x00,
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x40u8);
}
#[test]
///////////////////////////////////////////////
/// Tests CLC
///
fn cls() {
    let mut cpu = mos6502::Cpu::new();
    cpu.p.set_carry(true);
    let program = vec![
        0x18, // CLC
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests CLD
///
fn cld() {
    let mut cpu = mos6502::Cpu::new();
    cpu.p.set_decimal_mode(true);
    let program = vec![
        0xD8, // CLC
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_decimal_mode(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests CLI
///
fn cli() {
    let mut cpu = mos6502::Cpu::new();
    cpu.p.set_interrupt_disable(true);
    cpu.sp = 0xff;
    let program = vec![
        0x58, // CLI
        0x08, // PHP; push status to stack as break will set it to 1
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let status = cpu.memory.read_byte(0x1ff);
    assert_eq!(status, status::mos6502::BREAK | status::mos6502::UNUSED);
}
#[test]
///////////////////////////////////////////////
/// Tests CLV
///
fn clv() {
    let mut cpu = mos6502::Cpu::new();
    cpu.p.set_overflow(true);
    let program = vec![
        0xB8, // CLC
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_overflow(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests CMP
/// A == #$imm
///
fn cmp_equal() {
    let mut cpu = mos6502::Cpu::new();
    cpu.p.set_overflow(true);
    let program = vec![
        0xA9, 0x40, // LDA #$40
        0xC9, 0x40, // CMP #$040
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), true);
    assert_eq!(cpu.p.is_carry(), true);
}
#[test]
///
/// Tests CMP
/// A > #$imm
///
fn cmp_bigger() {
    let mut cpu = mos6502::Cpu::new();
    cpu.p.set_overflow(true);
    let program = vec![
        0xA9, 0x40, // LDA #$40
        0xC9, 0x20, // CMP #$40
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), true);
}
#[test]
///
/// Tests CMP
/// A < #$imm
///
fn cmp_smaller() {
    let mut cpu = mos6502::Cpu::new();
    cpu.p.set_overflow(true);
    let program = vec![
        0xA9, 0x20, // LDA #$40
        0xC9, 0x40, // CMP #$40
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests CMP
/// A < zp
///
fn cmp_zp() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte_zero_page(0x10, 0x40);
    let program = vec![
        0xA9, 0x20, // LDA #$40
        0xC5, 0x10, // CMP #$10
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests CMP
/// A < zp_x
///
fn cmp_zp_x() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0x0F;
    cpu.memory.write_byte_zero_page(0x10 + cpu.x, 0x40);
    let program = vec![
        0xA9, 0x20, // LDA #$40
        0xD5, 0x10, // CMP #$10,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests CMP
/// A < abs
///
fn cmp_abs() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte(0x1234, 0x40);
    let program = vec![
        0xA9, 0x20, // LDA #$40
        0xCD, 0x34, 0x12, // CMP #$040
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests CMP
/// A < abs,X
///
fn cmp_abs_x() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0x0f;
    cpu.memory.write_byte(0x1234 + cpu.x as u16, 0x40);
    let program = vec![
        0xA9, 0x20, // LDA #$40
        0xDD, 0x34, 0x12, // CMP #$1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests CMP
/// A < abs,
///
fn cmp_abs_y() {
    let mut cpu = mos6502::Cpu::new();
    cpu.y = 0x0f;
    cpu.memory.write_byte(0x1234 + cpu.y as u16, 0x40);
    let program = vec![
        0xA9, 0x20, // LDA #$40
        0xD9, 0x34, 0x12, // CMP #$1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests CMP indexed indirect
/// A < (abs,X)
///
fn cmp_indirect_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x001Fu16;
    let value = 0x40u8;
    cpu.x = 0x0F;
    cpu.memory.write_word(addr, 0x1234);
    cpu.memory.write_byte(0x1234 as u16, value);
    let program = vec![
        0xA9, 0x20, // LDA #$40
        0xC1, 0x10, // CMP ($10,x)
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests CMP indirect indexed
/// A < (abs),Y
///
fn cmp_ind_y() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x0010u16;
    let value = 0x40u8;
    cpu.y = 0x0F;
    cpu.memory.write_word(addr, 0x1234);
    cpu.memory.write_byte(0x1234 + cpu.y as u16, value);
    let program = vec![
        0xA9, 0x20, // LDA #$40
        0xD1, 0x10, // CPX #$040
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests CPX #imm
///
fn cpx_z() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0x40;
    let program = vec![
        0xE0, 0x40, // CMP ($040),Y
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), true);
    assert_eq!(cpu.p.is_carry(), true);
}
#[test]
///
/// Tests CPX #imm
///
fn cpx_c() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0x20;
    let program = vec![
        0xE0, 0x40, // CPX #$40
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests CPX zp
///
fn cpx_zp() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x10u8;
    let value = 0x40u8;
    cpu.x = 0x20u8;
    cpu.memory.write_byte_zero_page(addr, value);
    let program = vec![
        0xE4, 0x10, // CPX $40
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests CPX abs
///
fn cpx_abs() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    let value = 0x40u8;
    cpu.x = 0x20u8;
    cpu.memory.write_byte(addr, value);
    let program = vec![
        0xEC, 0x34, 0x12, // CPX $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests CPY #imm
///
fn cpy_z() {
    let mut cpu = mos6502::Cpu::new();
    cpu.y = 0x40;
    let program = vec![
        0xC0, 0x40, // CPY #$040
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), true);
    assert_eq!(cpu.p.is_carry(), true);
}
#[test]
///
/// Tests CPY #imm
///
fn cpy_c() {
    let mut cpu = mos6502::Cpu::new();
    cpu.y = 0x20;
    let program = vec![
        0xC0, 0x40, // CPY #$40
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests CPY zp
///
fn cpy_zp() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x10u8;
    let value = 0x40u8;
    cpu.y = 0x20u8;
    cpu.memory.write_byte_zero_page(addr, value);
    let program = vec![
        0xC4, 0x10, // CPY $10
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests CPY abs
///
fn cpy_abs() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    let value = 0x40u8;
    cpu.y = 0x20u8;
    cpu.memory.write_byte(addr, value);
    let program = vec![
        0xCC, 0x34, 0x12, // CPY $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests DEC zp
///
fn dec_zp() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x10u8;
    let value = 0x40u8;
    cpu.memory.write_byte_zero_page(addr, value);
    let program = vec![
        0xC6, 0x10, // DEC $40
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let result = cpu.memory.read_byte_zero_page(addr);
    assert_eq!(result, value.wrapping_sub(1));
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests DEC zp,x
///
fn dec_zp_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x10u8;
    let value = 0x40u8;
    cpu.x = 0x0f;
    cpu.memory
        .write_byte_zero_page(addr.wrapping_add(cpu.x), value);
    let program = vec![
        0xD6, 0x10, // DEC $10,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let result = cpu.memory.read_byte_zero_page(addr.wrapping_add(cpu.x));
    assert_eq!(result, value.wrapping_sub(1));
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests DEC zp,x
///
fn dec_zp_x_wrapping_ff() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0xffu8;
    let value = 0x40u8;
    cpu.x = 0x01;
    cpu.memory
        .write_byte_zero_page(addr.wrapping_add(cpu.x), value);
    let program = vec![
        0xD6, 0xff, // DEC $FF,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let result = cpu.memory.read_byte_zero_page(0x00u8);
    assert_eq!(result, value.wrapping_sub(1));
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests DEC abs
///
fn dec_abs() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    let value = 0x40u8;
    cpu.memory.write_byte(addr, value);
    let program = vec![
        0xCE, 0x34, 0x12, // DEC $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let result = cpu.memory.read_byte(addr);
    assert_eq!(result, value.wrapping_sub(1));
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests DEC abs,X
///
fn dec_abs_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    let value = 0x40u8;
    cpu.x = 0x0f;
    cpu.memory
        .write_byte(addr.wrapping_add(cpu.x as u16), value);
    let program = vec![
        0xDE, 0x34, 0x12, // DEC $1234,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let result = cpu.memory.read_byte(addr.wrapping_add(cpu.x as u16));
    assert_eq!(result, value.wrapping_sub(1));
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests DEC abs,X
///
fn dec_abs_wrapping_ff() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0xffffu16;
    let value = 0x40u8;
    cpu.x = 0x01;
    cpu.memory
        .write_byte(addr.wrapping_add(cpu.x as u16), value);
    let program = vec![
        0xDE, 0xff, 0xff, // DEC $FFFF,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let result = cpu.memory.read_byte(0x0000u16);
    assert_eq!(result, value.wrapping_sub(1));
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests DEX
///
fn dex() {
    let mut cpu = mos6502::Cpu::new();
    let value = 0x05u8;
    cpu.x = value;
    let program = vec![
        0xCA, // DEX
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.x, 0x04u8);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests DEX
///
fn dex_wrapping_00() {
    let mut cpu = mos6502::Cpu::new();
    let value = 0x00u8;
    cpu.x = value;
    let program = vec![
        0xCA, // DEX
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.x, 0xffu8);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests DEY
///
fn dey() {
    let mut cpu = mos6502::Cpu::new();
    let value = 0x05u8;
    cpu.y = value;
    let program = vec![
        0x88, // DEY
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.y, 0x04u8);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests DEY
///
fn dey_wrapping_00() {
    let mut cpu = mos6502::Cpu::new();
    let value = 0x00u8;
    cpu.y = value;
    let program = vec![
        0x88, // DEY
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.y, 0xffu8);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests EOR imm
///
fn eor_imm() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0xF0, // LDA #$F0
        0x49, 0xF0, // EOR #$F0
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), true);
}
#[test]
///
/// Tests EOR imm a = 0
///
fn eor_imm_a0() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x00, // LDA #$00
        0x49, 0xF0, // EOR #$f0
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0xF0u8);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests EOR zp
fn eor_zp() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x10u8;
    let value = 0x0Fu8;
    cpu.memory.write_byte_zero_page(addr, value);
    let program = vec![
        0xA9, 0xF0, // LDA #$f0
        0x45, 0x10, // EOR zp
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests EOR zp,X
fn eor_zp_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x10u8;
    let value = 0x0Fu8;
    cpu.x = 0x0fu8;
    cpu.memory
        .write_byte_zero_page(addr.wrapping_add(cpu.x), value);
    let program = vec![
        0xA9, 0xF0, // LDA #$f0
        0x55, addr, // EOR zp,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests abs
fn eor_zp_abs() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    let value = 0x0Fu8;
    cpu.memory.write_byte(addr, value);
    let program = vec![
        0xA9, 0xF0, // LDA #$f0
        0x4D, 0x34, 0x12, // EOR $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests EOR abs,X
fn eor_zp_abs_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    let value = 0x0Fu8;
    cpu.x = 0x0fu8;
    cpu.memory
        .write_byte(addr.wrapping_add(cpu.x as u16), value);
    let program = vec![
        0xA9, 0xF0, // LDA #$f0
        0x5D, 0x34, 0x12, // EOR #1234,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests EOR abs,Y
fn eor_zp_abs_y() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    let value = 0x0Fu8;
    cpu.y = 0x0fu8;
    cpu.memory
        .write_byte(addr.wrapping_add(cpu.y as u16), value);
    let program = vec![
        0xA9, 0xF0, // LDA #$f0
        0x59, 0x34, 0x12, // EOR abs,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests EOR indexed,X
fn eor_indirect_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x0010u16;
    let value = 0x0Fu8;
    cpu.x = 0x0F;
    cpu.memory
        .write_word(addr.wrapping_add(cpu.x as u16), 0x1234);
    cpu.memory.write_byte(0x1234 as u16, value);
    let program = vec![
        0xA9, 0xF0, // LDA #$f0
        0x41, 0x10, // EOR ($10,X)
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests EOR indexed,Y
fn eor_indirect_y() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x0010u16;
    let value = 0x0Fu8;
    cpu.y = 0x0F;
    cpu.memory.write_word(addr, 0x1234);
    cpu.memory
        .write_byte(0x1234u16.wrapping_add(cpu.y as u16), value);
    let program = vec![
        0xA9, 0xF0, // LDA #$f0
        0x51, 0x10, // EOR (zp),Y
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests INC zp
///
fn inc_zp() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x10u8;
    let value = 0x40u8;
    cpu.memory.write_byte_zero_page(addr, value);
    let program = vec![
        0xE6, 0x10, // INC $10
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let result = cpu.memory.read_byte_zero_page(addr);
    assert_eq!(result, value.wrapping_add(1));
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests INC zp,x
///
fn inc_zp_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x10u8;
    let value = 0x40u8;
    cpu.x = 0x0f;
    cpu.memory
        .write_byte_zero_page(addr.wrapping_add(cpu.x), value);
    let program = vec![
        0xF6, 0x10, // INC $10,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let result = cpu.memory.read_byte_zero_page(addr.wrapping_add(cpu.x));
    assert_eq!(result, value.wrapping_add(1));
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests INC zp,x wrapping
///
fn inc_zp_x_wrapping_ff() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0xffu8;
    let value = 0x40u8;
    cpu.x = 0x01;
    cpu.memory
        .write_byte_zero_page(addr.wrapping_add(cpu.x), value);
    let program = vec![
        0xF6, 0xff, // INC $FF,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let result = cpu.memory.read_byte_zero_page(0x00u8);
    assert_eq!(result, value.wrapping_add(1));
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests INC abs
///
fn inc_abs() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    let value = 0x40u8;
    cpu.memory.write_byte(addr, value);
    let program = vec![
        0xEE, 0x34, 0x12, // INC $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let result = cpu.memory.read_byte(addr);
    assert_eq!(result, value.wrapping_add(1));
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests INC abs,X
///
fn inc_abs_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    let value = 0x40u8;
    cpu.x = 0x0f;
    cpu.memory
        .write_byte(addr.wrapping_add(cpu.x as u16), value);
    let program = vec![
        0xFE, 0x34, 0x12, // INC $1234,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let result = cpu.memory.read_byte(addr.wrapping_add(cpu.x as u16));
    assert_eq!(result, value.wrapping_add(1));
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests INC abs,X wrapping FF
///
fn inc_abs_wrapping_ff() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0xffffu16;
    let value = 0x40u8;
    cpu.x = 0x01;
    cpu.memory
        .write_byte(addr.wrapping_add(cpu.x as u16), value);
    let program = vec![
        0xFE, 0xff, 0xff, // INC $1234,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let result = cpu.memory.read_byte(0x0000u16);
    assert_eq!(result, value.wrapping_add(1));
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests INX
///
fn inx() {
    let mut cpu = mos6502::Cpu::new();
    let value = 0x05u8;
    cpu.x = value;
    let program = vec![
        0xE8, // INX
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.x, value.wrapping_add(1));
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests INX
///
fn inx_wrapping_ff() {
    let mut cpu = mos6502::Cpu::new();
    let value = 0xffu8;
    cpu.x = value;
    let program = vec![
        0xE8, // INX
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.x, value.wrapping_add(1));
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), true);
}
#[test]
///////////////////////////////////////////////
/// Tests INY
///
fn iny() {
    let mut cpu = mos6502::Cpu::new();
    let value = 0x05u8;
    cpu.y = value;
    let program = vec![
        0xC8, // INY
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.y, value.wrapping_add(1));
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests INY
///
fn iny_wrapping_ff() {
    let mut cpu = mos6502::Cpu::new();
    let value = 0xffu8;
    cpu.y = value;
    let program = vec![
        0xC8, // INY
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.y, value.wrapping_add(1));
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), true);
}
#[test]
///////////////////////////////////////////////
/// Tests JMP abs
///
fn jmp_absolute() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0xF0, // LDA #$f0
        0x4C, 0x06, 0x06, // JMP $0606
        0x00, // BRK
        0xA9, 0x0F, // LDA #$0F
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x0Fu8);
}
#[test]
///
/// Tests JMP indirectabs
///
fn jmp_indirect() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x55, // LDA #$55
        0x6C, 0x06, 0x06, // JMP ($0606)
        0x00, // BRK
        0x09, 0x06, // Indirect address $0609
        0x00, // BRK
        0x0A, 0xAA, // LDA #$AA
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0xAAu8);
}
#[test]
///////////////////////////////////////////////
/// Tests JMP indirectabs
///
/// if address $3000 contains $40, $30FF contains $80, and $3100 contains $50,
/// the result of JMP ($30FF) will be a transfer of control to $4080 rather than $5080 as you intended
/// i.e. the 6502 took the low byte of the address from $30FF and the high byte from $3000.
///
fn jmp_indirect_30ff() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte(0x3000, 0x40);
    cpu.memory.write_byte(0x30ff, 0x80);
    cpu.memory.write_byte(0x3100, 0x50);
    let program = vec![
        0xA9, 0x55, // LDA #$55
        0x6C, 0xff, 0x30, // JMP ($30FF)
        0x00, // BRK
    ];
    let program_jmp = vec![
        0xA9, 0xAA, // LDA #$FF
        0x00, // BRK
    ];
    cpu.load_program(&program_jmp, 0x4080);
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0xAA);
    assert_eq!(cpu.pc, 0x4082);
}
#[test]
///////////////////////////////////////////////
/// Tests JSR indirectabs
///
fn jsr() {
    let mut cpu = mos6502::Cpu::new();
    cpu.sp = 0xff;
    // load from 0x0600
    let program = vec![
        0xA9, 0x55, // LDA #$55
        0x20, 0x34, 0x12, // JSR $1234
        0x00, // BRK
    ];
    // Load from 0x1234
    let program_2 = vec![
        0xA9, 0xAA, // LDA #$55
        0x00, // BRK
    ];
    cpu.load_program(&program_2, 0x1234);
    cpu.load_program(&program, 0x0600); // This also sets PC to start address: 0x600
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    let addr_instack = cpu
        .memory
        .read_word((cpu.sp as u16).wrapping_add(0x100u16).wrapping_add(1));
    assert_eq!(cpu.a, 0xAAu8);
    assert_eq!(cpu.pc, 0x1236);
    assert_eq!(cpu.sp, 0xfd);
    assert_eq!(addr_instack, 0x0604);
}
#[test]
///////////////////////////////////////////////
/// Tests LDA immediate
///
fn lda_direct() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x42u8, // LDA #$42
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x42u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDA immediate with Zero flag
///
fn lda_direct_zero() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x00u8, // LDA #$00
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.p.is_zero(), true);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDA immediate with Negative flag
///
fn lda_direct_negative() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x80u8, // LDA #$80
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x80u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
}
#[test]
///
/// Tests LDA zero page
///
fn lda_zero_page() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte_zero_page(0x10, 0x55);
    let program = vec![
        0xA5, 0x10u8, // LDA $10
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDA zero page,X
///
fn lda_zero_page_x() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0x0Fu8;
    cpu.memory.write_byte_zero_page(0x10 + cpu.x, 0x55);
    let program = vec![
        0xB5, 0x10u8, // LDA $10,X
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDA absolute
///
fn lda_absolute() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte(0x1234, 0x55);
    let program = vec![
        0xAD, 0x34u8, 0x12, // LDA $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDA absolute,X
///
fn lda_absolute_x() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0xF;
    cpu.memory.write_byte(0x1234 + cpu.x as u16, 0x55);
    let program = vec![
        0xBD, 0x34u8, 0x12, // LDA $1234,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDA absolute,Y
///
fn lda_absolute_y() {
    let mut cpu = mos6502::Cpu::new();
    cpu.y = 0xF;
    cpu.memory.write_byte(0x1234 + cpu.y as u16, 0x55);
    let program = vec![
        0xB9, 0x34u8, 0x12, // LDA $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDA indexed indirect, X
///
fn lda_indirect_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x001Fu16;
    let value = 0x55u8;
    cpu.x = 0x0F;
    cpu.memory.write_word(addr, 0x1234);
    cpu.memory.write_byte(0x1234 as u16, value);

    let program = vec![
        0xA1, 0x10u8, // LDA ($10,X) ;X=0x0f
        0x00,   // BRK
    ];

    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests lda indirect indexed,Y
///
fn lda_indirect_y() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x0010u16;
    let value = 0x55u8;
    cpu.y = 0x0F;
    cpu.memory.write_word(addr, 0x1234);
    cpu.memory.write_byte(0x1234 + cpu.y as u16, value);
    let program = vec![
        0xB1, 0x10u8, // LDA ($10),Y ;Y=0x0f
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests LDX immediate
///
fn ldx_direct() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA2, 0x55u8, // LDX #$55
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.x, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDX immediate zero
///
fn ldx_direct_zero() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA2, 0x00u8, // LDX #$00
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.x, 0x00u8);
    assert_eq!(cpu.p.is_zero(), true);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDX immediate negative
///
fn ldx_direct_negative() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA2, 0x80u8, // LDX #$80
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.x, 0x80u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
}
#[test]
///
/// Tests LDX zp
///
fn ldx_zero_page() {
    let mut cpu = mos6502::Cpu::new();
    let value = 0x55u8;
    cpu.memory.write_byte(0x10, value);
    let program = vec![
        0xA6, 0x10u8, // LDX $10
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.x, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDX zp, Y
///
fn ldx_zero_page_y() {
    let mut cpu = mos6502::Cpu::new();
    let value = 0x55u8;
    cpu.y = 0x0Fu8;
    cpu.memory
        .write_byte(0x10u8.wrapping_add(cpu.y) as u16, value);
    let program = vec![
        0xB6, 0x10u8, // LDX $10,Y
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.x, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDX abs
///
fn ldx_abs() {
    let mut cpu = mos6502::Cpu::new();
    let value = 0x55u8;
    cpu.memory.write_byte(0x1234u16, value);
    let program = vec![
        0xAE, 0x34u8, 0x12u8, // LDX $1234
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.x, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDX abs,Y
///
fn ldx_abs_y() {
    let mut cpu = mos6502::Cpu::new();
    let value = 0x55u8;
    cpu.y = 0x0Fu8;
    cpu.memory
        .write_byte(0x1234u16.wrapping_add(cpu.y as u16), value);
    let program = vec![
        0xBE, 0x34u8, 0x12u8, // LDX $1234
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.x, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests LDY immediate
///
fn ldy_direct() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA0, 0x55u8, // LDY #$55
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.y, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDY immediate zero
///
fn ldy_direct_zero() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA0, 0x00u8, // LDY #$00
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.y, 0x00u8);
    assert_eq!(cpu.p.is_zero(), true);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDY immediate negative
///
fn ldy_direct_negative() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA0, 0x80u8, // LDY #$80
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.y, 0x80u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
}
#[test]
///
/// Tests LDY zp
///
fn ldy_zero_page() {
    let mut cpu = mos6502::Cpu::new();
    let value = 0x55u8;
    cpu.memory.write_byte(0x10, value);
    let program = vec![
        0xA4, 0x10u8, // LDY #$80
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.y, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDY zp, X
///
fn ldy_zero_page_x() {
    let mut cpu = mos6502::Cpu::new();
    let value = 0x55u8;
    cpu.x = 0x0Fu8;
    cpu.memory
        .write_byte(0x10u8.wrapping_add(cpu.x) as u16, value);
    let program = vec![
        0xB4, 0x10u8, // LDY $10,X
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.y, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDY abs
///
fn ldy_abs() {
    let mut cpu = mos6502::Cpu::new();
    let value = 0x55u8;
    cpu.memory.write_byte(0x1234u16, value);
    let program = vec![
        0xAC, 0x34u8, 0x12u8, // LDY $1234
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.y, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests LDY abs,X
///
fn ldy_abs_x() {
    let mut cpu = mos6502::Cpu::new();
    let value = 0x55u8;
    cpu.x = 0x0Fu8;
    cpu.memory
        .write_byte(0x1234u16.wrapping_add(cpu.x as u16), value);
    let program = vec![
        0xBC, 0x34u8, 0x12u8, // LDX $1234
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.y, 0x55u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests LSR accumulator with cary flag
///
fn lsr_accumulator() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x45u8, // LDA #$00
        0x4A,   // LSR A
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x22u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_carry(), true);
}
#[test]
///
/// Tests LSR accumulator with zero flag
///
fn lsr_accumulator_z() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x00u8, // LDA #$00
        0x38,   //SEC
        0x4a,   // LSR A
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.p.is_zero(), true);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests LSR zero page
///
fn lsr_zerro_page() {
    let mut cpu = mos6502::Cpu::new();
    let z_addr = 0x10;
    cpu.memory.write_byte_zero_page(z_addr, 0x42);
    let program = vec![
        0x46, 0x10, // LSR $10
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let result = cpu.memory.read_byte_zero_page(z_addr);
    assert_eq!(result, 0x21u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests LSR zero page,X
///
fn lsr_zerro_page_x() {
    let mut cpu = mos6502::Cpu::new();
    let z_addr = 0x1F;
    cpu.memory.write_byte_zero_page(z_addr, 0x42);
    cpu.x = 0x0Fu8;
    let program = vec![
        0x56, 0x10, // LSR $10,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.memory.read_byte_zero_page(z_addr), 0x21u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests LSR absolute
///
fn lsr_absolute() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    cpu.memory.write_byte(addr, 0x42);
    let program = vec![
        0x4E, 0x34, 0x12, // ASL $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.memory.read_byte(addr), 0x21u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests LSR absolute,X
///
fn lsr_absolute_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    cpu.x = 0x0Fu8;
    cpu.memory.write_byte(addr.wrapping_add(cpu.x as u16), 0x42);
    let program = vec![
        0x5E, 0x34, 0x12, // ASL $1234,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(
        cpu.memory.read_byte(addr.wrapping_add(cpu.x as u16)),
        0x21u8
    );
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests NOP
///
fn nop() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x45u8, // LDA #$00
        0xEA,   // NOP
        0xA9, 0x55u8, // LDA #$00
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0x55u8);
    assert_eq!(cpu.memory.read_byte(cpu.pc - 0x03), 0x0EA);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests ORA immediate
///
fn ora_direct() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0xAAu8, // LDA #$AA
        0x09, 0x55u8, // ORA #$55
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0xFFu8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
}
#[test]
///
/// Tests ORA immediate with Zero flag
///
fn ora_direct_zero() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x00u8, // LDA #$00
        0x09, 0x00u8, // ORA 0#$00
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.p.is_zero(), true);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests ORA immediate with Negative flag
///
fn ora_direct_negative() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x40u8, // LDA #$40
        0x09, 0x85u8, // ORA #$85
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0xC5u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
}
#[test]
///
/// Tests ORA zero page
///
fn ora_zero_page() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte_zero_page(0x10, 0x55);
    let program = vec![
        0xA9, 0x42u8, // LDA #$42
        0x05, 0x10u8, // ORA $10
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x57u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests ORA zp,X
///
fn ora_zero_page_x() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0x0Fu8;
    cpu.memory.write_byte_zero_page(0x10 + cpu.x, 0x55);
    let program = vec![
        0xA9, 0x42u8, // LDA #$42
        0x15, 0x10u8, // ORA $10,X
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x57u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests ORA absolute
///
fn ora_absolute() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte(0x1234, 0x55);
    let program = vec![
        0xA9, 0x42u8, // LDA #$42
        0x0D, 0x34u8, 0x12, // ORA $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x57u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests ORA absolute,X
///
fn ora_absolute_x() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0xF;
    cpu.memory.write_byte(0x1234 + cpu.x as u16, 0x55);
    let program = vec![
        0xA9, 0x42u8, // LDA #$42
        0x1D, 0x34u8, 0x12, // AND $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x57u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests ORA absolute,Y
///
fn ora_absolute_y() {
    let mut cpu = mos6502::Cpu::new();
    cpu.y = 0xF;
    cpu.memory.write_byte(0x1234 + cpu.y as u16, 0x55);
    let program = vec![
        0xA9, 0x42u8, // LDA #$42
        0x19, 0x34u8, 0x12, // ORA $1234,Y
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x57u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests ORA indexed indirect, X
///
fn ora_indirect_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x001Fu16;
    let value = 0x55u8;
    cpu.x = 0x0F;
    cpu.memory.write_word(addr, 0x1234);
    cpu.memory.write_byte(0x1234 as u16, value);

    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x01, 0x10u8, // ORA ($10,X) ;X=0x0f
        0x00,   // BRK
    ];

    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x57u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///
/// Tests ORA indirect indexed,Y
///
fn ora_indirect_y() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x0010u16;
    let value = 0x55u8;
    cpu.y = 0x0F;
    cpu.memory.write_word(addr, 0x1234);
    cpu.memory.write_byte(0x1234 + cpu.y as u16, value);
    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x11, 0x10u8, // ORA ($10),Y ;Y=0x0f
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x57u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests PHA
///
fn pha() {
    let mut cpu = mos6502::Cpu::new();
    cpu.sp = 0xFFu8;
    let program = vec![
        0xA9, 0xAAu8, // LDA #$AA
        0x48,   // PHA
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0xAAu8);
    assert_eq!(cpu.memory.read_byte(0x01ff), 0xAAu8);
    assert_eq!(cpu.sp, 0xFE);
}
#[test]
///////////////////////////////////////////////
/// Tests PHP
///
fn php() {
    let mut cpu = mos6502::Cpu::new();
    cpu.sp = 0xFFu8;
    cpu.p.set_carry(true);
    cpu.p.set_negative(true);
    let program = vec![
        0x08, // PHP
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.memory.read_byte(0x01ff), 0xB1u8);
    assert_eq!(cpu.sp, 0xFE);
    assert_eq!(cpu.p.value, 0x81);
}
#[test]
///////////////////////////////////////////////
/// Tests PLA
///
fn pla() {
    let mut cpu = mos6502::Cpu::new();
    cpu.sp = 0xFFu8;
    let program = vec![
        0xA9, 0xAAu8, // LDA #$AA
        0x48,   // PHA
        0xA9, 0x55u8, // LDA #$55
        0x68,   // PLA
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0xAAu8);
    assert_eq!(cpu.memory.read_byte(0x01ff), 0xAAu8);
    assert_eq!(cpu.sp, 0xFF);
}
#[test]
///////////////////////////////////////////////
/// Tests PLP
///
fn plp() {
    let mut cpu = mos6502::Cpu::new();
    cpu.sp = 0xFFu8;
    cpu.p.set_carry(true);
    cpu.p.set_negative(true);
    let program = vec![
        0x08, // PHA
        0x18, // CLC (Clear carry)
        0x28, // PLP
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.memory.read_byte(0x01ff), 0xB1u8);
    assert_eq!(cpu.sp, 0xFF);
    assert_eq!(cpu.p.value, 0x81);
}
#[test]
///////////////////////////////////////////////
/// Tests ROL accumulator with cary flag
///
fn rol_accumulator() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x45u8, // LDA #$00
        0x2A,   // ROL A
        0x00,   // BRK
    ];
    cpu.p.set_carry(true);
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x8Bu8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests ROL accumulator with zero flag
///
fn rol_accumulator_c() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x80u8, // LDA #$00
        0x38,   //SEC
        0x2a,   // ROL A
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x01u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_carry(), true);
}
#[test]
///
/// Tests ROL accumulator with zero flag
///
fn rol_accumulator_z() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x00u8, // LDA #$00
        0x2a,   // ROL A
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.p.is_zero(), true);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests ROL zero page
///
fn rol_zerro_page() {
    let mut cpu = mos6502::Cpu::new();
    let z_addr = 0x10;
    cpu.memory.write_byte_zero_page(z_addr, 0x42);
    let program = vec![
        0x26, 0x10, // ROL $10
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let result = cpu.memory.read_byte_zero_page(z_addr);
    assert_eq!(result, 0x84u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests ROL zero page,X
///
fn rol_zerro_page_x() {
    let mut cpu = mos6502::Cpu::new();
    let z_addr = 0x1F;
    cpu.memory.write_byte_zero_page(z_addr, 0x42);
    cpu.x = 0x0Fu8;
    let program = vec![
        0x36, 0x10, // ROL $10
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.memory.read_byte_zero_page(z_addr), 0x84u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests ROL absolute
///
fn rol_absolute() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    cpu.memory.write_byte(addr, 0x42);
    let program = vec![
        0x2E, 0x34, 0x12, // ROL $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.memory.read_byte(addr), 0x84u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests ROL absolute,X
///
fn rol_absolute_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    cpu.x = 0x0Fu8;
    cpu.memory.write_byte(addr.wrapping_add(cpu.x as u16), 0x42);
    let program = vec![
        0x3E, 0x34, 0x12, // ROL $1234,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(
        cpu.memory.read_byte(addr.wrapping_add(cpu.x as u16)),
        0x84u8
    );
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests ROR accumulator with cary flag
///
fn ror_accumulator() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x45u8, // LDA #$45
        0x6A,   // ROR A
        0x00,   // BRK
    ];
    cpu.p.set_carry(true);
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0xA2u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_carry(), true);
}
#[test]
///
/// Tests ROR accumulator with carryo flag
///
fn ror_accumulator_c() {
    let mut cpu = mos6502::Cpu::new();
    cpu.p.set_carry(true);
    let program = vec![
        0xA9, 0x03u8, // LDA #$00
        0x6A,   // ROR A
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x81u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_carry(), true);
}
#[test]
///
/// Tests ROR accumulator with zero flag
///
fn ror_accumulator_z() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x00u8, // LDA #$00
        0x6a,   // ROR A
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.p.is_zero(), true);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests ROR zero page,X
///
fn ror_zerro_page_x() {
    let mut cpu = mos6502::Cpu::new();
    let z_addr = 0x1F;
    cpu.memory.write_byte_zero_page(z_addr, 0x42);
    cpu.x = 0x0Fu8;
    let program = vec![
        0x76, 0x10, // ROR $10,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.memory.read_byte_zero_page(z_addr), 0x21u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests ROR absolute
///
fn ror_absolute() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    cpu.memory.write_byte(addr, 0x42);
    let program = vec![
        0x6E, 0x34, 0x12, // ROR $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(cpu.memory.read_byte(addr), 0x21u8);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///
/// Tests ROR absolute,X
///
fn ror_absolute_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x1234u16;
    cpu.x = 0x0Fu8;
    cpu.memory.write_byte(addr.wrapping_add(cpu.x as u16), 0x42);
    let program = vec![
        0x7E, 0x34, 0x12, // ROR $1234,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    assert_eq!(
        cpu.memory.read_byte(addr.wrapping_add(cpu.x as u16)),
        0x21u8
    );
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_carry(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests RTI
///
fn rti() {
    let mut cpu = mos6502::Cpu::new();
    cpu.sp = 0xff;
    let program = vec![
        0xA9, 0x12u8, // LDA #$12
        0x48,   // PHA ;push $12 to stack
        0xA9, 0x34u8, // LDA #$34
        0x48,   // PHA ;push $34 to stack
        0x08,   // PHP
        0x40,   // RTI
        0x00,   // BRK
    ];
    cpu.p.set_carry(true);
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    let status = cpu.memory.read_byte(0x1fd);
    assert_eq!(
        status,
        status::mos6502::CARRY | status::mos6502::UNUSED | status::mos6502::BREAK
    );
    assert_eq!(cpu.pc, 0x1234);
    assert_eq!(cpu.p.value, status::mos6502::CARRY);
}
#[test]
///////////////////////////////////////////////
/// Tests RTS
///
fn rts() {
    let mut cpu = mos6502::Cpu::new();
    cpu.sp = 0xff;
    let program = vec![
        0xA9, 0x12u8, // LDA #$12
        0x48,   // PHA ;push $12 to stack
        0xA9, 0x34u8, // LDA #$34
        0x48,   // PHA ;push $34 to stack
        0x60,   // RTS
        0x00,   // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.pc, 0x1235);
}
#[test]
///////////////////////////////////////////////
/// Tests SBC
///
fn sbc_imm() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x55u8, // LDA #$55
        0x18,   // CLC
        0xE9, 0x50, // SBC #$50
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0x05u8);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), true);
    assert_eq!(cpu.p.is_overflow(), false);
}
#[test]
///
/// Tests SBC with carry
///
fn sbc_imm_c() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x55u8, // LDA #$55
        0x38,   // SEC
        0xE9, 0x50, // SBC #$50
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0x04u8);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), true);
    assert_eq!(cpu.p.is_overflow(), false);
}
#[test]
///
/// Tests SBC with zero
///
fn sbc_imm_z() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x55u8, // LDA #$55
        0x18,   // CLC
        0xE9, 0x55, // SBC #$55
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), true);
    assert_eq!(cpu.p.is_carry(), true);
    assert_eq!(cpu.p.is_overflow(), false);
}
#[test]
///
/// Tests SBC with negative and carry
///
fn sbc_imm_n_c() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x55u8, // LDA #$55
        0x38,   // SEC
        0xE9, 0xAA, // SBC #$AA
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0xAAu8);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests SBC with negative and carry
///
fn sbc_imm_c_v() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x80u8, // LDA #$80
        0x18,   // CEC
        0xE9, 0x40, // SBC #$40
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0x40u8);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests SBC zp
///
fn sbc_zp() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte_zero_page(0x10, 0x50);
    let program = vec![
        0xA9, 0x80u8, // LDA #$80
        0x18,   // CEC
        0xE5, 0x10, // SBC #$10
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0x30u8);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests SBC zp,X
///
fn sbc_zp_x() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte_zero_page(0x20, 0x50);
    cpu.x = 0x10u8;
    let program = vec![
        0xA9, 0x80u8, // LDA #$80
        0x18,   // CEC
        0xF5, 0x10, // SBC #$10,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0x30u8);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests SBC absolute
///
fn sbc_abs() {
    let mut cpu = mos6502::Cpu::new();
    cpu.memory.write_byte(0x1234, 0x50);
    cpu.x = 0x10u8;
    let program = vec![
        0xA9, 0x80, // LDA #$80
        0x18, // CEC
        0xED, 0x34, 0x12, // SBC $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0x30u8);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests SBC absolute
///
fn sbc_abs_x() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0x10u8;
    cpu.memory
        .write_byte(0x1234u16.wrapping_add(cpu.x as u16), 0x50u8);
    let program = vec![
        0xA9, 0x80, // LDA #$80
        0x18, // CEC
        0xFD, 0x34, 0x12, // SBC $1234,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0x30u8);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests SBC absolute
///
fn sbc_abs_y() {
    let mut cpu = mos6502::Cpu::new();
    cpu.y = 0x10u8;
    cpu.memory
        .write_byte(0x1234u16.wrapping_add(cpu.y as u16), 0x50u8);
    let program = vec![
        0xA9, 0x80, // LDA #$80
        0x18, // CEC
        0xF9, 0x34, 0x12, // SBC $1234,Y
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0x30u8);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests SBC (indirect,X)
///
fn sbc_abs_indirect_x() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x001Fu16;
    let value = 0x50u8;
    cpu.x = 0x0F;
    cpu.memory.write_word(addr, 0x1234);
    cpu.memory.write_byte(0x1234 as u16, value);
    let program = vec![
        0xA9, 0x80, // LDA #$80
        0x18, // CEC
        0xE1, 0x10, // SBC ($10,X)
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0x30u8);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///
/// Tests SBC (indirect),Y
///
fn sbc_abs_indirect_y() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x0010u16;
    let value = 0x50u8;
    cpu.y = 0x0F;
    cpu.memory.write_word(addr, 0x1234);
    cpu.memory.write_byte(0x1234 + cpu.y as u16, value);
    let program = vec![
        0xA9, 0x80, // LDA #$80
        0x18, // CEC
        0xF1, 0x10, // SBC $10,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0x30u8);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), true);
    assert_eq!(cpu.p.is_overflow(), true);
}
#[test]
///////////////////////////////////////////////
/// Tests SEC
///
fn sec() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0x18, // CLC
        0x38, // SEC
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), true);
    assert_eq!(cpu.p.is_overflow(), false);
    assert_eq!(cpu.p.is_decimal_mode(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests SED
///
fn sed() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xD8, // CLD
        0xF8, // SEd
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
    assert_eq!(cpu.p.is_decimal_mode(), true);
}
#[test]
///////////////////////////////////////////////
/// Tests SEI
///
fn sei() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0x58, // CLI
        0x78, // SEI
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
    assert_eq!(cpu.p.is_carry(), false);
    assert_eq!(cpu.p.is_decimal_mode(), false);
    assert_eq!(cpu.p.is_interrupt_disable(), true);
}
#[test]
///////////////////////////////////////////////
/// Tests STA zp
///
fn sta_zp() {
    let mut cpu = mos6502::Cpu::new();
    let program = vec![
        0xA9, 0x55, // LDA #$55
        0x85, 0x10, // STA zp
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.memory.read_byte_zero_page(0x10u8), 0x55u8);
}
#[test]
///
/// Tests STA zp,X
///
fn sta_zp_x() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0x0fu8;
    let program = vec![
        0xA9, 0x55, // LDA #$55
        0x95, 0x10, // STA $10,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.memory.read_byte_zero_page(0x1fu8), 0x55u8);
}
#[test]
///
/// Tests STA absolute
///
fn sta_abs() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0x0fu8;
    let program = vec![
        0xA9, 0x55, // LDA #$55
        0x8D, 0x34, 0x12, // STA $1234
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.memory.read_byte(0x1234u16), 0x55u8);
}
#[test]
///
/// Tests STA absolute,X
///
fn sta_abs_x() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0x0fu8;
    let program = vec![
        0xA9, 0x55, // LDA #$55
        0x9D, 0x34, 0x12, // STA a$1234,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(
        cpu.memory.read_byte(0x1234u16.wrapping_add(cpu.x as u16)),
        0x55u8
    );
}
#[test]
///
/// Tests STA absolute,Y
///
fn sta_abs_y() {
    let mut cpu = mos6502::Cpu::new();
    cpu.y = 0x0fu8;
    let program = vec![
        0xA9, 0x55, // LDA #$55
        0x99, 0x34, 0x12, // STA $1234,Y
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(
        cpu.memory.read_byte(0x1234u16.wrapping_add(cpu.y as u16)),
        0x55u8
    );
}
#[test]
///
/// Tests STA (indirect,X)
///
fn sta_indirect_x() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0x0fu8;
    let program = vec![
        0xA9, 0x55, // LDA #$55
        0x81, 0x10, // STA ($10,X)
        0x00, // BRK
    ];
    cpu.memory.write_word_zero_page(0x1f, 0x1234u16);
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.memory.read_byte(0x1234u16), 0x55u8);
}
#[test]
///
/// Tests STA (indirect),Y
///
fn sta_indirect_y() {
    let mut cpu = mos6502::Cpu::new();
    cpu.y = 0x0fu8;
    let program = vec![
        0xA9, 0x55, // LDA #$55
        0x91, 0x10, // STA ($10),Y
        0x00, // BRK
    ];
    cpu.memory.write_word_zero_page(0x10, 0x1234u16);
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(
        cpu.memory.read_byte(0x1234u16.wrapping_add(cpu.y as u16)),
        0x55u8
    );
}
#[test]
///////////////////////////////////////////////
/// Tests STX zp
///
fn stx_zp() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0x55u8;
    let program = vec![
        0x86, 0x10, // STX zp
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.memory.read_byte_zero_page(0x10u8), 0x55u8);
}
#[test]
///
/// Tests STX zp,Y
///
fn stx_zp_y() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0x55u8;
    cpu.y = 0x0fu8;
    let program = vec![
        0x96, 0x10, // STX $10,Y
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(
        cpu.memory.read_byte_zero_page(0x10u8.wrapping_add(cpu.y)),
        0x55u8
    );
}
#[test]
///
/// Tests STX abs
///
fn stx_abs() {
    let mut cpu = mos6502::Cpu::new();
    cpu.x = 0x55u8;
    let program = vec![
        0x8E, 0x34, 0x12, // STX abs
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.memory.read_byte(0x1234u16), 0x55u8);
}
#[test]
///////////////////////////////////////////////
/// Tests STY zp
///
fn sty_zp() {
    let mut cpu = mos6502::Cpu::new();
    cpu.y = 0x55u8;
    let program = vec![
        0x84, 0x10, // STY $10
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.memory.read_byte_zero_page(0x10u8), 0x55u8);
}
#[test]
///
/// Tests STY zp,X
///
fn sty_zp_x() {
    let mut cpu = mos6502::Cpu::new();
    cpu.y = 0x55u8;
    cpu.x = 0x0fu8;
    let program = vec![
        0x94, 0x10, // STY $10,X
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(
        cpu.memory.read_byte_zero_page(0x10u8.wrapping_add(cpu.x)),
        0x55u8
    );
}
#[test]
///
/// Tests STY abs
///
fn sty_abs() {
    let mut cpu = mos6502::Cpu::new();
    cpu.y = 0x55u8;
    let program = vec![
        0x8C, 0x34, 0x12, // STY abs
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.memory.read_byte(0x1234u16), 0x55u8);
}
#[test]
///////////////////////////////////////////////
/// Tests TAX
fn tax() {
    let mut cpu = mos6502::Cpu::new();
    cpu.a = 0x55u8;
    cpu.x = 0xaau8;
    let program = vec![
        0xAA, // TAX
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.x, 0x55);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests TAX with zero flag
///
fn tax_z() {
    let mut cpu = mos6502::Cpu::new();
    cpu.a = 0x00u8;
    cpu.x = 0xaau8;
    let program = vec![
        0xAA, // TAX
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.x, 0x00);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), true);
}
#[test]
///
/// Tests TAX with negative  flag
///
fn tax_n() {
    let mut cpu = mos6502::Cpu::new();
    cpu.a = 0x80u8;
    cpu.x = 0xaau8;
    let program = vec![
        0xAA, // TAX
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.x, 0x80);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests TAY
fn tay() {
    let mut cpu = mos6502::Cpu::new();
    cpu.a = 0x55u8;
    cpu.y = 0xaau8;
    let program = vec![
        0xA8, // TAY
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.y, 0x55);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests TAY with zero flag
fn tay_z() {
    let mut cpu = mos6502::Cpu::new();
    cpu.a = 0x00u8;
    cpu.y = 0xaau8;
    let program = vec![
        0xA8, // TAY
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.y, 0x00);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), true);
}
#[test]
///
/// Tests TAY with negative flag
fn tay_n() {
    let mut cpu = mos6502::Cpu::new();
    cpu.a = 0x80u8;
    cpu.y = 0xaau8;
    let program = vec![
        0xA8, // TAY
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.y, 0x80);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests TSX
fn tsx() {
    let mut cpu = mos6502::Cpu::new();
    cpu.sp = 0x55u8;
    cpu.x = 0xaau8;
    let program = vec![
        0xBA, // TSX
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.sp, 0x55);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests TSX with zero flag
fn tsx_z() {
    let mut cpu = mos6502::Cpu::new();
    cpu.sp = 0x00u8;
    cpu.x = 0xaau8;
    let program = vec![
        0xBA, // TSX
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.sp, 0x00);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), true);
}
#[test]
///
/// Tests TSX with negative flag
fn tsx_n() {
    let mut cpu = mos6502::Cpu::new();
    cpu.sp = 0x80u8;
    cpu.x = 0xaau8;
    let program = vec![
        0xBA, // TSX
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.sp, 0x80);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///////////////////////////////////////////////
/// Tests TXA
///
fn txa() {
    let mut cpu = mos6502::Cpu::new();
    cpu.a = 0x55u8;
    cpu.x = 0xaau8;
    let program = vec![
        0x8A, // TXA
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0xaau8);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
}
#[test]
///
/// Tests TXA with zero flag
///
fn txa_z() {
    let mut cpu = mos6502::Cpu::new();
    cpu.a = 0x55u8;
    cpu.x = 0x00u8;
    let program = vec![
        0x8A, // TXA
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), true);
}
#[test]
///////////////////////////////////////////////
/// Tests TXS
///
fn txs() {
    let mut cpu = mos6502::Cpu::new();
    cpu.sp = 0x55u8;
    cpu.x = 0xAAu8;
    let program = vec![
        0x9A, // TXS
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.sp, 0xAAu8);
}
#[test]
///////////////////////////////////////////////
/// Tests TYA with zero flag
///
fn tya_z() {
    let mut cpu = mos6502::Cpu::new();
    cpu.a = 0x55u8;
    cpu.y = 0x00u8;
    let program = vec![
        0x98, // TYA
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0x00u8);
    assert_eq!(cpu.p.is_negative(), false);
    assert_eq!(cpu.p.is_zero(), true);
}
#[test]
///
/// Tests TYA with negative flag
///
fn tya_n() {
    let mut cpu = mos6502::Cpu::new();
    cpu.a = 0x55u8;
    cpu.y = 0x80u8;
    let program = vec![
        0x98, // TYA
        0x00, // BRK
    ];
    cpu.load_program(&program, 0x0600);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        if opcode == 0x00 {
            break;
        }
        cpu.step();
    }
    assert_eq!(cpu.a, 0x80u8);
    assert_eq!(cpu.p.is_negative(), true);
    assert_eq!(cpu.p.is_zero(), false);
}
