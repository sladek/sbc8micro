//! MOS Technology 6502 CPU
//!
//! Emulates MOS 6502 CPU at register level. No timing is emulated
//!
//! Below is an example of its usage
//! ```
//! use sbc8micro::disassembler;
//! use sbc8micro::disassembler::mos6502_opcode_consts::*;
//! use sbc8micro::memory;
//! use sbc8micro::cpu;
//! use sbc8micro::status;
//!
//!
//! let mut cpu = cpu::mos6502::Cpu::new();
//! let program = vec![
//!     0xA9, 0x00, // LDA #0x00 => sets Z flag
//!     0xA2, 0xFF, // LDX #0xFF => sets N flag
//!     0x00,       // BRK
//! ];
//! cpu.memory.load_program(&program, 0x0600);
//! loop {
//!     let opcode = cpu.memory.read_byte(cpu.pc);
//!     cpu.step();
//!     if opcode == 0x00 {
//!         break;
//!     }
//! }
//! dbg!("A = {:02X}, X = {:02X}", cpu.a, cpu.x);
//! dbg!("Flags: Z={}, N={}", cpu.p.is_zero(), cpu.p.is_negative());
//! ```
//!
//! Result should be:<br/>
//! LDA #$00<br/>
//! LDX #$FF<br/>
//! BRK (break)<br/>
//! ----------------------<br/>
//! A = 00, X = FF<br/>
//! Flags: Z=false, N=true
//!
use crate::cpu::CpuUi;
use crate::disassembler::mos6502_opcode_consts::*;
use crate::memory::Memory;
use crate::status::mos6502;

/// Internal registers and flags for MOS6502 CPU
#[derive(Default)]
pub struct Cpu {
    /// Accumulator
    pub a: u8,
    /// X - Index register
    pub x: u8,
    /// Y - Index register
    pub y: u8,
    /// Stack pointer
    pub sp: u8,
    /// Program counter
    pub pc: u16,
    /// Status register
    pub p: mos6502::Status,
    /// Memory assigned to CPU
    pub memory: Memory,
    /// Debug flag
    ///
    /// If frue opcode is also outputed when the programm is executed.
    /// This can slow the execution so it should be used mainly
    /// during debuging process.
    debug: bool,
}

impl Cpu {
    /// Returns initialised instance of MOS6502 CPU
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
    pub fn get_cpu_ui() -> Option<Box<dyn CpuUi>> {
        Some(Box::new(Self::new()))
    }
    /// Loads program to the memory and set PC to start address of the programm
    pub fn load_program(&mut self, program: &[u8], start_addr: u16) {
        let _ = self.memory.load_program(program, start_addr);
        self.pc = start_addr;
    }
    ///
    ///  Prints content of registers and flags
    ///
    /// It printes a content of registers in table form which can be useful for debugging
    ///
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
    /// Sets debug flag
    ///
    /// If debug flag is set to true, then when stepping through instructions
    /// also mnemonic code of instruction is printed, which is very convenient
    /// during debugging of the programm
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
    /// BRK
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
        let pc = self.memory.read_word(0xFFFE);
        self.pc = pc;
    }
    /// Pushes a byte to stack
    fn push(&mut self, value: u8) {
        let addr = 0x0100u16 + self.sp as u16;
        self.memory.write_byte(addr, value);
        self.sp = self.sp.wrapping_sub(1);
    }
    /// Pops a byte from stack
    fn pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let addr = 0x0100u16 + self.sp as u16;
        self.memory.read_byte(addr)
    }
    /// Pushes a word into stack
    fn push_word(&mut self, val: u16) {
        self.push((val >> 8) as u8);
        self.push((val & 0xFF) as u8);
    }
    /// Pops a word from stack
    fn pop_word(&mut self) -> u16 {
        let low = self.pop() as u16;
        let high = self.pop() as u16;
        (high << 8) | low
    }
    /// Reads immediate byte from memory
    fn read_immediate_byte(&mut self) -> u8 {
        let value = self.memory.read_byte(self.pc);
        self.pc += 1;
        value
    }
    /// Reads immediate word from memory
    fn read_immediate_word(&mut self) -> u16 {
        let value = self.memory.read_word(self.pc);
        self.pc += 2;
        value
    }
    /// Reads byte from zero page
    ///
    /// Reads the content of zero page addressed by immediate byte
    fn read_zero_page(&mut self) -> u8 {
        let addr = self.memory.read_byte(self.pc) as u16;
        self.pc += 1;
        self.memory.read_byte(addr)
    }
    /// Reads a byte addressed by address in immediate word
    fn read_absolute(&mut self) -> u8 {
        let addr = self.memory.read_word(self.pc);
        self.pc += 2;
        self.memory.read_byte(addr)
    }
    /// Gets zero page address
    ///
    /// Gets address in zero page as immediate byte
    fn get_zero_page_address(&self) -> u8 {
        self.memory.read_byte(self.pc)
    }
    ///  Gets zero page X
    ///
    /// Gets address in zero page increased by the offset from X register
    fn get_zero_page_address_x(&self) -> u8 {
        self.memory.read_byte(self.pc).wrapping_add(self.x)
    }
    /// Gets absolute immediate address
    fn get_absolute_address(&self) -> u16 {
        self.memory.read_word(self.pc)
    }
    /// Gets indirect address X - (indirect,X)
    ///
    /// It first gets intermediate address of zero page address increased by content of X register,
    /// then it reads a byte from zero page
    fn get_indirect_address_x(&self) -> u16 {
        self.memory
            .read_word_zero_page(self.get_zero_page_address_x())
    }
    /// Gets indirect address Y - (indirect),Y
    ///
    /// It first reads immediate address for zero page, then reads a new address from that zero page address
    /// which is afterwords increased by the content of Y register
    fn get_indirect_address_y(&self) -> u16 {
        self.memory
            .read_word_zero_page(self.get_zero_page_address())
            .wrapping_add(self.y as u16)
    }
    /// Reads an absolut address X
    ///
    /// Reads immediate address and increases it by the content of X register
    fn get_absolute_address_x(&self) -> u16 {
        self.memory.read_word(self.pc).wrapping_add(self.x as u16)
    }
    /// Reads an absolut address Y
    ///
    /// Reads immediate address and increases it by the content of Y register
    fn get_absolute_address_y(&self) -> u16 {
        self.memory.read_word(self.pc).wrapping_add(self.y as u16)
    }
    /// Reads zero page X
    ///
    /// Reads an immediate zero page address, increases it by the content of X register
    /// and then returns byte from this new zero page address
    fn read_zero_page_x(&mut self) -> u8 {
        let base = self.memory.read_byte(self.pc);
        self.pc += 1;
        let addr = base.wrapping_add(self.x) as u16;
        self.memory.read_byte(addr)
    }
    /// Reads zero page Y
    ///
    /// Reads an immediate zero page address, increases it by the content of Y register
    /// and then returns byte from this new zero page address
    fn read_zero_page_y(&mut self) -> u8 {
        let base = self.memory.read_byte(self.pc);
        self.pc += 1;
        let addr = base.wrapping_add(self.y) as u16;
        self.memory.read_byte(addr)
    }
    /// Reads absolute X
    ///
    /// Reads an immediate address (16 bit), increases it by the content of X register and
    /// returns byte from this new address
    fn read_absolute_x(&mut self) -> u8 {
        let base = self.memory.read_word(self.pc);
        self.pc += 2;
        let addr = base.wrapping_add(self.x as u16);
        self.memory.read_byte(addr)
    }
    /// Reads absolute Y
    ///
    /// Reads an immediate address (16 bit), increases it by the content of Y register and
    /// returns byte from this new address
    fn read_absolute_y(&mut self) -> u8 {
        let base = self.memory.read_word(self.pc);
        self.pc += 2;
        let addr = base.wrapping_add(self.y as u16);
        self.memory.read_byte(addr)
    }
    /// Reads byte indexed indirect - ($addr, X)
    ///
    /// Reads immediate byte increased by the content of X register and uses that value
    /// for reading the new aabsolute (16 bit) address from zero page then returns a byte from this new address
    fn read_indexed_indirect(&mut self) -> u8 {
        let base = self.memory.read_byte(self.pc).wrapping_add(self.x);
        self.pc += 1;
        let addr = self.memory.read_word_zero_page(base);
        self.memory.read_byte(addr)
    }
    /// Reads byte indirect indexed -  ($addr), Y
    ///
    /// Reads immediate byte as zero page address, then reads a word from this zero page address,
    /// increases it by the content of Y register and returns a byte from this new absolute address
    fn read_indirect_indexed(&mut self) -> u8 {
        let zp_addr = self.memory.read_byte(self.pc);
        self.pc += 1;
        let base = self.memory.read_word_zero_page(zp_addr);
        let addr = base.wrapping_add(self.y as u16);
        self.memory.read_byte(addr)
    }
    /// ASL
    fn asl(&mut self, value: u8) -> u8 {
        let result = value << 1;
        self.p.set_carry((value & 0x80) != 0);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0);
        result
    }
    /// LSR
    fn lsr(&mut self, value: u8) -> u8 {
        let result = value >> 1;
        self.p.set_carry((value & 0x01) != 0);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0); // always 0, but set for consistency
        result
    }
    /// ROL
    fn rol(&mut self, value: u8) -> u8 {
        let carry_in = self.p.is_carry() as u8;
        let result = (value << 1) | carry_in;
        self.p.set_carry((value & 0x80) != 0);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0);
        result
    }
    /// ROR
    fn ror(&mut self, value: u8) -> u8 {
        let carry_in = if self.p.is_carry() { 0x80 } else { 0 };
        let result = (value >> 1) | carry_in;
        self.p.set_carry((value & 0x01) != 0);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0);
        result
    }
    /// INC
    fn inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0);
        result
    }
    /// DEC
    fn dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0);
        result
    }
    /// BIT
    fn bit(&mut self, value: u8) {
        let result = self.a & value;
        self.p.set_zero(result == 0);
        self.p.set_negative(value & 0x80 != 0);
        self.p.set_overflow(value & 0x40 != 0);
    }
    /// ADC
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
    /// AND
    fn and(&mut self, value: u8) {
        self.a &= value;
        self.p.set_zero(self.a == 0);
        self.p.set_negative(self.a & 0x80 != 0);
    }
    /// SBC
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
    /// CMP
    fn cmp(&mut self, value: u8) {
        let result = self.a.wrapping_sub(value);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0);
        self.p.set_carry(self.a >= value);
    }
    /// CPX
    fn cpx(&mut self, value: u8) {
        let x = self.x;
        let result = x.wrapping_sub(value);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0);
        self.p.set_carry(self.x >= value);
    }
    /// CPY
    fn cpy(&mut self, value: u8) {
        let y = self.y;
        let result = y.wrapping_sub(value);
        self.p.set_zero(result == 0);
        self.p.set_negative(result & 0x80 != 0);
        self.p.set_carry(self.y >= value);
    }
    /// EOR
    fn eor(&mut self, value: u8) {
        self.a ^= value;
        self.p.set_zero(self.a == 0);
        self.p.set_negative(self.a & 0x80 != 0);
    }
    /// LDA
    fn lda(&mut self, value: u8) {
        self.a = value;
        self.p.set_zero(value == 0);
        self.p.set_negative(value & 0x80 != 0);
    }
    /// LDX
    fn ldx(&mut self, value: u8) {
        self.x = value;
        self.p.set_zero(value == 0);
        self.p.set_negative(value & 0x80 != 0);
    }
    /// LDY
    fn ldy(&mut self, value: u8) {
        self.y = value;
        self.p.set_zero(value == 0);
        self.p.set_negative(value & 0x80 != 0);
    }
    /// ORA
    fn ora(&mut self, value: u8) {
        self.a |= value;
        self.p.set_zero(self.a == 0);
        self.p.set_negative(self.a & 0x80 != 0);
    }
    /// Sets zero and negative flags
    fn set_n_z(&mut self, value: u8) {
        let mut flag = value == 0;
        self.p.set_zero(flag);
        flag = value & 0x80 != 0;
        self.p.set_negative(flag);
    }
    /// Code to string translation
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
    ///  Steps through the instructions
    ///
    /// Read instriction from memory, executes it and set PC to point to next instruction in memory.
    /// If debug flag is set to true it will also print mnemonic code of the instruction that is executed.
    pub fn step(&mut self) {
        macro_rules! dbg { ($($x:tt)*) => { if self.debug { println!($($x)*); } } }

        let opcode = self.memory.read_byte(self.pc);
        self.pc += 1;

        match opcode {
            // ADC #imm
            ADC_IMM => {
                let value = self.read_immediate_byte();
                self.adc(value);
                dbg!("{}ADC #${:02X}", self.code_to_str(2), value);
            }
            // ADC zp
            ADC_ZP => {
                let value = self.read_zero_page();
                self.adc(value);
                dbg!(
                    "{}ADC ${:02X}",
                    self.code_to_str(2),
                    self.memory.read_byte(self.pc.wrapping_sub(1))
                );
            }
            // ADC oper ;zero page,X
            ADC_ZP_X => {
                let value = self.read_zero_page_x();
                self.adc(value);
                dbg!(
                    "{}ADC ${:02X},X",
                    self.code_to_str(2),
                    self.memory.read_byte(self.pc.wrapping_sub(1))
                );
            }
            // ADC oper ;absolute
            ADC_ABS => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.adc(value);
                dbg!("{}ADC ${:04X}", self.code_to_str(3), addr);
            }
            // ADC oper ;absolute,X
            ADC_ABS_X => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_x();
                self.adc(value);
                dbg!("{}ADC ${:04X},X", self.code_to_str(3), addr);
            }
            // ADC abs,Y ;absolute,Y
            ADC_ABS_Y => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_y();
                self.adc(value);
                dbg!("{}ADC ${:02X},Y", self.code_to_str(3), addr);
            }
            // ADC (oper,X) ;(indexed indirect)
            ADC_IND_X => {
                let addr = self.get_zero_page_address();
                let value = self.read_indexed_indirect();
                self.adc(value);
                dbg!("{}ADC (${:02X},X)", self.code_to_str(2), addr);
            }
            // ADC (oper),Y ;(indexed indirect),Y
            ADC_IND_Y => {
                let addr = self.get_zero_page_address();
                let value = self.read_indirect_indexed();
                self.adc(value);
                dbg!("{}ADC (${:02X}),Y", self.code_to_str(2), addr);
            }
            // AND #imm
            AND_IMM => {
                let value = self.read_immediate_byte();
                self.and(value);
                dbg!("{}AND #${:02X}", self.code_to_str(2), value);
            }
            // AND zp
            AND_ZP => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.and(value);
                dbg!("{}AND ${:02X}", self.code_to_str(2), addr);
            }
            // AND zp,X
            AND_ZP_X => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page_x();
                self.and(value);
                dbg!("{}AND ${:02X},X", self.code_to_str(2), addr);
            }
            // AND abs
            AND_ABS => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.and(value);
                dbg!("{}AND ${:04X}", self.code_to_str(3), addr);
            }
            // AND abs,X
            AND_ABS_X => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_x();
                self.and(value);
                dbg!("{}AND ${:04X},X", self.code_to_str(3), addr);
            }
            // AND abs,Y
            AND_ABS_Y => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_y();
                self.and(value);
                dbg!("{}AND ${:04X},Y", self.code_to_str(3), addr);
            }
            // AND (indirect,X)
            AND_IND_X => {
                let addr = self.get_zero_page_address();
                let value = self.read_indexed_indirect();
                self.and(value);
                dbg!("{}AND (${:02X},X)", self.code_to_str(2), addr);
            }
            // AND (indirect),Y
            AND_IND_Y => {
                let addr = self.get_zero_page_address();
                let value = self.read_indirect_indexed();
                self.and(value);
                dbg!("{}AND (${:02X}),Y", self.code_to_str(2), addr);
            }
            // ASL A
            ASL_A => {
                self.a = self.asl(self.a);
                dbg!("{}ASL A", self.code_to_str(1));
            }
            // ASL Zero Page
            ASL_ZP => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                let result = self.asl(value);
                self.memory.write_byte_zero_page(addr, result);
                dbg!("{}ASL ${:02X}", self.code_to_str(2), addr);
            }
            // ASL Zero Page,X
            ASL_ZP_X => {
                let addr_zp = self.get_zero_page_address();
                let addr = self.get_zero_page_address_x();
                let value = self.read_zero_page_x();
                let result = self.asl(value);
                self.memory.write_byte(addr as u16, result);
                dbg!("{}ASL ${:02X},X", self.code_to_str(2), addr_zp);
            }
            // ASL Absolute
            ASL_ABS => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                let result = self.asl(value);
                self.memory.write_byte(addr, result);
                dbg!("{}ASL ${:04X}", self.code_to_str(3), addr);
            }
            // ASL Absolute,X
            ASL_ABS_X => {
                // ASL Absolute,X
                let addr_zp = self.get_absolute_address();
                let addr = addr_zp.wrapping_add(self.x as u16);
                let value = self.read_absolute_x();
                let result = self.asl(value);
                self.memory.write_byte(addr, result);
                dbg!("{}ASL ${:04X},X", self.code_to_str(3), addr_zp);
            }
            // BCC
            BCC => {
                let offset = self.read_immediate_byte() as i8;
                let addr = self.pc.wrapping_add(offset as u16);
                dbg!("{}BCC ${:04X}", self.code_to_str(2), addr);
                if !self.p.is_carry() {
                    self.pc = addr;
                    dbg!("----");
                }
            }
            // Start of BCS
            BCS => {
                // BCS
                let offset = self.read_immediate_byte() as i8;
                let addr = self.pc.wrapping_add(offset as u16);
                dbg!("{}BCS ${:04X}", self.code_to_str(2), addr);
                if self.p.is_carry() {
                    self.pc = addr;
                    dbg!("----");
                }
            }
            // BEQ (Branch if Equal / Zero flag set)
            BEQ => {
                let offset = self.read_immediate_byte() as i8;
                let addr = self.pc.wrapping_add(offset as u16);
                dbg!("{}BEQ ${:04X}", self.code_to_str(2), addr);
                if self.p.is_zero() {
                    self.pc = addr;
                    dbg!("----");
                }
            }
            // BIT Zero Page
            BIT_ZP => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.bit(value);
                dbg!("{}BIT ${:02X}", self.code_to_str(2), addr);
            }
            // BIT Absolute
            BIT_ABS => {
                let addr = self.read_immediate_word();
                let value = self.memory.read_byte(addr);
                self.bit(value);
                dbg!("{}BIT ${:04X}", self.code_to_str(3), addr);
            }
            // BMI
            BMI => {
                let offset = self.read_immediate_byte() as i8;
                let addr = self.pc.wrapping_add(offset as u16);
                dbg!("{}BMI ${:04X}", self.code_to_str(2), addr);
                if self.p.is_negative() {
                    self.pc = addr;
                    dbg!("----");
                }
            }
            // BNE (Branch if Not Equal / Zero flag clear)
            BNE => {
                let offset = self.read_immediate_byte() as i8;
                let addr = self.pc.wrapping_add(offset as u16);
                dbg!("{}BNE ${:04X}", self.code_to_str(3), addr);
                if !self.p.is_zero() {
                    self.pc = addr;
                    dbg!("----");
                }
            }
            // BPL
            BPL => {
                let offset = self.read_immediate_byte() as i8;
                let addr = self.pc.wrapping_add(offset as u16);
                dbg!("{}BPL ${:04X}", self.code_to_str(3), addr);
                if !self.p.is_negative() {
                    self.pc = addr;
                    dbg!("----");
                }
            }
            // BRK
            BRK => {
                dbg!("{}BRK", self.code_to_str(1));
                dbg!("----");
                self.brk();
            }
            // BVC
            BVC => {
                let offset = self.read_immediate_byte() as i8;
                let addr = self.pc.wrapping_add(offset as u16);
                dbg!("{}BVC ${:04X}", self.code_to_str(3), addr);
                if self.p.value & 0x40 == 0 {
                    self.pc = addr;
                    dbg!("----");
                }
            }
            // BVS
            BVS => {
                // BVS
                let offset = self.read_immediate_byte() as i8;
                let addr = self.pc.wrapping_add(offset as u16);
                dbg!("{}BVS ${:04X}", self.code_to_str(3), addr);
                if self.p.value & 0x40 != 0 {
                    self.pc = addr;
                    dbg!("----");
                }
            }
            // CLC
            CLC => {
                self.p.set_carry(false);
                dbg!("{}CLC", self.code_to_str(1));
            }
            // CLD
            CLD => {
                self.p.set_decimal_mode(false);
                dbg!("{}CLD", self.code_to_str(1));
            }
            // CLI
            CLI => {
                self.p.set_interrupt_disable(false);
                dbg!("{}CLI", self.code_to_str(1));
            }
            // CLV
            CLV => {
                self.p.set_overflow(false);
                dbg!("{}CLV", self.code_to_str(1));
            }
            // CMP #imm
            CMP_IMM => {
                let value = self.read_immediate_byte();
                self.cmp(value);
                dbg!("{}CMP #${:02X}", self.code_to_str(2), value);
            }
            // CMP zp
            CMP_ZP => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.cmp(value);
                dbg!("{}CMP ${:02X}", self.code_to_str(2), addr);
            }
            // CMP zp,X
            CMP_ZP_X => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page_x();
                self.cmp(value);
                dbg!("{}CMP ${:02X},X", self.code_to_str(2), addr);
            }
            // CMP abs
            CMP_ABS => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.cmp(value);
                dbg!("{}CMP ${:04X}", self.code_to_str(3), addr);
            }
            // CMP abs,X
            CMP_ABS_X => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_x();
                self.cmp(value);
                dbg!("{}CMP ${:04X},X", self.code_to_str(3), addr);
            }
            // CMP abs,Y
            CMP_ABS_Y => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_y();
                self.cmp(value);
                dbg!("{}CMP ${:04X},Y", self.code_to_str(3), addr);
            }
            // CMP (zp,X)
            CMP_IND_X => {
                let addr = self.get_zero_page_address();
                let value = self.read_indexed_indirect();
                self.cmp(value);
                dbg!("{}CMP (${:02X},X)", self.code_to_str(2), addr);
            }
            // CMP (zp),Y
            CMP_IND_Y => {
                let addr = self.get_zero_page_address();
                let value = self.read_indirect_indexed();
                self.cmp(value);
                dbg!("{}CMP (${:02X}),Y", self.code_to_str(2), addr);
            }
            // CPX #imm
            CPX_IMM => {
                let value = self.read_immediate_byte();
                self.cpx(value);
                dbg!("{}CPX #${:02X}", self.code_to_str(2), value);
            }
            // CPX zp
            CPX_ZP => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.cpx(value);
                dbg!("{}CPX ${:02X}", self.code_to_str(2), addr);
            }
            // CPX abs
            CPX_ABS => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.cpx(value);
                dbg!("{}CPX ${:04X}", self.code_to_str(3), addr);
            }
            // CPY #imm
            CPY_IMM => {
                let value = self.read_immediate_byte();
                self.cpy(value);
                dbg!("{}CPY #${:02X}", self.code_to_str(2), value);
            }
            // CPY zp
            CPY_ZP => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.cpy(value);
                dbg!("{}CPY ${:02X}", self.code_to_str(2), addr);
            }
            // CPY abs
            CPY_ABS => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.cpy(value);
                dbg!("{}CPY ${:04X}", self.code_to_str(3), addr);
            }
            // DEC Zero Page
            DEC_ZP => {
                let addr = self.get_zero_page_address();
                let val = self.read_zero_page();
                let result = self.dec(val);
                self.memory.write_byte(addr as u16, result);
                dbg!("{}DEC ${:02X}", self.code_to_str(2), addr);
            }
            // DEC Zero Page,X
            DEC_ZP_X => {
                let addr_zp = self.get_zero_page_address();
                let addr = self.get_zero_page_address_x();
                let val = self.read_zero_page_x();
                let result = self.dec(val);
                self.memory.write_byte(addr as u16, result);
                dbg!("{}DEC ${:02X},X", self.code_to_str(2), addr_zp);
            }
            // DEC Absolute
            DEC_ABS => {
                let addr = self.get_absolute_address();
                let val = self.read_absolute();
                let result = self.dec(val);
                self.memory.write_byte(addr, result);
                dbg!("{}DEC ${:04X}", self.code_to_str(3), addr);
            }
            DEC_ABS_X => {
                // DEC Absolute,X
                let addr_abs = self.get_absolute_address();
                let addr = addr_abs.wrapping_add(self.x as u16);
                let val = self.read_absolute_x();
                let result = self.dec(val);
                self.memory.write_byte(addr, result);
                dbg!("{}DEC ${:04X},X", self.code_to_str(3), addr_abs);
            }
            // DEX
            DEX => {
                self.x = self.dec(self.x);
                dbg!("{}DEX", self.code_to_str(1));
            }
            // DEY
            DEY => {
                self.y = self.dec(self.y);
                dbg!("{}DEY", self.code_to_str(1));
            }
            // EOR #imm
            EOR_IMM => {
                let value = self.read_immediate_byte();
                self.eor(value);
                dbg!("{}EOR #${:02X}", self.code_to_str(2), value);
            }
            // EOR zp
            EOR_ZP => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.eor(value);
                dbg!("{}EOR ${:02X}", self.code_to_str(2), addr);
            }
            // EOR zp,X
            EOR_ZP_X => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page_x();
                self.eor(value);
                dbg!("{}EOR ${:02X},X", self.code_to_str(2), addr);
            }
            // EOR abs
            EOR_ABS => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.eor(value);
                dbg!("{}EOR ${:04X}", self.code_to_str(3), addr);
            }
            // EOR abs,X
            EOR_ABS_X => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_x();
                self.eor(value);
                dbg!("{}EOR ${:04X},X", self.code_to_str(3), addr);
            }
            // EOR abs,Y
            EOR_ABS_Y => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_y();
                self.eor(value);
                dbg!("{}EOR ${:04X},Y", self.code_to_str(3), addr);
            }
            // EOR indirect,X
            EOR_IND_X => {
                let addr = self.get_zero_page_address();
                let value = self.read_indexed_indirect();
                self.eor(value);
                dbg!("{}EOR (${:02X},X)", self.code_to_str(2), addr);
            }
            EOR_IND_Y => {
                // EOR indirect,Y
                let addr = self.get_zero_page_address();
                let value = self.read_indirect_indexed();
                self.eor(value);
                dbg!("{}EOR (${:02X}),Y", self.code_to_str(2), addr);
            }
            // INC Zero Page
            INC_ZP => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                let result = self.inc(value);
                self.memory.write_byte_zero_page(addr, result);
                dbg!("{}INC ${:02X}", self.code_to_str(2), addr);
            }
            // INC Zero Page,X
            INC_ZP_X => {
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
            // INC Absolute
            INC_ABS => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                let result = self.inc(value);
                self.memory.write_byte(addr, result);
                dbg!("{}INC ${:04X}", self.code_to_str(3), addr);
            }
            // INC Absolute,X
            INC_ABS_X => {
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
            // INX
            INX => {
                // INX
                self.x = self.inc(self.x);
                dbg!("{}INX", self.code_to_str(1));
            }
            // INY
            INY => {
                // INY
                self.y = self.inc(self.y);
                dbg!("{}INY", self.code_to_str(1));
            }
            // JMP absolute
            JMP => {
                let addr = self.memory.read_word(self.pc);
                self.pc += 2;
                dbg!("{}JMP ${:04X}\n----", self.code_to_str(3), addr);
                self.pc = addr;
            }
            // JMP indirect
            JMP_IND => {
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
            // JSR
            JSR => {
                let addr = self.read_immediate_word();
                //                self.pc += 2;
                self.push_word(self.pc.wrapping_sub(1)); // push return address - 1
                dbg!("{}JSR ${:04X}\n----", self.code_to_str(3), addr);
                self.pc = addr;
            }
            // LDA Immediate
            LDA_IMM => {
                let value = self.read_immediate_byte();
                self.lda(value);
                dbg!("{}LDA #${:02X}", self.code_to_str(2), value);
            }
            // LDA Zero Page
            LDA_ZP => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.lda(value);
                dbg!("{}LDA ${:02X}", self.code_to_str(2), addr);
            }
            // LDA Zero Page,X
            LDA_ZP_X => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page_x();
                self.lda(value);
                dbg!("{}LDA ${:02X},X", self.code_to_str(2), addr);
            }
            // LDA Absolute
            LDA_ABS => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.lda(value);
                dbg!("{}LDA ${:04X}", self.code_to_str(3), addr);
            }
            // LDA Absolute,X
            LDA_ABS_X => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_x();
                self.lda(value);
                dbg!("{}LDA ${:04X},X", self.code_to_str(3), addr);
            }
            // LDA Absolute,Y
            LDA_ABS_Y => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_y();
                self.lda(value);
                dbg!("{}LDA ${:02X},Y", self.code_to_str(2), addr);
            }
            // LDA (zp,X)
            LDA_IND_X => {
                let addr = self.get_zero_page_address();
                let value = self.read_indexed_indirect();
                self.lda(value);
                dbg!("{}LDA (${:02X},X)", self.code_to_str(2), addr);
            }
            // LDA (zp),Y
            LDA_IND_Y => {
                let addr = self.get_zero_page_address();
                let value = self.read_indirect_indexed();
                self.lda(value);
                dbg!("{}LDA (${:02X}),Y", self.code_to_str(2), addr);
            }
            // LDX Immediate
            LDX_IMM => {
                let value = self.read_immediate_byte();
                self.ldx(value);
                dbg!("{}LDX #${:02X}", self.code_to_str(2), value);
            }
            // LDX zp
            LDX_ZP => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.ldx(value);
                dbg!("{}LDX ${:02X}", self.code_to_str(2), addr);
            }
            // LDX zp,Y
            LDX_ZP_Y => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page_y();
                self.ldx(value);
                dbg!("{}LDX ${:02X},Y", self.code_to_str(2), addr);
            }
            // LDX abs
            LDX_ABS => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.ldx(value);
                dbg!("{}LDX ${:04X}", self.code_to_str(3), addr);
            }
            // LDX abs,Y
            LDX_ABS_Y => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_y();
                self.ldx(value);
                dbg!("{}LDX ${:04X},Y", self.code_to_str(3), addr);
            }
            // LDY Immediate
            LDY_IMM => {
                let value = self.read_immediate_byte();
                self.ldy(value);
                dbg!("{}LDY #${:02X}", self.code_to_str(2), value);
            }
            // LDY Zero Page
            LDY_ZP => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.ldy(value);
                dbg!("{}LDY ${:02X}", self.code_to_str(2), addr);
            }
            // LDY zp,X
            LDY_ZP_X => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page_x();
                self.ldy(value);
                dbg!("{}LDY ${:02X},X", self.code_to_str(2), addr);
            }
            // LDY abs
            LDY_ABS => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.ldy(value);
                dbg!("{}LDY ${:04X}", self.code_to_str(3), addr);
            }
            // LDY abs,X
            LDY_ABS_X => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_x();
                self.ldy(value);
                dbg!("{}LDY ${:02X},X", self.code_to_str(3), addr);
            }
            // LSR A
            LSR_A => {
                self.a = self.lsr(self.a);
                dbg!("{}LSR A", self.code_to_str(1));
            }
            // LSR zp
            LSR_ZP => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                let result = self.lsr(value);
                self.memory.write_byte(addr as u16, result);
                dbg!("{}LSR ${:02X}", self.code_to_str(2), addr);
            }
            // LSR zp,X
            LSR_ZP_X => {
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
            // LSR abs
            LSR_ABS => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                let result = self.lsr(value);
                self.memory.write_byte(addr, result);
                dbg!("{}LSR ${:04X}", self.code_to_str(3), addr);
            }
            // LSR abs,X
            LSR_ABS_X => {
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
            // NOP
            NOP => {
                dbg!("{}NOP", self.code_to_str(1))
            }
            // ORA #imm
            ORA_IMM => {
                let value = self.read_immediate_byte();
                self.ora(value);
                dbg!("{}ORA #${:02X}", self.code_to_str(2), value);
            }
            // ORA zp
            ORA_ZP => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.ora(value);
                dbg!("{}ORA ${:02X}", self.code_to_str(2), addr);
            }
            // ORA zp,X
            ORA_ZP_X => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page_x();
                self.ora(value);
                dbg!("{}ORA ${:02X},X", self.code_to_str(2), addr);
            }
            // ORA abs
            ORA_ABS => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.ora(value);
                dbg!("{}ORA ${:04X}", self.code_to_str(3), addr);
            }
            // ORA abs,X
            ORA_ABS_X => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_x();
                self.ora(value);
                dbg!("{}ORA ${:04X},X", self.code_to_str(3), addr);
            }
            // ORA abs,Y
            ORA_ABS_Y => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_y();
                self.ora(value);
                dbg!("{}ORA ${:04X},Y", self.code_to_str(3), addr);
            }
            // ORA (zp,X)
            ORA_IND_X => {
                let addr = self.get_zero_page_address();
                let value = self.read_indexed_indirect();
                self.ora(value);
                dbg!("{}ORA (${:02X},X)", self.code_to_str(2), addr);
            }
            // ORA (zp),Y
            ORA_IND_Y => {
                let addr = self.get_zero_page_address();
                let value = self.read_indirect_indexed();
                self.ora(value);
                dbg!("{}ORA (${:02X}),Y", self.code_to_str(2), addr);
            }
            // PHA
            PHA => {
                self.push(self.a);
                dbg!("{}PHA", self.code_to_str(1));
            }
            // PHP
            PHP => {
                self.push(self.p.value | mos6502::BREAK | mos6502::UNUSED); // emulate B and Unused flag set when pushed
                dbg!("{}PHP", self.code_to_str(1));
            }
            // PLA
            PLA => {
                self.a = self.pop();
                self.p.set_zero(self.a == 0);
                self.p.set_negative(self.a & 0x80 != 0);
                dbg!("{}PLA", self.code_to_str(1));
            }
            // PLP
            PLP => {
                self.p.value = self.pop() & 0b1100_1111; // B and unused bits masked off
                dbg!("{}PLP (pull P)", self.code_to_str(1));
            }
            // ROL A
            ROL_A => {
                self.a = self.rol(self.a);
                dbg!("{}ROL A", self.code_to_str(1));
            }
            // ROL zp
            ROL_ZP => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                let result = self.rol(value);
                self.memory.write_byte(addr as u16, result);
                dbg!("{}ROL ${:02X}", self.code_to_str(2), addr);
            }
            // ROL zp,X
            ROL_ZP_X => {
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
            // ROL abs
            ROL_ABS => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                let result = self.rol(value);
                self.memory.write_byte(addr, result);
                dbg!("{}ROL ${:04X}", self.code_to_str(3), addr);
            }
            // ROL abs,X
            ROL_ABS_X => {
                let addr = self.get_absolute_address_x();
                let value = self.read_absolute_x();
                let result = self.rol(value);
                self.memory.write_byte(addr, result);
                dbg!(
                    "{}ROL ${:04X},X",
                    self.code_to_str(3),
                    addr.wrapping_sub(self.x as u16)
                );
            }
            // ROR A
            ROR_A => {
                self.a = self.ror(self.a);
                dbg!("{}ROR A", self.code_to_str(1));
            }
            // ROR zp
            ROR_ZP => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                let result = self.ror(value);
                self.memory.write_byte(addr as u16, result);
                dbg!("{}ROR ${:02X}", self.code_to_str(2), addr);
            }
            // ROR zp,X
            ROR_ZP_X => {
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
            // ROR abs
            ROR_ABS => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                let result = self.ror(value);
                self.memory.write_byte(addr, result);
                dbg!("{}ROR ${:04X}", self.code_to_str(3), addr);
            }
            // ROR abs,X
            ROR_ABS_X => {
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
            // RTI
            RTI => {
                dbg!("{}RTI", self.code_to_str(1));
                dbg!("----");
                self.p.value = self.pop() & !mos6502::BREAK & !mos6502::UNUSED; // B and unused bits masked off
                self.pc = self.pop_word();
            }
            // RTS
            RTS => {
                self.pc = self.pop_word().wrapping_add(1);
                dbg!("{}RTS\n----", self.code_to_str(1));
            }
            // SBC #imm
            SBC_IMM => {
                let value = self.read_immediate_byte();
                self.sbc(value);
                dbg!("{}SBC #${:02X}", self.code_to_str(2), value);
            }
            // SBC zp
            SBC_ZP => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page();
                self.sbc(value);
                dbg!("{}SBC ${:02x}", self.code_to_str(2), addr)
            }
            // SBC zp,X
            SBC_ZP_X => {
                let addr = self.get_zero_page_address();
                let value = self.read_zero_page_x();
                self.sbc(value);
                dbg!("{}SBC ${:02x},X", self.code_to_str(2), addr);
            }
            // SBC absolute
            SBC_ABS => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute();
                self.sbc(value);
                dbg!("{}SBC ${:04x}", self.code_to_str(3), addr);
            }
            // SBC absolute,X
            SBC_ABS_X => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_x();
                self.sbc(value);
                dbg!("{}SBC ${:04x},X", self.code_to_str(3), addr);
            }
            // SBC absolute,Y
            SBC_ABS_Y => {
                let addr = self.get_absolute_address();
                let value = self.read_absolute_y();
                self.sbc(value);
                dbg!("{}SBC ${:04x},Y", self.code_to_str(3), addr);
            }
            // SBC (indirect,X)
            SBC_IND_X => {
                let addr = self.get_absolute_address();
                let value = self.read_indexed_indirect();
                self.sbc(value);
                dbg!("{}SBC $({:02x},X)", self.code_to_str(2), addr);
            }
            // SBC(indirect),Y
            SBC_IND_Y => {
                let addr = self.get_absolute_address();
                let value = self.read_indirect_indexed();
                self.sbc(value);
                dbg!("{}SBC $({:02x}),Y", self.code_to_str(2), addr);
            }
            // SEC
            SEC => {
                self.p.set_carry(true);
                dbg!("{}SEC", self.code_to_str(1));
            }
            // SED
            SED => {
                self.p.set_decimal_mode(true);
                dbg!("{}SED", self.code_to_str(1));
            }
            // SEI
            SEI => {
                self.p.set_interrupt_disable(true);
                dbg!("{}SEI", self.code_to_str(1));
            }
            // STA zp
            STA_ZP => {
                let addr = self.get_zero_page_address();
                self.pc += 1;
                self.memory.write_byte_zero_page(addr, self.a);
                dbg!("{}STA ${:02X}", self.code_to_str(2), addr);
            }
            // STA zp,X
            STA_ZP_X => {
                let addr = self.get_zero_page_address_x();
                self.pc += 1;
                self.memory.write_byte_zero_page(addr, self.a);
                dbg!(
                    "{}STA ${:02X},X",
                    self.code_to_str(2),
                    addr.wrapping_sub(self.x)
                );
            }
            // STA $nnnn
            STA_ABS => {
                let addr = self.get_absolute_address();
                self.pc += 2;
                self.memory.write_byte(addr, self.a);
                dbg!("{}STA ${:04X}", self.code_to_str(3), addr);
            }
            // STA $nnnn,X
            STA_ABS_X => {
                let addr = self.get_absolute_address_x();
                self.pc += 2;
                self.memory.write_byte(addr, self.a);
                dbg!("{}STA ${:04X},X", self.code_to_str(3), addr);
            }
            // STA $nnnn,Y
            STA_ABS_Y => {
                let addr = self.get_absolute_address_y();
                self.pc += 2;
                self.memory.write_byte(addr, self.a);
                dbg!("{}STA ${:04X},Y", self.code_to_str(3), addr);
            }
            // STA (indirect,X)
            STA_IND_X => {
                let addr_zp = self.get_zero_page_address();
                let addr = self.get_indirect_address_x();
                self.pc += 1;
                self.memory.write_byte(addr, self.a);
                dbg!("{}STA (${:02X},X)", self.code_to_str(2), addr_zp);
            }
            // STA (indirect),Y
            STA_IND_Y => {
                let addr_zp = self.get_zero_page_address();
                let addr = self.get_indirect_address_y();
                self.pc += 1;
                self.memory.write_byte(addr, self.a);
                dbg!("{}STA (${:02X}),Y", self.code_to_str(2), addr_zp);
            }
            // STX zp
            STX_ZP => {
                let addr = self.read_immediate_byte();
                self.memory.write_byte_zero_page(addr, self.x);
                dbg!("{}STX ${:02X}", self.code_to_str(2), addr);
            }
            // STX zp,Y
            STX_ZP_Y => {
                let addr = self.read_immediate_byte();
                self.memory
                    .write_byte_zero_page(addr.wrapping_add(self.y), self.x);
                dbg!("{}STX ${:02X},Y", self.code_to_str(2), addr);
            }
            // STX abs
            STX_ABS => {
                let addr = self.read_immediate_word();
                self.memory.write_byte(addr, self.x);
                dbg!("{}STX ${:02X}", self.code_to_str(2), addr);
            }
            // STY zp
            STY_ZP => {
                let addr = self.read_immediate_byte();
                self.memory.write_byte_zero_page(addr, self.y);
                dbg!("{}STY ${:02X}", self.code_to_str(2), addr);
            }
            // STY zp,X
            STY_ZP_X => {
                let addr = self.read_immediate_byte();
                self.memory
                    .write_byte_zero_page(addr.wrapping_add(self.x), self.y);
                dbg!("{}STY ${:02X},X", self.code_to_str(2), addr);
            }
            // STY abs
            STY_ABS => {
                let addr = self.read_immediate_word();
                self.memory.write_byte(addr, self.y);
                dbg!("{}STY ${:04X}", self.code_to_str(3), addr);
            }
            // TAX
            TAX => {
                self.x = self.a;
                self.set_n_z(self.x);
                dbg!("{}TAX", self.code_to_str(1));
            }
            // TAY
            TAY => {
                self.y = self.a;
                self.set_n_z(self.y);
                dbg!("{}TAY", self.code_to_str(1));
            }
            // TSX
            TSX => {
                self.x = self.sp;
                self.set_n_z(self.x);
                dbg!("{}TSX", self.code_to_str(1));
            }
            // TXA
            TXA => {
                self.a = self.x;
                self.set_n_z(self.a);
                dbg!("{}TXA", self.code_to_str(1));
            }
            // TXS
            TXS => {
                self.sp = self.x;
                dbg!("{}TXS", self.code_to_str(1));
            }
            // TYA
            TYA => {
                self.a = self.y;
                self.set_n_z(self.a);
                dbg!("{}TYA", self.code_to_str(1));
            }
            // End of TYA
            _ => {
                dbg!("{}!byte ${:02X}", self.code_to_str(1), opcode);
            }
        }
    }
}

use crate::disassembler::mos6502::disassemble;
use crate::disassembler::mos6502::load_opcodes_table;
impl CpuUi for Cpu {
    fn memory_dump(&mut self, start: u16, end: u16) -> Vec<String> {
        self.memory.hex_dump(start, end)
    }
    fn get_memory(&mut self) -> &mut Memory {
        &mut self.memory
    }
    fn disasm(&mut self, start: u16, end: u16) -> Vec<String> {
        disassemble(&self.memory, start, end, &load_opcodes_table())
    }
}
