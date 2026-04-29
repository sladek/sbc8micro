/// Help item
#[derive(Default, Debug, Clone, serde::Deserialize)]
pub struct HelpItem {
    pub command: String,
    pub description: String,
    pub usage: String,
    pub examples: String,
}

/// Keeps list of help items
#[derive(Debug, Default)]
pub struct Help<HelpItem> {
    /// List of help informations
    pub help_items: Vec<HelpItem>,
}
impl Help<HelpItem> {
    /// Parses OPCODES and returns an instance of opcode view
    pub fn new() -> Self {
        Self {
            help_items: serde_json::from_str(HELP).unwrap(),
        }
    }
    /// Returns specific help item
    pub fn get_item(&self, name: &str) -> Option<HelpItem> {
        self.help_items
            .clone()
            .into_iter()
            .find(|item| Self::find_command(item.command.clone(), name))
    }
    /// Find command
    ///
    /// Finds command in "command" json member. command can contain multiple values like "help | ?" or "registers | regs"
    /// and both are valid commands
    fn find_command(commands: String, command: &str) -> bool {
        let lines: Vec<&str> = commands.split('|').collect();
        for line in lines {
            if line.trim() == command {
                return true;
            }
        }
        false
    }
    /// Return list of all commands
    /// 
    /// Parses the help yaml file and returns list of all commands
    pub fn items_list() -> Vec<String> {
      let help_items:Vec<HelpItem> = serde_json::from_str(HELP).unwrap();
      let mut list:Vec<String> = Vec::new();
      for item in help_items {
        let mut it:Vec<String> = item.command.split('|').map(|item| { item.trim().to_string()}).collect();
        list.append(&mut it);
      }
      list
    }
}

pub static HELP: &str = r#"
[
  {
    "command": "b",
    "description": "Sets or clears breakpoints.",
    "usage": "b or b <address> or b x",
    "examples": "\\n    b\\n    b 0xabcd\\n    b x\\n Note: b x is used to clear all the breakpoints.\\n       b <address> is used to set or clear the specific breakpoint. If the breakpoint doesn't exist it is set, if it exists it is cleard. "
  },
  {
    "command": "bl | bootloader",
    "description": "Sets bootloader filename.",
    "usage": "bl or bootloader [bootloader filename]",
    "examples": "\\n    bl bootloaders/cpm.hex\\n    bootloader bootloaders/cpm.hex\\n    bl\\n    bootloader\\n Note: Bootloader file must be in INTELHEX format.\\n       If file is not specified, it returns bootloader status (filename or 'Not defined')."
  },
  {
    "command": "cd",
    "description": "Change directory.",
    "usage": "cd or cd <directory>",
    "examples": "\\n    cd\\n    cd /\\n    cd \\\\n    cd /Program Files\\n    cd /home/user\\n Notes: Spaces ' ', like in \"Program Files\" are allowed in folder name.\\n        When no <directory> is provided it changes to home directory."
  },
  {
    "command": "clear | cls",
    "description": "Clears the output area.",
    "usage": "clear or cls",
    "examples": "\\n    clear\\n    cls"
  },
  {
    "command": "ch | command_history_length",
    "description": "Shows or sets length of command window history.",
    "usage": "ch [length] command_history_length [length]",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    ch\\n    command_history_length\\n    ch 100\\n    ch 0ffh\\n    command_history_length 0x00ff\\n    command_history_length $00ff\\n Note: If only oh or command_history_length is used, then the command history length is displayed; otherwise, the length is set to the value provided as a parameter."
  },
  {
    "command": "cpu",
    "description": "Sets specific CPU.",
    "usage": "cpu <CPU>",
    "examples": "\\n    cpu 8080\\n    cpu 6502\\n    cpu i8080\\n    cpu mos6502"
  },
  {
    "command": "cs | conf_switch",
    "description": "Sets configuration switch at specific address.",
    "usage": "cs <address> <data> [name]",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    cs 0xf0 0x02\\n    cs 0xf0 0x02 boot\\n    conf_switch 0xf0 0x02\\n    conf_swotch 0xf0 0x02 boot."
  },
  {
    "command": "d | dump",
    "description": "Displays an area of the RAM for a specific CPU.",
    "usage": "dump or d or dump <start address> or d <start address> or dump <start address> <end address> or d <start address> <end address>",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    dump\\n    dump 0xff\\n    dump 0 127\\n    d 0h 0ffh\\n    d $0 $ff\\n    d 0x0 0xff\\n Note: When <start address> and <end address> are defined they are stored internally and next usage of dump command without address range will use these value.\\n       When only \"dump <start address>\" is defined then end address is calculated as <start + dump_range> where dump_range is by default 128 and can be changed by \"set dump_range <value>\" command."
  },
  {
    "command": "dr | dump_range",
    "description": "Shows or sets range of default addresses for dump command.",
    "usage": "dr [range] dump_range [range]",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    dr\\n    dump_range\\n    dr 100\\n    dr 0ffh\\n    dump_range 0x00ff\\n    dump_range $00ff\\n Note: If only dr or dump_range is used, then the dump memory range is displayed; otherwise, the range is set to the value provided as a parameter."
  },
  {
    "command": "da | disasm",
    "description": "Shows a disassembled code of a specific memory region.",
    "usage": "da or disasm or da <start address> or disasm <start address> or da <start address> <end address> [file name] or disasm <start address> <end address> [file name]",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    da\\n    disasm\\n    da 0ffh\\n    disasm $ff\\n    da 0 255\\n    disasm 0x0000 0x00ff\\n    disasm 0x0000 0x00ff output.lst\\n Note: When <start address> and <end address> are defined they are stored internally and next usage of disasm command without address range will use these value.\\n       When only \"disasm <start address>\" is defined then <end address> is calculated as <start address> + <disasm_range> where disasm_range is by default 16 and can be changed\\n       by disasm_range <value> command or dr <value>.\\n       When <file name> is specified (like output.lst in examples above), then the output of disassembler is also recorded to that file."
  },
  {
    "command": "dar | disasm_range",
    "description": "Shows or sets range of default addresses for disasembler.",
    "usage": "dar [range] or disasm_range [range]",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    dar\\n    disasm_range\\n    dar 100\\n    dar 0ffh\\n    disasm_range 0x00ff\\n    disasm_range $00ff\\n Note: If only dar or disasm_range is used, then the disassembler range is displayed; otherwise, the range is set to the value provided as a parameter.\\n       End address of disassembler is calculated as start adrress + range."
  },
  {
    "command": "dev",
    "description": "Shows devices (serial, disk controllers, ...) currently attached to the CPU.",
    "usage": "dev or dev <address>",
    "examples": "\\n    dev\\n    dev 0x78"
  },
  {
    "command": "ec | empty_cycles",
    "description": "Sets/gets number of empty cycles for CPU.",
    "usage": "ec <cycles number> or empty_cycles <cycles number> or ec or empty_cycles",
    "examples": "\\n    ec 0x0f\\n    ec\\n    empty_cycles 0x0f\\n    empty_cycles\\n Note: If number of cycles is not specified it shows current number of cycles."
  },
  {
    "command": "fdc | floppy",
    "description": "Sets flppy controller port's address and disk names.",
    "usage": "fdc <address> [disk 1] [disk 2] [disk 3] [disk 4]",
    "examples": "\\n    fdc 0x78\\n    fdc 0x78 cpm.dsk\\n    floppy 0x78 cpm.dsk[ro] asm.dsk\\n    fdc M0x1234 cpm.dsk\\n    floppy M0x1234 cpm.dsk[RO] plm.dsk data1.dsk data2.dsk\\n Note: Address can be defined as <address 8bit> if mapped to io memory or as M<address 16bit> if mapped to cpu's memory.\\n       File name can have a suffix [ro] or [RO] (not case sensitive) to indicate that the disk will be mounted as read only.\\n       The controller can be used without any disk defined. This is usefull in the cases when booting to Monitor (ISIS II os)\\n       or BASIC for the case that no floppy is present."
  },
  {
    "command": "fdhdc | disk_controller",
    "description": "Sets disk controller port's address and disk names.",
    "usage": "fdhdc <address> <disk 1> [disk 2] [disk 3] [disk 4] or disc_controller <address> <disk 1> [disk 2] [disk 3] [disk 4]",
    "examples": "\\n    fdhdc 0x78 cpm.dsk\\n    disk_controller 0x78 cpm.dsk[ro] asm.dsk\\n    fdhdc M0x1234 cpm.dsk\\n    disk_controller M0x1234 cpm.dsk[RO] plm.dsk data1.dsk data2.dsk\\n Note: Address can be defined as <address 8bit> if mapped to io memory or as M<address 16bit> if mapped to cpu's memory.\\n       File name can have a suffix [ro] or [RO] (not case sensitive) to indicate that the disk will be mounted as read only.\\n       The controller can be used without any disk defined. This is usefull in the cases when booting to Monitor (ISIS II os)\\n       or BASIC for the case that no floppy is present."
  },
  {
    "command": "g | go",
    "description": "Starts a programm from an address defined in PC register or as a parameter on command line.",
    "usage": "g [address] or go [address]",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    g\\n    g 0x1000\\n    go\\n    go $1000\\n Note: This command can be used for debugging as it checks keyboard input and exits on CTRL-C. It can slow the execution.\\n       For full speed execution use run command. But in this case it can be interrupted only on predefined HLT instruction."
  },
  {
    "command": "h | help | ?",
    "description": "Shows a help for specific command",
    "usage": "help <command> or h <command> or ? <command>",
    "examples": "\\n    help pwd\\n    h ls\\n    ? disasm"
  },
  {
    "command": "hb | hex_bin",
    "description": "Converts INTELHEX file to binary file",
    "usage": "hb <offset> <input file> [output file] or hex_bin <offset> <input file> [output file]",
    "examples": "\\n    hb 0x100 asm.hex asm.com\\n    hb 0x100 asm.hex\\n Note: If output file name is not defined it is created with .com suffix.\\n       If <offset> is not 0 it is substracted from the offset of INTELHEX file.\\n       Normally .com files of CP/M system start at address 0x100 so if such\\n       a INTELHEX file is converted to .com file the offset of 0x100 must be subtracted."
  },
  {
    "command": "io",
    "description": "Input from/ output to specific address on io bus",
    "usage": "io <address> [data]",
    "examples": "\\n    io 0x78\\n    io 0x79 0x55\\n Note: If data is not defined it reads data from io address.\\n       If data is defined it writes data to io address."
  },
  {
    "command": "ih | ihex",
    "description": "Converts a file to INTELHEX format",
    "usage": "ih <offset> <input file> [output file] or ihex <offset> <input file> [output file]",
    "examples": "\\n    ihex 0x100 asm.com asm.hex\\n    ih 0x100 asm.com\\n Note: If output file name is not defined it is created with .hex suffix.\\n       If output file name is 'data.rdr' no suffix is added. File name 'data.rdr' is a special file name for CP/M 2.2 pip transfer."
  },
  {
    "command": "i2r | imd2raw",
    "description": "Converts the .imd file to raw binary file",
    "usage": "i2r <input file> [output file] or imd2raw <input file> [output file]",
    "examples": "\\n    i2r diskd.imd disk.raw\\n    i2r diskd.imd\\n    imd2raw diskd.imd disk.raw\\n    imd2raw diskd.imd\\n Note: If output file name is not defined it is created with the same name as input file, but with .raw suffix."
  },
  {
    "command": "l | load",
    "description": "Loads a content of specified binary file into memory.",
    "usage": "load <start address> <file name> or l <start address> <file name>",
    "examples": "\\n    l 0ffh file.o\nn    l 1024 file.obj\\n    load $ff file.o\\n    load 0xff file.obj"
  },
  {
    "command": "la | loada",
    "description": "Loads a content of specified ACME binary file into memory.",
    "usage": "loada <file name> or la <file name>",
    "examples": "\\n    loada file.o\\n    la file.o\\n Note: ACME binary file contains start address in first two bytes of the file so the start address doesn't have to be specified."
  },
  {
    "command": "lh | loadh",
    "description": "Loads a content of specified INTELHEX file into memory.",
    "usage": "loadh <file name> or lh <file name>",
    "examples": "\\n    loadh file.hex\\n    lh file.ohex"
  },
  {
    "command": "ls",
    "description": "Directory list.",
    "usage": "ls <directory name | filename>",
    "examples": "\\n    ls\\n    ls .\\n    ls /var/log\\n    ls \\Program Files\\n    ls *.asm\\n    ls myfiles.*\\n    ls my*.*\\n Note: Unix '/' and Windows '\\' separators are alowed."
  },
  {
    "command": "m | mem",
    "description": "Fills memory from specific address with defined data.",
    "usage": "m <address> <data> <data> <data> ... or mem <address> <data> <data> <data> ...",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    m 0ffh 0ffh 12h 55h 0aah\\n    mem $fff $ff $12 $55 $aa\\n Note: address is 16 bit and data 8 bit."
  },
  {
    "command": "oh | output_history_length",
    "description": "Shows or sets length of output window history.",
    "usage": "oh [length] output_history_length [length]",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    oh\\n    output_history_length\\n    oh 100\\n    oh 0ffh\\n    output_history_length 0x00ff\\n    output_history_length $00ff\\n Note: If only oh or output_history_length is used, then the output history length is displayed; otherwise, the length is set to the value provided as a parameter."
  },
  {
    "command": "op | opcodes",
    "description": "Shows a list of opcodes including description for specific CPU.",
    "usage": "opcodes or opcodes <CPU> or op or op <CPU>",
    "examples": "\\n    opcodes\\n    op\\n    opcodes 8080\\n    opcodes i8080\\n    op 6502\\n    op mos6502"
  },
  {
    "command": "pwd",
    "description": "Shows the name of current working directory.",
    "usage": "pwd",
    "examples": "\\n    pwd"
  },
  {
    "command": "r2d | raw2dsk",
    "description": "Converts the raw disk file to .dsk file that can be mounted in this simulator.",
    "usage": "r2d <input file> <disk format>:[output file] or raw2dsk <input file> <disk format>:[output file]",
    "examples": "\\n    r2d diska.raw sssd:\\n    r2d diska.raw sssd:diskb\\n    r2d diskc.raw sssd:diskd.dsk\\n    raw2dsk diska.raw sssd:\\n    raw2dsk diska.raw sssd:diskb\\n    raw2dsk diskc.raw sssd:diskd.dsk\\n Note: If output file name is not defined it is created with the same name as input file, but with .raw suffix."
  },
  {
    "command": "rdr | pun | rdr_pun",
    "description": "Sets tape reader's/puncher's io or memory address.",
    "usage": "rdr <address> or pun <address> or rdr_pun <address>",
    "examples": "\\n    rdr 0x40\\n    pun 0x40\\n    rdr_pun M0x1234\\n Note: Address can be defined as <address 8bit> if mapped to io memory or as M<address 16bit> if mapped to cpu's memory."
  },
  {
    "command": "reg | r",
    "description": "Shows or sets the content of the register of the currently set CPU.",
    "usage": "r or r <reg> [value] or reg or reg <reg> [value]",
    "examples": "\\n    r\\n    reg\\n    reg a\\n    reg a 0ffh\\n    r x\\n    r x $ff\\n    reg sp 0fffh\\n    r pc $ffff\\n Note: If value is defined, it is set, otherwise only the content of the register is displayed. If no register is provided it will display all registers."
  },
  {
    "command": "res | reset",
    "description": "Resets the CPU.",
    "usage": "res or reset",
    "examples": "\\n    res\\n    reset\\n Note: If bootloader is specified it is executed or it just jumps to address 0x0000"
  },
  {
    "command": "ro | read_only",
    "description": "Sets read only region of RAM.",
    "usage": "ro <start address> <end address> [true/false] or read_only <start address> <end address> [true/false]",
    "examples": "\\n    ro 0x1000 0x2000\\n    read_only 0x1000 0x2000\\n    ro 0x1000 0x2000 true\\n    read_only 0x1000 0x2000 false\\n Note: When read only flag is set to false, the memory becomes writable again."
  },
  {
    "command": "run",
    "description": "Starts a programm from an address defined in PC register or as a parameter on command line.",
    "usage": "run [address]",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    run\\n    run 0x1000\\n Note: This command can be used to start execution at full speed. It doesn't check keyboard input and cannot exit on CTRL-C.\\n       But it can be interrupted on predefined HLT instruction."
  },
  {
    "command": "ser | serial",
    "description": "Sets serial port's address, name, clock frequency and default parameters.",
    "usage": "ser <address> <port name> [clock] or serial <address> <port name> [clock]",
    "examples": "\\n    ser 0x40 COM3\\n    ser 0x40 /dev/tty4 2048000\\n    serial M0x1234 COM3\\n    serial M0x1234 /dev/tty4 20480000\\n Note: Address can be defined as <address 8bit> if mapped to io memory or as M<address 16bit> if mapped to cpu's memory.\\n Note: Default clock frequency is 416400, baud rate factor is 64x and other parameters are 8 bit, 1 stop bit, no parity."
  },
  {
    "command": "scr | script",
    "description": "Reads commands from script file and executes them one by one.",
    "usage": "scr <script name> or script <script name>",
    "examples": "\\n    scr cpm80.scr\\n    scr cpm80\\n    script scripts/plm80.scr\\n    script scripts/plm80"
  },
  {
    "command": "sh | set_hlt",
    "description": "Sets or shows an instruction code for HLT. HLT code is used to break from execution of the code and return to UI or closing the main program",
    "usage": "sh [HLT instruction code] or set_hlt [HLT instruction code]",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    sh\\n    set_hlt 0x78\\n Note: If no parameter is defined it shows current instruction code for HLT."
  }
]
"#;
