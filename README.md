# 6502 Interpreter

This project is a simple interpreter for the 6502 microprocessor. It reads and executes 6502 assembly files, allowing users to run programs written for this classic CPU.

## Project Structure

- `src/main.rs`: Entry point of the application. Initializes the interpreter, loads the assembly file, and starts the execution loop.
- `src/cpu.rs`: Defines the `CPU` struct, representing the state of the 6502 CPU. Includes methods for executing instructions, managing registers, and handling the CPU's internal state.
- `src/memory.rs`: Defines the `Memory` struct, simulating the memory of the 6502 computer. Includes methods for reading from and writing to memory addresses.
- `src/assembler.rs`: Contains functions for parsing and assembling 6502 assembly code into machine code that the interpreter can execute.

## Setup Instructions

1. Clone the repository:
   ```
   git clone <repository-url>
   cd 6502-interpreter
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the interpreter with an assembly file:
   ```
   cargo run -- <path-to-assembly-file>
   ```

## Usage Example

To run a simple assembly program, create a `.asm` file with your 6502 assembly code and execute it using the interpreter.

## Overview

This interpreter aims to provide a basic environment for executing 6502 assembly code, making it easier to understand and experiment with the 6502 architecture.