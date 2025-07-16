use log::debug;

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
///     self.load_program(&program, 0x0600);
///
///     loop {
///         let opcode = self.memory.read_byte(self.pc);
///         self.step();
///         if opcode == 0x00 {
///             break;
///         }
///     }
///
///     dbg!("A = {:02X}, X = {:02X}", self.a, self.x);
///     dbg!("Flags: Z={}, N={}", self.p.is_zero(), self.p.is_negative());
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
use crate::status::mos6502;

pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub p: mos6502::Status,
    pub memory: Memory,
    debug: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFF,
            pc: 0,
            p: mos6502::Status::default(),
            memory: Memory::new(),
            debug: true,
        }
    }

    ///
    /// Loads program to the memory and set PC to start address of the programm
    ///
    pub fn load_program(&mut self, program: &[u8], start_addr: u16) {
        self.memory.load_program(program, start_addr);
        self.pc = start_addr;
    }
    pub fn print_registers(&self) -> String {
        format!(
"-------------------------------------------------------------------------
|  A  |  X  |  Y  |  SP   |  PC   |  P  | N | V | U | B | D | I | Z | C |
|-----|-----|-----|-------|-------|-----|---|---|---|---|---|---|---|---|
| {:02X}H | {:02X}H | {:02X}H | {:04X}H | {:04X}H | {:02X}H | {} | {} | {} | {} | {} | {} | {} | {} |
-------------------------------------------------------------------------\n",
            self.a,
            self.x,
            self.y,
            self.sp,
            self.pc,
            self.p.value,
            self.p.is_negative() as u8,
            self.p.is_overflow() as u8,
            self.p.is_unused() as u8,
            self.p.is_break() as u8,
            self.p.is_decimal_mode() as u8,
            self.p.is_interrupt_disable() as u8,
            self.p.is_zero() as u8,
            self.p.is_carry() as u8
        )
    }
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
    fn brk(&mut self) {
        self.pc += 1; // BRK is a 2-byte instruction (but the second byte is ignored)
        // Push PC to stack (high byte first)
        self.push((self.pc >> 8) as u8);
        self.push((self.pc & 0xFF) as u8);
        // Push status with Break flag set
        let mut status = self.p.value;
        status |= mos6502::BREAK;
        status |= mos6502::UNUSED; // Bit 5 is always set in stack copy
        self.push(status);
        // Set Interrupt Disable flag
        self.p.set_interrupt_disable(true);
        // Load IRQ vector
        let pc = self.memory.read_word(0xFFFE) as u16;
        self.pc = pc;
    }
    fn push(&mut self, value: u8) {
        let addr = 0x0100u16 + self.sp as u16;
        self.memory.write_byte(addr, value);
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
        self.memory.read_byte(self.pc)
    }
    fn get_zero_page_address_x(&self) -> u8 {
        self.memory.read_byte(self.pc).wrapping_add(self.x)
    }
    fn get_absolute_address(&self) -> u16 {
        self.memory.read_word(self.pc)
    }
    fn get_indirect_address_x(&self) -> u16 {
        self.memory
            .read_word_zero_page(self.get_zero_page_address_x())
    }
    fn get_indirect_address_y(&self) -> u16 {
        self.memory
            .read_word_zero_page(self.get_zero_page_address())
            .wrapping_add(self.y as u16)
    }
    fn get_absolute_address_x(&self) -> u16 {
        self.memory.read_word(self.pc).wrapping_add(self.x as u16)
    }
    fn get_absolute_address_y(&self) -> u16 {
        self.memory.read_word(self.pc).wrapping_add(self.y as u16)
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
        // Perform the subtraction using two's complement: A + (~M + 1) - C)) = A - M - C
        let value_inv = value ^ 0xFF;
        let sum = a as u16 + value_inv.wrapping_add(1) as u16 - carry as u16;
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
    fn set_n_z(&mut self, value: u8) {
        let mut flag = if value == 0 { true } else { false };
        self.p.set_zero(flag);
        flag = if value & 0x80 != 0 { true } else { false };
        self.p.set_negative(flag);
    }
    ///
    /// This function formats HEX string from address and opcode and
    /// is used in debug mode to print address and opcode of the instruction that
    /// is currently being processed
    /// For example if you are processing the following code at address 0x200
    /// LDA #$01
    /// Then it looks like this in memory
    /// 0x200 0xA9, 0x01,
    /// and PC points to the address of 0x0202 (just after the operand).
    /// To get the opcode you have to go back 2 bytes so neg_offset is 2.
    /// And the resulting string is '0200 3E'
    ///  
    fn code_to_str(&mut self, mut neg_offset: u8) -> String {
        let mut addr = self.pc.wrapping_sub(neg_offset as u16);
        let mut result = String::new();
        result.push_str(format!("{:04X}  ", addr).as_str());
        while neg_offset != 0 {
            result.push_str(format!("{:02X} ", self.memory.read_byte(addr)).as_str());
            neg_offset -= 1;
            addr += 1;
        }
        format!("{:<18}", result)
    }
    pub fn step(&mut self) {
        macro_rules! dbg { ($($x:tt)*) => { if self.debug { println!($($x)*); } } }

        let opcode = self.memory.read_byte(self.pc);
        self.pc += 1;

        match opcode {
            ////////////////// Start of ADC
            // ADC #imm
            0x69 => {
                let value = self.read_immediate_byte();
                self.adc(value);
                dbg!("{}ADC #${:02X}", self.code_to_str(2), value);
            }
            // ADC zp
            0x65 => {
                let value = self.read_zero_page();
                self.adc(value);
                dbg!(
                    "{}ADC ${:02X}",
                    self.code_to_str(2),
                    self.memory.read_byte(self.pc.wrapping_sub(1))
                );
            }
            // ADC oper ;zero page,X
            0x75 => {
                let value = self.read_zero_page_x();
                self.adc(value);
                dbg!(
                    "{}ADC ${:02X},X",
                    self.code_to_str(2),
                    self.memory.read_byte(self.pc.wrapping_sub(1))
                );
            }
            // ADC oper ;absolute
            0x6D => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.adc(value);
                dbg!("{}ADC ${:04X}", self.code_to_str(3), addr);
            }
            // ADC oper ;absolute,X
            0x7D => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_x();
                self.adc(value);
                dbg!("{}ADC ${:04X},X", self.code_to_str(3), addr);
            }
            // ADC abs,Y ;absolute,Y
            0x79 => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_y();
                self.adc(value);
                dbg!("{}ADC ${:02X},Y", self.code_to_str(3), addr);
            }
            // ADC (oper,X) ;(indexed indirect)
            0x61 => {
                let addr = self.get_zero_page_address();
                let value = self.read_indexed_indirect();
                self.adc(value);
                dbg!("{}ADC (${:02X},X)", self.code_to_str(2), addr);
            }
            // ADC (oper),Y ;(indexed indirect),Y
            0x71 => {
                let addr = self.get_zero_page_address();
                let value = self.read_indirect_indexed();
                self.adc(value);
                dbg!("{}ADC (${:02X}),Y", self.code_to_str(2), addr);
            }
            ////////////////// End of ADC
            ////////////////// Start of AND
            0x29 => {
                let value = self.read_immediate_byte();
                self.and(value);
                dbg!("{}AND #${:02X}", self.code_to_str(2), value);
            }
            0x25 => {
                // AND zp
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.and(value);
                dbg!("{}AND ${:02X}", self.code_to_str(2), addr);
            }
            0x35 => {
                // AND zp,X
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page_x();
                self.and(value);
                dbg!("{}AND ${:02X},X", self.code_to_str(2), addr);
            }
            0x2D => {
                // AND abs
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.and(value);
                dbg!("{}AND ${:04X}", self.code_to_str(3), addr);
            }
            0x3D => {
                // AND abs,X
                let addr = self.get_absolute_address();
                let value = self.read_absolute_x();
                self.and(value);
                dbg!("{}AND ${:04X},X", self.code_to_str(3), addr);
            }
            0x39 => {
                // AND abs,Y
                let addr = self.get_absolute_address();
                let value = self.read_absolute_y();
                self.and(value);
                dbg!("{}AND ${:04X},Y", self.code_to_str(3), addr);
            }
            0x21 => {
                // AND (indirect,X)
                let addr = self.get_zero_page_address();
                let value = self.read_indexed_indirect();
                self.and(value);
                dbg!("{}AND (${:02X},X)", self.code_to_str(2), addr);
            }
            0x31 => {
                // AND (indirect),Y
                let addr = self.get_zero_page_address();
                let value = self.read_indirect_indexed();
                self.and(value);
                dbg!("{}AND (${:02X}),Y", self.code_to_str(2), addr);
            }
            ////////////////// End of AND
            ////////////////// Start of ASL
            0x0A => {
                // ASL A
                self.a = self.asl(self.a);
                dbg!("{}ASL A", self.code_to_str(1));
            }
            0x06 => {
                // ASL Zero Page
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                let result = self.asl(value);
                self.memory.write_byte_zero_page(addr, result);
                dbg!("{}ASL ${:02X}", self.code_to_str(2), addr);
            }
            0x16 => {
                // ASL Zero Page,X
                let addr_zp = self.get_zero_page_address();
                let addr = self.get_zero_page_address_x();
                let value = self.read_zero_page_x();
                let result = self.asl(value);
                self.memory.write_byte(addr as u16, result);
                dbg!("{}ASL ${:02X},X", self.code_to_str(2), addr_zp);
            }
            0x0E => {
                // ASL Absolute
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                let result = self.asl(value);
                self.memory.write_byte(addr, result);
                dbg!("{}ASL ${:04X}", self.code_to_str(3), addr);
            }
            0x1E => {
                // ASL Absolute,X
                let addr_zp = self.get_absolute_address();
                let addr = addr_zp.wrapping_add(self.x as u16);
                let value = self.read_absolute_x();
                let result = self.asl(value);
                self.memory.write_byte(addr, result);
                dbg!("{}ASL ${:04X},X", self.code_to_str(3), addr_zp);
            }
            ////////////////// End of ASL
            ////////////////// Start of BCC
            0x90 => {
                // BCC
                let offset = self.read_immediate_byte() as i8;
                let addr = self.pc.wrapping_add(offset as u16);
                dbg!("{}BCC ${:04X}", self.code_to_str(2), addr);
                if !self.p.is_carry() {
                    self.pc = addr;
                    dbg!("----");
                }
            }
            ////////////////// End of BCC
            ////////////////// Start of BCS
            0xB0 => {
                // BCS
                let offset = self.read_immediate_byte() as i8;
                let addr = self.pc.wrapping_add(offset as u16);
                dbg!("{}BCS ${:04X}", self.code_to_str(2), addr);
                if self.p.is_carry() {
                    self.pc = addr;
                    dbg!("----");
                }
            }
            ////////////////// End of BCS
            ////////////////// Start of BEQ
            0xF0 => {
                // BEQ (Branch if Equal / Zero flag set)
                let offset = self.read_immediate_byte() as i8;
                let addr = self.pc.wrapping_add(offset as u16);
                dbg!("{}BEQ ${:04X}", self.code_to_str(2), addr);
                if self.p.is_zero() {
                    self.pc = addr;
                    dbg!("----");
                }
            }
            ////////////////// End of BEQ
            ////////////////// Start of BIT
            0x24 => {
                // BIT Zero Page
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.bit(value);
                dbg!("{}BIT ${:02X}", self.code_to_str(2), addr);
            }
            0x2C => {
                // BIT Absolute
                let addr = self.read_immediate_word();
                let value = self.memory.read_byte(addr);
                self.bit(value);
                dbg!("{}BIT ${:04X}", self.code_to_str(3), addr);
            }
            ////////////////// End of BIT
            ////////////////// Start of BMI
            0x30 => {
                // BMI
                let offset = self.read_immediate_byte() as i8;
                let addr = self.pc.wrapping_add(offset as u16);
                dbg!("{}BMI ${:04X}", self.code_to_str(2), addr);
                if self.p.is_negative() {
                    self.pc = addr;
                    dbg!("----");
                }
            }
            ////////////////// End of BMI
            ////////////////// Start of BNE
            0xD0 => {
                // BNE (Branch if Not Equal / Zero flag clear)
                let offset = self.read_immediate_byte() as i8;
                let addr = self.pc.wrapping_add(offset as u16);
                dbg!("{}BNE ${:04X}", self.code_to_str(3), addr);
                if !self.p.is_zero() {
                    self.pc = addr;
                    dbg!("----");
                }
            }
            ////////////////// End of BNE
            ////////////////// Start of BPL
            0x10 => {
                // BPL
                let offset = self.read_immediate_byte() as i8;
                let addr = self.pc.wrapping_add(offset as u16);
                dbg!("{}BPL ${:04X}", self.code_to_str(3), addr);
                if !self.p.is_negative() {
                    self.pc = addr;
                    dbg!("----");
                }
            }
            ////////////////// End of BPL
            ////////////////// Start of BRK
            0x00 => {
                dbg!("{}BRK", self.code_to_str(1));
                dbg!("----");
                self.brk();
            }
            ////////////////// End of BRK
            ////////////////// Start of BVC
            0x50 => {
                // BVC
                let offset = self.read_immediate_byte() as i8;
                let addr = self.pc.wrapping_add(offset as u16);
                dbg!("{}BVC ${:04X}", self.code_to_str(3), addr);
                if self.p.value & 0x40 == 0 {
                    self.pc = addr;
                    dbg!("----");
                }
            }
            ////////////////// End of BVC
            ////////////////// Start of BVS
            0x70 => {
                // BVS
                let offset = self.read_immediate_byte() as i8;
                let addr = self.pc.wrapping_add(offset as u16);
                dbg!("{}BVS ${:04X}", self.code_to_str(3), addr);
                if self.p.value & 0x40 != 0 {
                    self.pc = addr;
                    dbg!("----");
                }
            }
            ////////////////// End of BVS
            ////////////////// Start of CLC
            0x18 => {
                self.p.set_carry(false);
                dbg!("{}CLC", self.code_to_str(1));
            }
            ////////////////// End of CLC
            ////////////////// Start of CLD
            0xD8 => {
                self.p.set_decimal_mode(false);
                dbg!("{}CLD", self.code_to_str(1));
            }
            ////////////////// End of CLD
            ////////////////// Start of CLI
            0x58 => {
                self.p.set_interrupt_disable(false);
                dbg!("{}CLI", self.code_to_str(1));
            }
            ////////////////// End of CLI
            ////////////////// Start of CLV
            0xB8 => {
                self.p.set_overflow(false);
                dbg!("{}CLV", self.code_to_str(1));
            }
            ////////////////// End of CLV
            ////////////////// Start of CMP
            0xC9 => {
                // CMP #imm
                let value = self.read_immediate_byte();
                self.cmp(value);
                dbg!("{}CMP #${:02X}", self.code_to_str(2), value);
            }
            0xC5 => {
                // CMP zp
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.cmp(value);
                dbg!("{}CMP ${:02X}", self.code_to_str(2), addr);
            }
            0xD5 => {
                // CMP zp,X
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page_x();
                self.cmp(value);
                dbg!("{}CMP ${:02X},X", self.code_to_str(2), addr);
            }
            0xCD => {
                // CMP abs
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.cmp(value);
                dbg!("{}CMP ${:04X}", self.code_to_str(3), addr);
            }
            0xDD => {
                // CMP abs,X
                let addr = self.get_absolute_address();
                let value = self.read_absolute_x();
                self.cmp(value);
                dbg!("{}CMP ${:04X},X", self.code_to_str(3), addr);
            }
            0xD9 => {
                // CMP abs,Y
                let addr = self.get_absolute_address();
                let value = self.read_absolute_y();
                self.cmp(value);
                dbg!("{}CMP ${:04X},Y", self.code_to_str(3), addr);
            }
            0xC1 => {
                // CMP (zp,X)
                let addr = self.get_zero_page_address();
                let value = self.read_indexed_indirect();
                self.cmp(value);
                dbg!("{}CMP (${:02X},X)", self.code_to_str(2), addr);
            }
            0xD1 => {
                // CMP (zp),Y
                let addr = self.get_zero_page_address();
                let value = self.read_indirect_indexed();
                self.cmp(value);
                dbg!("{}CMP (${:02X}),Y", self.code_to_str(2), addr);
            }
            ////////////////// End of CMP
            ////////////////// Start of CPX
            0xE0 => {
                // CPX #imm
                let value = self.read_immediate_byte();
                self.cpx(value);
                dbg!("{}CPX #${:02X}", self.code_to_str(2), value);
            }
            0xE4 => {
                // CPX zp
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.cpx(value);
                dbg!("{}CPX ${:02X}", self.code_to_str(2), addr);
            }
            0xEC => {
                // CPX abs
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.cpx(value);
                dbg!("{}CPX ${:04X}", self.code_to_str(3), addr);
            }
            ////////////////// End of CPX
            ////////////////// Start of CPY
            0xC0 => {
                // CPY #imm
                let value = self.read_immediate_byte();
                self.cpy(value);
                dbg!("{}CPY #${:02X}", self.code_to_str(2), value);
            }
            0xC4 => {
                // CPY zp
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.cpy(value);
                dbg!("{}CPY ${:02X}", self.code_to_str(2), addr);
            }
            0xCC => {
                // CPY abs
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.cpy(value);
                dbg!("{}CPY ${:04X}", self.code_to_str(3), addr);
            }
            ////////////////// End of CPY
            ////////////////// Start of DEC
            0xC6 => {
                // DEC Zero Page
                let addr = self.get_zero_page_address();
                let val = self.read_zero_page();
                let result = self.dec(val);
                self.memory.write_byte(addr as u16, result);
                dbg!("{}DEC ${:02X}", self.code_to_str(2), addr);
            }
            0xD6 => {
                // DEC Zero Page,X
                let addr_zp = self.get_zero_page_address();
                let addr = self.get_zero_page_address_x();
                let val = self.read_zero_page_x();
                let result = self.dec(val);
                self.memory.write_byte(addr as u16, result);
                dbg!("{}DEC ${:02X},X", self.code_to_str(2), addr_zp);
            }
            0xCE => {
                // DEC Absolute
                let addr = self.get_absolute_address();
                let val = self.read_absolute();
                let result = self.dec(val);
                self.memory.write_byte(addr, result);
                dbg!("{}DEC ${:04X}", self.code_to_str(3), addr);
            }
            0xDE => {
                // DEC Absolute,X
                let addr_abs = self.get_absolute_address();
                let addr = addr_abs.wrapping_add(self.x as u16);
                let val = self.read_absolute_x();
                let result = self.dec(val);
                self.memory.write_byte(addr, result);
                dbg!("{}DEC ${:04X},X", self.code_to_str(3), addr_abs);
            }
            ////////////////// End of DEC
            ////////////////// Start of DEX
            0xCA => {
                // DEX
                self.x = self.dec(self.x);
                dbg!("{}DEX", self.code_to_str(1));
            }
            ////////////////// End of DEX
            ////////////////// Start of DEY
            0x88 => {
                // DEY
                self.y = self.dec(self.y);
                dbg!("{}DEY", self.code_to_str(1));
            }
            ////////////////// Stop of DEY
            ////////////////// Start of EOR
            0x49 => {
                // EOR #imm
                let value = self.read_immediate_byte();
                self.eor(value);
                dbg!("{}EOR #${:02X}", self.code_to_str(2), value);
            }
            0x45 => {
                // EOR zp
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.eor(value);
                dbg!("{}EOR ${:02X}", self.code_to_str(2), addr);
            }
            0x55 => {
                // EOR zp,X
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page_x();
                self.eor(value);
                dbg!("{}EOR ${:02X},X", self.code_to_str(2), addr);
            }
            0x4D => {
                // EOR abs
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.eor(value);
                dbg!("{}EOR ${:04X}", self.code_to_str(3), addr);
            }
            0x5D => {
                // EOR abs,X
                let addr = self.get_absolute_address();
                let value = self.read_absolute_x();
                self.eor(value);
                dbg!("{}EOR ${:04X},X", self.code_to_str(3), addr);
            }
            0x59 => {
                // EOR abs,Y
                let addr = self.get_absolute_address();
                let value = self.read_absolute_y();
                self.eor(value);
                dbg!("{}EOR ${:04X},Y", self.code_to_str(3), addr);
            }
            0x41 => {
                // EOR indirect,X
                let addr = self.get_zero_page_address();
                let value = self.read_indexed_indirect();
                self.eor(value);
                dbg!("{}EOR (${:02X},X)", self.code_to_str(2), addr);
            }
            0x51 => {
                // EOR indirect,Y
                let addr = self.get_zero_page_address();
                let value = self.read_indirect_indexed();
                self.eor(value);
                dbg!("{}EOR (${:02X}),Y", self.code_to_str(2), addr);
            }
            ////////////////// End of EOR
            ////////////////// Start of INC
            0xE6 => {
                // INC Zero Page
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                let result = self.inc(value);
                self.memory.write_byte_zero_page(addr, result);
                dbg!("{}INC ${:02X}", self.code_to_str(2), addr);
            }
            0xF6 => {
                // INC Zero Page,X
                let addr = self.get_zero_page_address_x();
                let value = self.read_zero_page_x();
                let result = self.inc(value);
                self.memory.write_byte_zero_page(addr, result);
                dbg!(
                    "{}INC ${:02X},X",
                    self.code_to_str(2),
                    addr.wrapping_sub(self.x)
                );
            }
            0xEE => {
                // INC Absolute
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                let result = self.inc(value);
                self.memory.write_byte(addr, result);
                dbg!("{}INC ${:04X}", self.code_to_str(3), addr);
            }
            0xFE => {
                // INC Absolute,X
                let addr = self.get_absolute_address_x();
                let value = self.read_absolute_x();
                let result = self.inc(value);
                self.memory.write_byte(addr, result);
                dbg!(
                    "{}INC ${:04X},X",
                    self.code_to_str(3),
                    addr.wrapping_sub(self.x as u16)
                );
            }
            ////////////////// End of INC
            ////////////////// Start of INX
            0xE8 => {
                // INX
                self.x = self.inc(self.x);
                dbg!("{}INX", self.code_to_str(1));
            }
            ////////////////// End of INX
            ////////////////// Start of INY
            0xC8 => {
                // INY
                self.y = self.inc(self.y);
                dbg!("{}INY", self.code_to_str(1));
            }
            ////////////////// End of INY
            ////////////////// Start of JMP
            0x4C => {
                // JMP absolute
                let addr = self.memory.read_word(self.pc);
                self.pc += 2;
                dbg!("{}JMP ${:04X}\n----", self.code_to_str(3), addr);
                self.pc = addr;
            }
            0x6C => {
                // JMP indirect
                let addr = self.get_absolute_address();
                let addr_lo = self.memory.read_byte(self.pc);
                let addr_hi = self.memory.read_byte(self.pc.wrapping_add(1));
                let jmp_addr_lo = self
                    .memory
                    .read_byte((addr_hi as u16) << 0x8 | addr_lo as u16);
                let jmp_addr_hi = self
                    .memory
                    .read_byte((addr_hi as u16) << 0x8 | addr_lo.wrapping_add(1) as u16);
                self.pc += 2;
                dbg!("{}JMP (${:04X})\n----", self.code_to_str(3), addr);
                self.pc = (jmp_addr_hi as u16) << 8 | jmp_addr_lo as u16;
            }
            ////////////////// End of JMP
            ////////////////// Start of JSR
            0x20 => {
                // JSR
                let addr = self.read_immediate_word();
                //                self.pc += 2;
                self.push_word(self.pc.wrapping_sub(1)); // push return address - 1
                dbg!("{}JSR ${:04X}\n----", self.code_to_str(3), addr);
                self.pc = addr;
            }
            ////////////////// End of JSR
            ////////////////// Start of LDA
            0xA9 => {
                // LDA Immediate
                let value = self.read_immediate_byte();
                self.lda(value);
                dbg!("{}LDA #${:02X}", self.code_to_str(2), value);
            }
            0xA5 => {
                // LDA Zero Page
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.lda(value);
                dbg!("{}LDA ${:02X}", self.code_to_str(2), addr);
            }
            0xB5 => {
                // LDA Zero Page,X
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page_x();
                self.lda(value);
                dbg!("{}LDA ${:02X},X", self.code_to_str(2), addr);
            }
            0xAD => {
                // LDA Absolute
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.lda(value);
                dbg!("{}LDA ${:04X}", self.code_to_str(3), addr);
            }
            0xBD => {
                // LDA Absolute,X
                let addr = self.get_absolute_address();
                let value = self.read_absolute_x();
                self.lda(value);
                dbg!("{}LDA ${:04X},X", self.code_to_str(3), addr);
            }
            0xB9 => {
                // LDA Absolute,Y
                let addr = self.get_absolute_address();
                let value = self.read_absolute_y();
                self.lda(value);
                dbg!("{}LDA ${:02X},Y", self.code_to_str(2), addr);
            }
            0xA1 => {
                // LDA (zp,X)
                let addr = self.get_zero_page_address();
                let value = self.read_indexed_indirect();
                self.lda(value);
                dbg!("{}LDA (${:02X},X)", self.code_to_str(2), addr);
            }
            0xB1 => {
                // LDA (zp),Y
                let addr = self.get_zero_page_address();
                let value = self.read_indirect_indexed();
                self.lda(value);
                dbg!("{}LDA (${:02X}),Y", self.code_to_str(2), addr);
            }
            ////////////////// Stop of LDA
            ////////////////// Start of LDX
            0xA2 => {
                // LDX Immediate
                let value = self.read_immediate_byte();
                self.ldx(value);
                dbg!("{}LDX #${:02X}", self.code_to_str(2), value);
            }
            0xA6 => {
                // LDX zp
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.ldx(value);
                dbg!("{}LDX ${:02X}", self.code_to_str(2), addr);
            }
            0xB6 => {
                // LDX zp,Y
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page_y();
                self.ldx(value);
                dbg!("{}LDX ${:02X},Y", self.code_to_str(2), addr);
            }
            0xAE => {
                // LDX abs
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.ldx(value);
                dbg!("{}LDX ${:04X}", self.code_to_str(3), addr);
            }
            0xBE => {
                // LDX abs,Y
                let addr = self.get_absolute_address();
                let value = self.read_absolute_y();
                self.ldx(value);
                dbg!("{}LDX ${:04X},Y", self.code_to_str(3), addr);
            }
            ////////////////// End of LDX
            ////////////////// Start of LDY
            0xA0 => {
                // LDY Immediate
                let value = self.read_immediate_byte();
                self.ldy(value);
                dbg!("{}LDY #${:02X}", self.code_to_str(2), value);
            }
            0xA4 => {
                // LDY Zero Page
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.ldy(value);
                dbg!("{}LDY ${:02X}", self.code_to_str(2), addr);
            }
            0xB4 => {
                // LDY zp,X
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page_x();
                self.ldy(value);
                dbg!("{}LDY ${:02X},X", self.code_to_str(2), addr);
            }
            0xAC => {
                // LDY abs
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.ldy(value);
                dbg!("{}LDY ${:04X}", self.code_to_str(3), addr);
            }
            0xBC => {
                // LDY abs,X
                let addr = self.get_absolute_address();
                let value = self.read_absolute_x();
                self.ldy(value);
                dbg!("{}LDY ${:02X},X", self.code_to_str(3), addr);
            }
            ////////////////// End of LDY
            ////////////////// Start of LSR
            0x4A => {
                // LSR A
                self.a = self.lsr(self.a);
                dbg!("{}LSR A", self.code_to_str(1));
            }
            0x46 => {
                // LSR zp
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                let result = self.lsr(value);
                self.memory.write_byte(addr as u16, result);
                dbg!("{}LSR ${:02X}", self.code_to_str(2), addr);
            }
            0x56 => {
                // LSR zp,X
                let addr = self.get_zero_page_address_x();
                let value = self.read_zero_page_x();
                let result = self.lsr(value);
                self.memory.write_byte(addr as u16, result);
                dbg!(
                    "{}LSR ${:02X},X",
                    self.code_to_str(2),
                    addr.wrapping_sub(self.x)
                );
            }
            0x4E => {
                // LSR abs
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                let result = self.lsr(value);
                self.memory.write_byte(addr, result);
                dbg!("{}LSR ${:04X}", self.code_to_str(3), addr);
            }
            0x5E => {
                // LSR abs,X
                let addr = self.get_absolute_address_x();
                let value = self.read_absolute_x();
                let result = self.lsr(value);
                self.memory.write_byte(addr, result);
                dbg!(
                    "{}LSR ${:04X},X",
                    self.code_to_str(3),
                    addr.wrapping_sub(self.x as u16)
                );
            }
            ////////////////// End of LSR
            ////////////////// Start of NOP
            0xEA => {
                dbg!("{}NOP", self.code_to_str(1))
            }
            ////////////////// Stop of NOP
            ////////////////// Start of ORA
            0x09 => {
                // ORA #imm
                let value = self.read_immediate_byte();
                self.ora(value);
                dbg!("{}ORA #${:02X}", self.code_to_str(2), value);
            }
            0x05 => {
                // ORA zp
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.ora(value);
                dbg!("{}ORA ${:02X}", self.code_to_str(2), addr);
            }
            0x15 => {
                // ORA zp,X
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page_x();
                self.ora(value);
                dbg!("{}ORA ${:02X},X", self.code_to_str(2), addr);
            }
            0x0D => {
                // ORA abs
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.ora(value);
                dbg!("{}ORA ${:04X}", self.code_to_str(3), addr);
            }
            0x1D => {
                // ORA abs,X
                let addr = self.get_absolute_address();
                let value = self.read_absolute_x();
                self.ora(value);
                dbg!("{}ORA ${:04X},X", self.code_to_str(3), addr);
            }
            0x19 => {
                // ORA abs,Y
                let addr = self.get_absolute_address();
                let value = self.read_absolute_y();
                self.ora(value);
                dbg!("{}ORA ${:04X},Y", self.code_to_str(3), addr);
            }
            0x01 => {
                // ORA (zp,X)
                let addr = self.get_zero_page_address();
                let value = self.read_indexed_indirect();
                self.ora(value);
                dbg!("{}ORA (${:02X},X)", self.code_to_str(2), addr);
            }
            0x11 => {
                // ORA (zp),Y
                let addr = self.get_zero_page_address();
                let value = self.read_indirect_indexed();
                self.ora(value);
                dbg!("{}ORA (${:02X}),Y", self.code_to_str(2), addr);
            }
            ////////////////// End of ORA
            ////////////////// Start of PHA
            0x48 => {
                // PHA
                self.push(self.a);
                dbg!("{}PHA", self.code_to_str(1));
            }
            ////////////////// End of PHA
            ////////////////// Start of PHP
            0x08 => {
                // PHP
                self.push(self.p.value | mos6502::BREAK | mos6502::UNUSED); // emulate B and Unused flag set when pushed
                dbg!("{}PHP", self.code_to_str(1));
            }
            ////////////////// Stop of PHP
            ////////////////// Start of PLA
            0x68 => {
                // PLA
                self.a = self.pop();
                self.p.set_zero(self.a == 0);
                self.p.set_negative(self.a & 0x80 != 0);
                dbg!("{}PLA", self.code_to_str(1));
            }
            ////////////////// End of PLA
            ////////////////// Start of PLP
            0x28 => {
                // PLP
                self.p.value = self.pop() & 0b1100_1111; // B and unused bits masked off
                dbg!("{}PLP (pull P)", self.code_to_str(1));
            }
            ////////////////// End of PLP
            ////////////////// Start of ROL
            0x2A => {
                // ROL A
                self.a = self.rol(self.a);
                dbg!("{}ROL A", self.code_to_str(1));
            }
            0x26 => {
                // ROL zp
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                let result = self.rol(value);
                self.memory.write_byte(addr as u16, result);
                dbg!("{}ROL ${:02X}", self.code_to_str(2), addr);
            }
            0x36 => {
                // ROL zp,X
                let addr = self.get_zero_page_address_x();
                let value = self.read_zero_page_x();
                let result = self.rol(value);
                self.memory.write_byte(addr as u16, result);
                dbg!(
                    "{}ROL ${:02X},X",
                    self.code_to_str(2),
                    addr.wrapping_sub(self.x)
                );
            }
            0x2E => {
                // ROL abs
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                let result = self.rol(value);
                self.memory.write_byte(addr as u16, result);
                dbg!("{}ROL ${:04X}", self.code_to_str(3), addr);
            }
            0x3E => {
                // ROL abs,X
                let addr = self.get_absolute_address_x();
                let value = self.read_absolute_x();
                let result = self.rol(value);
                self.memory.write_byte(addr as u16, result);
                dbg!(
                    "{}ROL ${:04X},X",
                    self.code_to_str(3),
                    addr.wrapping_sub(self.x as u16)
                );
            }
            ////////////////// End of ROL
            ////////////////// Start of ROR
            0x6A => {
                // ROR A
                self.a = self.ror(self.a);
                dbg!("{}ROR A", self.code_to_str(1));
            }
            0x66 => {
                // ROR zp
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                let result = self.ror(value);
                self.memory.write_byte(addr as u16, result);
                dbg!("{}ROR ${:02X}", self.code_to_str(2), addr);
            }
            0x76 => {
                // ROR zp,X
                let addr = self.get_zero_page_address_x();
                let value = self.read_zero_page_x();
                let result = self.ror(value);
                self.memory.write_byte(addr as u16, result);
                dbg!(
                    "{}ROR ${:02X},X",
                    self.code_to_str(2),
                    addr.wrapping_sub(self.x)
                );
            }
            0x6E => {
                // ROR abs
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                let result = self.ror(value);
                self.memory.write_byte(addr, result);
                dbg!("{}ROR ${:04X}", self.code_to_str(3), addr);
            }
            0x7E => {
                // ROR abs,X
                let addr = self.get_absolute_address_x();
                let value = self.read_absolute_x();
                let result = self.ror(value);
                self.memory.write_byte(addr, result);
                dbg!(
                    "{}ROR ${:04X},X",
                    self.code_to_str(3),
                    addr.wrapping_sub(self.x as u16)
                );
            }
            ////////////////// End of ROR
            ////////////////// Start of RTI
            0x40 => {
                // RTI
                dbg!("{}RTI", self.code_to_str(1));
                dbg!("----");
                self.p.value = self.pop() & !mos6502::BREAK & !mos6502::UNUSED; // B and unused bits masked off
                self.pc = self.pop_word();
            }
            ////////////////// End of RTI
            ////////////////// Start of RTS
            0x60 => {
                // RTS
                self.pc = self.pop_word().wrapping_add(1);
                dbg!("{}RTS\n----", self.code_to_str(1));
            }
            ////////////////// End of RTS
            ////////////////// Start of SBC
            0xE9 => {
                // SBC #imm
                let value = self.read_immediate_byte();
                self.sbc(value);
                dbg!("{}SBC #${:02X}", self.code_to_str(2), value);
            }
            0xE5 => {
                // SBC zp
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.sbc(value);
                dbg!("{}SBC ${:02x}", self.code_to_str(2), addr)
            }
            0xF5 => {
                // SBC zp,X
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page_x();
                self.sbc(value);
                dbg!("{}SBC ${:02x},X", self.code_to_str(2), addr);
            }
            0xED => {
                // SBC absolute
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.sbc(value);
                dbg!("{}SBC ${:04x}", self.code_to_str(3), addr);
            }
            0xFD => {
                // SBC absolute,X
                let addr = self.get_absolute_address();
                let value = self.read_absolute_x();
                self.sbc(value);
                dbg!("{}SBC ${:04x},X", self.code_to_str(3), addr);
            }
            0xF9 => {
                // SBC absolute,Y
                let addr = self.get_absolute_address();
                let value = self.read_absolute_y();
                self.sbc(value);
                dbg!("{}SBC ${:04x},Y", self.code_to_str(3), addr);
            }
            0xE1 => {
                // SBC (indirect,X)
                let addr = self.get_absolute_address();
                let value = self.read_indexed_indirect();
                self.sbc(value);
                dbg!("{}SBC $({:02x},X)", self.code_to_str(2), addr);
            }
            0xF1 => {
                // SBC(indirect),Y
                let addr = self.get_absolute_address();
                let value = self.read_indirect_indexed();
                self.sbc(value);
                dbg!("{}SBC $({:02x}),Y", self.code_to_str(2), addr);
            }
            ////////////////// End of SBC
            ////////////////// Start of SEC
            0x38 => {
                // SEC
                self.p.set_carry(true);
                dbg!("{}SEC", self.code_to_str(1));
            }
            ////////////////// End of SEC
            ////////////////// Start of SED
            0xF8 => {
                self.p.set_decimal_mode(true);
                dbg!("{}SED", self.code_to_str(1));
            }
            ////////////////// End of SED
            ////////////////// Start of SEI
            0x78 => {
                self.p.set_interrupt_disable(true);
                dbg!("{}SEI", self.code_to_str(1));
            }
            ////////////////// End of SEI
            ////////////////// Start of STA
            0x85 => {
                // STA zp
                let addr = self.get_zero_page_address();
                self.pc += 1;
                self.memory.write_byte_zero_page(addr, self.a);
                dbg!("{}STA ${:02X}", self.code_to_str(2), addr);
            }
            0x95 => {
                // STA zp,X
                let addr = self.get_zero_page_address_x();
                self.pc += 1;
                self.memory.write_byte_zero_page(addr, self.a);
                dbg!(
                    "{}STA ${:02X},X",
                    self.code_to_str(2),
                    addr.wrapping_sub(self.x)
                );
            }
            0x8D => {
                // STA $nnnn
                let addr = self.get_absolute_address();
                self.pc += 2;
                self.memory.write_byte(addr, self.a);
                dbg!("{}STA ${:04X}", self.code_to_str(3), addr);
            }
            0x9D => {
                // STA $nnnn,X
                let addr = self.get_absolute_address_x();
                self.pc += 2;
                self.memory.write_byte(addr, self.a);
                dbg!("{}STA ${:04X},X", self.code_to_str(3), addr);
            }
            0x99 => {
                // STA $nnnn,Y
                let addr = self.get_absolute_address_y();
                self.pc += 2;
                self.memory.write_byte(addr, self.a);
                dbg!("{}STA ${:04X},Y", self.code_to_str(3), addr);
            }
            0x81 => {
                // STA (indirect,X)
                let addr_zp = self.get_zero_page_address();
                let addr = self.get_indirect_address_x();
                self.pc += 1;
                self.memory.write_byte(addr, self.a);
                dbg!("{}STA (${:02X},X)", self.code_to_str(2), addr_zp);
            }
            0x91 => {
                let addr_zp = self.get_zero_page_address();
                let addr = self.get_indirect_address_y();
                self.pc += 1;
                self.memory.write_byte(addr, self.a);
                dbg!("{}STA (${:02X}),Y", self.code_to_str(2), addr_zp);
            }
            ////////////////// End of STA
            ////////////////// Start of STX
            0x86 => {
                // STX zp
                let addr = self.read_immediate_byte();
                self.memory.write_byte_zero_page(addr, self.x);
                dbg!("{}STX ${:02X}", self.code_to_str(2), addr);
            }
            0x96 => {
                // STX zp,Y
                let addr = self.read_immediate_byte();
                self.memory
                    .write_byte_zero_page(addr.wrapping_add(self.y), self.x);
                dbg!("{}STX ${:02X},Y", self.code_to_str(2), addr);
            }
            0x8E => {
                // STX abs
                let addr = self.read_immediate_word();
                self.memory.write_byte(addr, self.x);
                dbg!("{}STX ${:02X}", self.code_to_str(2), addr);
            }
            ////////////////// End of STX
            ////////////////// Start of STY
            0x84 => {
                // STY zp
                let addr = self.read_immediate_byte();
                self.memory.write_byte_zero_page(addr, self.y);
                dbg!("{}STY ${:02X}", self.code_to_str(2), addr);
            }
            0x94 => {
                // STY zp,X
                let addr = self.read_immediate_byte();
                self.memory
                    .write_byte_zero_page(addr.wrapping_add(self.x), self.y);
                dbg!("{}STY ${:02X},X", self.code_to_str(2), addr);
            }
            0x8C => {
                // STY abs
                let addr = self.read_immediate_word();
                self.memory.write_byte(addr, self.y);
                dbg!("{}STY ${:04X}", self.code_to_str(3), addr);
            }
            ////////////////// End of STY
            ////////////////// Start of TAX
            0xAA => {
                // TAX
                self.x = self.a;
                self.set_n_z(self.x);
                dbg!("{}TAX", self.code_to_str(1));
            }
            ////////////////// End of TAX
            ////////////////// Start of TAY
            0xA8 => {
                // TAY
                self.y = self.a;
                self.set_n_z(self.y);
                dbg!("{}TAY", self.code_to_str(1));
            }
            ////////////////// End of TAY
            ////////////////// Start of TSX
            0xBA => {
                // TSX
                self.x = self.sp;
                self.set_n_z(self.x);
                dbg!("{}TSX", self.code_to_str(1));
            }
            ////////////////// End of TSX
            ////////////////// Start of TXA
            0x8A => {
                // TXA
                self.a = self.x;
                self.set_n_z(self.a);
                dbg!("{}TXA", self.code_to_str(1));
            }
            ////////////////// End of TXA
            ////////////////// Start of TXS
            0x9A => {
                // TXS
                self.sp = self.x;
                dbg!("{}TXS", self.code_to_str(1));
            }
            ////////////////// End of TXS
            ////////////////// Start of TYA
            0x98 => {
                // TYA
                self.a = self.y;
                self.set_n_z(self.a);
                dbg!("{}TYA", self.code_to_str(1));
            }
            ////////////////// End of TYA
            _ => {
                dbg!("{}!byte ${:02X}", self.code_to_str(1), opcode);
            }
        }
    }
}
