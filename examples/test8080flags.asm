        ORG 400H

        XRA A
        LDA 1C00H
        ADI 090H
        DAA
        ACI 040H
        DAA
        PUSH PSW
        POP  H
        MOV  A, L       ; PSW TO A
        STA  1C01H      ; AND SORE IT
        MOV  A, H       ; RETORE A
        STA  1C02H      ; AND SORE IT 
        HLT
