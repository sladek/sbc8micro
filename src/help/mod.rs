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
              return true
          }
      }
      false
    }

  }

pub static HELP: &str = r#"
[
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
    "command": "disasm",
    "description": "Shows a disassembled code of a specific memory region.",
    "usage": "disasm or disasm <start address> or disasm <start address> <end address>",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    disasm\\n    disasm 0ffh\\n    disasm 0x0000 0x00ff\\n Note: When <start address> and <end address> are defined they are stored internally and next usage of disasm command without address range will use these value.\\n       When only \"disasm <start address>\" is defined then end address is calculated as <start + disasm_range> where dump_range is by default 64 and can be changed by \"set disasm_range <value>\" command."
  },
  {
    "command": "dump",
    "description": "Displays an area of the RAM for a specific CPU.",
    "usage": "dump or dump <start address> or dump <start address> <end address>",
    "examples": "Multiple hexadecimal representations are allowed like decimal (1234), intel (0abcdh), mos6502 ($abcd) and modern (0xabcd)\\n    dump\\n    dump 0xff\\n    dump 0 127\\n    dump 0h 0ffh\\n    dump $0 $ff\\n    dump 0x0 0xff\\n Note: When <start address> and <end address> are defined they are stored internally and next usage of dump command without address range will use these value.\\n       When only \"dump <start address>\" is defined then end address is calculated as <start + dump_range> where dump_range is by default 128 and can be changed by \"set dump_range <value>\" command."
  },
  {
    "command": "help | ?",
    "description": "Shows a help for specific command",
    "usage": "help <item> or ? <item>",
    "examples": "\\n    help set\\n    ? set"
  },
  {
    "command": "load",
    "description": "Loads a content of specified binary file into memory.",
    "usage": "load <start address> <file name>",
    "examples": "\\n    load 0ffh file.o\\n    load $ff file.o\\n    load 0xff file.obj"
  },
  {
    "command": "loada",
    "description": "Loads a content of specified ACME binary file into memory.",
    "usage": "loada <file name>",
    "examples": "\\n    load file.o\\n Note: ACME binary file contains start address in first two bytes of the file so the start address doesn't have to be specified."
  },
  {
    "command": "ls",
    "description": "Directory list.",
    "usage": "ls <directory name | filename>",
    "examples": "\\n    ls\\n    ls .\\n    ls /var/log\\n    ls \\Program Files\\n    ls *.asm\\n    ls myfiles.*\\n    ls my*.*\\n Note: Unix '/' and Windows '\\' separators are alowed."
  },
  {
    "command": "opcodes",
    "description": "Shows a list of opcodes including description for specific CPU.",
    "usage": "opcodes or opcodes <CPU>",
    "examples": "\\n    opcodes\\n    opcodes 8080\\n    opcodes i8080\\n    opcodes 6502\\n    opcodes mos6502"
  },
  {
    "command": "pwd",
    "description": "Shows the name of current working directory.",
    "usage": "pwd",
    "examples": "\\n    pwd"
  },
  {
    "command": "registers | regs",
    "description": "Shows the content of registers of currently set CPU.",
    "usage": "registers or regs",
    "examples": "\\n    registers\\n    regs"
  },

  {
    "command": "set",
    "description": "Sets application's parameters like cpu, range ...",
    "usage": "set <parameter> [<parameter>, ...]",
    "examples": "\\n    set cpu 8080\\n    set cpu i8080\\n    set cpu 6502\\n    set cpu mos6502\\n    set disasm_range 64\\n    set dump_range 64\\n    set command_history_size 200\\n    set output_history_size 2000\\n Note: dump_range is a number of bytes displayed by dump command.\\n Note: disasm_range is a number of bytes displayed by disasm command.\\n Note: output_history_size is a number of lines from Output window kept in history buffer.\\n Note: command_history_size is a number of lines from Command window kept in history buffer."
  }
]
"#;
