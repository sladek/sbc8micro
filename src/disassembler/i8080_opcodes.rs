pub static OPCODES: &str = r#"
[
  {
    "opcode": "CE",
    "mnemonic": "ACI data",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "states": 7,
    "description": "Content of the CY flag are added to the contents of the accumulator. \nThe result is placed in the accumulator. [(A) <- (A) + (byte 2) + (CY)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "88",
    "mnemonic": "ADC B",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register B and the content of the carry bit are added to the content of the accumulator. \nThe result is placed in the accumulator. [(A) <- (A) + (B) + (CY)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "89",
    "mnemonic": "ADC C",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register C and the content of the carry bit are added to the content of the accumulator. \nThe result is placed in the accumulator. [(A) <- (A) + (C) + (CY)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "8A",
    "mnemonic": "ADC D",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register D and the content of the carry bit are added to the content of the accumulator. \nThe result is placed in the accumulator. [(A) <- (A) + (D) + (CY)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "8B",
    "mnemonic": "ADC E",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register E and the content of the carry bit are added to the content of the accumulator. \nThe result is placed in the accumulator. [(A) <- (A) + (E) + (CY)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "8C",
    "mnemonic": "ADC H",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register H and the content of the carry bit are added to the content of the accumulator. \nThe result is placed in the accumulator. [(A) <- (A) + (H) + (CY)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "8D",
    "mnemonic": "ADC L",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register L and the content of the carry bit are added to the content of the accumulator. \nThe result is placed in the accumulator. [(A) <- (A) + (L) + (CY)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "8E",
    "mnemonic": "ADC M",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of the memory location whose address is contained in the H and L registers \nand the content of the CY flag are added to the accumulator. \nThe result is placed in the accumulator [(A) <- (A) + ((H) (L)) + (CY)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "8F",
    "mnemonic": "ADC A",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register A and the content of the carry bit are added to the content of the accumulator. \nThe result is placed in the accumulator. [(A) <- (A) + (A) + (CY)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "80",
    "mnemonic": "ADD B",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register B is added to the content of the accumulator.\n The result is placed in the accumulator. [(A) <- (A) + (B)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "81",
    "mnemonic": "ADD C",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register C is added to the content of the accumulator.\n The result is placed in the accumulator. [(A) <- (A) + (C)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "82",
    "mnemonic": "ADD D",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register D is added to the content of the accumulator.\n The result is placed in the accumulator. [(A) <- (A) + (D)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "83",
    "mnemonic": "ADD E",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register E is added to the content of the accumulator.\n The result is placed in the accumulator. [(A) <- (A) + (E)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "84",
    "mnemonic": "ADD H",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register H is added to the content of the accumulator.\n The result is placed in the accumulator. [(A) <- (A) + (H)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "85",
    "mnemonic": "ADD L",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register L is added to the content of the accumulator.\n The result is placed in the accumulator. [(A) <- (A) + (L)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "86",
    "mnemonic": "ADD M",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 4,
    "description": "The content of the memory location whose address is contained in the H and L registers \nis added to the content of the accumulator. The result is placed in the accumulator. \n[(A) <- (A) + ((H) (L))] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "87",
    "mnemonic": "ADD A",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register A is added to the content of the accumulator. \nThe result is placed in the accumulator. [(A) <- (A) + (A)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "C6",
    "mnemonic": "ADI data",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "states": 7,
    "description": "The content of the second byte of the instruction is added to the content of the accumulator.\nThe result is placed in the accumulator. [(A) <- (A) + (byte 2)] \n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "A0",
    "mnemonic": "ANA B",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register B is logically anded with the content of the accumulator.\nThe result is placed in the accumulator. The CY flag is cleared.\n[(A) <- (A) AND (B)].\n\nZ S P CY AC\nx x x 0  x"
  },
  {
    "opcode": "A1",
    "mnemonic": "ANA C",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register C is logically anded with the content of the accumulator.\nThe result is placed in the accumulator. The CY flag is cleared.\n[(A) <- (A) AND (C)].\n\nZ S P CY AC\nx x x 0  x"
  },
  {
    "opcode": "A2",
    "mnemonic": "ANA D",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register D is logically anded with the content of the accumulator.\nThe result is placed in the accumulator. The CY flag is cleared.\n[(A) <- (A) AND (D)].\n\nZ S P CY AC\nx x x 0  x"
  },
  {
    "opcode": "A3",
    "mnemonic": "ANA E",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register E is logically anded with the content of the accumulator.\nThe result is placed in the accumulator. The CY flag is cleared.\n[(A) <- (A) AND (E)].\n\nZ S P CY AC\nx x x 0  x"
  },
  {
    "opcode": "A4",
    "mnemonic": "ANA H",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register H is logically anded with the content of the accumulator.\nThe result is placed in the accumulator. The CY flag is cleared.\n[(A) <- (A) AND (H)].\n\nZ S P CY AC\nx x x 0  x"
  },
  {
    "opcode": "A5",
    "mnemonic": "ANA L",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register L is logically anded with the content of the accumulator.\nThe result is placed in the accumulator. The CY flag is cleared.\n[(A) <- (A) AND (L)].\n\nZ S P CY AC\nx x x 0  x"
  },
  {
    "opcode": "A6",
    "mnemonic": "ANA M",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The contents of the memory tocation whose address is contained in the H and L registers \nis logically anded with the content of the accumulator. \nThe result is placed in the accumulator. The CY ftag is cteared.\n[(A) <- (A) AND ((H)(L))].\n\nZ S P CY AC\nx x x 0  x"
  },
  {
    "opcode": "A7",
    "mnemonic": "ANA A",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register A is logically anded with the content of the accumulator.\nThe result is placed in the accumulator. The CY flag is cleared.\n[(A) <- (A) AND (A) ].\n\nZ S P CY AC\nx x x 0  x"
  },
  {
    "opcode": "E6",
    "mnemonic": "ANI data",
    "mode": "register",
    "bytes": 2,
    "cycles": 2,
    "states": 7,
    "description": "The content of the second byte of the instruction is logically anded with \nthe contents of the accumulator. The result is placed in the accumulator. \nThe CY and AC flags are cleared. [(A) <- (A) AND (byte 2)].\n\nZ S P CY AC\nx x x 0  0"
  },
  {
    "opcode": "2F",
    "mnemonic": "CMA",
    "mode": "accumulator",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The contents of the accumulator are complemented (zero bits become 1, one bits become 0).\nNo flags are affected. [(A) <- (~A)] \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "3F",
    "mnemonic": "CMC",
    "mode": "carry",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The CY flag is complemented. No other flags are affected. [(A) <- (~A)] \n\nN Z S P CY AC\n- - - - -  x"
  },
  {
    "opcode": "B8",
    "mnemonic": "CMP B",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register B is subtracted from the accumulator. The accumulator \nremains unchanged. The condition flags are set as a result of the subtraction. \nThe Z ftag is set to 1 if (A) = (B). The CY flag is set to 1 if (A < (B). \n[(A) - (B)].\n\nZ S P CY AC\nx x x x  x"
  },
  {
    "opcode": "B9",
    "mnemonic": "CMP C",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register C is subtracted from the accumulator. The accumulator \nremains unchanged. The condition flags are set as a result of the subtraction. \nThe Z ftag is set to 1 if (A) = (C). The CY flag is set to 1 if (A < (C). \n[(A) - (C)].\n\nZ S P CY AC\nx x x x  x"
  },
  {
    "opcode": "BA",
    "mnemonic": "CMP D",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register D is subtracted from the accumulator. The accumulator \nremains unchanged. The condition flags are set as a result of the subtraction. \nThe Z ftag is set to 1 if (A) = (D). The CY flag is set to 1 if (A < (D). \n[(A) - (D)].\n\nZ S P CY AC\nx x x x  x"
  },
  {
    "opcode": "BB",
    "mnemonic": "CMP E",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register E is subtracted from the accumulator. The accumulator \nremains unchanged. The condition flags are set as a result of the subtraction. \nThe Z ftag is set to 1 if (A) = (E). The CY flag is set to 1 if (A < (E). \n[(A) - (E)].\n\nZ S P CY AC\nx x x x  x"
  },
  {
    "opcode": "BC",
    "mnemonic": "CMP H",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register H is subtracted from the accumulator. The accumulator \nremains unchanged. The condition flags are set as a result of the subtraction. \nThe Z ftag is set to 1 if (A) = (H). The CY flag is set to 1 if (A < (H). \n[(A) - (H)].\n\nZ S P CY AC\nx x x x  x"
  },
  {
    "opcode": "BD",
    "mnemonic": "CMP L",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register L is subtracted from the accumulator. The accumulator \nremains unchanged. The condition flags are set as a result of the subtraction. \nThe Z ftag is set to 1 if (A) = (L). The CY flag is set to 1 if (A < (L). \n[(A) - (L)].\n\nZ S P CY AC\nx x x x  x"
  },
  {
    "opcode": "BE",
    "mnemonic": "CMP M",
    "mode": "register",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of the memory location whose address is contained in the H and L \nregisters is subtracted from the accumulator. The accumulator remains unchanged. \nThe condition flags are set as a result of the subtraction. The Z flag is set \nto 1 if (A) = ((H) (L)). The CY flag is set to 1  if (A) < ((H)(L)). [(A) - ((H)(L))].\n\nZ S P CY AC\nx x x x  x"
  },
  {
    "opcode": "BF",
    "mnemonic": "CMP A",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register A is subtracted from the accumulator. The accumulator \nremains unchanged. The condition flags are set as a result of the subtraction. \nThe Z ftag is set to 1. The CY flag is set to 0. \n[(A) - (A)].\n\nZ S P CY AC\n1 x x x  0"
  },
  {
    "opcode": "FE",
    "mnemonic": "CPI data",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "states": 7,
    "description": "The content of the second byte of the instruction is subtracted from the accumulator . \nThe condition flags are set by the result of the subtraction. The Z flag is set to 1 \nif (A) = (byte 2). The CY flag is set to 1 if (A) < (byte 2). [(A) - (byte 2)].\n\nZ S P CY AC\nx x x x  x"
  },
  {
    "opcode": "C4",
    "mnemonic": "CNZ address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 5,
    "states": 17,
    "description": "If the specified condition is true, the CALL instruction are performed; \notherwise, control continues sequentially. Condition fiags are not affected. \n[if (Z = 0),((SP) -1 ) <- (PCH), ((SP) -2 ) <- (PCL)\n (SP) <- (SP) - 2, (PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "CC",
    "mnemonic": "CZ address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 5,
    "states": 17,
    "description": "If the specified condition is true, the CALL instruction are performed; \notherwise, control continues sequentially. Condition fiags are not affected. \n[if (Z),((SP) -1 ) <- (PCH), ((SP) -2 ) <- (PCL)\n (SP) <- (SP) - 2, (PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "D4",
    "mnemonic": "CNC address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 5,
    "states": 17,
    "description": "If the specified condition is true, the CALL instruction are performed; \notherwise, control continues sequentially. Condition fiags are not affected. \n[if (C = 0),((SP) -1 ) <- (PCH), ((SP) -2 ) <- (PCL)\n (SP) <- (SP) - 2, (PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "DC",
    "mnemonic": "CC address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 5,
    "states": 17,
    "description": "If the specified condition is true, the CALL instruction are performed; \notherwise, control continues sequentially. Condition fiags are not affected. \n[if (C),((SP) -1 ) <- (PCH), ((SP) -2 ) <- (PCL)\n (SP) <- (SP) - 2, (PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "E4",
    "mnemonic": "CPO address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 5,
    "states": 17,
    "description": "If the specified condition is true, the CALL instruction are performed; \notherwise, control continues sequentially. Condition fiags are not affected. \n[if (P = 0),((SP) -1 ) <- (PCH), ((SP) -2 ) <- (PCL)\n (SP) <- (SP) - 2, (PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "EC",
    "mnemonic": "CPE address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 5,
    "states": 17,
    "description": "If the specified condition is true, the CALL instruction are performed; \notherwise, control continues sequentially. Condition fiags are not affected. \n[if (P),((SP) -1 ) <- (PCH), ((SP) -2 ) <- (PCL)\n (SP) <- (SP) - 2, (PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "F4",
    "mnemonic": "CP address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 5,
    "states": 17,
    "description": "If the specified condition is true, the CALL instruction are performed; \notherwise, control continues sequentially. Condition fiags are not affected. \n[if (S = 0),((SP) -1 ) <- (PCH), ((SP) -2 ) <- (PCL)\n (SP) <- (SP) - 2, (PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "FC",
    "mnemonic": "CM address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 5,
    "states": 17,
    "description": "If the specified condition is true, the CALL instruction are performed; \notherwise, control continues sequentially. Condition fiags are not affected. \n[if (S),((SP) -1 ) <- (PCH), ((SP) -2 ) <- (PCL)\n (SP) <- (SP) - 2, (PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "CD",
    "mnemonic": "CALL address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 5,
    "states": 17,
    "description": "Call subroutine unconditionally. \n[((SP) -1 ) <- (PCH), ((SP) -2 ) <- (PCL)\n (SP) <- (SP) - 2\n (PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "27",
    "mnemonic": "DAA",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The eight-bit number in the accumulator is adjusted to form two \nfour-bit Binary-Coded-Decimal digits.\n\nN Z S P CY AC\nx x x x x  x"
  },
  {
    "opcode": "09",
    "mnemonic": "DAD B",
    "mode": "register",
    "bytes": 1,
    "cycles": 3,
    "states": 10,
    "description": "The content of the register pair BC is added to the content of the register \npair H and L. The result is placed in the register pair H and L. \nNote: Only the CY flag is affected. It is set if there is a carry out of the \ndouble precision add; otherwise it is reset. [(H)(L) <- (H)(L ) + (B)(C)] \n\nN Z S P CY AC\n- - - - x  -"
  },
  {
    "opcode": "19",
    "mnemonic": "DAD D",
    "mode": "register",
    "bytes": 1,
    "cycles": 3,
    "states": 10,
    "description": "The content of the register pair DE is added to the content of the register \npair H and L. The result is placed in the register pair H and L. \nNote: Only the CY flag is affected. It is set if there is a carry out of the \ndouble precision add; otherwise it is reset. [(H)(L) <- (H)(L ) + (D)(E)] \n\nN Z S P CY AC\n- - - - x  -"
  },
  {
    "opcode": "29",
    "mnemonic": "DAD H",
    "mode": "register",
    "bytes": 1,
    "cycles": 3,
    "states": 10,
    "description": "The content of the register pair HL is added to the content of the register \npair H and L. The result is placed in the register pair H and L. \nNote: Only the CY flag is affected. It is set if there is a carry out of the \ndouble precision add; otherwise it is reset. [(H)(L) <- (H)(L ) + (H)(L)] \n\nN Z S P CY AC\n- - - - x  -"
  },
  {
    "opcode": "39",
    "mnemonic": "DAD SP",
    "mode": "register",
    "bytes": 1,
    "cycles": 3,
    "states": 10,
    "description": "The content of the register SP is added to the content of the register \npair H and L. The result is placed in the register pair H and L. \nNote: Only the CY flag is affected. It is set if there is a carry out of the \ndouble precision add; otherwise it is reset. [(H)(L) <- (H)(L ) + (SP)] \n\nN Z S P CY AC\n- - - - x  -"
  },
  {
    "opcode": "05",
    "mnemonic": "DCR B",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register B is decremented by one. \nNote: All condition flags except CY are affected.\n[(B) <- (B) -1]\n\nZ S P CY AC\nx x x -  x"
  },
  {
    "opcode": "0D",
    "mnemonic": "DCR C",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register C is decremented by one. \nNote: All condition flags except CY are affected.\n[(C) <- (C) -1]\n\nZ S P CY AC\nx x x -  x"
  },
  {
    "opcode": "15",
    "mnemonic": "DCR D",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register D is decremented by one. \nNote: All condition flags except CY are affected.\n[(D) <- (D) -1]\n\nZ S P CY AC\nx x x -  x"
  },
  {
    "opcode": "1D",
    "mnemonic": "DCR E",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register E is decremented by one. \nNote: All condition flags except CY are affected.\n[(E) <- (E) -1]\n\nZ S P CY AC\nx x x -  x"
  },
  {
    "opcode": "25",
    "mnemonic": "DCR H",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register L is decremented by one. \nNote: All condition flags except CY are affected.\n[(H) <- (H) -1]\n\nZ S P CY AC\nx x x -  x"
  },
  {
    "opcode": "2D",
    "mnemonic": "DCR L",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register L is decremented by one. \nNote: All condition flags except CY are affected.\n[(L) <- (L) -1]\n\nZ S P CY AC\nx x x -  x"
  },
  {
    "opcode": "35",
    "mnemonic": "DCR M",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 3,
    "states": 10,
    "description": "The content of the memory location whose address is contained in \nthe H and L registers is decremented by one. \nNote: All condition flags except CY are affected. \n[(H)(L)) <- ((H)(L))- 1]\n\nZ S P CY AC\nx x x -  x"
  },
  {
    "opcode": "3D",
    "mnemonic": "DCR A",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register A is decremented by one. \nNote: All condition flags except CY are affected.\n[(A) <- (A) -1]\n\nZ S P CY AC\nx x x -  x"
  },
  {
    "opcode": "0B",
    "mnemonic": "DCX B",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of the register pair BC is decremented by one. \nNote: No condition flags are affected. \n[(B)(C) <- (B)(C) - 1]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "1B",
    "mnemonic": "DCX D",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of the register pair DE is decremented by one. \nNote: No condition flags are affected. \n[(D)(E) <- (D)(E) - 1]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "2B",
    "mnemonic": "DCX H",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of the register pair HL is decremented by one. \nNote: No condition flags are affected. \n[(H)(L) <- (H)(L) - 1]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "3B",
    "mnemonic": "DCX SP",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of the register SP is decremented by one. \nNote: No condition flags are affected. \n[(SP) <- (SP) - 1]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "DB",
    "mnemonic": "IN port",
    "mode": "direct",
    "bytes": 2,
    "cycles": 3,
    "states": 10,
    "description": "The data placed on the eight bit bi-directional data bus by \nthe specified port is moved to register A.\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "04",
    "mnemonic": "INR B",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of the register B is incremented by one. \nNote: All condition flags except CY are affected. \n[(B) <- (B) + 1]\n\nZ S P CY AC\nx x x -  x"
  },
  {
    "opcode": "0C",
    "mnemonic": "INR C",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of the register C is incremented by one. \nNote: All condition flags except CY are affected. \n[(C) <- (C) + 1] \n\nZ S P CY AC\nx x x -  x"
  },
  {
    "opcode": "14",
    "mnemonic": "INR D",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of the register D is incremented by one. \nNote: All condition flags except CY are affected. \n[(D) <- (D) + 1] \n\nZ S P CY AC\nx x x -  x"
  },
  {
    "opcode": "1C",
    "mnemonic": "INR E",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of the register E is incremented by one. \nNote: All condition flags except CY are affected. \n[(E) <- (E) + 1] \n\nZ S P CY AC\nx x x -  x"
  },
  {
    "opcode": "24",
    "mnemonic": "INR H",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of the register H is incremented by one. \nNote: All condition flags except CY are affected. \n[(H) <- (H) + 1] \n\nZ S P CY AC\nx x x -  x"
  },
  {
    "opcode": "2C",
    "mnemonic": "INR L",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of the register L is incremented by one. \nNote: All condition flags except CY are affected. \n[(L) <- (L) + 1] \n\nZ S P CY AC\nx x x -  x"
  },
  {
    "opcode": "34",
    "mnemonic": "INR M",
    "mode": "register",
    "bytes": 3,
    "cycles": 3,
    "states": 10,
    "description": "The content of the memory location whose address is contained \nin the H and L registers is incremented by one. \nNote: All condition flags except CY areaffected.\n[((H)(L)) <- ((H)(L)) + 1] \n\nZ S P CY AC\nx x x -  x"
  },
  {
    "opcode": "3C",
    "mnemonic": "INR A",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of the register A is incremented by one. \nNote: All condition flags except CY are affected. \n[(A) <- (A) + 1] \n\nZ S P CY AC\nx x x -  x"
  },
  {
    "opcode": "03",
    "mnemonic": "INX B",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of the register pair BC is incremented by one. \nNote: No condition flags are affected. \n[(B)(C) <- (B)(C) + 1].\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "13",
    "mnemonic": "INX D",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of the register pair DE is incremented by one. \nNote: No condition flags are affected. \n[(D)(E) <- (D)(E) + 1].\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "23",
    "mnemonic": "INX H",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of the register pair HL is incremented by one. \nNote: No condition flags are affected. \n[(H)(L) <- (H)(L) + 1].\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "33",
    "mnemonic": "INX SP",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of the register SP is incremented by one. \nNote: No condition flags are affected. \n[(SP) <- (SP) + 1].\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "C2",
    "mnemonic": "JNZ address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 3,
    "states": 10,
    "description": "If the specified condition is true, control is transferred to the instruction \nwhose address is specified in byte 3 and byte 2 of the current instruction; \notherwise, control continues sequentially. Condition fiags are not affected. \n[if (Z = 0),(PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "CA",
    "mnemonic": "JZ address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 3,
    "states": 10,
    "description": "If the specified condition is true, control is transferred to the instruction \nwhose address is specified in byte 3 and byte 2 of the current instruction; \notherwise, control continues sequentially. Condition fiags are not affected. \n[if (Z),(PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "D2",
    "mnemonic": "JNC address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 3,
    "states": 10,
    "description": "If the specified condition is true, control is transferred to the instruction \nwhose address is specified in byte 3 and byte 2 of the current instruction; \notherwise, control continues sequentially. Condition fiags are not affected. \n[if (C = 0),(PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "DA",
    "mnemonic": "JC address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 3,
    "states": 10,
    "description": "If the specified condition is true, control is transferred to the instruction \nwhose address is specified in byte 3 and byte 2 of the current instruction; \notherwise, control continues sequentially. Condition fiags are not affected. \n[if (C),(PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "E2",
    "mnemonic": "JPO address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 3,
    "states": 10,
    "description": "If the specified condition is true, control is transferred to the instruction \nwhose address is specified in byte 3 and byte 2 of the current instruction; \notherwise, control continues sequentially. Condition fiags are not affected. \n[if (P = 0),(PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "EA",
    "mnemonic": "JPE address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 3,
    "states": 10,
    "description": "If the specified condition is true, control is transferred to the instruction \nwhose address is specified in byte 3 and byte 2 of the current instruction; \notherwise, control continues sequentially. Condition fiags are not affected. \n[if (P),(PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "F2",
    "mnemonic": "JP address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 3,
    "states": 10,
    "description": "If the specified condition is true, control is transferred to the instruction \nwhose address is specified in byte 3 and byte 2 of the current instruction; \notherwise, control continues sequentially. Condition fiags are not affected. \n[if (S = 0),(PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "FA",
    "mnemonic": "JM address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 3,
    "states": 10,
    "description": "If the specified condition is true, control is transferred to the instruction \nwhose address is specified in byte 3 and byte 2 of the current instruction; \notherwise, control continues sequentially. Condition fiags are not affected. \n[if (S),(PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "C3",
    "mnemonic": "JMP address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 3,
    "states": 10,
    "description": "Control is transferred to the instruction whose address is specified \nin byte 3 and byte 2 of the current instruction; otherwise, control \n continues sequentially. Condition fiags are not affected. \n[(PC) <- (byte 3) (byte 2)]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "3A",
    "mnemonic": "LDA addr",
    "mode": "direct",
    "bytes": 3,
    "cycles": 4,
    "states": 13,
    "description": "The content of the memory location, whose address is specified in byte 2\nand byte 3 of the instruction, is moved to register A. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "0A",
    "mnemonic": "LDAX B",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of the memory location, whose address is in the register pair rp, is moved to register A.\nNote: only register pairs rp=B (registers B and C) or rp=D (registers D and E) may be specified.\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "1A",
    "mnemonic": "LDAX D",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of the memory location, whose address is in the register pair rp, is moved to register A.\nNote: only register pairs rp=B (registers B and C) or rp=D (registers D and E) may be specified.\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "2A",
    "mnemonic": "LHLD addr",
    "mode": "direct",
    "bytes": 3,
    "cycles": 5,
    "states": 16,
    "description": "The content of the memory location, whose address is specified in byte 2\nand byte 3 of the instruction, is moved to register L.\nThe content of the memory location at the succeeding address is moved to register H.\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "01",
    "mnemonic": "LXI B,address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 2,
    "states": 7,
    "description": "Byte 3 of the instruction is moved into the high-order register (B) of the register pair BC.\nByte 2 of the instruction is moved into the low-order register (C) of the register pair BC.\n[(B)<- (byte 3), (C) <- (byte 2].\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "11",
    "mnemonic": "LXI D,address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 2,
    "states": 7,
    "description": "Byte 3 of the instruction is moved into the high-order register (D) of the register pair DE.\nByte 2 of the instruction is moved into the low-order register (E) of the register pair DE.\n[(D)<- (byte 3), (E) <- (byte 2].\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "21",
    "mnemonic": "LXI H,address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 2,
    "states": 7,
    "description": "Byte 3 of the instruction is moved into the high-order register (H) of the register pair HL.\nByte 2 of the instruction is moved into the low-order register (L) of the register pair HL.\n[(H)<- (byte 3), (L) <- (byte 2].\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "31",
    "mnemonic": "LXI SP,address",
    "mode": "immediate",
    "bytes": 3,
    "cycles": 2,
    "states": 7,
    "description": "Byte 3 of the instruction is moved into the high-order register (SP hi) of the register SP.\nByte 2 of the instruction is moved into the low-order register (SP low) of the register SP.\n[(SP hi)<- (byte 3), (SP low) <- (byte 2].\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "06",
    "mnemonic": "MVI B,data",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "states": 7,
    "description": "The content of byte 2 of the instruction is moved to B register.\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "0E",
    "mnemonic": "MVI C,data",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "states": 7,
    "description": "The content of byte 2 of the instruction is moved to C register.\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "16",
    "mnemonic": "MVI D,data",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "states": 7,
    "description": "The content of byte 2 of the instruction is moved to D register.\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "1E",
    "mnemonic": "MVI E,data",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "states": 7,
    "description": "The content of byte 2 of the instruction is moved to E register.\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "26",
    "mnemonic": "MVI H,data",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "states": 7,
    "description": "The content of byte 2 of the instruction is moved to H register.\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "2E",
    "mnemonic": "MVI L,data",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "states": 7,
    "description": "The content of byte 2 of the instruction is moved to L register.\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "36",
    "mnemonic": "MVI M,data",
    "mode": "register indirect",
    "bytes": 2,
    "cycles": 3,
    "states": 10,
    "description": "The content of byte 2 of the instruction is moved to the memory location whose address is in registers H and L.\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "3E",
    "mnemonic": "MVI A,data",
    "mode": "immediate",
    "bytes": 2,
    "cycles": 2,
    "states": 7,
    "description": "The content of byte 2 of the instruction is moved to A register.\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "78",
    "mnemonic": "MOV A,B",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register A is moved to register A. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "79",
    "mnemonic": "MOV A,C",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register C is moved to register A. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "7A",
    "mnemonic": "MOV A,D",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register D is moved to register A. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "7B",
    "mnemonic": "MOV A,E",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register E is moved to register A. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "7C",
    "mnemonic": "MOV A,H",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register H is moved to register A. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "7D",
    "mnemonic": "MOV A,L",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register L is moved to register A. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "7E",
    "mnemonic": "MOV A,M",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of the memory location, whose address is in registers H and L, is moved to register A. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "7F",
    "mnemonic": "MOV A,A",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register A is moved to register A. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "40",
    "mnemonic": "MOV B,B",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register B is moved to register B. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "41",
    "mnemonic": "MOV B,C",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register C is moved to register B. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "42",
    "mnemonic": "MOV B,D",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register D is moved to register B. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "43",
    "mnemonic": "MOV B,E",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register E is moved to register B. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "44",
    "mnemonic": "MOV B,H",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register H is moved to register B. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "45",
    "mnemonic": "MOV B,L",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register L is moved to register B. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "46",
    "mnemonic": "MOV B,M",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of the memory location, whose address is in registers H and L, is moved to register B. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "47",
    "mnemonic": "MOV B,A",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register A is moved to register B. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "48",
    "mnemonic": "MOV C,B",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register B is moved to register C. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "49",
    "mnemonic": "MOV C,C",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register C is moved to register C. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "4A",
    "mnemonic": "MOV C,D",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register D is moved to register C. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "4B",
    "mnemonic": "MOV C,E",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register E is moved to register C. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "4C",
    "mnemonic": "MOV C,H",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register H is moved to register C. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "4D",
    "mnemonic": "MOV C,L",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register L is moved to register C. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "4E",
    "mnemonic": "MOV C,M",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of the memory location, whose address is in registers H and L, is moved to register C.\n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "4F",
    "mnemonic": "MOV C,A",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register A is moved to register C. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "50",
    "mnemonic": "MOV D,B",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register B is moved to register D. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "51",
    "mnemonic": "MOV D,C",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register C is moved to register D. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "52",
    "mnemonic": "MOV D,D",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register D is moved to register D. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "53",
    "mnemonic": "MOV D,E",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register E is moved to register D. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "54",
    "mnemonic": "MOV D,H",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register H is moved to register D. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "55",
    "mnemonic": "MOV D,L",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register L is moved to register D. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "56",
    "mnemonic": "MOV D,M",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of the memory location, whose address is in registers H and L, is moved to register D. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "57",
    "mnemonic": "MOV D,A",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register A is moved to register D. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "58",
    "mnemonic": "MOV E,B",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register E is moved to register E. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "59",
    "mnemonic": "MOV E,C",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register C is moved to register E. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "5A",
    "mnemonic": "MOV E,D",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register E is moved to register E. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "5B",
    "mnemonic": "MOV E,E",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register E is moved to register E. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "5C",
    "mnemonic": "MOV E,H",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register H is moved to register E. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "5D",
    "mnemonic": "MOV E,L",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register L is moved to register E. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "5E",
    "mnemonic": "MOV E,M",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of the memory location, whose address is in registers H and L, is moved to register E. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "5F",
    "mnemonic": "MOV E,A",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register E is moved to register E. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "60",
    "mnemonic": "MOV H,B",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register B is moved to register H. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "61",
    "mnemonic": "MOV H,C",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register C is moved to register H. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "62",
    "mnemonic": "MOV H,D",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register D is moved to register H. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "63",
    "mnemonic": "MOV H,E",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register E is moved to register H. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "64",
    "mnemonic": "MOV H,H",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register H is moved to register H. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "65",
    "mnemonic": "MOV H,L",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register L is moved to register H. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "66",
    "mnemonic": "MOV H,M",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of the memory location, whose address is in registers H and L, is moved to register H. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "67",
    "mnemonic": "MOV H,A",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register A is moved to register H. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "68",
    "mnemonic": "MOV L,B",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register B is moved to register L. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "69",
    "mnemonic": "MOV L,C",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register C is moved to register L. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "6A",
    "mnemonic": "MOV L,D",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register D is moved to register L. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "6B",
    "mnemonic": "MOV L,E",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register E is moved to register L. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "6C",
    "mnemonic": "MOV L,H",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register H is moved to register L. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "6D",
    "mnemonic": "MOV L,L",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register L is moved to register L. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "6E",
    "mnemonic": "MOV L,M",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of the memory location, whose address is in registers H and L, is moved to register L. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "6F",
    "mnemonic": "MOV L,A",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 5,
    "description": "The content of register A is moved to register L. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "70",
    "mnemonic": "MOV M,B",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of register B is moved to the memory location whose address is in registers H and L. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "71",
    "mnemonic": "MOV M,C",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of register C is moved to the memory location whose address is in registers H and L. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "72",
    "mnemonic": "MOV M,D",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of register D is moved to the memory location whose address is in registers H and L. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "73",
    "mnemonic": "MOV M,E",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of register E is moved to the memory location whose address is in registers H and L. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "74",
    "mnemonic": "MOV M,H",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of register H is moved to the memory location whose address is in registers H and L. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "75",
    "mnemonic": "MOV M,L",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of register L is moved to the memory location whose address is in registers H and L. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "77",
    "mnemonic": "MOV M,A",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of register A is moved to the memory location whose address is in registers H and L. \n\nN Z S P CY AC\n- - - - -  -"
  },
  {
    "opcode": "B0",
    "mnemonic": "ORA B",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register B is inclusive-OR'd with the content of the accumulator. \nThe result is placed in the accumulator. The CY and AC flags are cleared. \n[(A) <- (A) OR (B)]\n\nZ S P CY AC\nx x x 0  0"
  },
  {
    "opcode": "B1",
    "mnemonic": "ORA C",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register C is inclusive-OR'd with the content of the accumulator. \nThe result is placed in the accumulator. The CY and AC flags are cleared. \n[(A) <- (A) OR (C)]\n\nZ S P CY AC\nx x x 0  0"
  },
  {
    "opcode": "B2",
    "mnemonic": "ORA D",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register D is inclusive-OR'd with the content of the accumulator. \nThe result is placed in the accumulator. The CY and AC flags are cleared. \n[(A) <- (A) OR (D)]\n\nZ S P CY AC\nx x x 0  0"
  },
  {
    "opcode": "B3",
    "mnemonic": "ORA E",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register E is inclusive-OR'd with the content of the accumulator. \nThe result is placed in the accumulator. The CY and AC flags are cleared. \n[(A) <- (A) OR (E)]\n\nZ S P CY AC\nx x x 0  0"
  },
  {
    "opcode": "B4",
    "mnemonic": "ORA H",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register H is inclusive-OR'd with the content of the accumulator. \nThe result is placed in the accumulator. The CY and AC flags are cleared. \n[(A) <- (A) OR (H)]\n\nZ S P CY AC\nx x x 0  0"
  },
  {
    "opcode": "B5",
    "mnemonic": "ORA L",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register L is inclusive-OR'd with the content of the accumulator. \nThe result is placed in the accumulator. The CY and AC flags are cleared. \n[(A) <- (A) OR (L)]\n\nZ S P CY AC\nx x x 0  0"
  },
  {
    "opcode": "B6",
    "mnemonic": "ORA M",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 2,
    "states": 7,
    "description": "The content of the memory location whose address is contained in \nthe H and L registers is inclusive-OR'd with the content of the accumulator. \nThe result is placed in the accumulator. The CY and AC flags are cleared.\n[(A) <- (A) OR ((H)(L))]\n\nZ S P CY AC\nx x x 0  0"
  },
  {
    "opcode": "F6",
    "mnemonic": "ORA A",
    "mode": "register",
    "bytes": 1,
    "cycles": 1,
    "states": 4,
    "description": "The content of register A is inclusive-OR'd with the content of the accumulator. \nThe result is placed in the accumulator. The CY and AC flags are cleared. \n[(A) <- (A) OR (A)]\n\nZ S P CY AC\nx x x 0  0"
  },
  {
    "opcode": "C1",
    "mnemonic": "POP B",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 3,
    "states": 10,
    "description": "Top of the stack is transferred to register pair BC. \nThe stack pointer is increased by 2. \n[(C) <- ((SP)), (B) <- ((SP)+ 1), (SP) <- (SP) + 2]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "D1",
    "mnemonic": "POP D",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 3,
    "states": 10,
    "description": "Top of the stack is transferred to register pair DE. \nThe stack pointer is increased by 2. \n[(E) <- ((SP)), (D) <- ((SP)+ 1), (SP) <- (SP) + 2]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "E1",
    "mnemonic": "POP H",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 3,
    "states": 10,
    "description": "Top of the stack is transferred to register pair HL. \nThe stack pointer is increased by 2. \n[(L) <- ((SP)), (H) <- ((SP)+ 1), (SP) <- (SP) + 2]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "F1",
    "mnemonic": "POP PSW",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 3,
    "states": 10,
    "description": "Pop processor status word. The stack pointer is increased by 2. \n[(CY) <- ((SP))0, (P) <- ((SP))2, (AC) <- (SP))4, (Z) <- ((SP))6, \n(S) <- ((SP))7; (A) <- ((SP) + 1); (SP) <- (SP) + 2]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "C5",
    "mnemonic": "PUSH B",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 3,
    "states": 11,
    "description": "Register pair BC is transferred to top of the stack. \nThe stack pointer is decreased by 2. \n[((SP )- 1) <- (B), ((SP) - 2) <- (C); (SP) <- (SP) - 2]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "D5",
    "mnemonic": "PUSH D",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 3,
    "states": 11,
    "description": "Register pair DE is transferred to top of the stack. \nThe stack pointer is decreased by 2. \n[((SP )- 1) <- (D), ((SP) - 2) <- (E); (SP) <- (SP) - 2]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "E5",
    "mnemonic": "PUSH H",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 3,
    "states": 11,
    "description": "Register pair DE is transferred to top of the stack. \nThe stack pointer is decreased by 2. \n[((SP )- 1) <- (H), ((SP) - 2) <- (L); (SP) <- (SP) - 2]\n\nZ S P CY AC\n- - - -  -"
  },
  {
    "opcode": "F5",
    "mnemonic": "PUSH PSW",
    "mode": "register indirect",
    "bytes": 1,
    "cycles": 3,
    "states": 11,
    "description": "Push processor status word. The stack pointer is decreased by 2. \n[((SP) -1 ) <- (A), ((SP) - 2)0 — (CY) , ((SP) - 2)1 <- 1, ((SP)-2)2  <— (P), \n((SP)-2)3 <- 0, ((SP) - 2)4 <- (AC) , ((SP) - 2 )5 <- 0, ((SP)-2)6 <— (Z) , \n((SP)-2)7 <— (S); (SP) <- (SP) - 2]\n\nZ S P CY AC\n- - - -  -"
  }
  ]
"#;
