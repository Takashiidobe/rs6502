  LDA #$05    ; Load 5 into accumulator
  ADC #$03    ; Add 3 to accumulator
  CMP #$08    ; Compare with 8
  BEQ success ; Branch if equal
  BRK         ; Break if not equal
success:
  RTS         ; Return from subroutine