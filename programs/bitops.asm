; Test program for 6502 instructions

    LDX #$00         ; Load 0 into X
    LDY #$FF         ; Load 255 into Y
    LDA #$AA         ; Load 170 into A
    STA $0200        ; Store A at $0200
    STX $0201        ; Store X at $0201
    STY $0202        ; Store Y at $0202

    LDA #$55         ; Load 85 into A
    AND #$0F         ; Logical AND with 15
    ORA #$F0         ; Logical OR with 240
    EOR #$FF         ; Exclusive OR with 255

    ADC #$01         ; Add 1 to A with carry
    SBC #$01         ; Subtract 1 from A with carry

    CMP #$55         ; Compare A with 85
    BEQ equal        ; Branch if equal
    BNE not_equal    ; Branch if not equal

equal:
    LDA #$01         ; Load 1 into A
    JMP end          ; Jump to end

not_equal:
    LDA #$00         ; Load 0 into A

end:
    BRK              ; Break (halt execution)
