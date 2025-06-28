pub const CARRY: u8 = 0b0000_0001;
pub const ZERO: u8 = 0b0000_0010;
pub const INTERRUPT: u8 = 0b0000_0100;
pub const DECIMAL: u8 = 0b0000_1000;
pub const BREAK: u8 = 0b0001_0000;
pub const UNUSED: u8 = 0b0010_0000;
pub const OVERFLOW: u8 = 0b0100_0000;
pub const NEGATIVE: u8 = 0b1000_0000;

#[derive(Default, Debug)]
pub struct Status {
    pub value: u8,
}

impl Status {
    pub fn set_unused(&mut self, val: bool) {
        if val {
            self.value |= UNUSED;
        } else {
            self.value &= !UNUSED;
        }
    }
    pub fn is_unused(&self) -> bool {
        self.value & UNUSED != 0
    }
    pub fn set_break(&mut self, val: bool) {
        if val {
            self.value |= BREAK;
        } else {
            self.value &= !BREAK;
        }
    }
    pub fn is_break(&self) -> bool {
        self.value & BREAK != 0
    }

    pub fn set_negative(&mut self, val: bool) {
        if val {
            self.value |= NEGATIVE;
        } else {
            self.value &= !NEGATIVE;
        }
    }
    pub fn is_negative(&self) -> bool {
        self.value & NEGATIVE != 0
    }

    pub fn set_zero(&mut self, val: bool) {
        if val {
            self.value |= ZERO;
        } else {
            self.value &= !ZERO;
        }
    }
    pub fn is_zero(&self) -> bool {
        self.value & ZERO != 0
    }

    pub fn set_carry(&mut self, val: bool) {
        if val {
            self.value |= CARRY;
        } else {
            self.value &= !CARRY;
        }
    }
    pub fn is_carry(&self) -> bool {
        self.value & CARRY != 0
    }

    pub fn set_interrupt_disable(&mut self, val: bool) {
        if val {
            self.value |= INTERRUPT;
        } else {
            self.value &= !INTERRUPT;
        }
    }
    pub fn is_interrupt_disable(&self) -> bool {
        self.value & INTERRUPT != 0
    }

    pub fn set_decimal_mode(&mut self, val: bool) {
        if val {
            self.value |= DECIMAL;
        } else {
            self.value &= !DECIMAL;
        }
    }
    pub fn is_decimal_mode(&self) -> bool {
        self.value & DECIMAL != 0
    }

    pub fn set_overflow(&mut self, val: bool) {
        if val {
            self.value |= OVERFLOW;
        } else {
            self.value &= !OVERFLOW;
        }
    }
    pub fn is_overflow(&self) -> bool {
        self.value & OVERFLOW != 0
    }
}
