use std::collections::HashMap;

// Addressing modes
#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

pub struct Instruction {
    opcode: u8,
    mode: AddressingMode,
    bytes: u8,
}

fn create_opcode_map() -> HashMap<&'static str, Vec<Instruction>> {
    let mut map = HashMap::new();
    
    // LDA (Load Accumulator)
    map.insert("LDA", vec![
        Instruction { opcode: 0xA9, mode: AddressingMode::Immediate, bytes: 2 },
        Instruction { opcode: 0xA5, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0xB5, mode: AddressingMode::ZeroPageX, bytes: 2 },
        Instruction { opcode: 0xAD, mode: AddressingMode::Absolute, bytes: 3 },
        Instruction { opcode: 0xBD, mode: AddressingMode::AbsoluteX, bytes: 3 },
        Instruction { opcode: 0xB9, mode: AddressingMode::AbsoluteY, bytes: 3 },
        Instruction { opcode: 0xA1, mode: AddressingMode::IndirectX, bytes: 2 },
        Instruction { opcode: 0xB1, mode: AddressingMode::IndirectY, bytes: 2 },
    ]);

    // LDX (Load X Register)
    map.insert("LDX", vec![
        Instruction { opcode: 0xA2, mode: AddressingMode::Immediate, bytes: 2 },
        Instruction { opcode: 0xA6, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0xB6, mode: AddressingMode::ZeroPageY, bytes: 2 },
        Instruction { opcode: 0xAE, mode: AddressingMode::Absolute, bytes: 3 },
        Instruction { opcode: 0xBE, mode: AddressingMode::AbsoluteY, bytes: 3 },
    ]);

    // LDY (Load Y Register)
    map.insert("LDY", vec![
        Instruction { opcode: 0xA0, mode: AddressingMode::Immediate, bytes: 2 },
        Instruction { opcode: 0xA4, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0xB4, mode: AddressingMode::ZeroPageX, bytes: 2 },
        Instruction { opcode: 0xAC, mode: AddressingMode::Absolute, bytes: 3 },
        Instruction { opcode: 0xBC, mode: AddressingMode::AbsoluteX, bytes: 3 },
    ]);

    // STX (Store X Register)
    map.insert("STX", vec![
        Instruction { opcode: 0x86, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0x96, mode: AddressingMode::ZeroPageY, bytes: 2 },
        Instruction { opcode: 0x8E, mode: AddressingMode::Absolute, bytes: 3 },
    ]);

    // STY (Store Y Register)
    map.insert("STY", vec![
        Instruction { opcode: 0x84, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0x94, mode: AddressingMode::ZeroPageX, bytes: 2 },
        Instruction { opcode: 0x8C, mode: AddressingMode::Absolute, bytes: 3 },
    ]);

    // STA (Store Accumulator)
    map.insert("STA", vec![
        Instruction { opcode: 0x85, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0x95, mode: AddressingMode::ZeroPageX, bytes: 2 },
        Instruction { opcode: 0x8D, mode: AddressingMode::Absolute, bytes: 3 },
        Instruction { opcode: 0x9D, mode: AddressingMode::AbsoluteX, bytes: 3 },
        Instruction { opcode: 0x99, mode: AddressingMode::AbsoluteY, bytes: 3 },
        Instruction { opcode: 0x81, mode: AddressingMode::IndirectX, bytes: 2 },
        Instruction { opcode: 0x91, mode: AddressingMode::IndirectY, bytes: 2 },
    ]);

    // ADC (Add with Carry)
    map.insert("ADC", vec![
        Instruction { opcode: 0x69, mode: AddressingMode::Immediate, bytes: 2 },
        Instruction { opcode: 0x65, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0x75, mode: AddressingMode::ZeroPageX, bytes: 2 },
        Instruction { opcode: 0x6D, mode: AddressingMode::Absolute, bytes: 3 },
        Instruction { opcode: 0x7D, mode: AddressingMode::AbsoluteX, bytes: 3 },
        Instruction { opcode: 0x79, mode: AddressingMode::AbsoluteY, bytes: 3 },
        Instruction { opcode: 0x61, mode: AddressingMode::IndirectX, bytes: 2 },
        Instruction { opcode: 0x71, mode: AddressingMode::IndirectY, bytes: 2 },
    ]);

    // CMP (Compare Accumulator)
    map.insert("CMP", vec![
        Instruction { opcode: 0xC9, mode: AddressingMode::Immediate, bytes: 2 },
        Instruction { opcode: 0xC5, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0xD5, mode: AddressingMode::ZeroPageX, bytes: 2 },
        Instruction { opcode: 0xCD, mode: AddressingMode::Absolute, bytes: 3 },
        Instruction { opcode: 0xDD, mode: AddressingMode::AbsoluteX, bytes: 3 },
        Instruction { opcode: 0xD9, mode: AddressingMode::AbsoluteY, bytes: 3 },
        Instruction { opcode: 0xC1, mode: AddressingMode::IndirectX, bytes: 2 },
        Instruction { opcode: 0xD1, mode: AddressingMode::IndirectY, bytes: 2 },
    ]);

    // Branch Instructions
    map.insert("BEQ", vec![
        Instruction { opcode: 0xF0, mode: AddressingMode::Immediate, bytes: 2 }, // Relative addressing
    ]);
    
    map.insert("BNE", vec![
        Instruction { opcode: 0xD0, mode: AddressingMode::Immediate, bytes: 2 }, // Relative addressing
    ]);

    map.insert("BCS", vec![Instruction { opcode: 0xB0, mode: AddressingMode::Immediate, bytes: 2 }]); // Relative addressing
    map.insert("BCC", vec![Instruction { opcode: 0x90, mode: AddressingMode::Immediate, bytes: 2 }]); // Relative addressing
    map.insert("BMI", vec![Instruction { opcode: 0x30, mode: AddressingMode::Immediate, bytes: 2 }]); // Relative addressing
    map.insert("BPL", vec![Instruction { opcode: 0x10, mode: AddressingMode::Immediate, bytes: 2 }]); // Relative addressing
    map.insert("BVC", vec![Instruction { opcode: 0x50, mode: AddressingMode::Immediate, bytes: 2 }]); // Relative addressing
    map.insert("BVS", vec![Instruction { opcode: 0x70, mode: AddressingMode::Immediate, bytes: 2 }]); // Relative addressing

    // Return Instructions
    map.insert("RTS", vec![
        Instruction { opcode: 0x60, mode: AddressingMode::Implied, bytes: 1 },
    ]);
    
    map.insert("BRK", vec![
        Instruction { opcode: 0x00, mode: AddressingMode::Implied, bytes: 1 },
    ]);

    // Register Transfers
    map.insert("TAX", vec![
        Instruction { opcode: 0xAA, mode: AddressingMode::Implied, bytes: 1 },
    ]);
    
    map.insert("TAY", vec![
        Instruction { opcode: 0xA8, mode: AddressingMode::Implied, bytes: 1 },
    ]);
    
    map.insert("TXA", vec![
        Instruction { opcode: 0x8A, mode: AddressingMode::Implied, bytes: 1 },
    ]);
    
    map.insert("TYA", vec![
        Instruction { opcode: 0x98, mode: AddressingMode::Implied, bytes: 1 },
    ]);

    // Stack Operations
    map.insert("TSX", vec![
        Instruction { opcode: 0xBA, mode: AddressingMode::Implied, bytes: 1 },
    ]);
    
    map.insert("TXS", vec![
        Instruction { opcode: 0x9A, mode: AddressingMode::Implied, bytes: 1 },
    ]);
    
    map.insert("PHA", vec![
        Instruction { opcode: 0x48, mode: AddressingMode::Implied, bytes: 1 },
    ]);
    
    map.insert("PHP", vec![
        Instruction { opcode: 0x08, mode: AddressingMode::Implied, bytes: 1 },
    ]);
    
    map.insert("PLA", vec![
        Instruction { opcode: 0x68, mode: AddressingMode::Implied, bytes: 1 },
    ]);
    
    map.insert("PLP", vec![
        Instruction { opcode: 0x28, mode: AddressingMode::Implied, bytes: 1 },
    ]);

    // SBC
    map.insert("SBC", vec![
        Instruction { opcode: 0xE9, mode: AddressingMode::Immediate, bytes: 2 },
        Instruction { opcode: 0xE5, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0xF5, mode: AddressingMode::ZeroPageX, bytes: 2 },
        Instruction { opcode: 0xED, mode: AddressingMode::Absolute, bytes: 3 },
        Instruction { opcode: 0xFD, mode: AddressingMode::AbsoluteX, bytes: 3 },
        Instruction { opcode: 0xF9, mode: AddressingMode::AbsoluteY, bytes: 3 },
        Instruction { opcode: 0xE1, mode: AddressingMode::IndirectX, bytes: 2 },
        Instruction { opcode: 0xF1, mode: AddressingMode::IndirectY, bytes: 2 },
    ]);

    // INC/DEC
    map.insert("INC", vec![
        Instruction { opcode: 0xE6, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0xF6, mode: AddressingMode::ZeroPageX, bytes: 2 },
        Instruction { opcode: 0xEE, mode: AddressingMode::Absolute, bytes: 3 },
        Instruction { opcode: 0xFE, mode: AddressingMode::AbsoluteX, bytes: 3 },
    ]);
    
    map.insert("DEC", vec![
        Instruction { opcode: 0xC6, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0xD6, mode: AddressingMode::ZeroPageX, bytes: 2 },
        Instruction { opcode: 0xCE, mode: AddressingMode::Absolute, bytes: 3 },
        Instruction { opcode: 0xDE, mode: AddressingMode::AbsoluteX, bytes: 3 },
    ]);

    // Single byte instructions
    map.insert("INX", vec![Instruction { opcode: 0xE8, mode: AddressingMode::Implied, bytes: 1 }]);
    map.insert("INY", vec![Instruction { opcode: 0xC8, mode: AddressingMode::Implied, bytes: 1 }]);
    map.insert("DEX", vec![Instruction { opcode: 0xCA, mode: AddressingMode::Implied, bytes: 1 }]);
    map.insert("DEY", vec![Instruction { opcode: 0x88, mode: AddressingMode::Implied, bytes: 1 }]);

    // AND (Logical AND)
    map.insert("AND", vec![
        Instruction { opcode: 0x29, mode: AddressingMode::Immediate, bytes: 2 },
        Instruction { opcode: 0x25, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0x35, mode: AddressingMode::ZeroPageX, bytes: 2 },
        Instruction { opcode: 0x2D, mode: AddressingMode::Absolute, bytes: 3 },
        Instruction { opcode: 0x3D, mode: AddressingMode::AbsoluteX, bytes: 3 },
        Instruction { opcode: 0x39, mode: AddressingMode::AbsoluteY, bytes: 3 },
        Instruction { opcode: 0x21, mode: AddressingMode::IndirectX, bytes: 2 },
        Instruction { opcode: 0x31, mode: AddressingMode::IndirectY, bytes: 2 },
    ]);

    // ORA (Logical OR)
    map.insert("ORA", vec![
        Instruction { opcode: 0x09, mode: AddressingMode::Immediate, bytes: 2 },
        Instruction { opcode: 0x05, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0x15, mode: AddressingMode::ZeroPageX, bytes: 2 },
        Instruction { opcode: 0x0D, mode: AddressingMode::Absolute, bytes: 3 },
        Instruction { opcode: 0x1D, mode: AddressingMode::AbsoluteX, bytes: 3 },
        Instruction { opcode: 0x19, mode: AddressingMode::AbsoluteY, bytes: 3 },
        Instruction { opcode: 0x01, mode: AddressingMode::IndirectX, bytes: 2 },
        Instruction { opcode: 0x11, mode: AddressingMode::IndirectY, bytes: 2 },
    ]);

    // EOR (Exclusive OR)
    map.insert("EOR", vec![
        Instruction { opcode: 0x49, mode: AddressingMode::Immediate, bytes: 2 },
        Instruction { opcode: 0x45, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0x55, mode: AddressingMode::ZeroPageX, bytes: 2 },
        Instruction { opcode: 0x4D, mode: AddressingMode::Absolute, bytes: 3 },
        Instruction { opcode: 0x5D, mode: AddressingMode::AbsoluteX, bytes: 3 },
        Instruction { opcode: 0x59, mode: AddressingMode::AbsoluteY, bytes: 3 },
        Instruction { opcode: 0x41, mode: AddressingMode::IndirectX, bytes: 2 },
        Instruction { opcode: 0x51, mode: AddressingMode::IndirectY, bytes: 2 },
    ]);

    // ASL (Arithmetic Shift Left)
    map.insert("ASL", vec![
        Instruction { opcode: 0x0A, mode: AddressingMode::Implied, bytes: 1 },
        Instruction { opcode: 0x06, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0x16, mode: AddressingMode::ZeroPageX, bytes: 2 },
        Instruction { opcode: 0x0E, mode: AddressingMode::Absolute, bytes: 3 },
        Instruction { opcode: 0x1E, mode: AddressingMode::AbsoluteX, bytes: 3 },
    ]);

    // LSR (Logical Shift Right)
    map.insert("LSR", vec![
        Instruction { opcode: 0x4A, mode: AddressingMode::Implied, bytes: 1 },
        Instruction { opcode: 0x46, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0x56, mode: AddressingMode::ZeroPageX, bytes: 2 },
        Instruction { opcode: 0x4E, mode: AddressingMode::Absolute, bytes: 3 },
        Instruction { opcode: 0x5E, mode: AddressingMode::AbsoluteX, bytes: 3 },
    ]);

    // ROL (Rotate Left)
    map.insert("ROL", vec![
        Instruction { opcode: 0x2A, mode: AddressingMode::Implied, bytes: 1 },
        Instruction { opcode: 0x26, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0x36, mode: AddressingMode::ZeroPageX, bytes: 2 },
        Instruction { opcode: 0x2E, mode: AddressingMode::Absolute, bytes: 3 },
        Instruction { opcode: 0x3E, mode: AddressingMode::AbsoluteX, bytes: 3 },
    ]);

    // ROR (Rotate Right)
    map.insert("ROR", vec![
        Instruction { opcode: 0x6A, mode: AddressingMode::Implied, bytes: 1 },
        Instruction { opcode: 0x66, mode: AddressingMode::ZeroPage, bytes: 2 },
        Instruction { opcode: 0x76, mode: AddressingMode::ZeroPageX, bytes: 2 },
        Instruction { opcode: 0x6E, mode: AddressingMode::Absolute, bytes: 3 },
        Instruction { opcode: 0x7E, mode: AddressingMode::AbsoluteX, bytes: 3 },
    ]);
    
    map
}

fn parse_operand(operand: &str) -> (AddressingMode, u16) {
    if operand.starts_with('#') {
        // Immediate addressing - handle both #$2A and #42 formats
        let value_str = operand[1..].trim_start_matches('$');
        let value = u16::from_str_radix(value_str, 16)
            .unwrap_or_else(|_| {
                // Try parsing as decimal if hex fails
                value_str.parse::<u16>().unwrap_or(0)
            });
        (AddressingMode::Immediate, value)
    } else if operand.starts_with('$') {
        // Absolute or ZeroPage addressing
        let value = u16::from_str_radix(&operand[1..], 16).unwrap_or(0);
        if value <= 0xFF {
            (AddressingMode::ZeroPage, value)
        } else {
            (AddressingMode::Absolute, value)
        }
    } else {
        // Try parsing as hex without prefix, then as decimal
        let value = u16::from_str_radix(operand, 16)
            .unwrap_or_else(|_| operand.parse::<u16>().unwrap_or(0));
        (AddressingMode::Absolute, value)
    }
}

pub fn assemble(source: &str) -> Vec<u8> {
    let opcodes = create_opcode_map();
    let mut machine_code = Vec::new();
    
    for line in source.lines() {
        let line = line.split(';').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let mnemonic = parts[0].to_uppercase();
        
        if let Some(instructions) = opcodes.get(mnemonic.as_str()) {
            if parts.len() > 1 {
                let (mode, value) = parse_operand(parts[1]);
                
                // Find matching instruction for addressing mode
                if let Some(instruction) = instructions.iter().find(|i| i.mode == mode) {
                    machine_code.push(instruction.opcode);
                    
                    // Add operand bytes
                    match instruction.bytes {
                        2 => machine_code.push(value as u8),
                        3 => {
                            machine_code.push((value & 0xFF) as u8);
                            machine_code.push((value >> 8) as u8);
                        }
                        _ => {}
                    }
                }
            } else if let Some(instruction) = instructions.iter().find(|i| i.mode == AddressingMode::Implied) {
                machine_code.push(instruction.opcode);
            }
        }
    }

    machine_code
}
