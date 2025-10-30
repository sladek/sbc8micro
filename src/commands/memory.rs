use regex::Regex;

use crate::commands::MIN_MEMORY_RANGE;
use crate::commands::cpu_not_set_error;
use crate::ui::app::App;
use crate::ui::app::AppState;

pub struct Memory {}

impl Memory {
    /// Dump command
    ///
    /// Parses "dump" command
    /// Usage:
    ///   dump <start> <length>
    pub fn dump(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        let cpu = &mut app.cpu_ui;
        match cpu {
            Some(cpu) => {
                let dump_range = &app.dump;
                let dump: Vec<String>;
                let start_addr;
                let end_addr;
                match command.len() {
                    1 => {
                        dump = cpu.memory_dump(dump_range.start, dump_range.end);
                    }
                    2 => {
                        let temp_range = app.dump.range - 1;
                        start_addr = Self::from_hex_string(command[1].to_string())?;
                        if (start_addr as u32 + temp_range as u32) > 0xffff {
                            end_addr = 0xffffu16;
                        } else {
                            end_addr = start_addr + temp_range;
                        }
                        dump = cpu.memory_dump(start_addr, end_addr);
                    }
                    3 => {
                        start_addr = Self::from_hex_string(command[1].to_string())?;
                        end_addr = Self::from_hex_string(command[2].to_string())?;
                        if start_addr >= end_addr {
                            return Err("Start address must be lower than end address.".to_string());
                        }
                        app.dump.set_start_address(start_addr);
                        app.dump.set_end_address(end_addr);
                        dump = cpu.memory_dump(start_addr, end_addr);
                    }
                    _ => {
                        return Err(
                            "Wrong number of parameters. Usage: dump or dump <start_addr> <end_addr>"
                                .to_string(),
                    );
                    }
                }
                for line in dump {
                    app.messages.push(line.to_string());
                }
                Ok(AppState::Home)
            }
            None => cpu_not_set_error(),
        }
    }
    /// Translates decimal or hexadecimal numbers representation to u16
    ///
    /// This function translates decimal or hexadecimal numbers to u16.
    /// Hexadecimal numbers can have intel format like 0h - 0ffffh or 0H - 0FFFFH
    /// or format of mos 6502 assembler like $0 - $ffff or $0 - $FFFF
    /// or modern hexadecimal format like 0x0 - 0xffff or 0X0 - 0XFFFF.
    /// And of course a standard numbers can be used (0 - 65535)
    pub fn from_hex_string(mut value: String) -> Result<u16, String> {
        // Regex for checking intel hex format like 0ffffh
        let regex_intel_hex = Regex::new(r"\d[\da-fA-Z]+[h|H]$").unwrap();
        // Regex for modern hex format like 0xfffh
        let regex_rust_hex = Regex::new(r"0[x|X][\da-fA-Z]+$").unwrap();
        // Regex for mos6502 hex format like $hffff
        let regex_6502_hex = Regex::new(r"\$[\da-fA-Z]+$").unwrap();
        // regex for normal number
        let regex_number = Regex::new(r"[\d]+$").unwrap();
        if regex_intel_hex.is_match(&value) {
            let num: &str = if value.starts_with('0') {
                &value[1..value.len() - 1]
            } else {
                &value[..value.len() - 1]
            };
            return Self::from_hex_str_number(num);
        }
        if regex_rust_hex.is_match(&value) {
            let num = &value[2..];
            return Self::from_hex_str_number(num);
        }
        if regex_6502_hex.is_match(&value) {
            let num = &value[1..];
            return Self::from_hex_str_number(num);
        }
        if regex_number.is_match(&value) {
            let res = value.parse::<u16>();
            match res {
                Ok(num) => {
                    return Ok(num);
                }
                Err(err) => {
                    value.push_str(" - ");
                    value.push_str(&err.to_string());
                    value.push_str(". Expected type is u16 (0 - 65535).");
                    return Err(value);
                }
            }
        }
        value.push_str(" - Invalid format of hexadecimal number.");
        Err(value)
    }
    /// Translate hexadecimal string to u16
    fn from_hex_str_number(num: &str) -> Result<u16, String> {
        let input = num.to_uppercase();
        if input.len() > 4 {
            return Err(format!("Hexadecimal number 0x{num} is too long").to_string());
        }
        let mut result: u16 = 0;
        for c in input.as_bytes() {
            let res = Self::from_hex_char(*c);
            result = (result << 4) + res as u16;
        }
        Ok(result)
    }
    /// Translate hexadecimal character to u8
    ///
    /// Expect valid hexadecimal character [0-1][A - F]
    /// with upercase A - F and returns u8 representation
    /// or 0 if it cannot translate it so be carefull
    /// when using it and make sure that hexadecimal character is uppercase
    fn from_hex_char(c: u8) -> u8 {
        if (b'A'..=b'F').contains(&c) {
            return c - b'A' + 10;
        } else if c.is_ascii_digit() {
            return c - b'0';
        }
        0u8
    }
    /// Sets or displays memory range for dump command
    ///
    /// Usage:
    ///   memory_range
    ///   mr
    ///   memory_range 127
    ///   memory_range 0ffh
    ///   mr $ff
    ///   mr 0xff
    pub fn memory_range(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        match command.len() {
            1 => {
                let range = app.dump.range + 1;
                app.messages
                    .push(format!("Memory range: 0x{:04x} [{range}]", range));
            }
            2 => {
                let mut range = Memory::from_hex_string(command[1].to_string())?;
                if range < MIN_MEMORY_RANGE {
                    return Err(format!(
                        "Error: Minimum allowed memory range is {MIN_MEMORY_RANGE}"
                    ));
                }
                range -= 1;
                app.dump.set_range(range);
                let start_address = app.dump.start;
                if (start_address as u32 + range as u32) > 0xff {
                    app.dump.set_end_address(0xffu16);
                }
                app.dump.set_end_address(start_address + range);
            }
            _ => {
                app.messages
                    .push("Error: Wrong number of parameters.".to_string());
                app.messages.push("  Usage: set range <size>.".to_string());
            }
        }
        Ok(AppState::Home)
    }
    /// Sets  memory content
    ///
    /// Usage:
    ///   m 0x1234 0xc3 0x34 0x12
    pub fn set_memory(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        app.check_cpu()?; // Check if cpu is defined
        if command.len() == 1 {
            return Err("Error: Invalid number of parameters. Usage: m <address> <data> <data> <data> ... or mem <address> <data> <data> <data> ...".to_string());
        }
        let mut addr = Memory::from_hex_string(command[1].to_string())?;
        let mut data: Vec<u8> = Vec::new();
        for s_value in &command[2..] {
            let value = Memory::from_hex_string(s_value.to_string())?;
            if value > 0xff {
                return Err(format!(
                    "Error: Value {s_value} [{value}] is bigger than 255. It must fit to 8 bit data."
                ));
            }
            data.push(value as u8);
        }
        match &mut app.cpu_ui {
            Some(cpu) => {
                for value in data {
                    cpu.get_memory().write_byte(addr, value);
                    addr = addr.wrapping_add(1);
                }
            }
            None => {
                return cpu_not_set_error();
            }
        }
        Ok(AppState::Home)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex_char_1() {
        let c = Memory::from_hex_char(b'0');
        assert_eq!(c, 0);
    }
    #[test]
    fn test_from_hex_char_2() {
        let c = Memory::from_hex_char(b'9');
        assert_eq!(c, 9);
    }
    #[test]
    fn test_from_hex_char_3() {
        let c = Memory::from_hex_char(b'A');
        assert_eq!(c, 0xa);
    }
    #[test]
    fn test_from_hex_char_4() {
        let c = Memory::from_hex_char(b'F');
        assert_eq!(c, 0xf);
    }
    #[test]
    fn from_hex_str_number_1() {
        let c: u16 = Memory::from_hex_str_number("ffff").unwrap();
        assert_eq!(c, 0xffff);
    }
    #[test]
    fn from_hex_str_number_2() {
        let c: u16 = Memory::from_hex_str_number("55aa").unwrap();
        assert_eq!(c, 0x55aa);
    }
    #[test]
    fn from_hex_str_number_3() {
        let result = Memory::from_hex_str_number("55aabb");
        // string too long
        assert_eq!(
            result.err(),
            Some("Hexadecimal number 0x55aabb is too long".to_string())
        );
    }
    #[test]
    fn from_hex_string_1() {
        let result = Memory::from_hex_string("0000h".to_string()).unwrap();
        assert_eq!(result, 0x0000);
    }
    #[test]
    fn from_hex_string_2() {
        let result = Memory::from_hex_string("0ffffh".to_string()).unwrap();
        assert_eq!(result, 0xffff);
    }
    #[test]
    fn from_hex_string_3() {
        let result = Memory::from_hex_string("$0".to_string()).unwrap();
        assert_eq!(result, 0x0);
    }
    #[test]
    fn from_hex_string_4() {
        let result = Memory::from_hex_string("$ffff".to_string()).unwrap();
        assert_eq!(result, 0xffff);
    }
    #[test]
    fn from_hex_string_5() {
        let result = Memory::from_hex_string("0x0".to_string()).unwrap();
        assert_eq!(result, 0x0);
    }
    #[test]
    fn from_hex_string_6() {
        let result = Memory::from_hex_string("0xffff".to_string()).unwrap();
        assert_eq!(result, 0xffff);
    }
    #[test]
    fn from_hex_string_7() {
        let result = Memory::from_hex_string("0".to_string()).unwrap();
        assert_eq!(result, 0);
    }
    #[test]
    fn from_hex_string_8() {
        let result = Memory::from_hex_string("65535".to_string()).unwrap();
        assert_eq!(result, 65535);
    }
    #[test]
    fn from_hex_string_9() {
        let result = Memory::from_hex_string("655350".to_string());
        assert_eq!(result.err(), Some("655350 - number too large to fit in target type. Expected type is u16 (0 - 65535).".to_string()));
    }
    #[test]
    fn from_hex_string_10() {
        let result = Memory::from_hex_string("123ef".to_string());
        assert_eq!(
            result.err(),
            Some("123ef - Invalid format of hexadecimal number.".to_string())
        );
    }
}
