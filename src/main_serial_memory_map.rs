use sbc8micro::io::serial::{
    Async8251, BaudRateFactor, CharacterLength, Control, Status, StopBits,
};
use sbc8micro::memory::Memory;
use serialport::Parity;

fn main() {
    let port_name = "COM3";
    let data_address = 0x40u16;
    let control_address = data_address + 1;
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
    let mut serial = Async8251::new().open_port(port_name).unwrap();
    serial.set_memory_base_address(data_address);
    let mut memory = Memory::new();
    _ = memory.map_port(Box::new(serial));

    // Soft reset of the port
    memory.write_byte(control_address, 0);
    memory.write_byte(control_address, 0);
    memory.write_byte(control_address, 0);
    memory.write_byte(control_address, 0x40);
    // Set parameters 8,N,1 64x
    let data = CharacterLength::Eight as u8
        | Parity::None as u8
        | StopBits::One as u8
        | BaudRateFactor::X64 as u8;
    memory.write_byte(control_address, data);
    let control = Control::RxE as u8 | Control::TxEN as u8;
    memory.write_byte(control_address, control);
    loop {
        if memory.read_byte(control_address) & Status::RxRDY as u8 != 0x0 {
            // Test if data is ready
            let data = memory.read_byte(data_address);
            memory.write_byte(data_address, data);
            memory.write_byte(data_address, 0x0d);
            memory.write_byte(data_address, 0x0a);
        }
    }
}
