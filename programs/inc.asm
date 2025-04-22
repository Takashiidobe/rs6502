    .org $0200

test_inc_dec:
    ; Test INX/DEX
    LDX #$FF    ; Load max value
    INX         ; Should overflow to 0
    BNE fail    ; Branch if not zero
    DEX         ; Should become $FF
    BPL fail    ; Branch if plus (should be negative)

    ; Test INY/DEY
    LDY #$00
    DEY         ; Should underflow to $FF
    BPL fail    ; Branch if plus (should be negative)
    INY         ; Should become 0
    BNE fail    ; Branch if not zero

    ; Test INC/DEC memory
    LDA #$7F
    STA $80     ; Store in zero page
    INC $80     ; Increment to $80
    BVC fail    ; Branch if overflow clear (should be set)
    DEC $80     ; Decrement back to $7F
    BMI fail    ; Branch if minus (should be positive)

    ; Test SBC
    LDA #$50
    SEC         ; Set carry for proper subtraction
    SBC #$20    ; Should result in $30
    CMP #$30
    BNE fail
    
    JMP success

fail:
    BRK         ; Stop with error

success:
    BRK         ; Stop with success
