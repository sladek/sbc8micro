use crate::disassembler::i8080_opcodes_const::*;
use crate::memory::Memory;
use crate::status::i8080::Psw;

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
"Registers\n--------------------------------------------------------------------------------------------------
|  A  |  B  |  C  |  D  |  E  |  H  |  L  |  SP   |  PC   | PSW | S | Z | 0 | AC | 0 | P | 1 | C |
|-----|-----|-----|-----|-----|-----|-----|-------|-------|-----|---|---|---|----|---|---|---|---|
| {:02X}H | {:02X}H | {:02X}H | {:02X}H | {:02X}H | {:02X}H | {:02X}H | {:04X}H | {:04X}H | {:02X}H | {} | {} | 0 | {}  | 0 | {} | 1 | {} |
--------------------------------------------------------------------------------------------------\n",
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.sp,
            self.pc,
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
    fn read_immediate_byte(&mut self) -> u8 {
        let value = self.memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
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
    fn store_bc(&mut self, data: u16) {
        self.b = ((data & 0xff00) >> 8) as u8;
        self.c = (data & 0x0ff)  as u8;
    }
    fn store_de(&mut self, data: u16) {
        self.d = ((data & 0xff00) >> 8) as u8;
        self.e = (data & 0x0ff)  as u8;
    }
    fn store_hl(&mut self, data: u16) {
        self.h = ((data & 0xff00) >> 8) as u8;
        self.l = (data & 0x0ff)  as u8;
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
    fn read_m(&self) -> u8 {
        let h = self.h as u16;
        let l = self.l as u16;
        let hl = (h << 8) | l;
        self.memory.read_byte(hl)
    }
    fn push(&mut self, rph: u8, rpl: u8) {
        let mut addr = self.sp.wrapping_sub(1);
        self.memory.write_byte(addr, rph);
        addr = self.sp.wrapping_sub(2);
        self.memory.write_byte(addr, rpl);
        self.sp = addr;
    }
    fn call(&mut self) {
        let addr = self.memory.read_word(self.pc);
        self.pc = self.pc.wrapping_add(2);
        let pcl = (self.pc & 0xff) as u8;
        let pch = ((self.pc & 0xff00) >> 8) as u8;
        self.push(pch, pcl);
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
        self.a = acch.wrapping_add(accl as u16)as u8;
        if self.a == 0 {
            self.psw.set_zero(true);
        }
        if (self.a & 0x80) != 0 {
            self.psw.set_negative(true);
        }
        self.set_parity(self.a);
    }
    // Add data to HL pair and set CARRY if result is > 0x00ff.
    pub fn dad(&mut self, rp: u16){
        let result: u32 = self.get_hl()as u32 + rp as u32;
        println!("Result: {:08X}", result);
        self.store_hl(result as u16);
        self.psw.set_carry(result > 0x0ffff);
    }
    pub fn step(&mut self) {
        macro_rules! dbg { ($($x:tt)*) => { if self.debug { println!($($x)*); } } }

        let opcode = self.memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);

        match opcode {
            ////////////////// Start of ACI
            ACI => {
                let value = self.read_immediate_byte();
                self.addc(value);
                dbg!("{}ACI {:02X}H", self.code_to_str(2), value);
            }
            ////////////////// End of ACI
            ////////////////// Start of ADC B
            ADC_B => {
                let value = self.b;
                self.addc(value);
                dbg!("{}ADC B", self.code_to_str(1));
            }
            ////////////////// End of ADC B
            ////////////////// Start of ADC C
            ADC_C => {
                let value = self.c;
                self.addc(value);
                dbg!("{}ADC C", self.code_to_str(1));
            }
            ////////////////// End of ADC C
            ////////////////// Start of ADC D
            ADC_D => {
                let value = self.d;
                self.addc(value);
                dbg!("{}ADC D", self.code_to_str(1));
            }
            ////////////////// End of ADC D
            ////////////////// Start of ADC E
            ADC_E => {
                let value = self.e;
                self.addc(value);
                dbg!("{}ADC E", self.code_to_str(1));
            }
            ////////////////// End of ADC E
            ////////////////// Start of ADC H
            ADC_H => {
                let value = self.h;
                self.addc(value);
                dbg!("{}ADC H", self.code_to_str(1));
            }
            ////////////////// End of ADC H
            ////////////////// Start of ADC L
            ADC_L => {
                let value = self.l;
                self.addc(value);
                dbg!("{}ADC L", self.code_to_str(1));
            }
            ////////////////// End of ADC L
            ////////////////// Start of ADC M
            ADC_M => {
                let value = self.read_m();
                self.addc(value);
                dbg!("{}ADC M", self.code_to_str(1));
            }
            ////////////////// End of ADC M
            ////////////////// Start of ADC A
            ADC_A => {
                let value = self.a;
                self.addc(value);
                dbg!("{}ADC A", self.code_to_str(1));
            }
            ////////////////// End of ADC A
            ////////////////// Start of ADD B
            ADD_B => {
                let value = self.b;
                self.add(value, false);
                dbg!("{}ADD B", self.code_to_str(1));
            }
            ////////////////// End of ADD B
            ////////////////// Start of ADD C
            ADD_C => {
                let value = self.c;
                self.add(value, false);
                dbg!("{}ADD C", self.code_to_str(1));
            }
            ////////////////// End of ADD C
            ////////////////// Start of ADD D
            ADD_D => {
                let value = self.d;
                self.add(value, false);
                dbg!("{}ADD D", self.code_to_str(1));
            }
            ////////////////// End of ADD D
            ////////////////// Start of ADD E
            ADD_E => {
                let value = self.e;
                self.add(value, false);
                dbg!("{}ADD E", self.code_to_str(1));
            }
            ////////////////// End of ADD E
            ////////////////// Start of ADD H
            ADD_H => {
                let value = self.h;
                self.add(value, false);
                dbg!("{}ADD H", self.code_to_str(1));
            }
            ////////////////// End of ADD H
            ////////////////// Start of ADD L
            ADD_L => {
                let value = self.l;
                self.add(value, false);
                dbg!("{}ADD L", self.code_to_str(1));
            }
            ////////////////// End of ADD L
            ////////////////// Start of ADD M
            ADD_M => {
                let value = self.read_m();
                self.add(value, false);
                dbg!("{}ADD M", self.code_to_str(1));
            }
            ////////////////// End of ADD M
            ////////////////// Start of ADD A
            ADD_A => {
                let value = self.a;
                self.add(value, false);
                dbg!("{}ADD A", self.code_to_str(1));
            }
            ////////////////// End of ADD A
            ////////////////// Start of ADI
            ADI => {
                let value = self.read_immediate_byte();
                self.add(value, false);
                dbg!("{}ADI {:02X}H", self.code_to_str(2), value);
            }
            ////////////////// End of ADI
            ////////////////// Start of ANA B
            ANA_B => {
                let value = self.b;
                self.and(value);
                dbg!("{}ANA B", self.code_to_str(1));
            }
            ////////////////// End of ANA B
            ////////////////// Start of ANA C
            ANA_C => {
                let value = self.c;
                self.and(value);
                dbg!("{}ANA C", self.code_to_str(1));
            }
            ////////////////// End of ANA C
            ////////////////// Start of ANA D
            ANA_D => {
                let value = self.d;
                self.and(value);
                dbg!("{}ANA D", self.code_to_str(1));
            }
            ////////////////// End of ANA D
            ////////////////// Start of ANA E
            ANA_E => {
                let value = self.e;
                self.and(value);
                dbg!("{}ANA E", self.code_to_str(1));
            }
            ////////////////// End of ANA E
            ////////////////// Start of ANA H
            ANA_H => {
                let value = self.h;
                self.and(value);
                dbg!("{}ANA H", self.code_to_str(1));
            }
            ////////////////// End of ANA H
            ////////////////// Start of ANA L
            ANA_L => {
                let value = self.l;
                self.and(value);
                dbg!("{}ANA L", self.code_to_str(1));
            }
            ////////////////// End of ANA L
            ////////////////// Start of ANA M
            ANA_M => {
                let value = self.read_immediate_byte();
                self.and(value);
                dbg!("{}ANA M", self.code_to_str(1));
            }
            ////////////////// End of ANA M
            ////////////////// Start of ANA A
            ANA_A => {
                let value = self.a;
                self.and(value);
                dbg!("{}ANA A", self.code_to_str(1));
            }
            ////////////////// End of ANA A
            ////////////////// Start of ANI
            ANI => {
                let value = self.read_immediate_byte();
                self.and(value);
                dbg!("{}ANI {:02X}H", self.code_to_str(2), value);
            }
            ////////////////// End of ANI
            ////////////////// Start of CMA
            CMA => {
                self.a = !self.a;
                dbg!("{}CMA", self.code_to_str(1));
            }
            ////////////////// End of CMA
            ////////////////// Start of CMC
            CMC => {
                self.psw.set_carry(!self.psw.is_carry());
                dbg!("{}CMC", self.code_to_str(1));
            }
            ////////////////// End of CMC
            ////////////////// Start of CMP B
            CMP_B => {
                let tmp = self.a;
                self.sub(self.b);
                self.a = tmp;
                dbg!("{}CMP B", self.code_to_str(1));
            }
            ////////////////// End of CMP B
            ////////////////// Start of CMP C
            CMP_C => {
                let tmp = self.a;
                self.sub(self.c);
                self.a = tmp;
                dbg!("{}CMP C", self.code_to_str(1));
            }
            ////////////////// End of CMP C
            ////////////////// Start of CMP D
            CMP_D => {
                let tmp = self.a;
                self.sub(self.d);
                self.a = tmp;
                dbg!("{}CMP D", self.code_to_str(1));
            }
            ////////////////// End of CMP D
            ////////////////// Start of CMP E
            CMP_E => {
                let tmp = self.a;
                self.sub(self.e);
                self.a = tmp;
                dbg!("{}CMP E", self.code_to_str(1));
            }
            ////////////////// End of CMP E
            ////////////////// Start of CMP H
            CMP_H => {
                let tmp = self.a;
                self.sub(self.h);
                self.a = tmp;
                dbg!("{}CMP H", self.code_to_str(1));
            }
            ////////////////// End of CMP H
            ////////////////// Start of CMP L
            CMP_L => {
                let tmp = self.a;
                self.sub(self.l);
                self.a = tmp;
                dbg!("{}CMP L", self.code_to_str(1));
            }
            ////////////////// End of CMP L
            ////////////////// Start of CMP M
            CMP_M => {
                let tmp = self.a;
                self.sub(self.read_m());
                self.a = tmp;
                dbg!("{}CMP M", self.code_to_str(1));
            }
            ////////////////// End of CMP M
            ////////////////// Start of CMP A
            CMP_A => {
                let tmp = self.a;
                self.sub(self.a);
                self.a = tmp;
                dbg!("{}CMP A", self.code_to_str(1));
            }
            ////////////////// End of CMP A
            ////////////////// Start of CPI
            CPI => {
                let tmp = self.a;
                let data = self.read_immediate_byte();
                self.sub(data);
                self.a = tmp;
                dbg!("{}CPI {:02X}", self.code_to_str(2), data);
            }
            ////////////////// End of CPI
            ////////////////// Start of CALL
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
            ////////////////// End of CALL
            ////////////////// Start of CNZ
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
            ////////////////// End of CNZ
            ////////////////// Start of CZ
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
            ////////////////// End of CZ
            ////////////////// Start of CNC
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
            ////////////////// End of CNC
            ////////////////// Start of CC
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
            ////////////////// End of CC
            ////////////////// Start of CPO
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
            ////////////////// End of CPO
            ////////////////// Start of CPE
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
            ////////////////// End of CPE
            ////////////////// Start of CP
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
            ////////////////// End of CP
            ////////////////// Start of CM
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
            ////////////////// End of CM
            ////////////////// Start of DAA
            DAA => {
                self.daa();
                dbg!("{}DAA", self.code_to_str(1));
            }
            ////////////////// End of DAA
            ////////////////// Start of DAD B
            DAD_B => {
                self.dad(self.get_bc());
                dbg!("{}DAD B", self.code_to_str(1));
            }
            ////////////////// End of DAD B
            ////////////////// Start of DAD D
            DAD_D => {
                self.dad(self.get_de());
                dbg!("{}DAD D", self.code_to_str(1));
            }
            ////////////////// End of DAD D
            ////////////////// Start of DAD H
            DAD_H => {
                self.dad(self.get_hl());
                dbg!("{}DAD H", self.code_to_str(1));
            }
            ////////////////// End of DAD H
            ////////////////// Start of DAD SP
            DAD_SP => {
                self.dad(self.sp);
                dbg!("{}DAD H", self.code_to_str(1));
            }
            ////////////////// End of DAD SP

            ////////////////// Start of HLT
            HLT => {
                dbg!("{}HLT", self.code_to_str(1));
            }
            ////////////////// End of HLT
            ////////////////// Start of MVI A
            MVI_A => {
                let value = self.read_immediate_byte();
                self.a = value;
                dbg!("{}MVI A,{:02X}H", self.code_to_str(2), value);
            }
            ////////////////// End of MVI A
            ////////////////// Start of MVI B
            MVI_B => {
                let value = self.read_immediate_byte();
                self.b = value;
                dbg!("{}MVI B,{:02X}H", self.code_to_str(2), value);
            }
            ////////////////// End of MVI B
            ////////////////// Start of MVI C
            MVI_C => {
                let value = self.read_immediate_byte();
                self.c = value;
                dbg!("{}MVI C,{:02X}H", self.code_to_str(2), value);
            }
            ////////////////// End of MVI C
            ////////////////// Start of MVI D
            MVI_D => {
                let value = self.read_immediate_byte();
                self.d = value;
                dbg!("{}MVI D,{:02X}H", self.code_to_str(2), value);
            }
            ////////////////// End of MVI D
            ////////////////// Start of MVI E
            MVI_E => {
                let value = self.read_immediate_byte();
                self.e = value;
                dbg!("{}MVI E,{:02X}H", self.code_to_str(2), value);
            }
            ////////////////// End of MVI E
            ////////////////// Start of MVI H
            MVI_H => {
                let value = self.read_immediate_byte();
                self.h = value;
                dbg!("{}MVI H,{:02X}H", self.code_to_str(2), value);
            }
            ////////////////// End of MVI H
            ////////////////// Start of MVI L
            MVI_L => {
                let value = self.read_immediate_byte();
                self.l = value;
                dbg!("{}MVI L,{:02X}H", self.code_to_str(2), value);
            }
            ////////////////// End of MVI L
            ////////////////// Start of NOP
            NOP => {
                dbg!("{}NOP", self.code_to_str(1));
            }
            ////////////////// End of NOP
            ////////////////// Start of RET
            RET => {
                dbg!("{}RET", self.code_to_str(1));
                self.ret();
            }
            ////////////////// End of RET
            _ => {
                dbg!("{}!byte {:02X}H", self.code_to_str(1), opcode);
            }
        }
    }
}
