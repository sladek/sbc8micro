**Some examples** 

This folder contains some examples that I've used for testing of the emulator.

The first example is the file test. a

It contains all the instructions for the MOS6502 processor, and it was used for a verification of the emulator. It is quite messy, but it can be used for your own development. The assembly code is written for [Acme Cross Assembler, a multi-platform cross assembler for 6502/6510/65816 CPU](https://github.com/meonwax/acme)s. Sections of every instruction are separated by comments so they can be easily identified and used. Source code can be compiled by the following command:

$ acme test.a

It generates a test.o file that can be loaded to the memory of the emulator.

**Note:** In the .o file, the first bytes contain the start address of the program, and it is used by the load_program_from_acme_file function of the emulator.

The test program can look like the following one:

```
mod cpu;
mod disassembler;
mod memory;
mod status;

use cpu::mos6502;
use disassembler::mos6502::{disassemble, load_opcodes_table};

fn main() {
    let opcodes = load_opcodes_table();
    let mut cpu = mos6502::Cpu::new();
    cpu.set_debug(true);
    let start_addr = 0x0200;
    let size = cpu.memory.load_program_from_acme_file("test.o").unwrap();
    let disassembly = disassemble(&cpu.memory, start_addr, start_addr + size as u16, &opcodes);
    println!("---------------------------");
    println!("Main programm.");
    println!("---------------------------");
    for line in disassembly {
        println!("{}", line);
    }
    println!("---------------------------");
    println!("Debugger output");
    println!("---------------------------");
    cpu.pc = start_addr;
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0xff {
            println!("---------------------------");
            println!("End of simulation");
            break;
        }
    }
    println!("---------------------------");
    println!("Registers");
    print!("{}", cpu.print_registers());
    println!("Test zero page");
    cpu.memory.hex_dump(0x55, 0x55 + 31);
    println!("Test area");
    cpu.memory.hex_dump(0x04D0, 0x04D0 + 31);
    println!("Upper stack:");
    cpu.memory.hex_dump(0x018f - 0x5f, 0x018f);
    log::info!("Hahaha {:02X}", 0x34);
    log::debug!("I am here!");
}


```

The important part is the loop {} in which the instructions loaded to memory are executed. In this example, the loop ends when opcode 0xff is found. In this example you can learn how to use a disassembler, how to run a program loaded into memory, dump parts of memory, and also how to print the instructions executed.

Instructions executed during the running of the program are not printed "by default," but it can be enabled with the following function.

`cpu.set_debug(true);`

If enabled, it will slow the execution, so it should be used only for debugging purposes.

The output should look like in the following picture:

![](C:\Users\User\AppData\Roaming\marktext\images\2025-06-29-01-54-36-image.png)


