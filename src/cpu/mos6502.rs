///////////////////////////////////////////////////////////////////////////////
/// ```
/// mod disassembler;
/// mod memory;
/// mod cpu;
/// mod status;
///
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
///////////////////////////////////////////////////////////////////////////////
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

    ///
    /// Loads program to the memory and set PC to start address of the programm
    ///
    pub fn load_program(&mut self, program: &[u8], start_addr: u16) {
        self.memory.load_program(program, start_addr);
        self.pc = start_addr;
    }

    fn push(&mut self, val: u8) {
        let addr = 0x0100u16 + self.sp as u16;
        self.memory.write_byte(addr, val);
        println!("push -> sp={:?}, sp_addr={:?}", self.sp, addr);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let addr = 0x0100u16 + self.sp as u16;
        self.memory.read_byte(addr)
    }

    fn push_word(&mut self, val: u16) {
        println!("Pushing word={:?}", val);
        self.push((val >> 8) as u8);
        self.push((val & 0xFF) as u8);
    }

    fn pop_word(&mut self) -> u16 {
        let low = self.pop() as u16;
        let high = self.pop() as u16;
        (high << 8) | low
    }

    fn read_immediate_byte(&mut self) -> u8 {
        let value = self.memory.read_byte(self.pc);
        self.pc += 1;
        value
    }
    fn read_immediate_word(&mut self) -> u16 {
        let value = self.memory.read_word(self.pc);
        self.pc += 2;
        value
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
    fn get_zero_page_address(&self) -> u8 {
        let addr: u8 = self.memory.read_byte(self.pc);
        addr
    }
    fn get_zero_page_address_x(&self) -> u8 {
        let addr: u8 = self.memory.read_byte(self.pc).wrapping_add(self.x);
        addr
    }
    fn get_absolute_address(&self) -> u16 {
        let addr: u16 = self.memory.read_word(self.pc);
        addr
    }
    fn get_absolute_address_x(&self) -> u16 {
        let addr: u16 = self.memory.read_word(self.pc).wrapping_add(self.x as u16);
        addr
    }
    fn read_zero_page_x(&mut self) -> u8 {
        let base = self.memory.read_byte(self.pc);
        self.pc += 1;
        let addr = base.wrapping_add(self.x) as u16;
        self.memory.read_byte(addr)
    }
    fn read_zero_page_y(&mut self) -> u8 {
        let base = self.memory.read_byte(self.pc);
        self.pc += 1;
        let addr = base.wrapping_add(self.y) as u16;
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

    fn asl(&mut self, value: u8) -> u8 {
        let result = value << 1;
        self.p.set_carry((value & 0x80) != 0);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0);
        result
    }

    fn lsr(&mut self, value: u8) -> u8 {
        let result = value >> 1;
        self.p.set_carry((value & 0x01) != 0);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0); // always 0, but set for consistency
        result
    }

    fn rol(&mut self, value: u8) -> u8 {
        let carry_in = self.p.is_carry() as u8;
        let result = (value << 1) | carry_in;
        self.p.set_carry((value & 0x80) != 0);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0);
        result
    }

    fn ror(&mut self, value: u8) -> u8 {
        let carry_in = if self.p.is_carry() { 0x80 } else { 0 };
        let result = (value >> 1) | carry_in;
        self.p.set_carry((value & 0x01) != 0);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0);
        result
    }
    fn inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0);
        result
    }

    fn dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0);
        result
    }

    fn bit(&mut self, value: u8) {
        let result = self.a & value;
        self.p.set_zero(result == 0);
        self.p.set_negative(value & 0x80 != 0);
        self.p.set_overflow(value & 0x40 != 0);
    }
    fn adc(&mut self, value: u8) {
        let carry = if self.p.is_carry() { 1 } else { 0 };
        let sum = self.a as u16 + value as u16 + carry;
        self.p
            .set_overflow(((self.a ^ sum as u8) & (value ^ sum as u8) & 0x80) != 0);
        self.a = sum as u8;
        self.p.set_carry(sum > 0xFF);
        self.p.set_zero(self.a == 0);
        self.p.set_negative(self.a & 0x80 != 0);
    }

    fn and(&mut self, value: u8) {
        self.a &= value;
        self.p.set_zero(self.a == 0);
        self.p.set_negative(self.a & 0x80 != 0);
    }

    fn sbc(&mut self, value: u8) {
        let carry = if self.p.is_carry() { 1 } else { 0 };
        let a = self.a;
        // Perform the subtraction using two's complement: A + (~M + 1 - (1 - C)) = A - M - (1 - C)
        let value = value ^ 0xFF;
        let sum = a as u16 + value as u16 + carry as u16;
        self.p
            .set_overflow(((self.a ^ sum as u8) & (self.a ^ value) & 0x80) != 0);
        self.a = sum as u8;
        // Update flags
        self.p.set_carry(sum > 0xFF);
        self.p.set_zero(self.a == 0);
        self.p.set_negative(self.a & 0x80 != 0);
    }
    fn cmp(&mut self, value: u8) {
        let result = self.a.wrapping_sub(value);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0);
        self.p.set_carry(self.a >= value);
    }
    fn cpx(&mut self, value: u8) {
        let x = self.x;
        let result = x.wrapping_sub(value);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0);
        self.p.set_carry(self.x >= value);
    }
    fn cpy(&mut self, value: u8) {
        let y = self.y;
        let result = y.wrapping_sub(value);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0);
        self.p.set_carry(self.y >= value);
    }
    fn eor(&mut self, value: u8) {
        self.pc += 1;
        self.a ^= value;
        self.p.set_zero(self.a == 0);
        self.p.set_negative(self.a & 0x80 != 0);
    }
    fn lda(&mut self, value: u8) {
        self.a = value;
        self.p.set_zero(value == 0);
        self.p.set_negative(value & 0x80 != 0);
    }
    fn ldx(&mut self, value: u8) {
        self.x = value;
        self.p.set_zero(value == 0);
        self.p.set_negative(value & 0x80 != 0);
    }
    fn ldy(&mut self, value: u8) {
        self.y = value;
        self.p.set_zero(value == 0);
        self.p.set_negative(value & 0x80 != 0);
    }
    fn ora(&mut self, value: u8) {
        self.a |= value;
        self.p.set_zero(self.a == 0);
        self.p.set_negative(self.a & 0x80 != 0);
    }
    pub fn step(&mut self) {
        let opcode = self.memory.read_byte(self.pc);
        self.pc += 1;

        match opcode {
            ////////////////// Start of ADC
            // ADC #oper ;immediate
            0x69 => {
                let value = self.read_immediate_byte();
                self.adc(value);
                println!("ADC #imm => A=${:02X}", self.a);
            }
            // ADC oper ;zero page
            0x65 => {
                let value = self.read_zero_page();
                self.adc(value);
                println!("ADC zp => A=${:02X}", self.a);
            }
            // ADC oper ;zero page,X
            0x75 => {
                let value = self.read_zero_page_x();
                self.adc(value);
                println!("ADC zp,X => A=${:02X}", self.a);
            }
            // ADC oper ;absolute
            0x6D => {
                let value = self.read_absolute();
                self.adc(value);
                println!("ADC abs => A=${:02X}", self.a);
            }
            // ADC oper ;absolute,X
            0x7D => {
                let value = self.read_absolute_x();
                self.adc(value);
                println!("ADC abs,X => A=${:02X}", self.a);
            }
            // ADC abs,Y ;absolute,Y
            0x79 => {
                let value = self.read_absolute_y();
                self.adc(value);
                println!("ADC abs,Y => A=${:02X}", self.a);
            }
            // ADC (oper,X) ;(indexed indirect)
            0x61 => {
                let value = self.read_indexed_indirect();
                self.adc(value);
                println!("ADC (zp,X) => A=${:02X}", self.a);
            }
            // ADC (oper),Y ;(indexed indirect),Y
            0x71 => {
                let value = self.read_indirect_indexed();
                self.adc(value);
                println!("ADC (zp),Y => A=${:02X}", self.a);
            }
            ////////////////// End of ADC
            ////////////////// Start of AND
            0x29 => {
                let value = self.read_immediate_byte();
                self.and(value);
                println!("AND #imm => A = {:02X}", self.a);
            }
            0x25 => {
                // AND zp
                let value = self.read_zero_page();
                self.and(value);
                println!("AND zp => A = {:02X}", self.a);
            }
            0x35 => {
                // AND zp,X
                let val = self.read_zero_page_x();
                self.a &= val;
                self.p.set_zero(self.a == 0);
                self.p.set_negative(self.a & 0x80 != 0);
                println!("AND $zp,X => A = {:02X}", self.a);
            }
            0x2D => {
                // AND abs
                let val = self.read_absolute();
                self.a &= val;
                self.p.set_zero(self.a == 0);
                self.p.set_negative(self.a & 0x80 != 0);
                println!("AND $abs => A = {:02X}", self.a);
            }
            0x3D => {
                // AND abs,X
                let val = self.read_absolute_x();
                self.a &= val;
                self.p.set_zero(self.a == 0);
                self.p.set_negative(self.a & 0x80 != 0);
                println!("AND $abs,X => A = {:02X}", self.a);
            }
            0x39 => {
                // AND abs,Y
                let val = self.read_absolute_y();
                self.a &= val;
                self.p.set_zero(self.a == 0);
                self.p.set_negative(self.a & 0x80 != 0);
                println!("AND $abs,Y => A = {:02X}", self.a);
            }
            0x21 => {
                // AND (indirect,X)
                let val = self.read_indexed_indirect();
                self.a &= val;
                self.p.set_zero(self.a == 0);
                self.p.set_negative(self.a & 0x80 != 0);
                println!("AND ($zp,X) => A = {:02X}", self.a);
            }
            0x31 => {
                // AND (indirect),Y
                let val = self.read_indirect_indexed();
                self.a &= val;
                self.p.set_zero(self.a == 0);
                self.p.set_negative(self.a & 0x80 != 0);
                println!("AND ($zp),Y => A = {:02X}", self.a);
            }
            ////////////////// End of AND
            ////////////////// Start of ASL
            0x0A => {
                // ASL A
                self.a = self.asl(self.a);
                println!("ASL A => {:02X}", self.a);
            }
            0x06 => {
                // ASL Zero Page
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                let result = self.asl(value);
                self.memory.write_byte_zero_page(addr, result);
                println!("ASL $zp => {:02X}", result);
            }
            0x16 => {
                // ASL Zero Page,X
                let addr = self.get_zero_page_address_x();
                let value = self.read_zero_page_x();
                let result = self.asl(value);
                self.memory.write_byte(addr as u16, result);
                println!("ASL $zp => {:02X}", self.a);
            }
            0x0E => {
                // ASL Absolute
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                let result = self.asl(value);
                self.memory.write_byte(addr, result);
                println!("ASL absolute => {:02X}", result);
            }
            0x1E => {
                // ASL Absolute,X
                let addr = self.get_absolute_address().wrapping_add(self.x as u16);
                let value = self.read_absolute_x();
                let result = self.asl(value);
                self.memory.write_byte(addr, result);
                println!("ASL absolute => {:02X}", result);
            }
            ////////////////// End of ASL
            ////////////////// Start of BCC
            0x90 => {
                // BCC
                let offset = self.read_immediate_byte();
                if !self.p.is_carry() {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    println!("BCC taken");
                } else {
                    println!("BCC not taken");
                }
            }
            ////////////////// End of BCC
            ////////////////// Start of BCS
            0xB0 => {
                // BCS
                let offset = self.read_immediate_byte();
                if self.p.is_carry() {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    println!("BCS taken");
                } else {
                    println!("BCS not taken");
                }
            }
            ////////////////// End of BCS
            ////////////////// Start of BEQ
            0xF0 => {
                // BEQ (Branch if Equal / Zero flag set)
                let offset = self.read_immediate_byte();
                if self.p.is_zero() {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    println!("BEQ taken to ${:04X} (offset: {:+})", self.pc, offset);
                } else {
                    println!("BEQ not taken");
                }
            }
            ////////////////// End of BEQ
            ////////////////// Start of BIT
            0x24 => {
                // BIT Zero Page
                let value = self.read_zero_page();
                self.bit(value);
                println!("BIT zp");
            }
            0x2C => {
                // BIT Absolute
                let addr = self.read_immediate_word();
                let value = self.memory.read_byte(addr);
                self.bit(value);
                println!("BIT abs");
            }
            ////////////////// End of BIT
            ////////////////// Start of BMI
            0x30 => {
                // BMI
                let offset = self.read_immediate_byte();
                if self.p.is_negative() {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    println!("BMI taken");
                } else {
                    println!("BMI not taken");
                }
            }
            ////////////////// End of BMI
            ////////////////// Start of BNE
            0xD0 => {
                // BNE (Branch if Not Equal / Zero flag clear)
                let offset = self.read_immediate_byte();
                if !self.p.is_zero() {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    println!("BNE taken to ${:04X} (offset: {:+})", self.pc, offset);
                } else {
                    println!("BNE not taken");
                }
            }
            ////////////////// End of BNE
            ////////////////// Start of BPL
            0x10 => {
                // BPL
                let offset = self.read_immediate_byte();
                if !self.p.is_negative() {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    println!("BPL taken");
                } else {
                    println!("BPL not taken");
                }
            }
            ////////////////// End of BPL
            ////////////////// Start of BRK
            0x00 => {
                println!("BRK (break)");
            }
            ////////////////// End of BRK
            ////////////////// Start of BVC
            0x50 => {
                // BVC
                let offset = self.read_immediate_byte();
                if self.p.value & 0x40 == 0 {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    println!("BVC taken");
                } else {
                    println!("BVC not taken");
                }
            }
            ////////////////// End of BVC
            ////////////////// Start of BVS
            0x70 => {
                // BVS
                let offset = self.read_immediate_byte();
                if self.p.value & 0x40 != 0 {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    println!("BVS taken");
                } else {
                    println!("BVS not taken");
                }
            }
            ////////////////// End of BVS
            ////////////////// Start of CLC
            0x18 => {
                self.p.set_carry(false);
                println!("CLC");
            }
            ////////////////// End of CLC
            ////////////////// Start of CLD
            0xD8 => {
                self.p.set_decimal_mode(false);
                println!("CLD");
            }
            ////////////////// End of CLD
            ////////////////// Start of CLI
            0x58 => {
                self.p.set_interrupt_disable(false);
                println!("CLI");
            }
            ////////////////// End of CLI
            ////////////////// Start of CLV
            0xB8 => {
                self.p.set_overflow(false);
                println!("CLV");
            }
            ////////////////// End of CLV
            ////////////////// Start of CMP
            0xC9 => {
                // CMP #imm
                let value = self.read_immediate_byte();
                self.cmp(value);
                println!("CMP #$imm",);
            }
            0xC5 => {
                // CMP zp
                let value = self.read_zero_page();
                self.cmp(value);
                println!("CMP zp");
            }
            0xD5 => {
                let value = self.read_zero_page_x();
                self.cmp(value);
                println!("CMP zp,X");
            }
            0xCD => {
                // CMP abs
                let value = self.read_absolute();
                self.cmp(value);
                println!("CMP abs");
            }
            0xDD => {
                let value = self.read_absolute_x();
                self.cmp(value);
                println!("CMP abs,X");
            }
            0xD9 => {
                let value = self.read_absolute_y();
                self.cmp(value);
                println!("CMP abs,Y");
            }
            0xC1 => {
                let value = self.read_indexed_indirect();
                self.cmp(value);
                println!("CMP (zp,X)");
            }
            0xD1 => {
                let value = self.read_indirect_indexed();
                self.cmp(value);
                println!("CMP (zp),Y");
            }
            ////////////////// End of CMP
            ////////////////// Start of CPX
            0xE0 => {
                // CPX #imm
                let value = self.read_immediate_byte();
                self.cpx(value);
                println!("CPX #imm");
            }
            0xE4 => {
                // CPX zp
                let value = self.read_zero_page();
                self.cpx(value);
                println!("CPX zp");
            }
            0xEC => {
                // CPX abs
                let value = self.read_absolute();
                self.cpx(value);
                println!("CPX zp");
            }
            ////////////////// End of CPX
            ////////////////// Start of CPY
            0xC0 => {
                // CPY #imm
                let value = self.read_immediate_byte();
                self.cpy(value);
                println!("CPY #imm");
            }
            0xC4 => {
                // CPY zp
                let value = self.read_zero_page();
                self.cpy(value);
                println!("CPY zp");
            }
            0xCC => {
                // CPY abs
                let value = self.read_absolute();
                self.cpy(value);
                println!("CPY zp");
            }
            ////////////////// End of CPY
            ////////////////// Start of DEC
            0xC6 => {
                // DEC Zero Page
                let addr = self.get_zero_page_address();
                let val = self.read_zero_page();
                let result = self.dec(val);
                self.memory.write_byte(addr as u16, result);
                println!("DEC zp");
            }
            0xD6 => {
                // DEC Zero Page,X
                let addr = self.get_zero_page_address_x();
                let val = self.read_zero_page_x();
                let result = self.dec(val);
                self.memory.write_byte(addr as u16, result);
                println!("DEC zp,X");
            }
            0xCE => {
                // DEC Absolute
                let addr = self.get_absolute_address();
                let val = self.read_absolute();
                let result = self.dec(val);
                self.memory.write_byte(addr, result);
                println!("DEC abs");
            }
            0xDE => {
                // DEC Absolute
                let addr = self.get_absolute_address().wrapping_add(self.x as u16);
                let val = self.read_absolute_x();
                let result = self.dec(val);
                self.memory.write_byte(addr, result);
                println!("DEC abs,X");
            }
            ////////////////// End of DEC
            ////////////////// Start of DEX
            0xCA => {
                // DEX
                self.x = self.dec(self.x);
                println!("DEX");
            }
            ////////////////// End of DEX
            ////////////////// Start of DEY
            0x88 => {
                // DEY
                self.y = self.dec(self.y);
                println!("DEY");
            }
            ////////////////// Stop of DEY
            ////////////////// Start of EOR
            0x49 => {
                // EOR #imm
                let value = self.read_immediate_byte();
                self.eor(value);
                println!("EOR #imm");
            }
            0x45 => {
                // EOR zp
                let value = self.read_zero_page();
                self.eor(value);
                println!("EOR zp");
            }
            0x55 => {
                // EOR zp,X
                let value = self.read_zero_page_x();
                self.eor(value);
                println!("EOR zp");
            }
            0x4D => {
                // EOR abs
                let value = self.read_absolute();
                self.eor(value);
                println!("EOR abs");
            }
            0x5D => {
                // EOR abs,X
                let value = self.read_absolute_x();
                self.eor(value);
                println!("EOR abs,X");
            }
            0x59 => {
                // EOR abs,Y
                let value = self.read_absolute_y();
                self.eor(value);
                println!("EOR abs,Y");
            }
            0x41 => {
                // EOR indirect,X
                let value = self.read_indexed_indirect();
                self.eor(value);
                println!("EOR (zp,X)");
            }
            0x51 => {
                // EOR indirect,Y
                let value = self.read_indirect_indexed();
                self.eor(value);
                println!("EOR (zp),Y");
            }
            ////////////////// End of EOR
            ////////////////// Start of INC
            0xE6 => {
                // INC Zero Page
                let address = self.get_zero_page_address();
                let value = self.read_zero_page();
                let result = self.inc(value);
                self.memory.write_byte_zero_page(address, result);
                println!("INC zp");
            }
            0xF6 => {
                // INC Zero Page,X
                let address = self.get_zero_page_address_x();
                let value = self.read_zero_page_x();
                let result = self.inc(value);
                self.memory.write_byte_zero_page(address, result);
                println!("INC zp,X");
            }
            0xEE => {
                // INC Absolute
                let address = self.get_absolute_address();
                let value = self.read_absolute();
                let result = self.inc(value);
                self.memory.write_byte(address, result);
                println!("INC abs");
            }
            0xFE => {
                // INC Absolute,X
                let address = self.get_absolute_address_x();
                let value = self.read_absolute_x();
                let result = self.inc(value);
                self.memory.write_byte(address, result);
                println!("INC abs");
            }
            ////////////////// End of INC
            ////////////////// Start of INX
            0xE8 => {
                // INX
                self.x = self.inc(self.x);
                println!("INX");
            }
            ////////////////// End of INX
            ////////////////// Start of INY
            0xC8 => {
                // INY
                self.y = self.inc(self.y);
                println!("INY");
            }
            ////////////////// End of INY
            ////////////////// Start of JMP
            0x4C => {
                // JMP absolute
                let addr = self.memory.read_word(self.pc);
                self.pc = addr;
                println!("JMP ${:04X}", addr);
            }
            0x6C => {
                // JMP indirect
                let addr_lo = self.memory.read_byte(self.pc);
                let addr_hi = self.memory.read_byte(self.pc.wrapping_add(1));
                let jmp_addr_lo = self
                    .memory
                    .read_byte((addr_hi as u16) << 0x8 | addr_lo as u16);
                let jmp_addr_hi = self
                    .memory
                    .read_byte((addr_hi as u16) << 0x8 | addr_lo.wrapping_add(1) as u16);
                self.pc = (jmp_addr_hi as u16) << 8 | jmp_addr_lo as u16;
                println!("JMP ${:04X}", self.pc);
            }
            ////////////////// End of JMP
            ////////////////// Start of JSR
            0x20 => {
                // JSR
                let addr = self.read_immediate_word();
                self.push_word(self.pc.wrapping_sub(1)); // push return address - 1
                self.pc = addr;
                println!("JSR ${:04X}", addr);
            }
            ////////////////// End of JSR
            ////////////////// Start of LDA
            0xA9 => {
                // LDA Immediate
                let value = self.read_immediate_byte();
                self.lda(value);
                println!("LDA #imm");
            }
            0xA5 => {
                // LDA Zero Page
                let value = self.read_zero_page();
                self.lda(value);
                println!("LDA zp");
            }
            0xB5 => {
                // LDA Zero Page,X
                let value = self.read_zero_page_x();
                self.lda(value);
                println!("LDA zp");
            }
            0xAD => {
                // LDA Absolute
                let value = self.read_absolute();
                self.lda(value);
                println!("LDA $abs");
            }
            0xBD => {
                // LDA Absolute,X
                let value = self.read_absolute_x();
                self.lda(value);
                println!("LDA $abs,X");
            }
            0xB9 => {
                // LDA Absolute,Y
                let value = self.read_absolute_y();
                self.lda(value);
                println!("LDA $abs,X");
            }
            0xA1 => {
                // LDA (zp,X)
                let value = self.read_indexed_indirect();
                self.lda(value);
                println!("LDA (zp,X)");
            }
            0xB1 => {
                // LDA (zp),Y
                let value = self.read_indirect_indexed();
                self.lda(value);
                println!("LDA (zp),Y");
            }
            ////////////////// Stop of LDA
            ////////////////// Start of LDX
            0xA2 => {
                // LDX Immediate
                let value = self.read_immediate_byte();
                self.ldx(value);
                println!("LDX #${:02X}", value);
            }
            0xA6 => {
                // LDX zp
                let value = self.read_zero_page();
                self.ldx(value);
                println!("LDX #imm");
            }
            0xB6 => {
                // LDX zp,Y
                let value = self.read_zero_page_y();
                self.ldx(value);
                println!("LDX zp,Y");
            }
            0xAE => {
                // LDX abs
                let value = self.read_absolute();
                self.ldx(value);
                println!("LDX abs");
            }
            0xBE => {
                // LDX abs,Y
                let value = self.read_absolute_y();
                self.ldx(value);
                println!("LDX abs,Y");
            }
            ////////////////// End of LDX
            ////////////////// Start of LDY
            0xA0 => {
                // LDY Immediate
                let value = self.read_immediate_byte();
                self.ldy(value);
                println!("LDY #imm");
            }
            0xA4 => {
                // LDY Zero Page
                let value = self.read_zero_page();
                self.ldy(value);
                println!("LDY zp");
            }
            0xB4 => {
                // LDY zp,X
                let value = self.read_zero_page_x();
                self.ldy(value);
                println!("LDY zp,X");
            }
            0xAC => {
                // LDY abs
                let value = self.read_absolute();
                self.ldy(value);
                println!("LDY abs");
            }
            0xBC => {
                // LDY abs
                let value = self.read_absolute_x();
                self.ldy(value);
                println!("LDY abs,X");
            }
            ////////////////// End of LDY
            ////////////////// Start of LSR
            0x4A => {
                // LSR A
                self.a = self.lsr(self.a);
                println!("LSR A => {:02X}", self.a);
            }
            0x46 => {
                // LSR zp
                let addr = self.get_zero_page_address() as u16;
                let value = self.read_zero_page();
                let result = self.lsr(value);
                self.memory.write_byte(addr, result);
                println!("LSR zp");
            }
            0x56 => {
                // LSR zp,X
                let addr = self.get_zero_page_address_x() as u16;
                let value = self.read_zero_page_x();
                let result = self.lsr(value);
                self.memory.write_byte(addr, result);
                println!("LSR zp");
            }
            0x4E => {
                // LSR abs
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                let result = self.lsr(value);
                self.memory.write_byte(addr, result);
                println!("LSR abs");
            }
            0x5E => {
                // LSR abs,X
                let addr = self.get_absolute_address_x();
                let value = self.read_absolute_x();
                let result = self.lsr(value);
                self.memory.write_byte(addr, result);
                println!("LSR abs,X");
            }
            ////////////////// End of LSR
            ////////////////// Start of NOP
            0xEA => {
                println!("NOP")
            }
            ////////////////// Stop of NOP
            ////////////////// Start of ORA
            0x09 => {
                // ORA #imm
                let value = self.read_immediate_byte();
                self.ora(value);
                println!("ORA #imm");
            }
            0x05 => {
                // ORA zp
                let value = self.read_zero_page();
                self.ora(value);
                println!("ORA zp");
            }
            0x15 => {
                // ORA zp,X
                let value = self.read_zero_page_x();
                self.ora(value);
                println!("ORA zp,X");
            }
            0x0D => {
                // ORA abs
                let value = self.read_absolute();
                self.ora(value);
                println!("ORA abs");
            }
            0x1D => {
                // ORA abs,X
                let value = self.read_absolute_x();
                self.ora(value);
                println!("ORA abs,X");
            }
            0x19 => {
                // ORA abs,Y
                let value = self.read_absolute_y();
                self.ora(value);
                println!("ORA abs,Y");
            }
            0x01 => {
                // ORA (zp,X)
                let value = self.read_indexed_indirect();
                self.ora(value);
                println!("ORA (zp,X)");
            }
            0x11 => {
                // ORA (zp),Y
                let value = self.read_indirect_indexed();
                self.ora(value);
                println!("ORA (zp),Y");
            }
            ////////////////// End of ORA
            ////////////////// Start of PHA
            0x48 => {
                // PHA
                self.push(self.a);
                println!("PHA (push A");
            }
            ////////////////// End of PHA
            ////////////////// Start of PHP
            0x08 => {
                // PHP
                self.push(self.p.value | 0b0011_0000); // emulate B flag set when pushed
                println!("PHP (push P)");
            }
            ////////////////// Stop of PHP
            ////////////////// Start of PLA
            0x68 => {
                // PLA
                self.a = self.pop();
                self.p.set_zero(self.a == 0);
                self.p.set_negative(self.a & 0x80 != 0);
                println!("PLA (pull A)");
            }
            ////////////////// End of PLA
            ////////////////// Start of PLP
            0x28 => {
                // PLP
                self.p.value = self.pop() & 0b1100_1111; // B and unused bits masked off
                println!("PLP (pull P)");
            }
            ////////////////// End of PLP
            ////////////////// Start of ROL
            0x2A => {
                // ROL A
                self.a = self.rol(self.a);
                println!("ROL A");
            }
            0x26 => {
                // ROL zp
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                let result = self.rol(value);
                self.memory.write_byte(addr as u16, result);
                println!("ROL zp");
            }
            0x36 => {
                // ROL zp,X
                let addr = self.get_zero_page_address_x();
                let value = self.read_zero_page_x();
                let result = self.rol(value);
                self.memory.write_byte(addr as u16, result);
                println!("ROL zp,X");
            }
            0x2E => {
                // ROL abs
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                let result = self.rol(value);
                self.memory.write_byte(addr as u16, result);
                println!("ROL abs");
            }
            0x3E => {
                // ROL abs,X
                let addr = self.get_absolute_address_x();
                let value = self.read_absolute_x();
                let result = self.rol(value);
                self.memory.write_byte(addr as u16, result);
                println!("ROL abs,X");
            }
            ////////////////// End of ROL
            ////////////////// Start of ROR
            0x6A => {
                // ROR A
                self.a = self.ror(self.a);
                println!("ROR A");
            }
            0x66 => {
                // ROR zp
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                let result = self.ror(value);
                self.memory.write_byte(addr as u16, result);
                println!("ROR zp");
            }
            0x76 => {
                // ROR zp,X
                let addr = self.get_zero_page_address_x();
                let value = self.read_zero_page_x();
                let result = self.ror(value);
                self.memory.write_byte(addr as u16, result);
                println!("ROR zp");
            }
            0x6E => {
                // ROR abs
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                let result = self.ror(value);
                self.memory.write_byte(addr, result);
                println!("ROR abs");
            }
            0x7E => {
                // ROR abs,X
                let addr = self.get_absolute_address_x();
                let value = self.read_absolute_x();
                let result = self.ror(value);
                self.memory.write_byte(addr, result);
                println!("ROR abs,X");
            }
            ////////////////// End of ROR

            0x8D => {
                // STA $nnnn
                let addr = self.memory.read_word(self.pc);
                self.pc += 2;
                self.memory.write_byte(addr, self.a);
                println!("STA ${:04X}", addr);
            }

            0x60 => {
                // RTS
                let return_addr = self.pop_word().wrapping_add(1);
                self.pc = return_addr;
                println!("RTS to ${:04X}", return_addr);
            }

            0x55 => {
                let val = self.read_zero_page_x();
                self.a ^= val;
                self.p.set_zero(self.a == 0);
                self.p.set_negative(self.a & 0x80 != 0);
                println!("EOR $zp,X => A = {:02X}", self.a);
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
                self.p.set_zero(self.x == 0);
                self.p.set_negative(self.x & 0x80 != 0);
                println!("LDX #${:02X}", self.x);
            }
            0x86 => {
                // STX Zero Page
                let addr = self.memory.read_byte(self.pc) as u16;
                self.pc += 1;
                self.memory.write_byte(addr, self.x);
                println!("STX ${:02X}", addr);
            }
            0x84 => {
                // STY Zero Page
                let addr = self.memory.read_byte(self.pc) as u16;
                self.pc += 1;
                self.memory.write_byte(addr, self.y);
                println!("STY ${:02X}", addr);
            }
            0x38 => {
                self.p.set_carry(true);
                println!("SEC");
            }
            0x78 => {
                self.p.set_interrupt_disable(true);
                println!("SEI");
            }
            0xF8 => {
                self.p.set_decimal_mode(true);
                println!("SED");
            }
            _ => {
                println!("Unknown opcode ${:02X}", opcode);
            }
        }
    }
}
