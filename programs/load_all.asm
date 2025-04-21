        LDA #$42        ; Load 66 into A
        STA $20         ; Store A at zero page address $20
        LDX #$69        ; Load 105 into X
        STX $21         ; Store X at zero page address $21
        LDY #$96        ; Load 150 into Y
        STY $22         ; Store Y at zero page address $22
        LDA $21         ; Load X's stored value into A
        LDX $22         ; Load Y's stored value into X
        LDY $20         ; Load A's stored value into Y
        BRK             ; Break - A should now be 105
