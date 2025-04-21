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
- [x] LDX (Load X Register)
  - [x] Immediate
  - [x] Zero Page
- [x] LDY (Load Y Register)
  - [x] Immediate
  - [x] Zero Page
- [x] STA (Store Accumulator)
  - [x] Zero Page
- [x] STX (Store X Register)
  - [x] Zero Page
- [x] STY (Store Y Register)
  - [x] Zero Page

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

#### System
- [x] BRK (Force Break)

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
#### Arithmetic Operations
- [ ] SBC (Subtract with Carry)
- [ ] INC (Increment Memory)
- [ ] INX (Increment X)
- [ ] INY (Increment Y)
- [ ] DEC (Decrement Memory)
- [ ] DEX (Decrement X)
- [ ] DEY (Decrement Y)

#### Logical Operations
- [ ] AND (Logical AND)
- [ ] ORA (Logical OR)
- [ ] EOR (Exclusive OR)

#### Shifts and Rotates
- [ ] ASL (Arithmetic Shift Left)
- [ ] LSR (Logical Shift Right)
- [ ] ROL (Rotate Left)
- [ ] ROR (Rotate Right)

#### Stack Operations
- [ ] PHA (Push Accumulator)
- [ ] PHP (Push Processor Status)
- [ ] PLA (Pull Accumulator)
- [ ] PLP (Pull Processor Status)
- [ ] TSX (Transfer Stack Pointer to X)
- [ ] TXS (Transfer X to Stack Pointer)

#### Status Flag Changes
- [ ] CLC (Clear Carry)
- [ ] CLD (Clear Decimal)
- [ ] CLI (Clear Interrupt)
- [ ] CLV (Clear Overflow)
- [ ] SEC (Set Carry)
- [ ] SED (Set Decimal)
- [ ] SEI (Set Interrupt)

#### Branch Instructions
- [ ] BCS (Branch if Carry Set)
- [ ] BCC (Branch if Carry Clear)
- [ ] BMI (Branch if Minus)
- [ ] BPL (Branch if Plus)
- [ ] BVC (Branch if Overflow Clear)
- [ ] BVS (Branch if Overflow Set)

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
