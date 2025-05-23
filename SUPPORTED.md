# 6502 Interpreter Feature Status

## Currently Supported Features

### CPU Features
- [x] Basic CPU state (A, X, Y registers, PC, SP, status flags)
- [x] Status register manipulation (all flags)
- [x] Zero-page addressing mode
- [x] Immediate addressing mode

### Instructions
#### Load/Store Operations
- [x] LDA (Load Accumulator)
  - [x] Immediate
  - [x] Zero Page
  - [x] Zero Page,X
  - [x] Absolute
  - [x] Absolute,X
  - [x] Absolute,Y
  - [x] Indirect,X
  - [x] Indirect,Y
- [x] LDX (Load X Register)
  - [x] Immediate
  - [x] Zero Page
  - [x] Zero Page,Y
  - [x] Absolute
  - [x] Absolute,Y
- [x] LDY (Load Y Register)
  - [x] Immediate
  - [x] Zero Page
  - [x] Zero Page,X
  - [x] Absolute
  - [x] Absolute,X
- [x] STA (Store Accumulator)
  - [x] Zero Page
  - [x] Zero Page,X
  - [x] Absolute
  - [x] Absolute,X
  - [x] Absolute,Y
  - [x] Indirect,X
  - [x] Indirect,Y
- [x] STX (Store X Register)
  - [x] Zero Page
  - [x] Zero Page,Y
  - [x] Absolute
- [x] STY (Store Y Register)
  - [x] Zero Page
  - [x] Zero Page,X
  - [x] Absolute

#### Arithmetic
- [x] ADC (Add with Carry)
  - [x] Immediate mode
  - [ ] Other addressing modes

#### Compare
- [x] CMP (Compare Accumulator)
  - [x] Immediate mode
  - [ ] Other addressing modes

#### Branch Operations
- [x] BEQ (Branch if Equal)
- [x] BNE (Branch if Not Equal)

#### Register Transfers
- [x] TAX (Transfer A to X)
- [x] TAY (Transfer A to Y)
- [x] TXA (Transfer X to A)
- [x] TYA (Transfer Y to A)

#### Stack Operations
- [X] PHA (Push Accumulator)
- [X] PHP (Push Processor Status)
- [X] PLA (Pull Accumulator)
- [X] PLP (Pull Processor Status)
- [X] TSX (Transfer Stack Pointer to X)
- [X] TXS (Transfer X to Stack Pointer)

#### System
- [x] BRK (Force Break)

#### Arithmetic Operations
- [x] SBC (Subtract with Carry)
  - [x] Immediate mode
  - [x] Zero Page mode
  - [x] Other addressing modes
- [x] INC (Increment Memory)
  - [x] Zero Page
  - [x] Zero Page,X
  - [x] Absolute
  - [x] Absolute,X
- [x] INX (Increment X)
- [x] INY (Increment Y)
- [x] DEC (Decrement Memory)
  - [x] Zero Page
  - [x] Zero Page,X
  - [x] Absolute
  - [x] Absolute,X
- [x] DEX (Decrement X)
- [x] DEY (Decrement Y)

#### Logical Operations
- [x] AND (Logical AND)
- [x] ORA (Logical OR)
- [x] EOR (Exclusive OR)

#### Shifts and Rotates
- [x] ASL (Arithmetic Shift Left)
- [x] LSR (Logical Shift Right)
- [x] ROL (Rotate Left)
- [x] ROR (Rotate Right)

#### Branch Instructions
- [x] BCS (Branch if Carry Set)
- [x] BCC (Branch if Carry Clear)
- [x] BMI (Branch if Minus)
- [x] BPL (Branch if Plus)
- [x] BVC (Branch if Overflow Clear)
- [x] BVS (Branch if Overflow Set)

#### Status Flag Changes
- [x] CLC (Clear Carry)
- [x] CLD (Clear Decimal)
- [x] CLI (Clear Interrupt)
- [x] CLV (Clear Overflow)
- [x] SEC (Set Carry)
- [x] SED (Set Decimal)
- [x] SEI (Set Interrupt)

## Features To Be Implemented

### Addressing Modes
- [ ] Absolute addressing
- [ ] Absolute,X addressing
- [ ] Absolute,Y addressing
- [ ] Indirect addressing
- [ ] Indexed indirect addressing (X)
- [ ] Indirect indexed addressing (Y)
- [ ] Relative addressing for all branch instructions

### Instructions

#### Jump & Call Operations
- [ ] JMP (Jump)
- [ ] JSR (Jump to Subroutine)
- [ ] RTS (Return from Subroutine)

### System Features
- [ ] Proper interrupt handling
- [ ] Decimal mode
- [ ] Stack operations
- [ ] Cycle counting
- [ ] Memory-mapped I/O

### Debugging Features
- [ ] Memory dump
- [ ] Register dump
- [ ] Breakpoints
- [ ] Step-by-step execution
- [ ] Memory watchpoints
