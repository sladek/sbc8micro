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
    pub fn read_word_zero_page(&self, addr: u8) -> u16 {
        let lo = self.read_byte(addr as u16) as u16;
        let hi = self.read_byte(addr.wrapping_add(1) as u16) as u16;
        (hi << 8) | lo
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
