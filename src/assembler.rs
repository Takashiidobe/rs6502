use std::collections::HashMap;

// Addressing modes
#[derive(Debug, PartialEq)]
enum AddressingMode {
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

struct Instruction {
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

    // Return Instructions
    map.insert("RTS", vec![
        Instruction { opcode: 0x60, mode: AddressingMode::Implied, bytes: 1 },
    ]);
    
    map.insert("BRK", vec![
        Instruction { opcode: 0x00, mode: AddressingMode::Implied, bytes: 1 },
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
