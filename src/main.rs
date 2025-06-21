mod cpu;
mod memory;
mod status;

use cpu::mos6502;

fn main() {
    let mut cpu = mos6502::Cpu::new();
    let addr = 0x0010u16;
    let value16 = 0x1234u16;
    let value8 = 0x42u8;
    let z_addr: u8 = 0x10;
    cpu.x = 0x0F;
    cpu.y = 0x0F;
    cpu.memory.write_word(addr, value16);
    cpu.memory.write_byte(addr, value8);
    cpu.memory.write_byte_zero_page(z_addr, value8);
    cpu.p.set_carry(false);
    cpu.sp = 0xff;
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
    cpu.load_program(&program, 0x0600);
    cpu.load_program(&program_2, 0x1234);
    cpu.pc = 0x0600;
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0x00 {
            break;
        }
    }
    let mm = &cpu.memory.get_data()[0x100..=0x1ff];
    println!("stack = {:?}", mm);

    println!(
        "A = {:02X}, X = {:02X}, Y = {:02X}, N = {}, Z = {}, C = {}, I = {}, D = {}, V = {}, SP = {}",
        cpu.a,
        cpu.x,
        cpu.y,
        cpu.p.is_negative(),
        cpu.p.is_zero(),
        cpu.p.is_carry(),
        cpu.p.is_interrupt_disable(),
        cpu.p.is_decimal_mode(),
        cpu.p.is_overflow(),
        cpu.sp
    );
}
