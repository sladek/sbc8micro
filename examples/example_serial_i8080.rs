use sbc8micro::cpu::{CpuUi, i8080};
use sbc8micro::io::IoPort;
use sbc8micro::io::i8251a::Async8251;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Map serial port to io address space
    let port_name = "COM3";
    let base_address = 0x40;
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
    let mut serial = Async8251::new().open_port(port_name)?;

    serial.set_base_address(base_address);
    serial.set_name(port_name.to_string());
    let info = serial.get_io_port_info();
    // set CPU
    let mut cpu = i8080::Cpu::new();
    cpu.set_debug_flag(false);
    let io_memory = &mut cpu.io_memory;
    io_memory.map_port(Box::new(serial))?;
    println!("{}", info);
    //    let region = cpu.memory.load_data_from_intelhex_file("examples/serial_8080.hex")?;
    let region = cpu
        .memory
        .borrow_mut()
        .load_data_from_intelhex_file("examples/glitchworks.hex")?;
    println!("Start: {:04X}H, End {:04X}H.", region.start, region.end);
    cpu.pc = region.start;
    println!("Program started");
    loop {
        let opcode = cpu.memory.borrow_mut().read_byte(cpu.pc);
        cpu.step();
        // If HLT end simulation
        if opcode == 0x76 {
            println!("---------------------------");
            println!("End of simulation");
            break;
        }
    }
    Ok(())
}
