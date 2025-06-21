const CARRY: u8 = 0b0000_0001;
const ZERO: u8 = 0b0000_0010;
const INTERRUPT: u8 = 0b0000_0100;
const DECIMAL: u8 = 0b0000_1000;
//const BREAK: u8 = 0b0001_0000;
//const UNUSED: u8 = 0b0010_0000;
const OVERFLOW: u8 = 0b0100_0000;
const NEGATIVE: u8 = 0b1000_0000;

#[derive(Default, Debug)]
pub struct Status {
    pub value: u8,
}

impl Status {
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
