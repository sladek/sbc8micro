use crate::disassembler::i8080_opcodes_const::*;
use crate::memory::Memory;
use crate::status::i8080::*;

pub struct Cpu {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub psw: Psw,
    pub pc: u16,
    pub sp: u16,
    pub inte: bool,
    pub memory: Memory,
    pub debug: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            psw: Psw::new(),
            pc: 0,
            sp: 0,
            inte: false,
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
"Registers\n---------------------------------------------------------------------------------------------------------
|  A  |  B  |  C  |  D  |  E  |  H  |  L  |  SP   |  PC   | INTE | PSW | S | Z | 0 | AC | 0 | P | 1 | C |
|-----|-----|-----|-----|-----|-----|-----|-------|-------|------|-----|---|---|---|----|---|---|---|---|
| {:02X}H | {:02X}H | {:02X}H | {:02X}H | {:02X}H | {:02X}H | {:02X}H | {:04X}H | {:04X}H |  {}   | {:02X}H | {} | {} | 0 | {}  | 0 | {} | 1 | {} |
---------------------------------------------------------------------------------------------------------\n",
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.sp,
            self.pc,
            if self.inte {1} else {0},
            self.psw.value,
            self.psw.is_negative() as u8,
            self.psw.is_zero() as u8,
            self.psw.is_ac() as u8,
            self.psw.is_parity() as u8,
            self.psw.is_carry() as u8
        )
    }
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
    // Read data from data bus
    // This will be modified after all the instructions are implemented
    // Data bus needs to be introduced here
    fn inp(&mut self, _address: u8) -> u8 {
        0xff
    }
    fn read_immediate_byte(&mut self) -> u8 {
        let value = self.memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
    }
    fn read_immediate_word(&mut self) -> u16 {
        let w_low = self.read_immediate_byte() as u16;
        let w_high = self.read_immediate_byte() as u16;
        (w_high << 8) | w_low
    }
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
    fn get_m(&self) -> u8 {
        let addr = self.get_hl();
        self.memory.read_byte(addr)
    }
    fn set_bc(&mut self, data: u16) {
        self.b = ((data & 0xff00) >> 8) as u8;
        self.c = (data & 0x0ff) as u8;
    }
    fn set_de(&mut self, data: u16) {
        self.d = ((data & 0xff00) >> 8) as u8;
        self.e = (data & 0x0ff) as u8;
    }
    fn set_hl(&mut self, data: u16) {
        self.h = ((data & 0xff00) >> 8) as u8;
        self.l = (data & 0x0ff) as u8;
    }
    fn set_m(&mut self, value: u8) {
        let addr = self.get_hl();
        self.memory.write_byte(addr, value);
    }
    ///
    /// This function formats HEX string from address and opcode and
    /// is used in debug mode to print address and opcode of the instruction that
    /// is currently being processed
    /// For example if you are processing the following code at address 0x200
    /// MVI A, 55H
    /// Then it looks like this in memory
    /// 0x200 0x3E, 0x55
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
            addr = addr.wrapping_add(1);
        }
        format!("{:<18}", result)
    }

    fn set_parity(&mut self, data: u8) {
        let mut mask = 0x01;
        let mut result: u8 = 0;
        for i in 1..=8 {
            if (data & mask) != 0 {
                result += 1;
            };
            mask = mask << 1;
        }
        self.psw.set_parity(result % 2 == 0)
    }
    fn addc(&mut self, value: u8) {
        //        let carry = if self.psw.is_carry() { 1 } else { 0 };
        //        let sum = value as u16 + carry as u16;
        self.add(value, true);
    }
    fn add(&mut self, value: u8, with_carry: bool) {
        let mut carry = 0x0u8;
        if with_carry {
            carry = if self.psw.is_carry() { 1 } else { 0 } as u8;
        }
        //        let carry = if self.psw.is_carry() { 1 } else { 0 }
        if (self.a & 0x0f) + (value & 0x0f) + carry > 0x0f {
            self.psw.set_ac(true);
        } else {
            self.psw.set_ac(false)
        };
        let sum = self.a as u16 + value as u16 + carry as u16;
        self.a = sum as u8;
        self.psw.set_carry(sum > 0xFF);
        self.psw.set_zero(self.a == 0);
        self.psw.set_negative(self.a & 0x80 != 0);
        self.set_parity(sum as u8);
    }
/*
    fn sub(&mut self, value: u8) {
        self.psw.set_carry(value > self.a);
        let tmp = !value;
        if (self.a & 0x0f) + (tmp & 0x0f) + 1 > 0x0f {
            self.psw.set_ac(true);
        } else {
            self.psw.set_ac(false)
        };
        let sum = self.a as u16 + tmp as u16 + 1; // complement (!tmp + 1)
        self.a = sum as u8;
        self.psw.set_zero(self.a == 0);
        self.psw.set_negative(self.a & 0x80 != 0);
        self.set_parity(sum as u8);
    }
 */
    fn sub(&mut self, value: u8, with_carry: bool) {
        let mut operand = value as u16;
        let mut operand_lower = value & 0x0f;
        if self.psw.is_carry() && with_carry {
            operand = operand.wrapping_add(1);
            operand_lower = operand_lower.wrapping_add(1);
        }
        self.psw.set_carry(operand > self.a as u16);
        let two_compl = (!operand).wrapping_add(1);
        let two_compl_lower = (!operand_lower).wrapping_add(1);
        let sum = self.a.wrapping_add(two_compl as u8);
        let tmp = (self.a & 0x0f).wrapping_add (two_compl_lower & 0x0f);
        if tmp > 0x0f || operand_lower == 0 {
            self.psw.set_ac(true);
        } else {
            self.psw.set_ac(false);
        } 
        self.a = sum as u8;
        self.psw.set_zero(self.a == 0);
        self.psw.set_negative(self.a & 0x80 != 0);
        self.set_parity(sum as u8);
    }
    // ANA, ANI iinstructions clear CARRY but set AC based on bit 3
    // Some documentation states that ANI clears AC, but it is not true
    // it is set the same way as ANA. Verified on real HW. (TESLA 8080A)
    fn and(&mut self, value: u8) {
        let is_ac = (self.a | value) & 0x08 != 0;
        let result = self.a as u16 & value as u16;
        self.a = result as u8;
        self.psw.set_carry(false);
        self.psw.set_ac(is_ac);
        self.psw.set_zero(self.a == 0);
        self.psw.set_negative(self.a & 0x80 != 0);
        self.set_parity(self.a);
    }
    // ORA, ORI clears CARRY and AC flags
    fn or(&mut self, value: u8) {
        let result = self.a as u16 | value as u16;
        self.a = result as u8;
        self.psw.set_carry(false);
        self.psw.set_ac(false);
        self.psw.set_zero(self.a == 0);
        self.psw.set_negative(self.a & 0x80 != 0);
        self.set_parity(self.a);
    }
    // XRA
    fn xra(&mut self, value: u8) {
        let result = self.a ^ value;
        self.a = result;
        self.psw.set_carry(false);
        self.psw.set_ac(false);
        self.psw.set_zero(self.a == 0);
        self.psw.set_negative(self.a & 0x80 != 0);
        self.set_parity(self.a);
    }
    fn read_m(&self) -> u8 {
        let h = self.h as u16;
        let l = self.l as u16;
        let hl = (h << 8) | l;
        self.memory.read_byte(hl)
    }
    fn store_m(&mut self, data: u8) {
        let h = self.h as u16;
        let l = self.l as u16;
        let hl = (h << 8) | l;
        self.memory.write_byte(hl, data);
    }
    fn push(&mut self, rph: u8, rpl: u8) {
        let mut addr = self.sp.wrapping_sub(1);
        self.memory.write_byte(addr, rph);
        addr = self.sp.wrapping_sub(2);
        self.memory.write_byte(addr, rpl);
        self.sp = addr;
    }
    fn pop(&mut self) -> (u8, u8) {
        let mut addr = self.sp;
        let rpl = self.memory.read_byte(addr);
        addr = addr.wrapping_add(1);
        let rph = self.memory.read_byte(addr);
        self.sp = self.sp.wrapping_add(2);
        (rph, rpl)
    }
    fn call(&mut self) {
        let addr = self.memory.read_word(self.pc);
        self.pc = self.pc.wrapping_add(2);
        let pcl = (self.pc & 0xff) as u8;
        let pch = ((self.pc & 0xff00) >> 8) as u8;
        self.push(pch, pcl);
        self.pc = addr;
    }
    fn jmp(&mut self) {
        let addr = self.memory.read_word(self.pc);
        self.pc = addr;
    }
    fn ret(&mut self) {
        let addrl = self.memory.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let addrh = (self.memory.read_byte(self.sp) as u16) << 8;
        self.sp = self.sp.wrapping_add(1);
        self.pc = addrh | addrl;
    }
    fn daa(&mut self) {
        let mut accl: u8 = self.a & 0x0f;
        let mut acch: u16 = self.a as u16 & 0xf0;
        if (self.a & 0x0f > 0x09) || self.psw.is_ac() {
            accl = accl.wrapping_add(0x06);
            self.psw.set_ac(accl > 0x0f);
        }
        if (self.a & 0xf0 > 0x90) || self.psw.is_carry() {
            acch = acch.wrapping_add(0x60);
            self.psw.set_carry(true);
        }
        self.a = acch.wrapping_add(accl as u16) as u8;
        if self.a == 0 {
            self.psw.set_zero(true);
        }
        if (self.a & 0x80) != 0 {
            self.psw.set_negative(true);
        }
        self.set_parity(self.a);
    }
    // Add data to HL pair and set CARRY if result is > 0x00ff.
    pub fn dad(&mut self, rp: u16) {
        let result: u32 = self.get_hl() as u32 + rp as u32;
        println!("Result: {:08X}", result);
        self.set_hl(result as u16);
        self.psw.set_carry(result > 0x0ffff);
    }
    // DCR reg
    pub fn dcr(&mut self, reg: u8) -> u8 {
        let res = reg.wrapping_sub(1);
        self.set_parity(res);
        self.psw.set_zero(res == 0);
        self.psw.set_ac((res & 0x0fu8) != 0x0fu8);
        self.psw.set_negative((res & 0x80u8) != 0);
        res
    }
    // INR reg
    pub fn inr(&mut self, reg: u8) -> u8 {
        let res = reg.wrapping_add(1);
        self.set_parity(res);
        self.psw.set_zero(res == 0);
        self.psw.set_ac((res & 0x0fu8) == 0x00u8);
        self.psw.set_negative((res & 0x80u8) != 0);
        res
    }
    fn rst(&mut self, level: u8) {
        let pcl = (self.pc & 0xff) as u8;
        let pch = ((self.pc & 0xff00) >> 8) as u8;
        self.push(pch, pcl);
        self.pc = (level * 8) as u16;
    }
    // Step through the instructions
    pub fn step(&mut self) {
        macro_rules! dbg { ($($x:tt)*) => { if self.debug { println!($($x)*); } } }

        let opcode = self.memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);

        match opcode {
            ACI => {
                let value = self.read_immediate_byte();
                self.addc(value);
                dbg!("{}ACI {:02X}H", self.code_to_str(2), value);
            }
            ADC_B => {
                let value = self.b;
                self.addc(value);
                dbg!("{}ADC B", self.code_to_str(1));
            }
            ADC_C => {
                let value = self.c;
                self.addc(value);
                dbg!("{}ADC C", self.code_to_str(1));
            }
            ADC_D => {
                let value = self.d;
                self.addc(value);
                dbg!("{}ADC D", self.code_to_str(1));
            }
            ADC_E => {
                let value = self.e;
                self.addc(value);
                dbg!("{}ADC E", self.code_to_str(1));
            }
            ADC_H => {
                let value = self.h;
                self.addc(value);
                dbg!("{}ADC H", self.code_to_str(1));
            }
            ADC_L => {
                let value = self.l;
                self.addc(value);
                dbg!("{}ADC L", self.code_to_str(1));
            }
            ADC_M => {
                let value = self.read_m();
                self.addc(value);
                dbg!("{}ADC M", self.code_to_str(1));
            }
            ADC_A => {
                let value = self.a;
                self.addc(value);
                dbg!("{}ADC A", self.code_to_str(1));
            }
            ADD_B => {
                let value = self.b;
                self.add(value, false);
                dbg!("{}ADD B", self.code_to_str(1));
            }
            ADD_C => {
                let value = self.c;
                self.add(value, false);
                dbg!("{}ADD C", self.code_to_str(1));
            }
            ADD_D => {
                let value = self.d;
                self.add(value, false);
                dbg!("{}ADD D", self.code_to_str(1));
            }
            ADD_E => {
                let value = self.e;
                self.add(value, false);
                dbg!("{}ADD E", self.code_to_str(1));
            }
            ADD_H => {
                let value = self.h;
                self.add(value, false);
                dbg!("{}ADD H", self.code_to_str(1));
            }
            ADD_L => {
                let value = self.l;
                self.add(value, false);
                dbg!("{}ADD L", self.code_to_str(1));
            }
            ADD_M => {
                let value = self.read_m();
                self.add(value, false);
                dbg!("{}ADD M", self.code_to_str(1));
            }
            ADD_A => {
                let value = self.a;
                self.add(value, false);
                dbg!("{}ADD A", self.code_to_str(1));
            }
            ADI => {
                let value = self.read_immediate_byte();
                self.add(value, false);
                dbg!("{}ADI {:02X}H", self.code_to_str(2), value);
            }
            ANA_B => {
                let value = self.b;
                self.and(value);
                dbg!("{}ANA B", self.code_to_str(1));
            }
            ANA_C => {
                let value = self.c;
                self.and(value);
                dbg!("{}ANA C", self.code_to_str(1));
            }
            ANA_D => {
                let value = self.d;
                self.and(value);
                dbg!("{}ANA D", self.code_to_str(1));
            }
            ANA_E => {
                let value = self.e;
                self.and(value);
                dbg!("{}ANA E", self.code_to_str(1));
            }
            ANA_H => {
                let value = self.h;
                self.and(value);
                dbg!("{}ANA H", self.code_to_str(1));
            }
            ANA_L => {
                let value = self.l;
                self.and(value);
                dbg!("{}ANA L", self.code_to_str(1));
            }
            ANA_M => {
                let value = self.read_immediate_byte();
                self.and(value);
                dbg!("{}ANA M", self.code_to_str(1));
            }
            ANA_A => {
                let value = self.a;
                self.and(value);
                dbg!("{}ANA A", self.code_to_str(1));
            }
            ANI => {
                let value = self.read_immediate_byte();
                self.and(value);
                dbg!("{}ANI {:02X}H", self.code_to_str(2), value);
            }
            CMA => {
                self.a = !self.a;
                dbg!("{}CMA", self.code_to_str(1));
            }
            CMC => {
                self.psw.set_carry(!self.psw.is_carry());
                dbg!("{}CMC", self.code_to_str(1));
            }
            CMP_B => {
                let tmp = self.a;
                self.sub(self.b, false);
                self.a = tmp;
                dbg!("{}CMP B", self.code_to_str(1));
            }
            CMP_C => {
                let tmp = self.a;
                self.sub(self.c, false);
                self.a = tmp;
                dbg!("{}CMP C", self.code_to_str(1));
            }
            CMP_D => {
                let tmp = self.a;
                self.sub(self.d, false);
                self.a = tmp;
                dbg!("{}CMP D", self.code_to_str(1));
            }
            CMP_E => {
                let tmp = self.a;
                self.sub(self.e, false);
                self.a = tmp;
                dbg!("{}CMP E", self.code_to_str(1));
            }
            CMP_H => {
                let tmp = self.a;
                self.sub(self.h, false);
                self.a = tmp;
                dbg!("{}CMP H", self.code_to_str(1));
            }
            CMP_L => {
                let tmp = self.a;
                self.sub(self.l, false);
                self.a = tmp;
                dbg!("{}CMP L", self.code_to_str(1));
            }
            CMP_M => {
                let tmp = self.a;
                self.sub(self.read_m(), false);
                self.a = tmp;
                dbg!("{}CMP M", self.code_to_str(1));
            }
            CMP_A => {
                let tmp = self.a;
                self.sub(self.a, false);
                self.a = tmp;
                dbg!("{}CMP A", self.code_to_str(1));
            }
            CPI => {
                let tmp = self.a;
                let data = self.read_immediate_byte();
                self.sub(data, false);
                self.a = tmp;
                dbg!("{}CPI {:02X}", self.code_to_str(2), data);
            }
            CALL => {
                if self.debug {
                    let addr = self.memory.read_word(self.pc);
                    self.pc = self.pc.wrapping_add(2);
                    let code = self.code_to_str(3);
                    self.pc = self.pc.wrapping_sub(2);
                    dbg!("{}CALL {:04X}", code, addr);
                }
                self.call();
            }
            CNZ => {
                if !self.psw.is_zero() {
                    if self.debug {
                        let addr = self.memory.read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        dbg!("{}CNZ {:04X}", code, addr);
                    }
                    self.call();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            CZ => {
                if self.psw.is_zero() {
                    if self.debug {
                        let addr = self.memory.read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        dbg!("{}CZ {:04X}", code, addr);
                    }
                    self.call();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            CNC => {
                if !self.psw.is_carry() {
                    if self.debug {
                        let addr = self.memory.read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        dbg!("{}CNC {:04X}", code, addr);
                    }
                    self.call();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            CC => {
                if self.psw.is_carry() {
                    if self.debug {
                        let addr = self.memory.read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        dbg!("{}CC {:04X}", code, addr);
                    }
                    self.call();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            CPO => {
                if !self.psw.is_parity() {
                    if self.debug {
                        let addr = self.memory.read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        dbg!("{}CPO {:04X}", code, addr);
                    }
                    self.call();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            CPE => {
                if self.psw.is_parity() {
                    if self.debug {
                        let addr = self.memory.read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        dbg!("{}CPE {:04X}", code, addr);
                    }
                    self.call();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            CP => {
                if !self.psw.is_negative() {
                    if self.debug {
                        let addr = self.memory.read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        dbg!("{}CP {:04X}", code, addr);
                    }
                    self.call();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            CM => {
                if self.psw.is_negative() {
                    if self.debug {
                        let addr = self.memory.read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        dbg!("{}CM {:04X}", code, addr);
                    }
                    self.call();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            DAA => {
                self.daa();
                dbg!("{}DAA", self.code_to_str(1));
            }
            DAD_B => {
                self.dad(self.get_bc());
                dbg!("{}DAD B", self.code_to_str(1));
            }
            DAD_D => {
                self.dad(self.get_de());
                dbg!("{}DAD D", self.code_to_str(1));
            }
            DAD_H => {
                self.dad(self.get_hl());
                dbg!("{}DAD H", self.code_to_str(1));
            }
            DAD_SP => {
                self.dad(self.sp);
                dbg!("{}DAD H", self.code_to_str(1));
            }
            DCR_B => {
                self.b = self.dcr(self.b);
                dbg!("{}DCR B", self.code_to_str(1));
            }
            DCR_C => {
                self.c = self.dcr(self.c);
                dbg!("{}DCR C", self.code_to_str(1));
            }
            DCR_D => {
                self.d = self.dcr(self.d);
                dbg!("{}DCR D", self.code_to_str(1));
            }
            DCR_E => {
                self.e = self.dcr(self.e);
                dbg!("{}DCR E", self.code_to_str(1));
            }
            DCR_H => {
                self.h = self.dcr(self.h);
                dbg!("{}DCR H", self.code_to_str(1));
            }
            DCR_L => {
                self.l = self.dcr(self.l);
                dbg!("{}DCR L", self.code_to_str(1));
            }
            DCR_M => {
                let mut value = self.read_m();
                value = self.dcr(value);
                self.store_m(value);
                dbg!("{}DCR M", self.code_to_str(1));
            }
            DCR_A => {
                self.a = self.dcr(self.a);
                dbg!("{}DCR A", self.code_to_str(1));
            }
            DCX_B => {
                self.set_bc(self.get_bc().wrapping_sub(1));
                dbg!("{}DCX B", self.code_to_str(1));
            }
            DCX_D => {
                self.set_de(self.get_de().wrapping_sub(1));
                dbg!("{}DCX D", self.code_to_str(1));
            }
            DCX_H => {
                self.set_hl(self.get_hl().wrapping_sub(1));
                dbg!("{}DCX H", self.code_to_str(1));
            }
            DCX_SP => {
                self.sp = self.sp.wrapping_sub(1);
                dbg!("{}DCX SP", self.code_to_str(1));
            }
            DI => {
                self.inte = false;
                dbg!("{}EI", self.code_to_str(1));
            }
            EI => {
                self.inte = true;
                dbg!("{}EI", self.code_to_str(1));
            }
            HLT => {
                dbg!("{}HLT", self.code_to_str(1));
            }
            IN => {
                let addr = self.read_immediate_byte();
                self.a = self.inp(addr);
                dbg!("{}IN {:02X}H", self.code_to_str(2), addr);
            }
            INR_B => {
                self.b = self.inr(self.b);
                dbg!("{}INR B", self.code_to_str(1));
            }
            INR_C => {
                self.c = self.inr(self.c);
                dbg!("{}INR C", self.code_to_str(1));
            }
            INR_D => {
                self.d = self.inr(self.d);
                dbg!("{}INR D", self.code_to_str(1));
            }
            INR_E => {
                self.e = self.inr(self.e);
                dbg!("{}INR D", self.code_to_str(1));
            }
            INR_H => {
                self.h = self.inr(self.h);
                dbg!("{}INR H", self.code_to_str(1));
            }
            INR_L => {
                self.l = self.inr(self.l);
                dbg!("{}INR L", self.code_to_str(1));
            }
            INR_M => {
                let mut value = self.read_m();
                value = self.inr(value);
                self.store_m(value);
                dbg!("{}INR M", self.code_to_str(1));
            }
            INR_A => {
                self.a = self.inr(self.a);
                dbg!("{}INR A", self.code_to_str(1));
            }
            INX_B => {
                self.set_bc(self.get_bc().wrapping_add(1));
                dbg!("{}INX B", self.code_to_str(1));
            }
            INX_D => {
                self.set_de(self.get_de().wrapping_add(1));
                dbg!("{}INX D", self.code_to_str(1));
            }
            INX_H => {
                self.set_hl(self.get_hl().wrapping_add(1));
                dbg!("{}INX H", self.code_to_str(1));
            }
            INX_SP => {
                self.sp = self.sp.wrapping_add(1);
                dbg!("{}INX SP", self.code_to_str(1));
            }
            JNZ => {
                if !self.psw.is_zero() {
                    if self.debug {
                        let addr = self.memory.read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        dbg!("{}JNZ {:04X}", code, addr);
                    }
                    self.jmp();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            JZ => {
                if self.psw.is_zero() {
                    if self.debug {
                        let addr = self.memory.read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        dbg!("{}JZ {:04X}", code, addr);
                    }
                    self.jmp();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            JNC => {
                if !self.psw.is_carry() {
                    if self.debug {
                        let addr = self.memory.read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        dbg!("{}JNC {:04X}", code, addr);
                    }
                    self.jmp();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            JC => {
                if self.psw.is_carry() {
                    if self.debug {
                        let addr = self.memory.read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        dbg!("{}JC {:04X}", code, addr);
                    }
                    self.jmp();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            JPO => {
                if !self.psw.is_parity() {
                    if self.debug {
                        let addr = self.memory.read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        dbg!("{}JPO {:04X}", code, addr);
                    }
                    self.jmp();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            JPE => {
                if self.psw.is_parity() {
                    if self.debug {
                        let addr = self.memory.read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        dbg!("{}JPE {:04X}", code, addr);
                    }
                    self.jmp();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            JP => {
                if !self.psw.is_negative() {
                    if self.debug {
                        let addr = self.memory.read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        dbg!("{}JP {:04X}", code, addr);
                    }
                    self.jmp();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            JM => {
                if self.psw.is_negative() {
                    if self.debug {
                        let addr = self.memory.read_word(self.pc);
                        self.pc = self.pc.wrapping_add(2);
                        let code = self.code_to_str(3);
                        self.pc = self.pc.wrapping_sub(2);
                        dbg!("{}JM {:04X}", code, addr);
                    }
                    self.jmp();
                } else {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            JMP => {
                if self.debug {
                    let addr = self.memory.read_word(self.pc);
                    self.pc = self.pc.wrapping_add(2);
                    let code = self.code_to_str(3);
                    self.pc = self.pc.wrapping_sub(2);
                    dbg!("{}JMP {:04X}", code, addr);
                }
                self.jmp();
            }
            LDA => {
                let addr = self.read_immediate_word();
                self.a = self.memory.read_byte(addr);
                dbg!("{}LDA {:04X}H", self.code_to_str(3), addr);
            }
            LDAX_B => {
                let addr = self.get_bc();
                self.a = self.memory.read_byte(addr);
                dbg!("{}LDAX B", self.code_to_str(1));
            }
            LDAX_D => {
                let addr = self.get_de();
                self.a = self.memory.read_byte(addr);
                dbg!("{}LDAX B", self.code_to_str(1));
            }
            LHLD => {
                let addr = self.read_immediate_word();
                self.l = self.memory.read_byte(addr);
                self.h = self.memory.read_byte(addr + 1);
                dbg!("{}LHLD {:04X}H", self.code_to_str(3), addr);
            }
            LXI_B => {
                let word = self.read_immediate_word();
                self.set_bc(word);
                dbg!("{}LXI B {:04X}H", self.code_to_str(3), word);
            }
            LXI_D => {
                let word = self.read_immediate_word();
                self.set_de(word);
                dbg!("{}LXI D {:04X}H", self.code_to_str(3), word);
            }
            LXI_H => {
                let word = self.read_immediate_word();
                self.set_hl(word);
                dbg!("{}LXI H {:04X}H", self.code_to_str(3), word);
            }
            LXI_SP => {
                self.sp = self.read_immediate_word();
                dbg!("{}LXI SP {:04X}H", self.code_to_str(3), self.sp);
            }
            MVI_A => {
                let value = self.read_immediate_byte();
                self.a = value;
                dbg!("{}MVI A,{:02X}H", self.code_to_str(2), value);
            }
            MVI_B => {
                let value = self.read_immediate_byte();
                self.b = value;
                dbg!("{}MVI B,{:02X}H", self.code_to_str(2), value);
            }
            MVI_C => {
                let value = self.read_immediate_byte();
                self.c = value;
                dbg!("{}MVI C,{:02X}H", self.code_to_str(2), value);
            }
            MVI_D => {
                let value = self.read_immediate_byte();
                self.d = value;
                dbg!("{}MVI D,{:02X}H", self.code_to_str(2), value);
            }
            MVI_E => {
                let value = self.read_immediate_byte();
                self.e = value;
                dbg!("{}MVI E,{:02X}H", self.code_to_str(2), value);
            }
            MVI_H => {
                let value = self.read_immediate_byte();
                self.h = value;
                dbg!("{}MVI H,{:02X}H", self.code_to_str(2), value);
            }
            MVI_L => {
                let value = self.read_immediate_byte();
                self.l = value;
                dbg!("{}MVI L,{:02X}H", self.code_to_str(2), value);
            }
            MVI_M => {
                let addr = self.get_hl();
                let value = self.read_immediate_byte();
                self.memory.write_byte(addr, value);
                dbg!("{}MVI M,{:02X}H", self.code_to_str(2), value);
            }
            MOV_A_B => {
                self.a = self.b;
                dbg!("{}MOV A,B", self.code_to_str(1));
            }
            MOV_A_C => {
                self.a = self.c;
                dbg!("{}MOV A,C", self.code_to_str(1));
            }
            MOV_A_D => {
                self.a = self.d;
                dbg!("{}MOV A,D", self.code_to_str(1));
            }
            MOV_A_E => {
                self.a = self.e;
                dbg!("{}MOV A,E", self.code_to_str(1));
            }
            MOV_A_H => {
                self.a = self.h;
                dbg!("{}MOV A,H", self.code_to_str(1));
            }
            MOV_A_L => {
                self.a = self.l;
                dbg!("{}MOV A,L", self.code_to_str(1));
            }
            MOV_A_M => {
                self.a = self.memory.read_byte(self.get_hl());
                dbg!("{}MOV A,M", self.code_to_str(1));
            }
            MOV_A_A => {
                self.a = self.a;
                dbg!("{}MOV A,A", self.code_to_str(1));
            }
            MOV_B_B => {
                self.b = self.b;
                dbg!("{}MOV B,B", self.code_to_str(1));
            }
            MOV_B_C => {
                self.b = self.c;
                dbg!("{}MOV B,C", self.code_to_str(1));
            }
            MOV_B_D => {
                self.b = self.d;
                dbg!("{}MOV B,D", self.code_to_str(1));
            }
            MOV_B_E => {
                self.b = self.e;
                dbg!("{}MOV B,E", self.code_to_str(1));
            }
            MOV_B_H => {
                self.b = self.h;
                dbg!("{}MOV B,H", self.code_to_str(1));
            }
            MOV_B_L => {
                self.b = self.l;
                dbg!("{}MOV B,L", self.code_to_str(1));
            }
            MOV_B_M => {
                self.b = self.memory.read_byte(self.get_hl());
                dbg!("{}MOV B,M", self.code_to_str(1));
            }
            MOV_B_A => {
                self.b = self.a;
                dbg!("{}MOV B,A", self.code_to_str(1));
            }
            MOV_C_B => {
                self.c = self.b;
                dbg!("{}MOV C,B", self.code_to_str(1));
            }
            MOV_C_C => {
                self.c = self.c;
                dbg!("{}MOV C,C", self.code_to_str(1));
            }
            MOV_C_D => {
                self.c = self.d;
                dbg!("{}MOV C,D", self.code_to_str(1));
            }
            MOV_C_E => {
                self.c = self.e;
                dbg!("{}MOV C,E", self.code_to_str(1));
            }
            MOV_C_H => {
                self.c = self.h;
                dbg!("{}MOV C,H", self.code_to_str(1));
            }
            MOV_C_L => {
                self.c = self.l;
                dbg!("{}MOV C,L", self.code_to_str(1));
            }
            MOV_C_M => {
                self.c = self.memory.read_byte(self.get_hl());
                dbg!("{}MOV C,M", self.code_to_str(1));
            }
            MOV_C_A => {
                self.c = self.a;
                dbg!("{}MOV C,A", self.code_to_str(1));
            }
            MOV_D_B => {
                self.d = self.b;
                dbg!("{}MOV D,B", self.code_to_str(1));
            }
            MOV_D_C => {
                self.d = self.c;
                dbg!("{}MOV D,C", self.code_to_str(1));
            }
            MOV_D_D => {
                self.d = self.d;
                dbg!("{}MOV D,D", self.code_to_str(1));
            }
            MOV_D_E => {
                self.d = self.e;
                dbg!("{}MOV D,E", self.code_to_str(1));
            }
            MOV_D_H => {
                self.d = self.h;
                dbg!("{}MOV D,H", self.code_to_str(1));
            }
            MOV_D_L => {
                self.d = self.l;
                dbg!("{}MOV D,L", self.code_to_str(1));
            }
            MOV_D_M => {
                self.d = self.memory.read_byte(self.get_hl());
                dbg!("{}MOV D,M", self.code_to_str(1));
            }
            MOV_D_A => {
                self.d = self.a;
                dbg!("{}MOV D,A", self.code_to_str(1));
            }
            MOV_E_B => {
                self.e = self.b;
                dbg!("{}MOV E,B", self.code_to_str(1));
            }
            MOV_E_C => {
                self.e = self.c;
                dbg!("{}MOV E,C", self.code_to_str(1));
            }
            MOV_E_D => {
                self.e = self.d;
                dbg!("{}MOV E,D", self.code_to_str(1));
            }
            MOV_E_E => {
                self.e = self.e;
                dbg!("{}MOV E,E", self.code_to_str(1));
            }
            MOV_E_H => {
                self.e = self.h;
                dbg!("{}MOV E,H", self.code_to_str(1));
            }
            MOV_E_L => {
                self.e = self.l;
                dbg!("{}MOV E,L", self.code_to_str(1));
            }
            MOV_E_M => {
                self.e = self.memory.read_byte(self.get_hl());
                dbg!("{}MOV E,M", self.code_to_str(1));
            }
            MOV_E_A => {
                self.e = self.a;
                dbg!("{}MOV E,A", self.code_to_str(1));
            }
            MOV_H_B => {
                self.h = self.b;
                dbg!("{}MOV H,B", self.code_to_str(1));
            }
            MOV_H_C => {
                self.h = self.c;
                dbg!("{}MOV H,C", self.code_to_str(1));
            }
            MOV_H_D => {
                self.h = self.d;
                dbg!("{}MOV H,D", self.code_to_str(1));
            }
            MOV_H_E => {
                self.h = self.e;
                dbg!("{}MOV H,E", self.code_to_str(1));
            }
            MOV_H_H => {
                self.h = self.h;
                dbg!("{}MOV H,H", self.code_to_str(1));
            }
            MOV_H_L => {
                self.h = self.l;
                dbg!("{}MOV H,L", self.code_to_str(1));
            }
            MOV_H_M => {
                self.h = self.memory.read_byte(self.get_hl());
                dbg!("{}MOV H,M", self.code_to_str(1));
            }
            MOV_H_A => {
                self.h = self.a;
                dbg!("{}MOV H,A", self.code_to_str(1));
            }
            MOV_L_B => {
                self.l = self.b;
                dbg!("{}MOV L,B", self.code_to_str(1));
            }
            MOV_L_C => {
                self.l = self.c;
                dbg!("{}MOV L,C", self.code_to_str(1));
            }
            MOV_L_D => {
                self.l = self.d;
                dbg!("{}MOV L,D", self.code_to_str(1));
            }
            MOV_L_E => {
                self.l = self.e;
                dbg!("{}MOV L,E", self.code_to_str(1));
            }
            MOV_L_H => {
                self.l = self.h;
                dbg!("{}MOV L,H", self.code_to_str(1));
            }
            MOV_L_L => {
                self.l = self.l;
                dbg!("{}MOV L,L", self.code_to_str(1));
            }
            MOV_L_M => {
                self.l = self.memory.read_byte(self.get_hl());
                dbg!("{}MOV L,M", self.code_to_str(1));
            }
            MOV_L_A => {
                self.l = self.a;
                dbg!("{}MOV L,A", self.code_to_str(1));
            }
            MOV_M_B => {
                let addr = self.get_hl();
                self.memory.write_byte(addr, self.b);
                dbg!("{}MOV M,B", self.code_to_str(1));
            }
            MOV_M_C => {
                let addr = self.get_hl();
                self.memory.write_byte(addr, self.c);
                dbg!("{}MOV M,C", self.code_to_str(1));
            }
            MOV_M_D => {
                let addr = self.get_hl();
                self.memory.write_byte(addr, self.d);
                dbg!("{}MOV M,D", self.code_to_str(1));
            }
            MOV_M_E => {
                let addr = self.get_hl();
                self.memory.write_byte(addr, self.e);
                dbg!("{}MOV M,E", self.code_to_str(1));
            }
            MOV_M_H => {
                let addr = self.get_hl();
                self.memory.write_byte(addr, self.h);
                dbg!("{}MOV M,H", self.code_to_str(1));
            }
            MOV_M_L => {
                let addr = self.get_hl();
                self.memory.write_byte(addr, self.l);
                dbg!("{}MOV M,L", self.code_to_str(1));
            }
            MOV_M_A => {
                let addr = self.get_hl();
                self.memory.write_byte(addr, self.a);
                dbg!("{}MOV M,A", self.code_to_str(1));
            }

            NOP => {
                dbg!("{}NOP", self.code_to_str(1));
            }
            ORA_B => {
                let value = self.b;
                self.or(value);
                dbg!("{}ORA B", self.code_to_str(1));
            }
            ORA_C => {
                let value = self.c;
                self.or(value);
                dbg!("{}ORA C", self.code_to_str(1));
            }
            ORA_D => {
                let value = self.d;
                self.or(value);
                dbg!("{}ORA D", self.code_to_str(1));
            }
            ORA_E => {
                let value = self.e;
                self.or(value);
                dbg!("{}ORA E", self.code_to_str(1));
            }
            ORA_H => {
                let value = self.h;
                self.or(value);
                dbg!("{}ORA H", self.code_to_str(1));
            }
            ORA_L => {
                let value = self.l;
                self.or(value);
                dbg!("{}ORA L", self.code_to_str(1));
            }
            ORA_M => {
                let value = self.memory.read_byte(self.get_hl());
                self.or(value);
                dbg!("{}ORA M", self.code_to_str(1));
            }
            ORA_A => {
                let value = self.a;
                self.or(value);
                dbg!("{}ORA A", self.code_to_str(1));
            }
            ORI => {
                let value = self.read_immediate_byte();
                self.or(value);
                dbg!("{}ORI {:02X}H", self.code_to_str(2), value);
            }
            PCHL => {
                let hl = self.get_hl();
                self.set_hl(self.pc);
                self.pc = hl;
                dbg!("{}PCHL", self.code_to_str(1));
            }
            POP_B => {
                (self.b, self.c) = self.pop();
                dbg!("{}POP B", self.code_to_str(1));
            }
            POP_D => {
                (self.d, self.e) = self.pop();
                dbg!("{}POP D", self.code_to_str(1));
            }
            POP_H => {
                (self.h, self.l) = self.pop();
                dbg!("{}POP H", self.code_to_str(1));
            }
            POP_PSW => {
                let mut addr = self.sp;
                let value = self.memory.read_byte(addr);
                self.psw.set_negative((value & SIGN) != 0);
                self.psw.set_zero((value & ZERO) != 0);
                self.psw.set_ac((value & AUX_CARRY) != 0);
                self.psw.set_parity((value & PARITY) != 0);
                self.psw.set_carry((value & CARRY) != 0);
                addr = addr.wrapping_add(1);
                self.a = self.memory.read_byte(addr);
                self.sp = self.sp.wrapping_add(2);
                dbg!("{}POP PSW", self.code_to_str(1));
            }
            PUSH_B => {
                self.push(self.b, self.c);
                dbg!("{}PUSH B", self.code_to_str(1));
            }
            PUSH_D => {
                self.push(self.d, self.e);
                dbg!("{}PUSH D", self.code_to_str(1));
            }
            PUSH_H => {
                self.push(self.h, self.l);
                dbg!("{}PUSH H", self.code_to_str(1));
            }
            PUSH_PSW => {
                self.push(self.a, self.psw.value);
                dbg!("{}PUSH B", self.code_to_str(1));
            }
            RAL => {
                let mut val = (self.a as u16) << 1;
                if self.psw.is_carry() {
                    val |= 0b1u16;
                }
                self.psw.set_carry((self.a & 0x80) != 0);
                self.a = val as u8;
                dbg!("{}RAL", self.code_to_str(1));
            }
            RAR => {
                let mut val = (self.a as u16) >> 1;
                if self.psw.is_carry() {
                    val |= 0b1000_0000u16;
                }
                self.psw.set_carry((self.a & 0x01) != 0);
                self.a = val as u8;
                dbg!("{}RAR", self.code_to_str(1));
            }
            RLC => {
                let mut val = (self.a as u16) << 1;
                if self.a & 0x80 != 0 {
                    val |= 0b1u16;
                }
                self.psw.set_carry(self.a & 0x80 != 0);
                self.a = val as u8;
                dbg!("{}RLC", self.code_to_str(1));
            }
            RRC => {
                let mut val = (self.a as u16) >> 1;
                if self.a & 0x01 != 0 {
                    val |= 0b1000_0000u16;
                }
                self.psw.set_carry(self.a & 0x01 != 0);
                self.a = val as u8;
                dbg!("{}RRC", self.code_to_str(1));
            }
            RET => {
                dbg!("{}RET", self.code_to_str(1));
                self.ret();
            }
            RNZ => {
                if !self.psw.is_zero() {
                    dbg!("{}RNZ", self.code_to_str(1));
                    self.ret();
                }
            }
            RZ => {
                if self.psw.is_zero() {
                    dbg!("{}RZ", self.code_to_str(1));
                    self.ret();
                }
            }
            RNC => {
                if !self.psw.is_carry() {
                    dbg!("{}RNC", self.code_to_str(1));
                    self.ret();
                }
            }
            RC => {
                if self.psw.is_carry() {
                    dbg!("{}RC", self.code_to_str(1));
                    self.ret();
                }
            }
            RPO => {
                if !self.psw.is_parity() {
                    dbg!("{}RPO", self.code_to_str(1));
                    self.ret();
                }
            }
            RPE => {
                if self.psw.is_parity() {
                    dbg!("{}RPE", self.code_to_str(1));
                    self.ret();
                }
            }
            RP => {
                if !self.psw.is_negative() {
                    dbg!("{}RP", self.code_to_str(1));
                    self.ret();
                }
            }
            RM => {
                if self.psw.is_negative() {
                    dbg!("{}RM", self.code_to_str(1));
                    self.ret();
                }
            }
            RST_0 => {
                dbg!("{}RST 0", self.code_to_str(1));
                self.rst(0);
            }
            RST_1 => {
                dbg!("{}RST 1", self.code_to_str(1));
                self.rst(1);
            }
            RST_2 => {
                dbg!("{}RST 2", self.code_to_str(1));
                self.rst(2);
            }
            RST_3 => {
                dbg!("{}RST 3", self.code_to_str(1));
                self.rst(3);
            }
            RST_4 => {
                dbg!("{}RST 4", self.code_to_str(1));
                self.rst(4);
            }
            RST_5 => {
                dbg!("{}RST 5", self.code_to_str(1));
                self.rst(5);
            }
            RST_6 => {
                dbg!("{}RST 6", self.code_to_str(1));
                self.rst(6);
            }
            RST_7 => {
                dbg!("{}RST 7", self.code_to_str(1));
                self.rst(7);
            }
            SBB_B => {
                let value = self.b;
                self.sub(value, true);
                dbg!("{}SBB B", self.code_to_str(1));
            }
            SBB_C => {
                let value = self.c;
                self.sub(value, true);
                dbg!("{}SBB C", self.code_to_str(1));
            }
            SBB_D => {
                let value = self.d;
                self.sub(value, true);
                dbg!("{}SBB D", self.code_to_str(1));
            }
            SBB_E => {
                let value = self.e;
                self.sub(value, true);
                dbg!("{}SBB E", self.code_to_str(1));
            }
            SBB_H => {
                let value = self.h;
                self.sub(value, true);
                dbg!("{}SBB H", self.code_to_str(1));
            }
            SBB_L => {
                let value = self.l;
                self.sub(value, true);
                dbg!("{}SBB L", self.code_to_str(1));
            }
            SBB_M => {
                let value = self.get_m();
                self.sub(value, true);
                dbg!("{}SBB M", self.code_to_str(1));
            }
            SBB_A => {
                let value = self.a;
                self.sub(value, true);
                dbg!("{}SBB A", self.code_to_str(1));
            }
            SBI => {
                let value = self.read_immediate_byte();
                self.sub(value, true);
                dbg!("{}SBI {:02X}H", self.code_to_str(2), value);
            }
            SUB_B => {
                let value = self.b;
                self.sub(value, false);
                dbg!("{}SUB B", self.code_to_str(1));
            }
            SUB_C => {
                let value = self.c;
                self.sub(value, false);
                dbg!("{}SUB C", self.code_to_str(1));
            }
            SUB_D => {
                let value = self.d;
                self.sub(value, false);
                dbg!("{}SUB D", self.code_to_str(1));
            }
            SUB_E => {
                let value = self.e;
                self.sub(value, false);
                dbg!("{}SUB E", self.code_to_str(1));
            }
            SUB_H => {
                let value = self.h;
                self.sub(value, false);
                dbg!("{}SUB H", self.code_to_str(1));
            }
            SUB_L => {
                let value = self.l;
                self.sub(value, false);
                dbg!("{}SUB L", self.code_to_str(1));
            }
            SUB_M => {
                let value = self.read_m();
                self.sub(value, false);
                dbg!("{}SUB M", self.code_to_str(1));
            }
            SUB_A => {
                let value = self.a;
                self.sub(value, false);
                dbg!("{}SUB A", self.code_to_str(1));
            }
            SUI => {
                let value = self.read_immediate_byte();
                self.sub(value, false);
                dbg!("{}SUI {:02X}H", self.code_to_str(2), value);
            }
            SHLD => {
                let addr = self.read_immediate_word();
                self.memory.write_byte(addr, self.l);
                self.memory.write_byte(addr.wrapping_add(1), self.h);
                dbg!("{}SHLD {:04X}H", self.code_to_str(3), addr);
            }
            STA => {
                let addr = self.read_immediate_word();
                self.memory.write_byte(addr, self.a);
                dbg!("{}STA {:04X}H", self.code_to_str(3), addr);
            }
            STAX_B => {
                let addr = self.get_bc();
                self.memory.write_byte(addr, self.a);
                dbg!("{}STAX B", self.code_to_str(1));
            }
            STAX_D => {
                let addr = self.get_de();
                self.memory.write_byte(addr, self.a);
                dbg!("{}STAX B", self.code_to_str(1));
            }
            SPHL => {
                self.sp = self.get_hl();
                dbg!("{}SPHL", self.code_to_str(1));
            }
            XCHG => {
                let temp = self.get_hl();
                self.set_hl(self.get_de());
                self.set_de(temp);
                dbg!("{}XCHG", self.code_to_str(1));
            }
            XRA_B => {
                let value = self.b;
                self.xra(value);
                dbg!("{}XRA B", self.code_to_str(1));
            }
            XRA_C => {
                let value = self.c;
                self.xra(value);
                dbg!("{}XRA C", self.code_to_str(1));
            }
            XRA_D => {
                let value = self.d;
                self.xra(value);
                dbg!("{}XRA D", self.code_to_str(1));
            }
            XRA_E => {
                let value = self.e;
                self.xra(value);
                dbg!("{}XRA E", self.code_to_str(1));
            }
            XRA_H => {
                let value = self.h;
                self.xra(value);
                dbg!("{}XRA H", self.code_to_str(1));
            }
            XRA_L => {
                let value = self.l;
                self.xra(value);
                dbg!("{}XRA L", self.code_to_str(1));
            }
            XRA_M => {
                let value = self.get_m();
                self.xra(value);
                dbg!("{}XRA M", self.code_to_str(1));
            }
            XRA_A => {
                let value = self.a;
                self.xra(value);
                dbg!("{}XRA A", self.code_to_str(1));
            }
            XRI => {
                let value = self.read_immediate_byte();
                self.xra(value);
                dbg!("{}XRI {:02X}", self.code_to_str(2), value);
            }
            XTHL => {
                let addr = self.sp;
                let hl = self.get_hl();
                self.l = self.memory.read_byte(addr);
                self.h = self.memory.read_byte(addr+1);
                self.memory.write_word(addr, hl);
                dbg!("{}XTHL", self.code_to_str(1));
            }

            _ => {
                dbg!("{}!byte {:02X}H", self.code_to_str(1), opcode);
            }
        }
    }
}
