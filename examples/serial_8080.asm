DATA    EQU 0F4H
CONTROL EQU 0F5H

        ORG 100H               ;See README for more info

;Initialization and sign-on message
LOG:    JMP SETUP           ;See README for more info
SE1:    ;CALL CINNE
SE2:    LXI H, LOGMSG@
        CALL STROUT
        LXI H, MSG@
        CALL STROUT
        CALL CINNE
        CALL COUT
        JMP SE2


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;CINNE -- Get a char from the console, no echo
;
;pre: console device is initialized
;post: received char is in A register
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
CINNE:  IN CONTROL
        ANI 02H
        JZ CINNE
        IN DATA
        RET
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;COUT -- Output a character to the console
;
;pre: A register contains char to be printed
;post: character is printed to the console
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
COUT:   PUSH B
        MOV B, A
COUT1:  
        IN CONTROL
        ANI 01H
        JZ COUT1
        MOV A, B
        OUT DATA
        POP B
        RET
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;SETUP -- Prepare the system for running the
;   monitor
;
;pre: none
;post: stack and console are initialized
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
SETUP:  LXI SP, 0FFFFH
        LXI H, INIUART@
        MVI B, 06H              ; length of ini string
INURT:  MOV A, M
        OUT CONTROL
        INX H
        DCR B
        JNZ INURT
        JMP SE1

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;STROUT -- Print a null-terminated string
;
;pre: HL contains pointer to start of a null-
;     terminated string
;post: string at HL printed to console
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
STROUT: MOV A, M
        CPI 00
        RZ
        CALL COUT
        INX H
        JMP STROUT


;Init string for the 8251, x64 clock, 8N1
INIUART@:  db 00H, 00H, 00H, 40H, 4FH, 37H

;I/O Module description string
MSG@:     db 13, 10, 'Built with Intel 8251 I/O module', 13, 10, 0


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;Monitor Strings
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
LOGMSG@: db 13, 10, 10, 'GWMON-80 0.1.4 for 8080/8085/Z80 and Compatible', 13, 10
         db 'Copyright (c) 2019 The Glitch Works', 0
PROMPT@: db 13, 10, 10, '>', 0
CSERR@:  db 'CHECKSUM '
ERR@:    db 'ERROR', 0
CRLF@:   db 13, 10, 0

        END