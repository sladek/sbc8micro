pub static OPCODES: &str = r#"
[
  {
    "opcode": "69",
    "mnemonic": "ADC #oper",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "description": "Add Memory to Accumulator with Carry. [A + M + C -> A, C]\n\nN Z C I D V\n+ + + - - +"
  },
  {
    "opcode": "65",
    "mnemonic": "ADC oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 3,
    "description": "Add Memory to Accumulator with Carry. [A + M + C -> A, C]\n\nN Z C I D V\n+ + + - - +"
  },
  {
    "opcode": "75",
    "mnemonic": "ADC oper,X",
    "mode": "zeropage,X",
    "bytes": 2,
    "cycles": 4,
    "description": "Add Memory to Accumulator with Carry. [A + M + C -> A, C]\n\nN Z C I D V\n+ + + - - +"
  },
  {
    "opcode": "6D",
    "mnemonic": "ADC oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 4,
    "description": "Add Memory to Accumulator with Carry. [A + M + C -> A, C]\n\nN Z C I D V\n+ + + - - +"
  },
  {
    "opcode": "7D",
    "mnemonic": "ADC oper,X",
    "mode": "absolute,X",
    "bytes": 3,
    "cycles": 4,
    "description": "Add Memory to Accumulator with Carry. [A + M + C -> A, C]\n1) add 1 to cycles if page boundary is crossed\n\nN Z C I D V\n+ + + - - +"
  },
  {
    "opcode": "79",
    "mnemonic": "ADC oper,Y",
    "mode": "absolute,Y",
    "bytes": 3,
    "cycles": 4,
    "description": "Add Memory to Accumulator with Carry. [A + M + C -> A, C]\n1) add 1 to cycles if page boundary is crossed\n\nN Z C I D V\n+ + + - - +"
  },
  {
    "opcode": "61",
    "mnemonic": "ADC (oper,X)",
    "mode": "(indirect,X)",
    "bytes": 2,
    "cycles": 6,
    "description": "Add Memory to Accumulator with Carry. [A + M + C -> A, C]\n\nN Z C I D V\n+ + + - - +"
  },
  {
    "opcode": "71",
    "mnemonic": "ADC (oper),Y",
    "mode": "(indirect),Y",
    "bytes": 2,
    "cycles": 5,
    "description": "Add Memory to Accumulator with Carry. [A + M + C -> A, C]\n1) add 1 to cycles if page boundary is crossed\n\nN Z C I D V\n+ + + - - +"
  },
  {
    "opcode": "29",
    "mnemonic": "AND #oper",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "description": "AND Memory with Accumulator. [A AND M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "25",
    "mnemonic": "AND oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 3,
    "description": "AND Memory with Accumulator. [A AND M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "35",
    "mnemonic": "AND oper,X",
    "mode": "zeropage,X",
    "bytes": 2,
    "cycles": 4,
    "description": "AND Memory with Accumulator. [A AND M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "2D",
    "mnemonic": "AND oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 4,
    "description": "AND Memory with Accumulator. [A AND M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "3D",
    "mnemonic": "AND oper,X",
    "mode": "absolute,X",
    "bytes": 3,
    "cycles": 4,
    "description": "AND Memory with Accumulator. [A AND M -> A]\n1) add 1 to cycles if page boundary is crossed\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "39",
    "mnemonic": "AND oper,Y",
    "mode": "absolute,Y",
    "bytes": 3,
    "cycles": 4,
    "description": "AND Memory with Accumulator. [A AND M -> A]\n1) add 1 to cycles if page boundary is crossed\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "21",
    "mnemonic": "AND (oper,X)",
    "mode": "(indirect,X)",
    "bytes": 2,
    "cycles": 6,
    "description": "AND Memory with Accumulator. [A AND M -> A]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "31",
    "mnemonic": "AND (oper),Y",
    "mode": "(indirect),Y",
    "bytes": 2,
    "cycles": 5,
    "description": "AND Memory with Accumulator. [A AND M -> A] [A + M + C -> A, C]\n1) add 1 to cycles if page boundary is crossed)\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "0A",
    "mnemonic": "ASL A",
    "mode": "accumulator",
    "bytes": 1,
    "cycles": 2,
    "description": "Shift Left One Bit (Memory or Accumulator). [C <- [76543210] <- 0]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "06",
    "mnemonic": "ASL oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 5,
    "description": "Shift Left One Bit (Memory or Accumulator). [C <- [76543210] <- 0]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "16",
    "mnemonic": "ASL oper,X",
    "mode": "zeropage,X",
    "bytes": 2,
    "cycles": 6,
    "description": "Shift Left One Bit (Memory or Accumulator). [C <- [76543210] <- 0]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "0E",
    "mnemonic": "ASL oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 6,
    "description": "Shift Left One Bit (Memory or Accumulator). [C <- [76543210] <- 0]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "1E",
    "mnemonic": "ASL oper,X",
    "mode": "absolute,X",
    "bytes": 3,
    "cycles": 7,
    "description": "Shift Left One Bit (Memory or Accumulator). [C <- [76543210] <- 0]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "90",
    "mnemonic": "BCC oper",
    "mode": "relative",
    "bytes": 2,
    "cycles": 2,
    "description": "Branch on Carry Clear. [branch on C = 0]\n1) add 1 to cycles if branch occurs on same page \n2) add 2 to cycles if branch occurs to different page\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "B0",
    "mnemonic": "BCS oper",
    "mode": "relative",
    "bytes": 2,
    "cycles": 2,
    "description": "Branch on Carry Set. [branch on C = 1]\n1) add 1 to cycles if branch occurs on same page \n2) add 2 to cycles if branch occurs to different page\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "F0",
    "mnemonic": "BEQ oper",
    "mode": "relative",
    "bytes": 2,
    "cycles": 2,
    "description": "Branch on Result Zero. [branch on Z = 1]\n1) add 1 to cycles if branch occurs on same page \n2) add 2 to cycles if branch occurs to different page\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "24",
    "mnemonic": "BIT oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 3,
    "description": "Branch on Result Zero. [A AND M -> Z, M7 -> N, M6 -> V]\n1) bits 7 and 6 of operand are transfered to bit 7 and 6 of SR (N,V);\n   the zero-flag is set according to the result of the operand AND the accumulator (set, if the result is zero, unset otherwise).\n   This allows a quick check of a few bits at once without affecting any of the registers, other than the status register (SR).\n\nN Z C I D V\nM7- - - - M6"
  },
  {
    "opcode": "2C",
    "mnemonic": "BIT oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 4,
    "description": "Branch on Result Zero. [A AND M -> Z, M7 -> N, M6 -> V]\n1) bits 7 and 6 of operand are transfered to bit 7 and 6 of SR (N,V);\n   the zero-flag is set according to the result of the operand AND the accumulator (set, if the result is zero, unset otherwise).\n   This allows a quick check of a few bits at once without affecting any of the registers, other than the status register (SR).\n\nN Z C I D V\nM7- - - - M6"
  },
  {
    "opcode": "30",
    "mnemonic": "BMI oper",
    "mode": "relative",
    "bytes": 2,
    "cycles": 2,
    "description": "Branch on Result Minus. [branch on N = 1]\n1) add 1 to cycles if branch occurs on same page \n2) add 2 to cycles if branch occurs to different page\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "D0",
    "mnemonic": "BNE oper",
    "mode": "relative",
    "bytes": 2,
    "cycles": 2,
    "description": "Branch on Result not Zero. [branch on Z = 0]\n1) add 1 to cycles if branch occurs on same page \n2) add 2 to cycles if branch occurs to different page\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "10",
    "mnemonic": "BPL oper",
    "mode": "relative",
    "bytes": 2,
    "cycles": 2,
    "description": "Branch on Result Plus. [branch on N = 0]\n1) add 1 to cycles if branch occurs on same page \n2) add 2 to cycles if branch occurs to different page\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "00",
    "mnemonic": "BRK",
    "mode": "implied",
    "bytes": 1,
    "cycles": 7,
    "description": "Force Break. [interrupt, push PC+2, push SR]\n1) BRK initiates a software interrupt similar to a hardware interrupt (IRQ). The return address pushed to the stack is\n   PC+2, providing an extra byte of spacing for a break mark (identifying a reason for the break.)\n   The status register will be pushed to the stack with the break flag set to 1. However, when retrieved during RTI or by a PLP\n   instruction, the break flag will be ignored. The interrupt disable flag is not set automatically.\n\nN Z C I D V\n- - - 1 - -"
  },
  {
    "opcode": "50",
    "mnemonic": "BVC oper",
    "mode": "relative",
    "bytes": 2,
    "cycles": 2,
    "description": "Branch on Overflow Clear. [branch on V = 0]\n1) add 1 to cycles if branch occurs on same page \n2) add 2 to cycles if branch occurs to different page\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "70",
    "mnemonic": "BVS oper",
    "mode": "relative",
    "bytes": 2,
    "cycles": 2,
    "description": "Branch on Overflow Set. [branch on V = 1]\n1) add 1 to cycles if branch occurs on same page \n2) add 2 to cycles if branch occurs to different page\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "18",
    "mnemonic": "CLC",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Clear Carry Flag. [0 -> C]\n\nN Z C I D V\n- - 1 - - -"
  },
  {
    "opcode": "D8",
    "mnemonic": "CLD",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Clear Decimal Mode. [0 -> D]\n\nN Z C I D V\n- - - - 0 -"
  },
  {
    "opcode": "58",
    "mnemonic": "CLI",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Clear Interrupt Disable Bit. [0 -> I]\n\nN Z C I D V\n- - - 0 - -"
  },
  {
    "opcode": "B8",
    "mnemonic": "CLV",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Clear Overflow Flag. [0 -> V]\n\nN Z C I D V\n- - - - - 0"
  },
  {
    "opcode": "C9",
    "mnemonic": "CMP #oper",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "description": "Compare Memory with Accumulator. [A - M]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "C5",
    "mnemonic": "CMP oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 3,
    "description": "Compare Memory with Accumulator. [A - M]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "D5",
    "mnemonic": "CMP oper,X",
    "mode": "zeropage,X",
    "bytes": 2,
    "cycles": 4,
    "description": "Compare Memory with Accumulator. [A - M]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "CD",
    "mnemonic": "CMP oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 4,
    "description": "Compare Memory with Accumulator. [A - M]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "DD",
    "mnemonic": "CMP oper,X",
    "mode": "absolute,X",
    "bytes": 3,
    "cycles": 4,
    "description": "Compare Memory with Accumulator. [A - M]\n1) add 1 to cycles if page boundary is crossed\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "D9",
    "mnemonic": "CMP oper,Y",
    "mode": "absolute,Y",
    "bytes": 3,
    "cycles": 4,
    "description": "Compare Memory with Accumulator. [A - M]\n1) add 1 to cycles if page boundary is crossed\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "C1",
    "mnemonic": "CMP (oper,X)",
    "mode": "(indirect,X)",
    "bytes": 2,
    "cycles": 6,
    "description": "Compare Memory with Accumulator. [A - M]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "D1",
    "mnemonic": "CMP (oper),Y",
    "mode": "(indirect),Y",
    "bytes": 2,
    "cycles": 5,
    "description": "Compare Memory with Accumulator. [A - M]\n1) add 1 to cycles if page boundary is crossed\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "E0",
    "mnemonic": "CPX #oper",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "description": "Compare Memory and Index X. [X - M]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "E4",
    "mnemonic": "CPX oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 3,
    "description": "Compare Memory and Index X. [X - M]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "EC",
    "mnemonic": "CPX oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 4,
    "description": "Compare Memory and Index X. [X - M]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "C0",
    "mnemonic": "CPY #oper",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "description": "Compare Memory and Index Y. [Y - M]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "C4",
    "mnemonic": "CPY oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 3,
    "description": "Compare Memory and Index Y. [Y - M]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "CC",
    "mnemonic": "CPY oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 4,
    "description": "Compare Memory and Index Y. [Y - M]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "C6",
    "mnemonic": "DEC oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 5,
    "description": "Decrement Memory by One. [M - 1 -> M]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "D6",
    "mnemonic": "DEC oper,X",
    "mode": "zeropage,X",
    "bytes": 2,
    "cycles": 6,
    "description": "Decrement Memory by One. [M - 1 -> M]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "CE",
    "mnemonic": "DEC oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 6,
    "description": "Decrement Memory by One. [M - 1 -> M]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "DE",
    "mnemonic": "DEC oper,X",
    "mode": "absolute,X",
    "bytes": 3,
    "cycles": 7,
    "description": "Decrement Memory by One. [M - 1 -> M]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "CA",
    "mnemonic": "DEX",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Decrement Index X by One. [X - 1 -> X]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "88",
    "mnemonic": "DEY",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Decrement Index Y by One. [Y - 1 -> Y]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "49",
    "mnemonic": "EOR #oper",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "description": "Exclusive-OR Memory with Accumulator. [A EOR M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "45",
    "mnemonic": "EOR oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 3,
    "description": "Exclusive-OR Memory with Accumulator. [A EOR M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "55",
    "mnemonic": "EOR oper,X",
    "mode": "zeropage,X",
    "bytes": 2,
    "cycles": 4,
    "description": "Exclusive-OR Memory with Accumulator. [A EOR M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "4D",
    "mnemonic": "EOR oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 4,
    "description": "Exclusive-OR Memory with Accumulator. [A EOR M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "5D",
    "mnemonic": "EOR oper,X",
    "mode": "absolute,X",
    "bytes": 3,
    "cycles": 4,
    "description": "Exclusive-OR Memory with Accumulator. [A EOR M -> A]\n1) add 1 to cycles if page boundary is crossed)\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "59",
    "mnemonic": "EOR oper,Y",
    "mode": "absolute,Y",
    "bytes": 3,
    "cycles": 4,
    "description": "Exclusive-OR Memory with Accumulator. [A EOR M -> A]\n1) add 1 to cycles if page boundary is crossed)\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "41",
    "mnemonic": "EOR (oper,X)",
    "mode": "(indirect,X)",
    "bytes": 2,
    "cycles": 6,
    "description": "Exclusive-OR Memory with Accumulator. [A EOR M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "51",
    "mnemonic": "EOR (oper),Y",
    "mode": "(indirect),Y",
    "bytes": 2,
    "cycles": 5,
    "description": "Exclusive-OR Memory with Accumulator. [A EOR M -> A]\n1) add 1 to cycles if page boundary is crossed)\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "E6",
    "mnemonic": "INC oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 5,
    "description": "Increment Memory by One. [M + 1 -> M]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "F6",
    "mnemonic": "INC oper,X",
    "mode": "zeropage,X",
    "bytes": 2,
    "cycles": 6,
    "description": "Increment Memory by One. [M + 1 -> M]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "EE",
    "mnemonic": "INC oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 6,
    "description": "Increment Memory by One. [M + 1 -> M]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "FE",
    "mnemonic": "INC oper,X",
    "mode": "absolute,X",
    "bytes": 3,
    "cycles": 7,
    "description": "Increment Memory by One. [M + 1 -> M]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "E8",
    "mnemonic": "INX",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Increment Index X by One. [X + 1 -> X]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "C8",
    "mnemonic": "INY",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Increment Index Y by One. [Y + 1 -> Y]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "4C",
    "mnemonic": "JMP oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 3,
    "description": "Jump to New Location. [operand 1st byte -> PCL, operand 2nd byte -> PCH]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "6C",
    "mnemonic": "JMP (oper)",
    "mode": "indirect",
    "bytes": 3,
    "cycles": 5,
    "description": "Jump to New Location. [operand 1st byte -> PCL, operand 2nd byte -> PCH]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "20",
    "mnemonic": "JSR oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 6,
    "description": "Jump to New Location Saving Return Address. [push (PC+2), operand 1st byte -> PCL, operand 2nd byte -> PCH]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "A9",
    "mnemonic": "LDA #oper",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "description": "Load Accumulator with Memory. [M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "A5",
    "mnemonic": "LDA oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 3,
    "description": "Load Accumulator with Memory. [M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "B5",
    "mnemonic": "LDA oper,X",
    "mode": "zeropage,X",
    "bytes": 2,
    "cycles": 4,
    "description": "Load Accumulator with Memory. [M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "AD",
    "mnemonic": "LDA oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 4,
    "description": "Load Accumulator with Memory. [M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "BD",
    "mnemonic": "LDA oper,X",
    "mode": "absolute,X",
    "bytes": 3,
    "cycles": 4,
    "description": "Load Accumulator with Memory. [M -> A]\n1) add 1 to cycles if page boundary is crossed)\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "B9",
    "mnemonic": "LDA oper,Y",
    "mode": "absolute,Y",
    "bytes": 3,
    "cycles": 4,
    "description": "Load Accumulator with Memory. [M -> A]\n1) add 1 to cycles if page boundary is crossed)\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "A1",
    "mnemonic": "LDA (oper,X)",
    "mode": "(indirect,X)",
    "bytes": 2,
    "cycles": 6,
    "description": "Load Accumulator with Memory. [M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "B1",
    "mnemonic": "LDA (oper),Y",
    "mode": "(indirect),Y",
    "bytes": 2,
    "cycles": 5,
    "description": "Load Accumulator with Memory. [M -> A]\n1) add 1 to cycles if page boundary is crossed)\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "A2",
    "mnemonic": "LDX #oper",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "description": "Load Index X with Memory. [M -> X]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "A6",
    "mnemonic": "LDX oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 3,
    "description": "Load Index X with Memory. [M -> X]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "B6",
    "mnemonic": "LDX oper,Y",
    "mode": "zeropage,Y",
    "bytes": 2,
    "cycles": 4,
    "description": "Load Index X with Memory. [M -> X]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "AE",
    "mnemonic": "LDX oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 4,
    "description": "Load Index X with Memory. [M -> X]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "BE",
    "mnemonic": "LDX oper,Y",
    "mode": "absolute,Y",
    "bytes": 3,
    "cycles": 4,
    "description": "Load Index X with Memory. [M -> X]\n1) add 1 to cycles if page boundary is crossed)\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "A0",
    "mnemonic": "LDY #oper",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "description": "Load Index Y with Memory. [M -> Y]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "A4",
    "mnemonic": "LDY oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 3,
    "description": "Load Index Y with Memory. [M -> Y]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "B4",
    "mnemonic": "LDY oper,X",
    "mode": "zeropage,X",
    "bytes": 2,
    "cycles": 4,
    "description": "Load Index Y with Memory. [M -> Y]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "AC",
    "mnemonic": "LDY oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 4,
    "description": "Load Index Y with Memory. [M -> Y]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "BC",
    "mnemonic": "LDY oper,X",
    "mode": "absolute,X",
    "bytes": 3,
    "cycles": 4,
    "description": "Load Index Y with Memory. [M -> Y]\n1) add 1 to cycles if page boundary is crossed)\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "4A",
    "mnemonic": "LSR A",
    "mode": "accumulator",
    "bytes": 1,
    "cycles": 2,
    "description": "Shift One Bit Right (Memory or Accumulator). [0 -> [76543210] -> C]\n\nN Z C I D V\n0 + + - - -"
  },
  {
    "opcode": "46",
    "mnemonic": "LSR oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 5,
    "description": "Shift One Bit Right (Memory or Accumulator). [0 -> [76543210] -> C]\n\nN Z C I D V\n0 + + - - -"
  },
  {
    "opcode": "56",
    "mnemonic": "LSR oper,X",
    "mode": "zeropage,X",
    "bytes": 2,
    "cycles": 6,
    "description": "Shift One Bit Right (Memory or Accumulator). [0 -> [76543210] -> C]\n\nN Z C I D V\n0 + + - - -"
  },
  {
    "opcode": "4E",
    "mnemonic": "LSR oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 6,
    "description": "Shift One Bit Right (Memory or Accumulator). [0 -> [76543210] -> C]\n\nN Z C I D V\n0 + + - - -"
  },
  {
    "opcode": "5E",
    "mnemonic": "LSR oper,X",
    "mode": "absolute,X",
    "bytes": 3,
    "cycles": 7,
    "description": "Shift One Bit Right (Memory or Accumulator). [0 -> [76543210] -> C]\n\nN Z C I D V\n0 + + - - -"
  },
  {
    "opcode": "EA",
    "mnemonic": "NOP",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "No Operation. [---]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "09",
    "mnemonic": "ORA #oper",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "description": "OR Memory with Accumulator. [A OR M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "05",
    "mnemonic": "ORA oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 3,
    "description": "OR Memory with Accumulator. [A OR M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "15",
    "mnemonic": "ORA oper,X",
    "mode": "zeropage,X",
    "bytes": 2,
    "cycles": 4,
    "description": "OR Memory with Accumulator. [A OR M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "0D",
    "mnemonic": "ORA oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 4,
    "description": "OR Memory with Accumulator. [A OR M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "1D",
    "mnemonic": "ORA oper,X",
    "mode": "absolute,X",
    "bytes": 3,
    "cycles": 4,
    "description": "OR Memory with Accumulator. [A OR M -> A]\n1) add 1 to cycles if page boundary is crossed)\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "19",
    "mnemonic": "ORA oper,Y",
    "mode": "absolute,Y",
    "bytes": 3,
    "cycles": 4,
    "description": "OR Memory with Accumulator. [A OR M -> A]\n1) add 1 to cycles if page boundary is crossed)\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "01",
    "mnemonic": "ORA (oper,X)",
    "mode": "(indirect,X)",
    "bytes": 2,
    "cycles": 6,
    "description": "OR Memory with Accumulator. [A OR M -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "11",
    "mnemonic": "ORA (oper),Y",
    "mode": "(indirect),Y",
    "bytes": 2,
    "cycles": 5,
    "description": "OR Memory with Accumulator. [A OR M -> A]\n1) add 1 to cycles if page boundary is crossed)\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "48",
    "mnemonic": "PHA",
    "mode": "implied",
    "bytes": 1,
    "cycles": 3,
    "description": "Push Accumulator on Stack. [push A]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "08",
    "mnemonic": "PHP",
    "mode": "implied",
    "bytes": 1,
    "cycles": 3,
    "description": "Push Processor Status on Stack. [push A]\nThe status register will be pushed with the break flag and bit 5 set to 1.\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "68",
    "mnemonic": "PLA",
    "mode": "implied",
    "bytes": 1,
    "cycles": 4,
    "description": "Pull Accumulator from Stack. [pull A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "28",
    "mnemonic": "PLP",
    "mode": "implied",
    "bytes": 1,
    "cycles": 4,
    "description": "Pull Processor Status from Stack. [pull A]\nThe status register will be pulled with the break flag and bit 5 ignored.\n\nN Z C I D V\nfrom stack "
  },
  {
    "opcode": "2A",
    "mnemonic": "ROL A",
    "mode": "accumulator",
    "bytes": 1,
    "cycles": 2,
    "description": "Rotate One Bit Left (Memory or Accumulator). [C <- [76543210] <- C]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "26",
    "mnemonic": "ROL oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 5,
    "description": "Rotate One Bit Left (Memory or Accumulator). [C <- [76543210] <- C]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "36",
    "mnemonic": "ROL oper,X",
    "mode": "zeropage,X",
    "bytes": 2,
    "cycles": 6,
    "description": "Rotate One Bit Left (Memory or Accumulator). [C <- [76543210] <- C]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "2E",
    "mnemonic": "ROL oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 6,
    "description": "Rotate One Bit Left (Memory or Accumulator). [C <- [76543210] <- C]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "3E",
    "mnemonic": "ROL oper,X",
    "mode": "absolute,X",
    "bytes": 3,
    "cycles": 7,
    "description": "Rotate One Bit Left (Memory or Accumulator). [C <- [76543210] <- C]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "6A",
    "mnemonic": "ROR A",
    "mode": "accumulator",
    "bytes": 1,
    "cycles": 2,
    "description": "Rotate One Bit Right (Memory or Accumulator). [C -> [76543210] -> C]\n\nN Z C I D V\n+ + + - - -"
  },
  {
    "opcode": "40",
    "mnemonic": "RTI",
    "mode": "implied",
    "bytes": 1,
    "cycles": 6,
    "description": "Return from Interrupt. [pull SR, pull PC]\nThe status register is pulled with the break flag and bit 5 ignored.\nThen PC is pulled from the stack.\n\nN Z C I D V\nfrom stack"
  },
  {
    "opcode": "60",
    "mnemonic": "RTS",
    "mode": "implied",
    "bytes": 1,
    "cycles": 6,
    "description": "Return from Subroutine. [pull PC, PC+1 -> PC]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "E9",
    "mnemonic": "SBC #oper",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "description": "Subtract Memory from Accumulator with Borrow. [A - M - ~C -> A]\n\nN Z C I D V\n+ + + - - +"
  },
  {
    "opcode": "E5",
    "mnemonic": "SBC oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 3,
    "description": "Subtract Memory from Accumulator with Borrow. [A - M - ~C -> A]\n\nN Z C I D V\n+ + + - - +"
  },
  {
    "opcode": "F5",
    "mnemonic": "SBC oper,X",
    "mode": "zeropage,X",
    "bytes": 2,
    "cycles": 4,
    "description": "Subtract Memory from Accumulator with Borrow. [A - M - ~C -> A]\n\nN Z C I D V\n+ + + - - +"
  },
  {
    "opcode": "ED",
    "mnemonic": "SBC oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 4,
    "description": "Subtract Memory from Accumulator with Borrow. [A - M - ~C -> A]\n\nN Z C I D V\n+ + + - - +"
  },
  {
    "opcode": "FD",
    "mnemonic": "SBC oper,X",
    "mode": "absolute,X",
    "bytes": 3,
    "cycles": 4,
    "description": "Subtract Memory from Accumulator with Borrow. [A - M - ~C -> A]\n1) add 1 to cycles if page boundary is crossed)\n\nN Z C I D V\n+ + + - - +"
  },
  {
    "opcode": "F9",
    "mnemonic": "SBC oper,Y",
    "mode": "absolute,Y",
    "bytes": 3,
    "cycles": 4,
    "description": "Subtract Memory from Accumulator with Borrow. [A - M - ~C -> A]\n1) add 1 to cycles if page boundary is crossed)\n\nN Z C I D V\n+ + + - - +"
  },
  {
    "opcode": "E1",
    "mnemonic": "SBC (oper,X)",
    "mode": "(indirect,X)",
    "bytes": 2,
    "cycles": 6,
    "description": "Subtract Memory from Accumulator with Borrow. [A - M - ~C -> A]\n\nN Z C I D V\n+ + + - - +"
  },
  {
    "opcode": "F1",
    "mnemonic": "SBC (oper),Y",
    "mode": "(indirect),Y",
    "bytes": 2,
    "cycles": 5,
    "description": "Subtract Memory from Accumulator with Borrow. [A - M - ~C -> A]\n1) add 1 to cycles if page boundary is crossed)\n\nN Z C I D V\n+ + + - - +"
  },
  {
    "opcode": "38",
    "mnemonic": "SEC",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Set Carry Flag. [1 -> C]\n\nN Z C I D V\n- - 1 - - -"
  },
  {
    "opcode": "F8",
    "mnemonic": "SED",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Set Decimal Flag. [1 -> D]\n\nN Z C I D V\n- - - - 1 -"
  },
  {
    "opcode": "78",
    "mnemonic": "SEI",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Set Interrupt Disable Status. [1 -> I]\n\nN Z C I D V\n- - - 1 - -"
  },
  {
    "opcode": "85",
    "mnemonic": "STA oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 3,
    "description": "Store Accumulator in Memory. [A -> M]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "95",
    "mnemonic": "STA oper,X",
    "mode": "zeropage,X",
    "bytes": 2,
    "cycles": 4,
    "description": "Store Accumulator in Memory. [A -> M]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "8D",
    "mnemonic": "STA oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 4,
    "description": "Store Accumulator in Memory. [A -> M]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "9D",
    "mnemonic": "STA oper,X",
    "mode": "absolute,X",
    "bytes": 3,
    "cycles": 5,
    "description": "Store Accumulator in Memory. [A -> M]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "99",
    "mnemonic": "STA oper,Y",
    "mode": "absolute,Y",
    "bytes": 3,
    "cycles": 5,
    "description": "Store Accumulator in Memory. [A -> M]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "81",
    "mnemonic": "STA (oper,X)",
    "mode": "(indirect,X)",
    "bytes": 2,
    "cycles": 6,
    "description": "Store Accumulator in Memory. [A -> M]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "91",
    "mnemonic": "STA (oper),Y",
    "mode": "(indirect),Y",
    "bytes": 2,
    "cycles": 6,
    "description": "Store Accumulator in Memory. [A -> M]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "86",
    "mnemonic": "STX oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 3,
    "description": "Store Index X in Memory. [X -> M]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "96",
    "mnemonic": "STX oper,Y",
    "mode": "zeropage,Y",
    "bytes": 2,
    "cycles": 4,
    "description": "Store Index X in Memory. [X -> M]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "8E",
    "mnemonic": "STX oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 4,
    "description": "Store Index X in Memory. [X -> M]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "84",
    "mnemonic": "STY oper",
    "mode": "zeropage",
    "bytes": 2,
    "cycles": 3,
    "description": "Store Index Y in Memory. [Y -> M]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "94",
    "mnemonic": "STY oper,X",
    "mode": "zeropage,X",
    "bytes": 2,
    "cycles": 4,
    "description": "Store Index Y in Memory. [Y -> M]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "8C",
    "mnemonic": "STY oper",
    "mode": "absolute",
    "bytes": 3,
    "cycles": 4,
    "description": "Store Index Y in Memory. [Y -> M]\n\nN Z C I D V\n- - - - - -"
  },
  {
    "opcode": "AA",
    "mnemonic": "TAX",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Transfer Accumulator to Index X. [A -> X]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "A8",
    "mnemonic": "TAY",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Transfer Accumulator to Index Y. [A -> Y]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "BA",
    "mnemonic": "TSX",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Transfer Stack Pointer to Index X. [SP -> X]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "8A",
    "mnemonic": "TXA",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Transfer Index X to Accumulator. [X -> A]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "9A",
    "mnemonic": "TXS",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Transfer Index X to Stack Register. [X -> SP]\n\nN Z C I D V\n+ + - - - -"
  },
  {
    "opcode": "98",
    "mnemonic": "TYA",
    "mode": "implied",
    "bytes": 1,
    "cycles": 2,
    "description": "Transfer Index Y to Accumulator. [Y -> A]\n\nN Z C I D V\n+ + - - - -"
  }
]
"#;
