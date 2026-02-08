SETDSK  EQU 4B5AH

        ORG 100H
        MVI C,02
        CALL SETDSK
LOOP:   JMP LOOP
        HLT
