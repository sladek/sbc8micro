pub const SIGN: u8 = 0b1000_0000;
pub const ZERO: u8 = 0b0100_0000;
pub const AUX_CARRY: u8 = 0b0001_0000;
pub const PARITY: u8 = 0b0000_0100;
pub const CARRY: u8 = 0b0000_0001;

#[derive(Default, Debug)]
pub struct Psw {
    pub value: u8,
}

impl Psw {
    pub fn new() -> Self {
        Self { value: 0b0000_0010 }
    }
    pub fn set_negative(&mut self, value: bool) {
        if value {
            self.value |= SIGN;
        } else {
            self.value &= !SIGN;
        }
    }
    pub fn set_zero(&mut self, value: bool) {
        if value {
            self.value |= ZERO;
        } else {
            self.value &= !ZERO;
        }
    }
    pub fn set_ac(&mut self, value: bool) {
        if value {
            self.value |= AUX_CARRY;
        } else {
            self.value &= !AUX_CARRY;
        }
    }
    pub fn set_parity(&mut self, value: bool) {
        if value {
            self.value |= PARITY;
        } else {
            self.value &= !PARITY;
        }
    }
    pub fn set_carry(&mut self, value: bool) {
        if value {
            self.value |= CARRY;
        } else {
            self.value &= !CARRY;
        }
    }
    pub fn is_negative(&self) -> bool {
        self.value & SIGN != 0
    }
    pub fn is_zero(&self) -> bool {
        self.value & ZERO != 0
    }
    pub fn is_ac(&self) -> bool {
        self.value & AUX_CARRY != 0
    }
    pub fn is_parity(&self) -> bool {
        self.value & PARITY != 0
    }
    pub fn is_carry(&self) -> bool {
        self.value & CARRY != 0
    }
}
