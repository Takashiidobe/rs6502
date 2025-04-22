    LDA #$01       ; Load 1 into A
    ASL A          ; Shift left, A = 2
    STA $0200      ; Store A at $0200
    LSR A          ; Shift right, A = 1
    STA $0201      ; Store A at $0201
    ROL A          ; Rotate left, A = 2
    STA $0202      ; Store A at $0202
    ROR A          ; Rotate right, A = 1
    STA $0203      ; Store A at $0203
    BRK            ; Break
