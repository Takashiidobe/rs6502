; Stack operations test program
; First, load some test values
LDA #$42        ; Load test value into A
LDX #$FF        ; Initialize X with stack pointer value
TXS             ; Transfer X to stack pointer
PHA             ; Push accumulator ($42) onto stack
PHP             ; Push processor status onto stack

LDA #$00        ; Clear A
LDX #$00        ; Clear X

TSX             ; Transfer stack pointer to X
PLA             ; Pull original value ($42) back to A
PLP             ; Pull processor status

BRK             ; Break - end program
