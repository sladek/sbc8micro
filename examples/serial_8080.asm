DATA    EQU 40H
CONTROL EQU 41H

        ORG     0100H
        MVI     A, 00H
        OUT     CONTROL
        OUT     CONTROL
        OUT     CONTROL
        MVI     A, 40H
        OUT     CONTROL
        MVI     A, 01001111B ; 8,N, 1
        OUT     CONTROL
        MVI     A, 00000101B ; RxE, TxEN
        OUT     CONTROL
LOOP:   IN      CONTROL
        ANI     00000010B ; Rxrdy
        JZ      LOOP
        IN      DATA
        OUT     DATA
        MVI     A, 0DH
        OUT     DATA
        MVI     A, 0AH
        OUT     DATA
        JMP LOOP
xa