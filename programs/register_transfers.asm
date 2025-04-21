; Test program for register transfer instructions
; Expected results:
; A = $55 (original value moved through X and back)
; X = $55 (from accumulator)
; Y = $AA (from accumulator)

    LDA #$55    ; Load $55 into accumulator
    TAX         ; Transfer A to X (X should now be $55)
    LDA #$AA    ; Load $AA into accumulator
    TAY         ; Transfer A to Y (Y should now be $AA)
    TXA         ; Transfer X back to A (A should now be $55)
    BRK         ; Stop program execution
