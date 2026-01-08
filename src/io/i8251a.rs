//! Limited async mode Intel 8251A serial interface
//!
//! It emulates async mode of Intel 8251A serial interface
use crate::io::IoPort;
use crate::io::serial::Serial;
use crate::io::serial::{Error, ErrorKind};
use serialport::{DataBits, Parity, SerialPortInfo};
use std::collections::VecDeque;
use crate::memory::{MemCell, dma::Dma};
use crate::io::ErrorIndicators;

pub struct Async8251 {
    data: Data,
    status_control: StatusControl,
    mode: u8,
    state: State,
    sync_char_num: u8,
    clock: u32,
    serial: Option<Serial>,
    base_address: Option<u8>,
    memory_base_address: Option<u16>,
    port_offsets: [u8; 2],
    name: Option<String>,
}

const DEFAULT_CAPACITY: usize = 16;
impl Default for Async8251 {
    fn default() -> Self {
        Async8251 {
            data: Data::new(),
            status_control: StatusControl::new(),
            mode: 0,
            state: State::Mode,
            sync_char_num: 0,
            clock: 614_400, // with 64x baud rate factor baud rate us 9600
            serial: None,
            base_address: None,
            memory_base_address: None,
            port_offsets: [0, 1], // 0 - control, 1 - data
            name: None,
        }
    }
}

#[derive(Clone)]
pub enum BaseAddress {
    A8(u8),
    A16(u16),
    NotSet,
}
#[derive(Clone, Default)]
pub struct Data {
    rx_buffer: FiFo<u8>,
    rx_data: u8,
}
impl Data {
    pub fn new() -> Self {
        Self {
            rx_buffer: FiFo::new(DEFAULT_CAPACITY),
            rx_data: 0,
        }
    }
}

#[derive(Clone, Default)]
pub struct StatusControl {
    control: u8, // Defines TxE, RxE, DTR, RTS, ... It is set when writing to Control register
    status: u8,  // Keeps RxRDY, TxRDY, TxEMPTY, ... It read  when reading Control register
}
impl StatusControl {
    pub fn new() -> Self {
        Self {
            control: !(Control::TxEN as u8
                | Control::DTR as u8
                | Control::RxE as u8
                | Control::SBRK as u8
                | Control::ER as u8
                | Control::RTS as u8
                | Control::IR as u8
                | Control::EH as u8),
            // Initialize status. It could be also as 0x0, but this is to satisfy clippy and avoid
            // 'variants .. are never constructed' error.
            status: !(Status::DSR as u8 // First clear all the flags
                | Status::SyndetBrkdet as u8
                | Status::FE as u8
                | Status::OE as u8
                | Status::PE as u8
                | Status::TxEMPTY as u8
                | Status::RxRDY as u8
                | Status::TxRDY as u8)
//                | Status:: RxRDY as u8 // Set some initial flags
                | Status:: TxRDY as u8
                | Status:: TxEMPTY as u8,
        }
    }
}

/// State machine for mode/control/reset configuration.
#[derive(PartialEq, Debug, Clone)]
pub enum State {
    Mode,
    Command,
    SyncChar1,
    SyncChar2,
}
pub enum Mode {
    // BAUD RATE FACTOR
    B1 = 0b0000_0001, // 0 - SYNC MODE | 1 - 1X | 0 - 16X | 1 - 64X
    B2 = 0b0000_0010, // 0             | 0      | 1       | 1
    // CHARACTER LENGTH
    L1 = 0b0000_0100, // 0 - 5 BITS | 1 - 6 BITS | 0 - 7 BITS | 1 - 8 BITS
    L2 = 0b0000_1000, // 0          | 0          | 1          | 1
    // PARITY ENABLE
    PEN = 0b0001_0000, // 1 - ENABLE | 0 - DISABLE
    // EVEN PARITY GENERATION/CHECK
    EP = 0b0010_0000, // 1 - EVEN | 0 - ODD
    // NUMBER OF STOPBITS
    S1 = 0b0100_0000, // 0 - INVALID | 1 - 1 BIT | 0 - 1,5 BIT | 1 - 2 BITS
    S2 = 0b1000_0000, // 0           | 0         | 1           | 1
}

enum SyncMode {
    SyncCharacterSync = 0b1000_0000,
}

pub enum BaudRateFactor {
    SYNC = 0b0000_0000,
    X1 = Mode::B1 as isize,
    X16 = Mode::B2 as isize,
    X64 = (Mode::B1 as u8 | Mode::B2 as u8) as isize,
}

pub enum CharacterLength {
    Five = 0b0000_0000,
    Six = Mode::L1 as isize,
    Seven = Mode::L2 as isize,
    Eight = (Mode::L1 as u8 | Mode::L2 as u8) as isize,
}

#[derive(PartialEq, Debug)]
pub enum StopBits {
    One = Mode::S1 as isize,
    OneAndHalf = Mode::S2 as isize,
    Two = (Mode::S1 as u8 | Mode::S2 as u8) as isize,
    Invald = 0x0,
}

pub enum Control {
    TxEN = 0b0000_0001,
    #[allow(clippy::upper_case_acronyms)]
    DTR = 0b0000_0010,
    RxE = 0b0000_0100,
    #[allow(clippy::upper_case_acronyms)]
    SBRK = 0b0000_1000,
    #[allow(clippy::upper_case_acronyms)]
    ER = 0b0001_0000,
    #[allow(clippy::upper_case_acronyms)]
    RTS = 0b0010_0000,
    #[allow(clippy::upper_case_acronyms)]
    IR = 0b0100_0000,
    #[allow(clippy::upper_case_acronyms)]
    EH = 0b1000_0000,
}
pub enum Status {
    TxRDY = 0b0000_0001,   // Output buffer is ready to receive a character from CPU
    RxRDY = 0b0000_0010,   // Character is ready in an input buffer
    TxEMPTY = 0b0000_0100, // Output buffer is empty
    #[allow(clippy::upper_case_acronyms)]
    PE = 0b0000_1000,
    #[allow(clippy::upper_case_acronyms)]
    OE = 0b0001_0000, // Overrun Error
    #[allow(clippy::upper_case_acronyms)]
    FE = 0b0010_0000,
    SyndetBrkdet = 0b0100_0000,
    #[allow(clippy::upper_case_acronyms)]
    DSR = 0b1000_0000,
}
#[derive(Default, Clone)]
struct FiFo<T> {
    buf: VecDeque<T>,
    capacity: usize,
}
impl<T> FiFo<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: VecDeque::with_capacity(capacity),
            capacity,
        }
    }
    fn push(&mut self, item: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::new(ErrorKind::BufferOverflow, "Buffer overflow"));
        }
        self.buf.push_back(item);
        Ok(())
    }
    fn pop(&mut self) -> Option<T> {
        if self.buf.is_empty() {
            return None;
        }
        self.buf.pop_front()
    }
    fn is_full(&self) -> bool {
        self.buf.len() == self.capacity
    }
}
impl Async8251 {
    /// Creates a new instance of serial port
    ///
    /// Returns a new instance of serial port, but it doesn't actually open the port
    /// it only initialises the basic fields of the structure with clock field filled in so
    /// the baud rate can be set later.
    pub fn new() -> Self {
        Self {
            mode: (StopBits::One as u8)
                | (CharacterLength::Eight as u8)
                | (BaudRateFactor::X64 as u8), // 8 bit, 1 stop bit, parity disabled,
            clock: 614_400,
            state: State::Mode,
            sync_char_num: 0, // Part of mode state machine
            serial: Some(Serial::new()),
            base_address: None,
            memory_base_address: None,
            ..Default::default()
        }
    }
    /// Reads a byte from rx_data
    ///
    /// Reads a byte from rx_data and returns new data if data is available
    /// or old data if buffer is empty
    pub fn read_rx_data(&mut self) -> u8 {
        // Check if Rx is enabled
        if self.status_control.control & Control::RxE as u8 == 0x0 {
            return self.data.rx_data;
        };
        // Rx is enabled. If RxRDY is set , clear ir and return rx_data.
        if (self.status_control.status & Status::RxRDY as u8) != 0 {
            self.status_control.status &= !(Status::RxRDY as u8);
            return self.data.rx_data;
        }
        // Rx_RDY is not set, check the rx_buffer
        if let Some(data) = self.data.rx_buffer.pop() {
            self.data.rx_data = data;
            return self.data.rx_data;
        }
        // Buffer is empty, let's try to read data from serial port
        self.read_serial_to_rx_buffer();
        self.data.rx_data
    }
    /// Reads a status of USART
    ///
    /// If RxRDY is set just returns staus or reads data from serial port and sets status flags accordingly.
    pub fn read_status(&mut self) -> u8 {
        if self.serial.is_none() {
            self.status_control.status = 0xff; // USART is not present se return 0xff as in real situation
            return self.status_control.status;
        }
        // Check RxRDY
        if (self.status_control.status & Status::RxRDY as u8) != 0 {
            // If RxRDY is true return status
            return self.status_control.status;
        }
        // Else check buffer
        self.read_serial_to_rx_buffer();
        self.status_control.status
    }
    /// Reads data from serial port and pushes it to the rx_buffer and set status flags accordingly
    fn read_serial_to_rx_buffer(&mut self) {
        if let Some(port) = self.serial.as_mut() {
            self.status_control.status &= !(Status::RxRDY as u8); // Clear RxRDY
            while let Some(data) = port.read_data() {
                if self.data.rx_buffer.push(data).is_err() {
                    self.status_control.status |= Status::OE as u8; // Buffer overflow
                    break;
                }
            }
            if let Some(data) = self.data.rx_buffer.pop() {
                self.status_control.status |= Status::RxRDY as u8; // Set RxRDY
                self.data.rx_data = data
            }
        };
    }
    /// Writes data to serial port
    pub fn write_tx_data(&mut self, data: u8) {
        if (self.status_control.control & Control::TxEN as u8) == 0x0 {
            return;
        }
        if let Some(port) = self.serial.as_mut() {
            _ = port.write_data(data);
            self.status_control.status |= Status::TxEMPTY as u8;
            self.status_control.status |= Status::TxRDY as u8;
        }
    }
    /// Gets names of available serial ports
    pub fn get_available_ports() -> Option<Vec<SerialPortInfo>> {
        serialport::available_ports().ok()
    }
    /// Sets baud_rate of serial port
    pub fn set_baud_rate_factor(&mut self, baud_rate_factor: BaudRateFactor) {
        self.mode &= !(BaudRateFactor::X64 as u8);
        self.mode |= baud_rate_factor as u8;
    }
    /// Calculates baud rate factor from mode register (1x, 16x, 64x)
    fn get_baud_rate_factor(&self) -> u32 {
        match (self.mode & (Mode::B1 as u8 | Mode::B2 as u8)) as isize {
            value if value == BaudRateFactor::X1 as isize => 1,
            value if value == BaudRateFactor::X16 as isize => 16,
            value if value == BaudRateFactor::X64 as isize => 64,
            _ => 0,
        }
    }
    /// Calculates and returns baud rate
    ///
    /// Calculates baud rate from clock and baudrate factor
    pub fn get_baud_rate(&self) -> u32 {
        let baud_rate_factor = self.get_baud_rate_factor();
        if baud_rate_factor == 0 {
            self.clock
        } else {
            self.clock / baud_rate_factor
        }
    }
    /// Sets clock frequency. Default is 614400 Hz
    pub fn set_clock(&mut self, clock: u32) {
        self.clock = clock;
    }
    /// Gets clock frequency (Hz)
    pub fn get_clock(&self) -> u32 {
        self.clock
    }
    /// Sets lebgth of the character
    pub fn set_character_length(&mut self, character_length: CharacterLength) {
        self.mode &= !(CharacterLength::Eight as u8); // Mask character length bits
        self.mode |= character_length as u8; // Set new bits
    }
    /// Gets length of the character
    pub fn get_character_length(&self) -> u8 {
        match (self.mode & (Mode::L1 as u8 | Mode::L2 as u8)) as isize {
            value if value == CharacterLength::Six as isize => 6,
            value if value == CharacterLength::Seven as isize => 7,
            value if value == CharacterLength::Eight as isize => 8,
            _ => 5,
        }
    }
    /// Gets DataBits
    fn get_data_bits(&self) -> DataBits {
        match self.get_character_length() {
            5 => DataBits::Five,
            6 => DataBits::Six,
            7 => DataBits::Seven,
            _ => DataBits::Eight,
        }
    }
    /// Gets parity enable flag
    pub fn get_parity_enable(&self) -> bool {
        (self.mode & Mode::PEN as u8) != 0
    }
    /// Sets PEN flag
    pub fn set_parity_enable(&mut self, pen: bool) {
        if pen {
            self.mode |= Mode::PEN as u8;
        } else {
            self.mode &= !(Mode::PEN as u8);
        }
    }
    /// Gets parity even flag
    pub fn is_even_parity(&self) -> bool {
        (self.mode & Mode::EP as u8) != 0
    }
    /// Sets even parity
    pub fn set_even_parity(&mut self, ep: bool) {
        if ep {
            self.mode |= Mode::EP as u8;
        } else {
            self.mode &= !(Mode::EP as u8);
        }
    }
    /// Gets number of stop bits
    pub fn get_stop_bits(&self) -> StopBits {
        match (self.mode & (Mode::S1 as u8 | Mode::S2 as u8)) as isize {
            value if value == StopBits::One as isize => StopBits::One,
            value if value == StopBits::OneAndHalf as isize => StopBits::OneAndHalf,
            value if value == StopBits::Two as isize => StopBits::Two,
            _ => StopBits::Invald,
        }
    }
    /// Sets stop bits
    pub fn set_stop_bits(&mut self, stop_bits: StopBits) {
        self.mode &= !(StopBits::Two as u8); // Remove stop bits
        match stop_bits {
            StopBits::One => self.mode |= StopBits::One as u8,
            StopBits::OneAndHalf => self.mode |= StopBits::OneAndHalf as u8,
            StopBits::Two => self.mode |= StopBits::Two as u8,
            _ => (),
        }
    }
    /// Plain set mode
    fn set_mode(&mut self, mode: u8) {
        self.mode = mode;
    }
    /// Gets port configuration
    /// Opens new serial port based on parameters defined in Self structure
    pub fn open_port(mut self, port_name: &str) -> Result<Self, serialport::Error> {
        let stop_bits = match self.get_stop_bits() {
            StopBits::One => serialport::StopBits::One,
            _ => serialport::StopBits::Two,
        };
        let parity = match self.get_parity_enable() {
            true => {
                if self.is_even_parity() {
                    Parity::Even
                } else {
                    Parity::Odd
                }
            }
            false => Parity::None,
        };
        let baud_rate = self.get_baud_rate();
        let data_bits = self.get_data_bits();
        let serial: &mut Serial = match &mut self.serial {
            Some(port) => port,
            None => {
                return Err(serialport::Error {
                    description: "Port is not set".to_string(),
                    kind: serialport::ErrorKind::Unknown,
                });
            }
        };
        match serial.open(port_name, baud_rate, data_bits, stop_bits, parity) {
            Ok(port) => {
                _ = port.start();
                Ok(self)
            }
            Err(err) => Err(err)?,
        }
    }

    /// Writes to control register
    pub fn write_to_control(&mut self, data: u8) {
        match self.state {
            State::Mode => {
                match data & (Mode::B1 as u8 | Mode::B2 as u8) {
                    0 => {
                        // Sync mode
                        match data & SyncMode::SyncCharacterSync as u8 {
                            0 => {
                                self.state = State::SyncChar2;
                            }
                            _ => {
                                self.state = State::SyncChar1;
                            }
                        }
                    }
                    _ => {
                        // Async mode so we can write parameters for async mode
                        self.set_mode(data);
                        self.state = State::Command;
                    }
                }
            }
            State::SyncChar1 => {
                self.state = State::Command;
            }
            State::SyncChar2 => {
                self.sync_char_num += 1;
                if self.sync_char_num >= 2 {
                    self.state = State::Command; // Second sync character received
                }
            }
            State::Command => {
                if data == 0x40 {
                    // Reset command
                    self.sync_char_num = 0;
                    self.status_control.control = 0;
                    self.state = State::Mode;
                } else {
                    self.status_control.control = data;
                    if data & Status::PE as u8 != 0 {
                        self.status_control.status &= !(Status::PE as u8);
                    }
                    if data & Status::OE as u8 != 0 {
                        self.status_control.status &= !(Status::OE as u8);
                    }
                    if data & Status::FE as u8 != 0 {
                        self.status_control.status &= !(Status::FE as u8);
                    }
                }
            }
        }
    }
    /// Sets base address of serial port (8251).
    ///
    /// Base address is data port for 8251
    /// Base address + 1 is address of controll and status port of 8251
    pub fn set_base_address(&mut self, address: u8) {
        self.memory_base_address = None;
        self.base_address = Some(address)
    }
    /// Sets base address of serial port (8251) mapped to memory (16 bits).
    ///
    /// Base address is data port for 8251
    /// Base address + 1 is address of controll and status port of 8251    
    pub fn set_memory_base_address(&mut self, address: u16) {
        self.base_address = None;
        self.memory_base_address = Some(address)
    }
    /// Gets base address of serial port (8251)
    pub fn get_base_address(&self) -> Option<u8> {
        self.base_address
    }
    /// Gets base address of serial port (8251) mapped to memory (16 bits)
    pub fn get_memory_base_address(&self) -> Option<u16> {
        self.memory_base_address
    }
    /// Sets name of the serial port.   
    ///
    /// Sets name of the serial port as COM3, /dev/tty3 ...
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
    /// Gets name of the serial port.
    ///
    /// Gets name of the serial port as COM3, /dev/tty3 ...
    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }
}
impl IoPort for Async8251 {
    fn write_to_address(&mut self, memory: &mut [MemCell], address: u8, data: u8) -> Result<Option<Dma>, ErrorIndicators> {
        if let Some(base_address) = self.base_address {
            if address == base_address {
                self.write_tx_data(data);
                return Ok(None);
            };
            if address == base_address + 1 {
                self.write_to_control(data);
            }
        }
        return Ok(None)
    }
    fn write_to_memory_address(&mut self, _memory: &mut [MemCell], address: u16, data: u8) -> Result<Option<Dma>, ErrorIndicators> {
        if let Some(base_address) = self.memory_base_address {
            if address == base_address {
                self.write_tx_data(data);
                return Ok(None);
            }
            if address == base_address + 1 {
                self.write_to_control(data);
            }
        }
        return Ok(None)
    }
    fn read_from_address(&mut self, address: u8) -> Option<u8> {
        if let Some(base_address) = self.base_address {
            if address == base_address {
                return Some(self.read_rx_data());
            }
            if address == base_address + 1 {
                return Some(self.read_status());
            }
        }
        None
    }
    fn read_from_mem_address(&mut self, address: u16) -> Option<u8> {
        if let Some(base_address) = self.memory_base_address {
            if address == base_address {
                return Some(self.read_rx_data());
            }
            if address == base_address + 1 {
                return Some(self.read_status());
            }
        }
        None
    }
    fn get_ports_offset(&self) -> &[u8] {
        &self.port_offsets
    }
    fn get_base_address(&self) -> Option<u8> {
        self.base_address
    }
    fn get_memory_base_address(&self) -> Option<u16> {
        self.memory_base_address
    }
    fn get_io_port_info(&self) -> String {
        let base_address = match (self.get_base_address(), self.get_memory_base_address()) {
            (Some(address), None) => {
                format!("0x{:02X}", address)
            }
            (None, Some(address)) => {
                format!("M0x{:04X}", address)
            }
            _ => "Not defined".to_string(),
        };
        let name = match self.get_name() {
            Some(name) => name,
            None => "Not defined".to_string(),
        };
        let parity = if self.get_parity_enable() {
            if self.is_even_parity() { "even" } else { "odd" }
        } else {
            "none"
        };
        let stop_bits = match self.get_stop_bits() {
            StopBits::One => "one",
            StopBits::OneAndHalf => "one and half",
            StopBits::Two => "two",
            _ => "invalid",
        };
        format!(
            "Serial port: base address[{base_address}], name[{name}], baud rate[{}], character length[{}], parity[{}], stop bits[{}]",
            self.get_baud_rate(),
            self.get_character_length(),
            parity,
            stop_bits
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::io::i8251a::{Async8251, BaudRateFactor, CharacterLength, FiFo, State, StopBits};

    #[test]
    /// Baudrete for SYNC mode (the same as clock)
    fn test_baud_rate_sync() {
        let mut serial = Async8251::new();
        serial.set_baud_rate_factor(super::BaudRateFactor::SYNC);
        assert_eq!(614400, serial.get_baud_rate());
    }
    #[test]
    /// Baudrete for async mode 1x
    fn test_baud_rate_x1() {
        let mut serial = Async8251::new();
        serial.set_baud_rate_factor(super::BaudRateFactor::X64);
        serial.set_baud_rate_factor(super::BaudRateFactor::X1);
        assert_eq!(614400, serial.get_baud_rate());
    }
    #[test]
    /// Baudrete for async mode 16x
    fn test_baud_rate_x16() {
        let mut serial = Async8251::new();
        serial.set_baud_rate_factor(super::BaudRateFactor::X1);
        serial.set_baud_rate_factor(super::BaudRateFactor::X16);
        assert_eq!(38_400, serial.get_baud_rate());
    }
    #[test]
    /// Baudrete for async mode 64x
    fn test_baud_rate_x64() {
        let mut serial = Async8251::new();
        serial.set_baud_rate_factor(super::BaudRateFactor::X64);
        assert_eq!(9600, serial.get_baud_rate());
    }
    #[test]
    /// Sets 5 bit
    fn test_set_bits_5() {
        let mut serial = Async8251::new();
        serial.set_character_length(CharacterLength::Eight);
        serial.set_character_length(CharacterLength::Five);
        assert_eq!(0x43, serial.mode);
    }
    #[test]
    /// Gets 5 bit
    fn test_get_bits_5() {
        let mut serial = Async8251::new();
        serial.set_character_length(CharacterLength::Eight);
        serial.set_character_length(CharacterLength::Five);
        assert_eq!(5, serial.get_character_length());
    }
    #[test]
    /// Sets 6 bit
    fn test_set_bits_6() {
        let mut serial = Async8251::new();
        serial.set_character_length(CharacterLength::Eight);
        serial.set_character_length(CharacterLength::Six);
        assert_eq!(0x47, serial.mode);
    }
    #[test]
    /// Gets 5 bit
    fn test_get_bits_6() {
        let mut serial = Async8251::new();
        serial.set_character_length(CharacterLength::Eight);
        serial.set_character_length(CharacterLength::Six);
        assert_eq!(6, serial.get_character_length());
    }
    #[test]
    /// Sets 7 bit
    fn test_set_bits_7() {
        let mut serial = Async8251::new();
        serial.set_character_length(CharacterLength::Eight);
        serial.set_character_length(CharacterLength::Seven);
        assert_eq!(0x4b, serial.mode);
    }
    #[test]
    /// Gets 7 bit
    fn test_get_bits_7() {
        let mut serial = Async8251::new();
        serial.set_character_length(CharacterLength::Eight);
        serial.set_character_length(CharacterLength::Seven);
        assert_eq!(7, serial.get_character_length());
    }
    #[test]
    /// Sets 8 bit
    fn test_set_bits_8() {
        let mut serial = Async8251::new();
        serial.set_character_length(CharacterLength::Eight);
        assert_eq!(0x4f, serial.mode);
    }
    #[test]
    /// Gets 8 bit
    fn test_get_bits_8() {
        let mut serial = Async8251::new();
        serial.set_character_length(CharacterLength::Eight);
        assert_eq!(8, serial.get_character_length());
    }
    #[test]
    /// Test set PEN
    fn test_set_pen_0() {
        let mut serial = Async8251::new();
        serial.set_parity_enable(false);
        assert_eq!(0x4f, serial.mode);
    }
    #[test]
    /// Test set PEN
    fn test_set_pen_1() {
        let mut serial = Async8251::new();
        serial.set_parity_enable(true);
        assert_eq!(0x5f, serial.mode);
    }
    #[test]
    /// Test set PEN
    fn test_set_pe_1() {
        let mut serial = Async8251::new();
        serial.set_even_parity(true);
        assert_eq!(0x6f, serial.mode);
        assert_eq!(true, serial.is_even_parity())
    }
    #[test]
    /// Test set PEN
    fn test_set_pe_0() {
        let mut serial = Async8251::new();
        serial.set_even_parity(false);
        assert_eq!(0x4f, serial.mode);
        assert_eq!(false, serial.is_even_parity())
    }
    #[test]
    /// Test set stop bits
    fn test_set_stop_1() {
        let mut serial = Async8251::new();
        serial.set_stop_bits(StopBits::One);
        assert_eq!(0x4f, serial.mode);
        assert_eq!(StopBits::One, serial.get_stop_bits())
    }
    #[test]
    /// Test set stop bits
    fn test_set_stop_1_5() {
        let mut serial = Async8251::new();
        serial.set_stop_bits(StopBits::OneAndHalf);
        println!("Stop bits: {:?}", StopBits::OneAndHalf);
        assert_eq!(0x8f, serial.mode);
        assert_eq!(StopBits::OneAndHalf, serial.get_stop_bits())
    }
    #[test]
    /// Test set stop bits
    fn test_set_stop_2() {
        let mut serial = Async8251::new();
        serial.set_stop_bits(StopBits::Two);
        assert_eq!(0xcf, serial.mode);
        assert_eq!(StopBits::Two, serial.get_stop_bits())
    }
    #[test]
    /// Test set stop bits
    fn test_fifo() {
        let mut fifo = FiFo::new(3);
        assert!(fifo.buf.is_empty()); // Fifo empty again
        _ = fifo.push(0x99);
        _ = fifo.push(0x55);
        _ = fifo.push(0xaa);
        assert!(fifo.is_full()); // Fifo full
        let res = fifo.push(0xff); // This should fail
        assert!(res.is_err());
        let mut value = fifo.pop().unwrap();
        assert_eq!(0x99, value);
        value = fifo.pop().unwrap();
        assert_eq!(0x55, value);
        value = fifo.pop().unwrap();
        assert_eq!(0xaa, value);
        assert!(fifo.buf.is_empty()); // Fifo empty again
        let value = fifo.pop(); // Reading from empty buffer
        assert_eq!(None, value);
    }
    #[test]
    fn test_soft_reset() {
        let mut serial = Async8251::new();
        serial.write_to_control(0); // Write synchronous mode with 2 sync characters
        assert_eq!(State::SyncChar2, serial.state);
        serial.write_to_control(0); // Write first sync character
        assert_eq!(State::SyncChar2, serial.state);
        serial.write_to_control(0); // Write second sync character
        assert_eq!(State::Command, serial.state);
        serial.write_to_control(0x40); // Write Reset
        assert_eq!(State::Mode, serial.state);
    }
    #[test]
    fn test_port_parameters() {
        let mut serial = Async8251::new();
        serial.write_to_control(0); // Write synchronous mode with 2 sync characters
        assert_eq!(State::SyncChar2, serial.state);
        serial.write_to_control(0); // Write first sync character
        assert_eq!(State::SyncChar2, serial.state);
        serial.write_to_control(0); // Write second sync character
        assert_eq!(State::Command, serial.state);
        serial.write_to_control(0x40); // Write Reset
        assert_eq!(State::Mode, serial.state);
        let data = CharacterLength::Eight as u8 | StopBits::One as u8 | BaudRateFactor::X64 as u8;
        serial.write_to_control(data);
        assert_eq!(0x4f, serial.mode);
    }
}
