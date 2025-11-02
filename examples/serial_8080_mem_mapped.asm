DATA    EQU 1234H
CONTROL EQU 1235H

        ORG     0100H
        MVI     A, 00H
        STA     CONTROL
        STA     CONTROL
        STA     CONTROL
        MVI     A, 40H       ; Reset USART
        STA     CONTROL
        MVI     A, 01001111B ; 8,N, 1
        STA     CONTROL
        MVI     A, 00000101B ; RxE, TxEN
        STA     CONTROL
LOOP:   LDA     CONTROL
        ANI     00000010B ; Rxrdy
        JZ      LOOP
        LDA     DATA
        STA     DATA
        MVI     A, 0DH
        STA     DATA
        MVI     A, 0AH
        STA     DATA
        JMP LOOP
