#[derive(Default)]
pub struct Status {
    pub value: u8,
}

impl Status {
    pub fn set_zero(&mut self, val: u8) {
        if val == 0 {
            self.value |= 0b0000_0010;
        } else {
            self.value &= !0b0000_0010;
        }
    }

    pub fn set_negative(&mut self, val: u8) {
        if (val & 0b1000_0000) != 0 {
            self.value |= 0b1000_0000;
        } else {
            self.value &= !0b1000_0000;
        }
    }

    pub fn is_zero(&self) -> bool {
        self.value & 0b0000_0010 != 0
    }

    pub fn is_negative(&self) -> bool {
        self.value & 0b1000_0000 != 0
    }

    pub fn set_carry(&mut self, val: bool) {
        if val {
            self.value |= 0b0000_0001;
        } else {
            self.value &= !0b0000_0001;
        }
    }

    pub fn set_overflow(&mut self, val: bool) {
        if val {
            self.value |= 0b0100_0000;
        } else {
            self.value &= !0b0100_0000;
        }
    }

    pub fn is_carry(&self) -> bool {
        self.value & 0b0000_0001 != 0
    }

    pub fn is_overflow(&self) -> bool {
        self.value & 0b01000_0000 != 0
    }

    pub fn set_decimal_mode(&mut self, val: bool) {
        if val {
            self.value |= 0b0000_1000;
        } else {
            self.value &= !0b0000_1000;
        }
    }

    pub fn is_decimal_mode(&self) -> bool {
        self.value & 0b0000_1000 != 0
    }

    pub fn set_interrupt_disable(&mut self, val: bool) {
        if val {
            self.value |= 0b0000_0100;
        } else {
            self.value &= !0b0000_0100;
        }
    }

    pub fn is_interrupt_disable(&self) -> bool {
        self.value & 0b0000_0100 != 0
    }
}
