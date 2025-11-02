use crate::memory::MemCell;
use crate::ui::app::{AppState, App};
use crate::commands::memory::Memory;

enum IoAddress {
    Io(u8),
    Mem(u16),
}
pub struct Dev;

impl Dev {
    pub fn list_devices(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        app.is_cpu_set()?; // Check if cpu is defined
        if let Some(cpu) = &mut app.cpu_ui {
            match command.len() {
                1 => {
                    let mut mapped = false;
                    let io_info = match cpu.get_io_memory() {
                        Some(io_memory) => {
                            io_memory.get_io_ports_info()
                        }
                        None => {
                            Vec::new() // Just a fake empty memory if it is not supported by CPU (like 6502)
                        }
                    };
                    let mem_io_info = cpu.get_memory().get_io_ports_info();
                    if !io_info.is_empty() {
                        mapped = true;
                        app.messages.push("Io mapped:".to_string());
                        Self::push_info(app, io_info);
                    }
                    if !mem_io_info.is_empty() {
                        mapped = true;
                        app.messages.push("Memory mapped:".to_string());
                        Self::push_info(app, mem_io_info);
                    }
                    if !mapped {
                        app.messages.push("No device mapped.".to_string());
                    }
                }
                2 => {
                    let address = Self::parse_address(command[1].to_string())?;
                    match address {
                        IoAddress::Io(address) => {
                            match cpu.get_io_memory() {
                                Some(memory) => {
                                    let mem = memory.get_port_map();
                                    match mem[address as usize] {
                                        Some(port_address) => {
                                            match memory.get_ports().get(&port_address) {
                                                Some(port) => {
                                                    app.messages.push(port.get_io_port_info());
                                                    return Ok(AppState::Home);
                                                }
                                                None => {
                                                    return Self::error_not_mapped(address as u16);
                                                }
                                            }
                                        }
                                        None => {
                                            return Self::error_not_mapped(address as u16);
                                        }
                                    }
                                }
                                None => {
                                    return Err("Io memory is not supported by this CPU".to_string());
                                }
                            }
                        }
                        IoAddress::Mem(address) => {
                            let memory =  cpu.get_memory();
                            match memory.get_data()[address as usize] {
                                MemCell::Io(addres) => {
                                    match memory.get_ports().get(&addres) {
                                        Some(port) => {
                                            app.messages.push(port.get_io_port_info());
                                            return Ok(AppState::Home);
                                        }
                                        None => {
                                            return Self::error_not_mapped(address);
                                        }
                                    }
                                }
                                MemCell::Memory(address) => {
                                    return Self::error_not_mapped(address as u16);
                                }
                            }                
                        }
                    }
                }
                3 => {
                    if command[1].to_uppercase() != "REMOVE" {
                        return Err(format!("Invalid parameter: {}. Usage: \'dev\' or \'dev <address>\' or \'dev remove <address>\' ", command[1]));
                    }
                    let address = Self::parse_address(command[2].to_string())?;
                    match address {
                        IoAddress::Io(address) => {
                            match cpu.get_io_memory() {
                                Some(memory) => {
                                    let ports = memory.get_ports();
                                    ports.remove(&address);
                                }
                                None => {
                                    return Err("Io memory is not supported by this CPU".to_string());
                                }
                            }
                        }
                        IoAddress::Mem(address) => {
                            let memory =  cpu.get_memory();
                            match memory.get_data()[address as usize] {
                                MemCell::Io(address) => {
                                    memory.get_ports().remove(&address);
                                }
                                MemCell::Memory(address) => {
                                    return Self::error_not_mapped(address as u16);
                                }
                            }                
                        }
                    }

                }        
                _ => {
                    app.messages.push("Invalid number of parameters. Usage: \'dev\' or \'dev <address>\' or \'dev remove <address>\'".to_string());
                }
            }
        }
        Ok(AppState::Home)
    }
    fn error_not_mapped(address: u16) -> std::result::Result<AppState, String> {
        Err(format!("No port mapped to this address[0x{:04X}]", address))
    }
    fn push_info(app: &mut App, info: Vec<String>) {
        for info in info {
            app.messages.push(info);
        }
        app.messages.push("---".to_string());
    }
    /// Parses io address
    /// 
    /// Parses Io address and resturns IoAddress which indicates if it is memoru or io mapped
    /// IoMemory::Io(address) means that it is Io mapped, IoMemory::Mem(address) means that it is memory mapped
    fn parse_address(address: String) -> Result<IoAddress, String> {
        if address.to_uppercase().starts_with("M") {
            let address = address[1..].to_string();
            match Memory::from_hex_string(address) {
                Ok(address) => {
                    Ok(IoAddress::Mem(address))
                }
                Err(err) => {
                    Err(err.to_string())
                }
            }   
        }
        else {
            match Memory::from_hex_string(address) {
                Ok(address) => {
                    if address > 0xff {
                        return Err(format!("Address [0x{:4X}] of Io mapped dvice cannot be bigger than 0xff.", address))
                    }
                    Ok(IoAddress::Io(address as u8))
                }
                Err(err) => {
                    Err(err.to_string())
                }
            }   
        }
    }
}