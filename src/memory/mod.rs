const CAPACITY: usize = 0x10000;

pub struct Memory {
    data: [u8; CAPACITY], // 64KB
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: [0; CAPACITY],
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let lo = self.read_byte(addr) as u16;
        let hi = self.read_byte(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    pub fn write_word(&mut self, addr: u16, value: u16) {
        self.write_byte(addr, (value & 0xFF) as u8);
        self.write_byte(addr.wrapping_add(1), (value >> 8) as u8);
    }
    pub fn read_byte_zero_page(&self, addr: u8) -> u8 {
        self.read_byte(addr as u16)
    }
    pub fn write_byte_zero_page(&mut self, addr: u8, value: u8) {
        self.write_byte(addr as u16, value);
    }
    pub fn read_word_zero_page(&self, addr: u8) -> u16 {
        let lo = self.read_byte(addr as u16) as u16;
        let hi = self.read_byte(addr.wrapping_add(1) as u16) as u16;
        (hi << 8) | lo
    }
    pub fn write_word_zero_page(&mut self, addr: u8, value: u16) {
        self.write_word(addr as u16, value);
    }
    pub fn load_program(&mut self, program: &[u8], start_addr: u16) {
        for (i, &byte) in program.iter().enumerate() {
            self.write_byte(start_addr + i as u16, byte);
        }
    }
    pub fn get_data(&self) -> [u8; CAPACITY] {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::{self, Memory};
    #[test]
    ///
    /// Writes and reads back byte from memory
    ///
    fn write_read_byte() {
        let mut memory = Memory::new();
        let addr = 0x0100u16;
        let value = 0x55u8;
        memory.write_byte(addr, value);
        let result = memory.read_byte(addr);
        assert_eq! {result, value};
    }
    #[test]
    ///
    /// Writes and reads back word from memory
    ///
    fn write_read_word() {
        let mut memory = Memory::new();
        let addr = 0x0100u16;
        let value = 0x55AAu16;
        memory.write_word(addr, value);
        let result = memory.read_word(addr);
        assert_eq!(result, value);
    }
    #[test]
    ///
    /// Writes word and reads back bytes from memory
    /// to verify proper order
    ///
    fn write_word_read_bytes() {
        let mut memory = Memory::new();
        let addr = 0x0100u16;
        let value = 0x55AAu16;
        memory.write_word(addr, value);
        let byte0 = memory.read_byte(addr) as u16;
        let byte1 = memory.read_byte(addr.wrapping_add(1)) as u16;
        let result: u16 = byte1 << 8 | byte0;
        assert_eq!(result, value);
    }
    #[test]
    ///
    /// Writes bytes and reads back word from memory
    /// to verify proper order
    ///
    fn write_bytes_read_word() {
        let mut memory = Memory::new();
        let addr = 0x0100u16;
        let value0 = 0x55u16;
        let value1 = 0xAAu16;
        memory.write_byte(addr, value0 as u8);
        memory.write_byte(addr.wrapping_add(1), value1 as u8);
        let result = memory.read_word(addr);
        let value = value1 << 8 | value0;
        assert_eq!(result, value);
    }
    #[test]
    ///
    /// Writes word to the end of memory (0xffff)
    /// to verify wrapping to the beginning (0x0000)
    ///
    fn write_word_end_of_memory() {
        let mut memory = Memory::new();
        let addr = 0xFFFFu16;
        let value = 0x55AAu16;
        memory.write_word(addr, value);
        let result1 = memory.read_byte(addr) as u16;
        let result2 = memory.read_byte(0x0) as u16;
        let result = result2 << 8 | result1;
        assert_eq!(result, value);
    }
    #[test]
    ///
    /// Writes byte to the zero page and then reads it back
    ///
    fn read_byte_zero_page() {
        let mut memory = Memory::new();
        let addr = 0x0010u16;
        let value = 0x55u8;
        memory.write_byte(addr, value);
        let result = memory.read_byte_zero_page(addr as u8);
        assert_eq!(result, value);
    }

    #[test]
    ///
    /// Writes word to the zero page and then reads it back
    ///
    fn read_word_zero_page() {
        let mut memory = Memory::new();
        let addr = 0x0010u16;
        let value = 0x55AAu16;
        memory.write_word(addr, value);
        let result = memory.read_word_zero_page(addr as u8);
        assert_eq!(result, value);
    }
    #[test]
    ///
    /// Writes word to the end of zero page (0xff)
    /// to verify wrapping to the beginning (0x0000)
    ///
    fn read_word_zero_page_wrap_around() {
        let mut memory = Memory::new();
        let addr = 0xFFu8;
        let value = 0x55AAu16;
        memory.write_byte(addr as u16, (value & 0x00ff) as u8);
        memory.write_byte(0x0000u16, ((value & 0xff00) >> 8) as u8);
        let result = memory.read_word_zero_page(addr);
        assert_eq!(result, value);
    }
    #[test]
    ///
    /// Load a short probram and verifies if it is written properly
    ///
    fn load_program() {
        let mut memory = memory::Memory::new();
        let program = vec![
            0xA9, 0x01, // LDA #$01
            0x8D, 0x00, 0x02, // STA $0200
            0xE8, // INX
            0xF0, 0xFD, // BEQ $0600
            0x00, // BRK
        ];
        let start = 0x0000;
        memory.load_program(&program, start);
        let mem_slice = &memory.data[0..=8];
        assert_eq!(mem_slice, program);
    }
}
