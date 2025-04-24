use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, PartialOrd, Ord)]
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

use strum_macros::EnumString;

#[allow(clippy::upper_case_acronyms)]
#[derive(EnumString, Debug, PartialEq, Clone, Copy, Hash, Eq, PartialOrd, Ord)]
pub enum OpCode {
    LDA,
    SBC,
    INC,
    DEC,
    INX,
    INY,
    DEX,
    DEY,
    AND,
    ORA,
    EOR,
    ASL,
    LSR,
    ROL,
    ROR,
    CLC,
    SEI,
    SED,
    SEC,
    CLV,
    CLI,
    CLD,
    PLP,
    PLA,
    PHP,
    PHA,
    TXS,
    TSX,
    TYA,
    TXA,
    TAY,
    TAX,
    BRK,
    RTS,
    BVS,
    BVC,
    BPL,
    BMI,
    BCC,
    BCS,
    BNE,
    BEQ,
    CMP,
    ADC,
    STA,
    STY,
    STX,
    LDY,
    LDX,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, PartialOrd, Ord)]
pub struct Instruction {
    opname: OpCode,
    opcode: u8,
    mode: AddressingMode,
    bytes: u8,
    cycles: u8,
}

use phf_macros::phf_map;

static INSTRUCTION_LOOKUP: phf::Map<u8, Instruction> = phf_map! {
    0xA9u8 => Instruction {
        opname: OpCode::LDA,
        opcode: 0xA9,
        mode: AddressingMode::Immediate,
        bytes: 2,
        cycles: 2,
    },
0xA5u8 => Instruction {
    opname: OpCode::LDA,
                opcode: 0xA5,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            },
            0xB5u8 =>
            Instruction {
                opname: OpCode::LDA,
                opcode: 0xB5,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            },
            0xADu8 =>
            Instruction {
                opname: OpCode::LDA,
                opcode: 0xAD,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4
            },
            0xBDu8 =>
            Instruction {
                opname: OpCode::LDA,
                opcode: 0xBD,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4
            },
            0xB9u8 =>  Instruction {
                opname: OpCode::LDA,
                opcode: 0xB9,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4
            },

            0xA1u8 => Instruction {
                opname: OpCode::LDA,
                opcode: 0xA1,
                mode: AddressingMode::IndirectX,
                bytes: 2,
                cycles: 6
            },

            0xB1u8 =>
        Instruction {
                opname: OpCode::LDA,
            opcode: 0xB1,
            mode: AddressingMode::IndirectY,
            bytes: 2,
                cycles: 5
        },
        0xA2u8 =>  Instruction {
                opcode: 0xA2,
                mode: AddressingMode::Immediate,
                bytes: 2,
                opname: OpCode::LDX,
                cycles: 2,
            },
    0xA6u8 => Instruction {
            opcode: 0xA6,
            mode: AddressingMode::ZeroPage,
            bytes: 2,
            opname: OpCode::LDX,
            cycles: 3,
        },
        0xB6u8 =>
        Instruction {
            opcode: 0xB6,
            mode: AddressingMode::ZeroPageY,
            bytes: 2,
            opname: OpCode::LDX,
            cycles: 4,
        },
        0xAEu8 =>
        Instruction {
            opcode: 0xAE,
            mode: AddressingMode::Absolute,
            bytes: 3,
            opname: OpCode::LDX,
            cycles: 4,
        },
        0xBEu8 =>
        Instruction {
            opcode: 0xBE,
            mode: AddressingMode::AbsoluteY,
            bytes: 3,
            opname: OpCode::LDX,
            cycles: 4,
        },
};

pub fn create_opcode_map() -> HashMap<OpCode, Vec<Instruction>> {
    let mut map = HashMap::new();

    // LDY (Load Y Register)
    map.insert(
        OpCode::LDY,
        vec![
            Instruction {
                opcode: 0xA0,
                mode: AddressingMode::Immediate,
                bytes: 2,
            },
            Instruction {
                opcode: 0xA4,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
            },
            Instruction {
                opcode: 0xB4,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
            },
            Instruction {
                opcode: 0xAC,
                mode: AddressingMode::Absolute,
                bytes: 3,
            },
            Instruction {
                opcode: 0xBC,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
            },
        ],
    );

    // STX (Store X Register)
    map.insert(
        OpCode::STX,
        vec![
            Instruction {
                opcode: 0x86,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
            },
            Instruction {
                opcode: 0x96,
                mode: AddressingMode::ZeroPageY,
                bytes: 2,
            },
            Instruction {
                opcode: 0x8E,
                mode: AddressingMode::Absolute,
                bytes: 3,
            },
        ],
    );

    // STY (Store Y Register)
    map.insert(
        OpCode::STY,
        vec![
            Instruction {
                opcode: 0x84,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
            },
            Instruction {
                opcode: 0x94,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
            },
            Instruction {
                opcode: 0x8C,
                mode: AddressingMode::Absolute,
                bytes: 3,
            },
        ],
    );

    // STA (Store Accumulator)
    map.insert(
        OpCode::STA,
        vec![
            Instruction {
                opcode: 0x85,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
            },
            Instruction {
                opcode: 0x95,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
            },
            Instruction {
                opcode: 0x8D,
                mode: AddressingMode::Absolute,
                bytes: 3,
            },
            Instruction {
                opcode: 0x9D,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
            },
            Instruction {
                opcode: 0x99,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
            },
            Instruction {
                opcode: 0x81,
                mode: AddressingMode::IndirectX,
                bytes: 2,
            },
            Instruction {
                opcode: 0x91,
                mode: AddressingMode::IndirectY,
                bytes: 2,
            },
        ],
    );

    // ADC (Add with Carry)
    map.insert(
        OpCode::ADC,
        vec![
            Instruction {
                opname: OpCode::ADC,
                opcode: 0x69,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            },
            Instruction {
                opcode: 0x65,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
            },
            Instruction {
                opcode: 0x75,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
            },
            Instruction {
                opcode: 0x6D,
                mode: AddressingMode::Absolute,
                bytes: 3,
            },
            Instruction {
                opcode: 0x7D,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
            },
            Instruction {
                opcode: 0x79,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
            },
            Instruction {
                opcode: 0x61,
                mode: AddressingMode::IndirectX,
                bytes: 2,
            },
            Instruction {
                opcode: 0x71,
                mode: AddressingMode::IndirectY,
                bytes: 2,
            },
        ],
    );

    // CMP (Compare Accumulator)
    map.insert(
        OpCode::CMP,
        vec![
            Instruction {
                opcode: 0xC9,
                mode: AddressingMode::Immediate,
                bytes: 2,
            },
            Instruction {
                opcode: 0xC5,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
            },
            Instruction {
                opcode: 0xD5,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
            },
            Instruction {
                opcode: 0xCD,
                mode: AddressingMode::Absolute,
                bytes: 3,
            },
            Instruction {
                opcode: 0xDD,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
            },
            Instruction {
                opcode: 0xD9,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
            },
            Instruction {
                opcode: 0xC1,
                mode: AddressingMode::IndirectX,
                bytes: 2,
            },
            Instruction {
                opcode: 0xD1,
                mode: AddressingMode::IndirectY,
                bytes: 2,
            },
        ],
    );

    // Branch Instructions
    map.insert(
        OpCode::BEQ,
        vec![
            Instruction {
                opcode: 0xF0,
                mode: AddressingMode::Immediate,
                bytes: 2,
            }, // Relative addressing
        ],
    );

    map.insert(
        OpCode::BNE,
        vec![
            Instruction {
                opcode: 0xD0,
                mode: AddressingMode::Immediate,
                bytes: 2,
            }, // Relative addressing
        ],
    );

    map.insert(
        OpCode::BCS,
        vec![Instruction {
            opcode: 0xB0,
            mode: AddressingMode::Immediate,
            bytes: 2,
        }],
    ); // Relative addressing
    map.insert(
        OpCode::BCC,
        vec![Instruction {
            opcode: 0x90,
            mode: AddressingMode::Immediate,
            bytes: 2,
        }],
    ); // Relative addressing
    map.insert(
        OpCode::BMI,
        vec![Instruction {
            opcode: 0x30,
            mode: AddressingMode::Immediate,
            bytes: 2,
        }],
    ); // Relative addressing
    map.insert(
        OpCode::BPL,
        vec![Instruction {
            opcode: 0x10,
            mode: AddressingMode::Immediate,
            bytes: 2,
        }],
    ); // Relative addressing
    map.insert(
        OpCode::BVC,
        vec![Instruction {
            opcode: 0x50,
            mode: AddressingMode::Immediate,
            bytes: 2,
        }],
    ); // Relative addressing
    map.insert(
        OpCode::BVS,
        vec![Instruction {
            opcode: 0x70,
            mode: AddressingMode::Immediate,
            bytes: 2,
        }],
    ); // Relative addressing

    // Return Instructions
    map.insert(
        OpCode::RTS,
        vec![Instruction {
            opcode: 0x60,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );

    map.insert(
        OpCode::BRK,
        vec![Instruction {
            opcode: 0x00,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );

    // Register Transfers
    map.insert(
        OpCode::TAX,
        vec![Instruction {
            opcode: 0xAA,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );

    map.insert(
        OpCode::TAY,
        vec![Instruction {
            opcode: 0xA8,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );

    map.insert(
        OpCode::TXA,
        vec![Instruction {
            opcode: 0x8A,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );

    map.insert(
        OpCode::TYA,
        vec![Instruction {
            opcode: 0x98,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );

    // Stack Operations
    map.insert(
        OpCode::TSX,
        vec![Instruction {
            opcode: 0xBA,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );

    map.insert(
        OpCode::TXS,
        vec![Instruction {
            opcode: 0x9A,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );

    map.insert(
        OpCode::PHA,
        vec![Instruction {
            opcode: 0x48,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );

    map.insert(
        OpCode::PHP,
        vec![Instruction {
            opcode: 0x08,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );

    map.insert(
        OpCode::PLA,
        vec![Instruction {
            opcode: 0x68,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );

    map.insert(
        OpCode::PLP,
        vec![Instruction {
            opcode: 0x28,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );

    // SBC
    map.insert(
        OpCode::SBC,
        vec![
            Instruction {
                opcode: 0xE9,
                mode: AddressingMode::Immediate,
                bytes: 2,
            },
            Instruction {
                opcode: 0xE5,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
            },
            Instruction {
                opcode: 0xF5,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
            },
            Instruction {
                opcode: 0xED,
                mode: AddressingMode::Absolute,
                bytes: 3,
            },
            Instruction {
                opcode: 0xFD,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
            },
            Instruction {
                opcode: 0xF9,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
            },
            Instruction {
                opcode: 0xE1,
                mode: AddressingMode::IndirectX,
                bytes: 2,
            },
            Instruction {
                opcode: 0xF1,
                mode: AddressingMode::IndirectY,
                bytes: 2,
            },
        ],
    );

    // INC/DEC
    map.insert(
        OpCode::INC,
        vec![
            Instruction {
                opcode: 0xE6,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
            },
            Instruction {
                opcode: 0xF6,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
            },
            Instruction {
                opcode: 0xEE,
                mode: AddressingMode::Absolute,
                bytes: 3,
            },
            Instruction {
                opcode: 0xFE,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
            },
        ],
    );

    map.insert(
        OpCode::DEC,
        vec![
            Instruction {
                opcode: 0xC6,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
            },
            Instruction {
                opcode: 0xD6,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
            },
            Instruction {
                opcode: 0xCE,
                mode: AddressingMode::Absolute,
                bytes: 3,
            },
            Instruction {
                opcode: 0xDE,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
            },
        ],
    );

    // Single byte instructions
    map.insert(
        OpCode::INX,
        vec![Instruction {
            opcode: 0xE8,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );
    map.insert(
        OpCode::INY,
        vec![Instruction {
            opcode: 0xC8,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );
    map.insert(
        OpCode::DEX,
        vec![Instruction {
            opcode: 0xCA,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );
    map.insert(
        OpCode::DEY,
        vec![Instruction {
            opcode: 0x88,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );

    // AND (Logical AND)
    map.insert(
        OpCode::AND,
        vec![
            Instruction {
                opcode: 0x29,
                mode: AddressingMode::Immediate,
                bytes: 2,
            },
            Instruction {
                opcode: 0x25,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
            },
            Instruction {
                opcode: 0x35,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
            },
            Instruction {
                opcode: 0x2D,
                mode: AddressingMode::Absolute,
                bytes: 3,
            },
            Instruction {
                opcode: 0x3D,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
            },
            Instruction {
                opcode: 0x39,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
            },
            Instruction {
                opcode: 0x21,
                mode: AddressingMode::IndirectX,
                bytes: 2,
            },
            Instruction {
                opcode: 0x31,
                mode: AddressingMode::IndirectY,
                bytes: 2,
            },
        ],
    );

    // ORA (Logical OR)
    map.insert(
        OpCode::ORA,
        vec![
            Instruction {
                opcode: 0x09,
                mode: AddressingMode::Immediate,
                bytes: 2,
            },
            Instruction {
                opcode: 0x05,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
            },
            Instruction {
                opcode: 0x15,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
            },
            Instruction {
                opcode: 0x0D,
                mode: AddressingMode::Absolute,
                bytes: 3,
            },
            Instruction {
                opcode: 0x1D,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
            },
            Instruction {
                opcode: 0x19,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
            },
            Instruction {
                opcode: 0x01,
                mode: AddressingMode::IndirectX,
                bytes: 2,
            },
            Instruction {
                opcode: 0x11,
                mode: AddressingMode::IndirectY,
                bytes: 2,
            },
        ],
    );

    // EOR (Exclusive OR)
    map.insert(
        OpCode::EOR,
        vec![
            Instruction {
                opcode: 0x49,
                mode: AddressingMode::Immediate,
                bytes: 2,
            },
            Instruction {
                opcode: 0x45,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
            },
            Instruction {
                opcode: 0x55,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
            },
            Instruction {
                opcode: 0x4D,
                mode: AddressingMode::Absolute,
                bytes: 3,
            },
            Instruction {
                opcode: 0x5D,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
            },
            Instruction {
                opcode: 0x59,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
            },
            Instruction {
                opcode: 0x41,
                mode: AddressingMode::IndirectX,
                bytes: 2,
            },
            Instruction {
                opcode: 0x51,
                mode: AddressingMode::IndirectY,
                bytes: 2,
            },
        ],
    );

    // ASL (Arithmetic Shift Left)
    map.insert(
        OpCode::ASL,
        vec![
            Instruction {
                opcode: 0x0A,
                mode: AddressingMode::Implied,
                bytes: 1,
            },
            Instruction {
                opcode: 0x06,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
            },
            Instruction {
                opcode: 0x16,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
            },
            Instruction {
                opcode: 0x0E,
                mode: AddressingMode::Absolute,
                bytes: 3,
            },
            Instruction {
                opcode: 0x1E,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
            },
        ],
    );

    // LSR (Logical Shift Right)
    map.insert(
        OpCode::LSR,
        vec![
            Instruction {
                opcode: 0x4A,
                mode: AddressingMode::Implied,
                bytes: 1,
            },
            Instruction {
                opcode: 0x46,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
            },
            Instruction {
                opcode: 0x56,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
            },
            Instruction {
                opcode: 0x4E,
                mode: AddressingMode::Absolute,
                bytes: 3,
            },
            Instruction {
                opcode: 0x5E,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
            },
        ],
    );

    // ROL (Rotate Left)
    map.insert(
        OpCode::ROL,
        vec![
            Instruction {
                opcode: 0x2A,
                mode: AddressingMode::Implied,
                bytes: 1,
            },
            Instruction {
                opcode: 0x26,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
            },
            Instruction {
                opcode: 0x36,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
            },
            Instruction {
                opcode: 0x2E,
                mode: AddressingMode::Absolute,
                bytes: 3,
            },
            Instruction {
                opcode: 0x3E,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
            },
        ],
    );

    // ROR (Rotate Right)
    map.insert(
        OpCode::ROR,
        vec![
            Instruction {
                opcode: 0x6A,
                mode: AddressingMode::Implied,
                bytes: 1,
            },
            Instruction {
                opcode: 0x66,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
            },
            Instruction {
                opcode: 0x76,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
            },
            Instruction {
                opcode: 0x6E,
                mode: AddressingMode::Absolute,
                bytes: 3,
            },
            Instruction {
                opcode: 0x7E,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
            },
        ],
    );

    // Status Flag Changes
    map.insert(
        OpCode::CLC,
        vec![Instruction {
            opcode: 0x18,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );
    map.insert(
        OpCode::CLD,
        vec![Instruction {
            opcode: 0xD8,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );
    map.insert(
        OpCode::CLI,
        vec![Instruction {
            opcode: 0x58,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );
    map.insert(
        OpCode::CLV,
        vec![Instruction {
            opcode: 0xB8,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );
    map.insert(
        OpCode::SEC,
        vec![Instruction {
            opcode: 0x38,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );
    map.insert(
        OpCode::SED,
        vec![Instruction {
            opcode: 0xF8,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );
    map.insert(
        OpCode::SEI,
        vec![Instruction {
            opcode: 0x78,
            mode: AddressingMode::Implied,
            bytes: 1,
        }],
    );

    map
}

fn parse_operand(operand: &str) -> (AddressingMode, u16) {
    if let Some(stripped) = operand.strip_prefix('#') {
        // Immediate addressing - handle both #$2A and #42 formats
        let value_str = stripped.trim_start_matches('$');
        let value = u16::from_str_radix(value_str, 16).unwrap_or_else(|_| {
            // Try parsing as decimal if hex fails
            value_str.parse::<u16>().unwrap_or(0)
        });
        (AddressingMode::Immediate, value)
    } else if let Some(stripped) = operand.strip_prefix('$') {
        // Absolute or ZeroPage addressing
        let value = u16::from_str_radix(stripped, 16).unwrap_or(0);
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
        let mnemonic = OpCode::from_str(&parts[0].to_uppercase()).unwrap();

        if let Some(instructions) = opcodes.get(&mnemonic) {
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
            } else if let Some(instruction) = instructions
                .iter()
                .find(|i| i.mode == AddressingMode::Implied)
            {
                machine_code.push(instruction.opcode);
            }
        }
    }

    machine_code
}
