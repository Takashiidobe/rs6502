    LDA #$00       ; Load 0 into A
    BEQ equal      ; Branch if zero flag is set
    BNE notequal   ; This should be skipped
equal:
    LDA #$01       ; Load 1 into A
    BCS carryset   ; This should be skipped
    BCC carryclear ; Branch if carry flag is clear
carryclear:
    LDA #$02       ; Load 2 into A
    BMI minus      ; This should be skipped
    BPL plus       ; Branch if negative flag is clear
plus:
    LDA #$03       ; Load 3 into A
    BVC overflowclear ; Branch if overflow flag is clear
    BVS overflowset   ; This should be skipped
overflowclear:
    BRK            ; Break
