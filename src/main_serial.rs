use sbc8micro::io::{memory::Memory, serial::{
    Async8251, BaudRateFactor, CharacterLength, Control, Status, StopBits
}, IoPort};
use serialport::Parity;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let port_name = "COM3";
    let data_address = 0x40;
    let control_address = data_address+1;
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
    let mut serial = Async8251::new().open_port(port_name).unwrap();
    serial.set_base_address(data_address);
    let mut io_memory = Memory::new();
    io_memory.map_port(Box::new(serial));

    // Soft reset of the port
    io_memory.write(control_address, 0);
    io_memory.write(control_address, 0);
    io_memory.write(control_address, 0);
    io_memory.write(control_address, 0x40);
    // Set parameters 8,N,1 64x
    let data = CharacterLength::Eight as u8
        | Parity::None as u8
        | StopBits::One as u8
        | BaudRateFactor::X64 as u8;
    io_memory.write(control_address, data);
    let control = Control::RxE as u8 | Control::TxEN as u8;
    io_memory.write(control_address, control);
    loop {
        if io_memory.read(control_address) & Status::RxRDY as u8 != 0x0 { // Test if data is ready
            let data = io_memory.read(data_address);
            io_memory.write(data_address, data);
            io_memory.write(data_address, 0x0d);
            io_memory.write(data_address, 0x0a);
        }
    }
}
