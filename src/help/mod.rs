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
    /// Finds command
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
    "command": "da | disasm",
    "description": "Shows a disassembled code of a specific memory region.",
    "usage": "da or disasm or da <start address> or disasm <start address> or da <start address> <end address> or disasm <start address> <end address>",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    da\\n    disasm\\n    da 0ffh\\n    disasm $ff\\n    da 0 255\\n    disasm 0x0000 0x00ff\\n Note: When <start address> and <end address> are defined they are stored internally and next usage of disasm command without address range will use these value.\\n       When only \"disasm <start address>\" is defined then end address is calculated as <start + disasm_range> where dump_range is by default 64 and can be changed\\n       by disasm_range <value> command or dr <value>."
  },
  {
    "command": "dr | disasm_range",
    "description": "Shows or sets range of default addresses for disasembler.",
    "usage": "dr [range] or disasm_range [range]",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    dr\\n    disasm_range\\n    dr 100\\n    dr 0ffh\\n    disasm_range 0x00ff\\n    disasm_range $00ff\\n Note: If only dr or disasm_range is used, then the disassembler range is displayed; otherwise, the range is set to the value provided as a parameter."
  },
  {
    "command": "dump",
    "description": "Displays an area of the RAM for a specific CPU.",
    "usage": "dump or dump <start address> or dump <start address> <end address>",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    dump\\n    dump 0xff\\n    dump 0 127\\n    dump 0h 0ffh\\n    dump $0 $ff\\n    dump 0x0 0xff\\n Note: When <start address> and <end address> are defined they are stored internally and next usage of dump command without address range will use these value.\\n       When only \"dump <start address>\" is defined then end address is calculated as <start + dump_range> where dump_range is by default 128 and can be changed by \"set dump_range <value>\" command."
  },
  {
    "command": "help | h | ?",
    "description": "Shows a help for specific command",
    "usage": "help <command> or h <command> or ? <command>",
    "examples": "\\n    help pwd\\n    h ls\\n    ? disasm"
  },
  {
    "command": "load | l",
    "description": "Loads a content of specified binary file into memory.",
    "usage": "load <start address> <file name> or l <start address> <file name>",
    "examples": "\\n    l 0ffh file.o\nn    l 1024 file.obj\\n    load $ff file.o\\n    load 0xff file.obj"
  },
  {
    "command": "loada | la",
    "description": "Loads a content of specified ACME binary file into memory.",
    "usage": "loada <file name> or la <file name>",
    "examples": "\\n    load file.o\\n    la file.o\\n Note: ACME binary file contains start address in first two bytes of the file so the start address doesn't have to be specified."
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
    "command": "mr | memory_range",
    "description": "Shows or sets range of default addresses for dump command.",
    "usage": "mr [range] memory_range [range]",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    mr\\n    memory_range\\n    mr 100\\n    mr 0ffh\\n    memory_range 0x00ff\\n    memory_range $00ff\\n Note: If only mr or memory_range is used, then the dump memory range is displayed; otherwise, the range is set to the value provided as a parameter."
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
    "command": "reg | r",
    "description": "Shows or sets the content of the register of the currently set CPU.",
    "usage": "r or r <reg> [value] or reg or reg <reg> [value]",
    "examples": "\\n    r\\n    reg\\n    reg a\\n    reg a 0ffh\\n    r x\\n    r x $ff\\n    reg sp 0fffh\\n    r pc $ffff\\n Note: If value is defined, it is set, otherwise only the content of the register is displayed. If no register is provided it will display all registers."
  },
  {
    "command": "set",
    "description": "Sets application's parameters like cpu, range ...",
    "usage": "set <parameter> [<parameter>, ...]",
    "examples": "\\n    set cpu 8080\\n    set cpu i8080\\n    set cpu 6502\\n    set cpu mos6502\\n    set disasm_range 64\\n    set dump_range 64\\n    set command_history_size 200\\n    set output_history_size 2000\\n Note: dump_range is a number of bytes displayed by dump command.\\n Note: disasm_range is a number of bytes displayed by disasm command.\\n Note: output_history_size is a number of lines from Output window kept in history buffer.\\n Note: command_history_size is a number of lines from Command window kept in history buffer."
  }
]
"#;
