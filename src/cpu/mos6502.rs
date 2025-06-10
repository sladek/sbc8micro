////////////////////////////////
/// mod disassembler;
/// mod memory;
/// mod cpu;
/// mod status;
/// 
/// ```
///  fn main() {
///     let mut cpu = cpu::mos6502::Cpu::new();
/// 
///     // Program: LDA #$00; LDX #$FF; BRK
///     let program = vec![
///         0xA9, 0x00, // LDA #0x00 => sets Z flag
///         0xA2, 0xFF, // LDX #0xFF => sets N flag
///         0x00,       // BRK
///     ];
/// 
///     cpu.load_program(&program, 0x0600);
/// 
///     loop {
///         let opcode = cpu.memory.read_byte(cpu.pc);
///         cpu.step();
///         if opcode == 0x00 {
///             break;
///         }
///     }
/// 
///     println!("A = {:02X}, X = {:02X}", cpu.a, cpu.x);
///     println!("Flags: Z={}, N={}", cpu.p.is_zero(), cpu.p.is_negative());
/// }
/// ```
/// 
/// Result should be:
/// LDA #$00
/// LDX #$FF
/// BRK (break)
/// A = 00, X = FF
/// Flags: Z=false, N=true
////////////////////////////////
use crate::memory::Memory;
use crate::status::mos6502::Status;

pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub p: Status,
    pub memory: Memory,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,
            pc: 0,
            p: Status::default(),
            memory: Memory::new(),
        }
    }

    pub fn load_program(&mut self, program: &[u8], start_addr: u16) {
        self.memory.load_program(program, start_addr);
        self.pc = start_addr;
    }

    fn push(&mut self, val: u8) {
        let addr = 0x0100u16 + self.sp as u16;
        self.memory.write_byte(addr, val);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let addr = 0x0100u16 + self.sp as u16;
        self.memory.read_byte(addr)
    }

    fn push_word(&mut self, val: u16) {
        self.push((val >> 8) as u8);
        self.push((val & 0xFF) as u8);
    }

    fn pop_word(&mut self) -> u16 {
        let low = self.pop() as u16;
        let high = self.pop() as u16;
        (high << 8) | low
    }

    fn read_zero_page(&mut self) -> u8 {
        let addr = self.memory.read_byte(self.pc) as u16;
        self.pc += 1;
        self.memory.read_byte(addr)
    }

    fn read_absolute(&mut self) -> u8 {
        let addr = self.memory.read_word(self.pc);
        self.pc += 2;
        self.memory.read_byte(addr)
    }

    fn read_zero_page_x(&mut self) -> u8 {
        let base = self.memory.read_byte(self.pc);
        self.pc += 1;
        let addr = base.wrapping_add(self.x) as u16;
        self.memory.read_byte(addr)
    }

    fn read_absolute_x(&mut self) -> u8 {
        let base = self.memory.read_word(self.pc);
        self.pc += 2;
        let addr = base.wrapping_add(self.x as u16);
        self.memory.read_byte(addr)
    }

    fn read_absolute_y(&mut self) -> u8 {
        let base = self.memory.read_word(self.pc);
        self.pc += 2;
        let addr = base.wrapping_add(self.y as u16);
        self.memory.read_byte(addr)
    }
    // ($addr, X)
    fn read_indexed_indirect(&mut self) -> u8 {
        let base = self.memory.read_byte(self.pc).wrapping_add(self.x);
        self.pc += 1;
        let addr = self.memory.read_word_zero_page(base);
        self.memory.read_byte(addr)
    }

    // ($addr), Y
    fn read_indirect_indexed(&mut self) -> u8 {
        let zp_addr = self.memory.read_byte(self.pc);
        self.pc += 1;
        let base = self.memory.read_word_zero_page(zp_addr);
        let addr = base.wrapping_add(self.y as u16);
        self.memory.read_byte(addr)
    }
    fn write_zero_page_x(&mut self, value: u8) {
        let base = self.memory.read_byte(self.pc);
        self.pc += 1;
        let addr = base.wrapping_add(self.x);
        self.memory.write_byte(addr as u16, value);
    }

    fn write_absolute_x(&mut self, value: u8) {
        let base = self.memory.read_word(self.pc);
        self.pc += 2;
        let addr = base.wrapping_add(self.x as u16);
        self.memory.write_byte(addr, value);
    }

    fn write_absolute_y(&mut self, value: u8) {
        let base = self.memory.read_word(self.pc);
        self.pc += 2;
        let addr = base.wrapping_add(self.y as u16);
        self.memory.write_byte(addr, value);
    }

    fn write_indexed_indirect(&mut self, value: u8) {
        let base = self.memory.read_byte(self.pc).wrapping_add(self.x);
        self.pc += 1;
        let addr = self.memory.read_word_zero_page(base);
        self.memory.write_byte(addr, value);
    }

    fn write_indirect_indexed(&mut self, value: u8) {
        let zp_addr = self.memory.read_byte(self.pc);
        self.pc += 1;
        let base = self.memory.read_word_zero_page(zp_addr);
        let addr = base.wrapping_add(self.y as u16);
        self.memory.write_byte(addr, value);
    }
    fn asl(&mut self, value: u8) -> u8 {
        let result = value << 1;
        self.p.set_carry((value & 0x80) != 0);
        self.p.set_zero(result);
        self.p.set_negative(result);
        result
    }

    fn lsr(&mut self, value: u8) -> u8 {
        let result = value >> 1;
        self.p.set_carry((value & 0x01) != 0);
        self.p.set_zero(result);
        self.p.set_negative(result); // always 0, but set for consistency
        result
    }

    fn rol(&mut self, value: u8) -> u8 {
        let carry_in = self.p.is_carry() as u8;
        let result = (value << 1) | carry_in;
        self.p.set_carry((value & 0x80) != 0);
        self.p.set_zero(result);
        self.p.set_negative(result);
        result
    }

    fn ror(&mut self, value: u8) -> u8 {
        let carry_in = if self.p.is_carry() { 0x80 } else { 0 };
        let result = (value >> 1) | carry_in;
        self.p.set_carry((value & 0x01) != 0);
        self.p.set_zero(result);
        self.p.set_negative(result);
        result
    }
    fn inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.p.set_zero(result);
        self.p.set_negative(result);
        result
    }

    fn dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.p.set_zero(result);
        self.p.set_negative(result);
        result
    }

    fn bit(&mut self, value: u8) {
        let result = self.a & value;
        self.p.set_zero(result);
        self.p.set_negative(value);
        self.p.set_overflow((value & 0x40) != 0);
    }

    pub fn step(&mut self) {
        let opcode = self.memory.read_byte(self.pc);
        self.pc += 1;

        match opcode {
            0xA9 => {
                // LDA Immediate
                let value = self.memory.read_byte(self.pc);
                self.pc += 1;
                self.a = value;
                self.p.set_zero(value);
                self.p.set_negative(value);
                println!("LDA #${:02X}", value);
            }

            0xA2 => {
                // LDX Immediate
                let value = self.memory.read_byte(self.pc);
                self.pc += 1;
                self.x = value;
                self.p.set_zero(value);
                self.p.set_negative(value);
                println!("LDX #${:02X}", value);
            }

            0x00 => {
                println!("BRK (break)");
            }

            0x8D => {
                // STA $nnnn
                let addr = self.memory.read_word(self.pc);
                self.pc += 2;
                self.memory.write_byte(addr, self.a);
                println!("STA ${:04X}", addr);
            }

            0x69 => {
                // ADC #imm
                let value = self.memory.read_byte(self.pc);
                self.pc += 1;

                let carry_in = if self.p.is_carry() { 1 } else { 0 };
                let sum = self.a as u16 + value as u16 + carry_in;
                let result = sum as u8;

                self.p.set_carry(sum > 0xFF);
                self.p.set_zero(result);
                self.p.set_negative(result);

                let overflow = ((self.a ^ result) & (value ^ result) & 0x80) != 0;
                self.p.set_overflow(overflow);

                self.a = result;

                println!("ADC #${:02X} => A=${:02X}", value, self.a);
            }

            0xF0 => {
                // BEQ (Branch if Equal / Zero flag set)
                let offset = self.memory.read_byte(self.pc) as i8;
                self.pc += 1;
                if self.p.is_zero() {
                    let old_pc = self.pc;
                    self.pc = self.pc.wrapping_add(offset as u16);
                    println!("BEQ taken to ${:04X} (offset: {:+})", self.pc, offset);
                } else {
                    println!("BEQ not taken");
                }
            }

            0xD0 => {
                // BNE (Branch if Not Equal / Zero flag clear)
                let offset = self.memory.read_byte(self.pc) as i8;
                self.pc += 1;
                if !self.p.is_zero() {
                    let old_pc = self.pc;
                    self.pc = self.pc.wrapping_add(offset as u16);
                    println!("BNE taken to ${:04X} (offset: {:+})", self.pc, offset);
                } else {
                    println!("BNE not taken");
                }
            }
            0x10 => {
                // BPL
                let offset = self.memory.read_byte(self.pc) as i8;
                self.pc += 1;
                if !self.p.is_negative() {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    println!("BPL taken");
                } else {
                    println!("BPL not taken");
                }
            }

            0x30 => {
                // BMI
                let offset = self.memory.read_byte(self.pc) as i8;
                self.pc += 1;
                if self.p.is_negative() {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    println!("BMI taken");
                } else {
                    println!("BMI not taken");
                }
            }

            0x90 => {
                // BCC
                let offset = self.memory.read_byte(self.pc) as i8;
                self.pc += 1;
                if !self.p.is_carry() {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    println!("BCC taken");
                } else {
                    println!("BCC not taken");
                }
            }

            0xB0 => {
                // BCS
                let offset = self.memory.read_byte(self.pc) as i8;
                self.pc += 1;
                if self.p.is_carry() {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    println!("BCS taken");
                } else {
                    println!("BCS not taken");
                }
            }

            0x50 => {
                // BVC
                let offset = self.memory.read_byte(self.pc) as i8;
                self.pc += 1;
                if self.p.value & 0x40 == 0 {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    println!("BVC taken");
                } else {
                    println!("BVC not taken");
                }
            }

            0x70 => {
                // BVS
                let offset = self.memory.read_byte(self.pc) as i8;
                self.pc += 1;
                if self.p.value & 0x40 != 0 {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    println!("BVS taken");
                } else {
                    println!("BVS not taken");
                }
            }
            0x29 => {
                // AND #imm
                let value = self.memory.read_byte(self.pc);
                self.pc += 1;
                self.a &= value;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("AND #${:02X} => A = {:02X}", value, self.a);
            }

            0x09 => {
                // ORA #imm
                let value = self.memory.read_byte(self.pc);
                self.pc += 1;
                self.a |= value;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("ORA #${:02X} => A = {:02X}", value, self.a);
            }

            0xC9 => {
                // CMP #imm
                let value = self.memory.read_byte(self.pc);
                self.pc += 1;
                let result = self.a.wrapping_sub(value);
                self.p.set_zero(result);
                self.p.set_negative(result);
                self.p.set_carry(self.a >= value);
                println!(
                    "CMP #${:02X} => result = {:02X}, C={}, Z={}, N={}",
                    value,
                    result,
                    self.p.is_carry(),
                    self.p.is_zero(),
                    self.p.is_negative()
                );
            }

            0x4C => {
                // JMP absolute
                let addr = self.memory.read_word(self.pc);
                self.pc = addr;
                println!("JMP ${:04X}", addr);
            }

            0x20 => {
                // JSR
                let addr = self.memory.read_word(self.pc);
                self.pc += 2;
                self.push_word(self.pc.wrapping_sub(1)); // push return address - 1
                self.pc = addr;
                println!("JSR ${:04X}", addr);
            }

            0x60 => {
                // RTS
                let return_addr = self.pop_word().wrapping_add(1);
                self.pc = return_addr;
                println!("RTS to ${:04X}", return_addr);
            }

            0x48 => {
                // PHA
                self.push(self.a);
                println!("PHA (push A = {:02X})", self.a);
            }

            0x68 => {
                // PLA
                self.a = self.pop();
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("PLA (pull A = {:02X})", self.a);
            }

            0x08 => {
                // PHP
                self.push(self.p.value | 0b0011_0000); // emulate B flag set when pushed
                println!("PHP (push P = {:08b})", self.p.value);
            }

            0x28 => {
                // PLP
                self.p.value = self.pop() & 0b1100_1111; // B and unused bits masked off
                println!("PLP (pull P = {:08b})", self.p.value);
            }

            0x49 => {
                // EOR #imm
                let value = self.memory.read_byte(self.pc);
                self.pc += 1;
                self.a ^= value;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("EOR #${:02X} => A = {:02X}", value, self.a);
            }

            // === AND ===
            0x25 => {
                // AND zp
                let val = self.read_zero_page();
                self.a &= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("AND $zp => A = {:02X}", self.a);
            }
            0x2D => {
                // AND abs
                let val = self.read_absolute();
                self.a &= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("AND $abs => A = {:02X}", self.a);
            }

            // === ORA ===
            0x05 => {
                // ORA zp
                let val = self.read_zero_page();
                self.a |= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("ORA $zp => A = {:02X}", self.a);
            }
            0x0D => {
                // ORA abs
                let val = self.read_absolute();
                self.a |= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("ORA $abs => A = {:02X}", self.a);
            }

            // === EOR ===
            0x45 => {
                // EOR zp
                let val = self.read_zero_page();
                self.a ^= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("EOR $zp => A = {:02X}", self.a);
            }
            0x4D => {
                // EOR abs
                let val = self.read_absolute();
                self.a ^= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("EOR $abs => A = {:02X}", self.a);
            }

            // === CMP ===
            0xC5 => {
                // CMP zp
                let val = self.read_zero_page();
                let result = self.a.wrapping_sub(val);
                self.p.set_zero(result);
                self.p.set_negative(result);
                self.p.set_carry(self.a >= val);
                println!("CMP $zp => result = {:02X}", result);
            }
            0xCD => {
                // CMP abs
                let val = self.read_absolute();
                let result = self.a.wrapping_sub(val);
                self.p.set_zero(result);
                self.p.set_negative(result);
                self.p.set_carry(self.a >= val);
                println!("CMP $abs => result = {:02X}", result);
            }

            0x35 => {
                // AND zp,X
                let val = self.read_zero_page_x();
                self.a &= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("AND $zp,X => A = {:02X}", self.a);
            }
            0x3D => {
                // AND abs,X
                let val = self.read_absolute_x();
                self.a &= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("AND $abs,X => A = {:02X}", self.a);
            }

            0x15 => {
                let val = self.read_zero_page_x();
                self.a |= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("ORA $zp,X => A = {:02X}", self.a);
            }
            0x1D => {
                let val = self.read_absolute_x();
                self.a |= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("ORA $abs,X => A = {:02X}", self.a);
            }
            0x55 => {
                let val = self.read_zero_page_x();
                self.a ^= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("EOR $zp,X => A = {:02X}", self.a);
            }
            0x5D => {
                let val = self.read_absolute_x();
                self.a ^= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("EOR $abs,X => A = {:02X}", self.a);
            }
            0xD5 => {
                let val = self.read_zero_page_x();
                let result = self.a.wrapping_sub(val);
                self.p.set_zero(result);
                self.p.set_negative(result);
                self.p.set_carry(self.a >= val);
                println!("CMP $zp,X => result = {:02X}", result);
            }
            0xDD => {
                let val = self.read_absolute_x();
                let result = self.a.wrapping_sub(val);
                self.p.set_zero(result);
                self.p.set_negative(result);
                self.p.set_carry(self.a >= val);
                println!("CMP $abs,X => result = {:02X}", result);
            }
            0x39 => {
                let val = self.read_absolute_y();
                self.a &= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("AND $abs,Y => A = {:02X}", self.a);
            }
            0x19 => {
                let val = self.read_absolute_y();
                self.a |= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("ORA $abs,Y => A = {:02X}", self.a);
            }
            0x59 => {
                let val = self.read_absolute_y();
                self.a ^= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("EOR $abs,Y => A = {:02X}", self.a);
            }
            0xD9 => {
                let val = self.read_absolute_y();
                let result = self.a.wrapping_sub(val);
                self.p.set_zero(result);
                self.p.set_negative(result);
                self.p.set_carry(self.a >= val);
                println!("CMP $abs,Y => result = {:02X}", result);
            }
            0x21 => {
                let val = self.read_indexed_indirect();
                self.a &= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("AND ($zp,X) => A = {:02X}", self.a);
            }
            0x31 => {
                let val = self.read_indirect_indexed();
                self.a &= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("AND ($zp),Y => A = {:02X}", self.a);
            }
            0x01 => {
                let val = self.read_indexed_indirect();
                self.a |= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("ORA ($zp,X) => A = {:02X}", self.a);
            }
            0x11 => {
                let val = self.read_indirect_indexed();
                self.a |= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("ORA ($zp),Y => A = {:02X}", self.a);
            }
            0x41 => {
                let val = self.read_indexed_indirect();
                self.a ^= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("EOR ($zp,X) => A = {:02X}", self.a);
            }
            0x51 => {
                let val = self.read_indirect_indexed();
                self.a ^= val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("EOR ($zp),Y => A = {:02X}", self.a);
            }
            0xC1 => {
                let val = self.read_indexed_indirect();
                let result = self.a.wrapping_sub(val);
                self.p.set_zero(result);
                self.p.set_negative(result);
                self.p.set_carry(self.a >= val);
                println!("CMP ($zp,X) => result = {:02X}", result);
            }
            0xD1 => {
                let val = self.read_indirect_indexed();
                let result = self.a.wrapping_sub(val);
                self.p.set_zero(result);
                self.p.set_negative(result);
                self.p.set_carry(self.a >= val);
                println!("CMP ($zp),Y => result = {:02X}", result);
            }
            0xA9 => {
                // LDA Immediate
                let val = self.memory.read_byte(self.pc);
                self.pc += 1;
                self.a = val;
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("LDA #${:02X}", self.a);
            }
            0xA5 => {
                // LDA Zero Page
                let addr = self.memory.read_byte(self.pc) as u16;
                self.pc += 1;
                self.a = self.memory.read_byte(addr);
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("LDA ${:02X}", addr);
            }
            0xAD => {
                // LDA Absolute
                let addr = self.memory.read_word(self.pc);
                self.pc += 2;
                self.a = self.memory.read_byte(addr);
                self.p.set_zero(self.a);
                self.p.set_negative(self.a);
                println!("LDA ${:04X}", addr);
            }
            0x85 => {
                // STA Zero Page
                let addr = self.memory.read_byte(self.pc) as u16;
                self.pc += 1;
                self.memory.write_byte(addr, self.a);
                println!("STA ${:02X}", addr);
            }
            0x8D => {
                // STA Absolute
                let addr = self.memory.read_word(self.pc);
                self.pc += 2;
                self.memory.write_byte(addr, self.a);
                println!("STA ${:04X}", addr);
            }
            0xA2 => {
                // LDX Immediate
                let val = self.memory.read_byte(self.pc);
                self.pc += 1;
                self.x = val;
                self.p.set_zero(self.x);
                self.p.set_negative(self.x);
                println!("LDX #${:02X}", self.x);
            }
            0xA6 => {
                // LDX Zero Page
                let addr = self.memory.read_byte(self.pc) as u16;
                self.pc += 1;
                self.x = self.memory.read_byte(addr);
                self.p.set_zero(self.x);
                self.p.set_negative(self.x);
                println!("LDX ${:02X}", addr);
            }
            0x86 => {
                // STX Zero Page
                let addr = self.memory.read_byte(self.pc) as u16;
                self.pc += 1;
                self.memory.write_byte(addr, self.x);
                println!("STX ${:02X}", addr);
            }
            0xA0 => {
                // LDY Immediate
                let val = self.memory.read_byte(self.pc);
                self.pc += 1;
                self.y = val;
                self.p.set_zero(self.y);
                self.p.set_negative(self.y);
                println!("LDY #${:02X}", self.y);
            }
            0xA4 => {
                // LDY Zero Page
                let addr = self.memory.read_byte(self.pc) as u16;
                self.pc += 1;
                self.y = self.memory.read_byte(addr);
                self.p.set_zero(self.y);
                self.p.set_negative(self.y);
                println!("LDY ${:02X}", addr);
            }
            0x84 => {
                // STY Zero Page
                let addr = self.memory.read_byte(self.pc) as u16;
                self.pc += 1;
                self.memory.write_byte(addr, self.y);
                println!("STY ${:02X}", addr);
            }
            0x0A => {
                // ASL A
                self.a = self.asl(self.a);
                println!("ASL A => {:02X}", self.a);
            }
            0x06 => {
                // ASL Zero Page
                let addr = self.memory.read_byte(self.pc) as u16;
                self.pc += 1;
                let val = self.memory.read_byte(addr);
                let result = self.asl(val);
                self.memory.write_byte(addr, result);
                println!("ASL ${:02X} => {:02X}", addr, result);
            }
            0x0E => {
                // ASL Absolute
                let addr = self.memory.read_word(self.pc);
                self.pc += 2;
                let val = self.memory.read_byte(addr);
                let result = self.asl(val);
                self.memory.write_byte(addr, result);
                println!("ASL ${:04X} => {:02X}", addr, result);
            }
            0x4A => {
                // LSR A
                self.a = self.lsr(self.a);
                println!("LSR A => {:02X}", self.a);
            }
            0x46 => {
                let addr = self.memory.read_byte(self.pc) as u16;
                self.pc += 1;
                let val = self.memory.read_byte(addr);
                let result = self.lsr(val);
                self.memory.write_byte(addr, result);
                println!("LSR ${:02X} => {:02X}", addr, result);
            }
            0x4E => {
                let addr = self.memory.read_word(self.pc);
                self.pc += 2;
                let val = self.memory.read_byte(addr);
                let result = self.lsr(val);
                self.memory.write_byte(addr, result);
                println!("LSR ${:04X} => {:02X}", addr, result);
            }
            0x2A => {
                self.a = self.rol(self.a);
                println!("ROL A => {:02X}", self.a);
            }
            0x26 => {
                let addr = self.memory.read_byte(self.pc) as u16;
                self.pc += 1;
                let val = self.memory.read_byte(addr);
                let result = self.rol(val);
                self.memory.write_byte(addr, result);
                println!("ROL ${:02X} => {:02X}", addr, result);
            }
            0x2E => {
                let addr = self.memory.read_word(self.pc);
                self.pc += 2;
                let val = self.memory.read_byte(addr);
                let result = self.rol(val);
                self.memory.write_byte(addr, result);
                println!("ROL ${:04X} => {:02X}", addr, result);
            }
            0x6A => {
                self.a = self.ror(self.a);
                println!("ROR A => {:02X}", self.a);
            }
            0x66 => {
                let addr = self.memory.read_byte(self.pc) as u16;
                self.pc += 1;
                let val = self.memory.read_byte(addr);
                let result = self.ror(val);
                self.memory.write_byte(addr, result);
                println!("ROR ${:02X} => {:02X}", addr, result);
            }
            0x6E => {
                let addr = self.memory.read_word(self.pc);
                self.pc += 2;
                let val = self.memory.read_byte(addr);
                let result = self.ror(val);
                self.memory.write_byte(addr, result);
                println!("ROR ${:04X} => {:02X}", addr, result);
            }
            0xE6 => {
                // INC Zero Page
                let addr = self.memory.read_byte(self.pc) as u16;
                self.pc += 1;
                let val = self.memory.read_byte(addr);
                let result = self.inc(val);
                self.memory.write_byte(addr, result);
                println!("INC ${:02X} => {:02X}", addr, result);
            }
            0xEE => {
                // INC Absolute
                let addr = self.memory.read_word(self.pc);
                self.pc += 2;
                let val = self.memory.read_byte(addr);
                let result = self.inc(val);
                self.memory.write_byte(addr, result);
                println!("INC ${:04X} => {:02X}", addr, result);
            }
            0xC6 => {
                // DEC Zero Page
                let addr = self.memory.read_byte(self.pc) as u16;
                self.pc += 1;
                let val = self.memory.read_byte(addr);
                let result = self.dec(val);
                self.memory.write_byte(addr, result);
                println!("DEC ${:02X} => {:02X}", addr, result);
            }
            0xCE => {
                // DEC Absolute
                let addr = self.memory.read_word(self.pc);
                self.pc += 2;
                let val = self.memory.read_byte(addr);
                let result = self.dec(val);
                self.memory.write_byte(addr, result);
                println!("DEC ${:04X} => {:02X}", addr, result);
            }
            0xE8 => {
                // INX
                self.x = self.inc(self.x);
                println!("INX => {:02X}", self.x);
            }
            0xCA => {
                // DEX
                self.x = self.dec(self.x);
                println!("DEX => {:02X}", self.x);
            }
            0xC8 => {
                // INY
                self.y = self.inc(self.y);
                println!("INY => {:02X}", self.y);
            }
            0x88 => {
                // DEY
                self.y = self.dec(self.y);
                println!("DEY => {:02X}", self.y);
            }
            0x24 => {
                // BIT Zero Page
                let addr = self.memory.read_byte(self.pc) as u16;
                self.pc += 1;
                let value = self.memory.read_byte(addr);
                self.bit(value);
                println!("BIT ${:02X}", addr);
            }
            0x2C => {
                // BIT Absolute
                let addr = self.memory.read_word(self.pc);
                self.pc += 2;
                let value = self.memory.read_byte(addr);
                self.bit(value);
                println!("BIT ${:04X}", addr);
            }
            0x18 => {
                self.p.set_carry(false);
                println!("CLC");
            }
            0x38 => {
                self.p.set_carry(true);
                println!("SEC");
            }
            0x58 => {
                self.p.set_interrupt_disable(false);
                println!("CLI");
            }
            0x78 => {
                self.p.set_interrupt_disable(true);
                println!("SEI");
            }
            0xD8 => {
                self.p.set_decimal_mode(false);
                println!("CLD");
            }
            0xF8 => {
                self.p.set_decimal_mode(true);
                println!("SED");
            }
            0xB8 => {
                self.p.set_overflow(false);
                println!("CLV");
            }

            _ => {
                println!("Unknown opcode ${:02X}", opcode);
            }
        }
    }
}
