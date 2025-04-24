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
    BRA,
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
    pub opname: OpCode,
    pub opcode: u8,
    pub mode: AddressingMode,
    pub bytes: u8,
    pub cycles: u8,
}

use phf_macros::phf_map;

pub const INSTRUCTION_LOOKUP: phf::Map<u8, Instruction> = phf_map! {
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
        0xA0u8 =>
            Instruction {
                opcode: 0xA0,
                mode: AddressingMode::Immediate,
                bytes: 2,
                opname: OpCode::LDY,
                cycles: 2,
            },
            0xA4u8 =>
            Instruction {
                opcode: 0xA4,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                opname: OpCode::LDY,
                cycles: 3,
            },
            0xB4u8 =>
            Instruction {
                opcode: 0xB4,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                opname: OpCode::LDY,
                cycles: 4,
            },
0xACu8=>
            Instruction {
                opcode: 0xAC,
                mode: AddressingMode::Absolute,
                bytes: 3,
                opname: OpCode::LDY,
                cycles: 4,
            },
    0xBCu8=>
        Instruction {
            opcode: 0xBC,
            mode: AddressingMode::AbsoluteX,
            bytes: 3,
            opname: OpCode::LDY,
            cycles: 4,
        },

       0x86u8 =>     Instruction {
                opcode: 0x86,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                opname: OpCode::STX,
                cycles: 3,
            },
          0x96u8=>  Instruction {
                opcode: 0x96,
                mode: AddressingMode::ZeroPageY,
                bytes: 2,

                opname: OpCode::STX,
                cycles: 4,
},
          0x8eu8 =>
            Instruction {
                opcode: 0x8E,
                mode: AddressingMode::Absolute,
                bytes: 3,
                opname: OpCode::STX,
                cycles: 4,
            },
        0x84u8 =>
            Instruction {
                opcode: 0x84,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                opname: OpCode::STY,
                cycles: 3,
            },

        0x94u8 =>    Instruction {
                opcode: 0x94,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                opname: OpCode::STY,
                cycles: 4,
            },
         0x8Cu8 =>   Instruction {
                opcode: 0x8C,
                mode: AddressingMode::Absolute,
                bytes: 3,
                opname: OpCode::STY,
                cycles: 4,
            },
         0x85u8=>   Instruction {
                opcode: 0x85,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                opname: OpCode::STA,
                cycles: 3,
            },
         0x95u8 =>   Instruction {
                opcode: 0x95,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                opname: OpCode::STA,
                cycles: 4,
            },
          0x8Du8 =>  Instruction {
                opcode: 0x8D,
                mode: AddressingMode::Absolute,
                bytes: 3,
                opname: OpCode::STA,
                cycles: 4,
            },
         0x9Du8 =>   Instruction {
                opcode: 0x9D,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                opname: OpCode::STA,
                cycles: 5,
            },
          0x99u8 =>  Instruction {
                opcode: 0x99,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                opname: OpCode::STA,
                cycles: 5,
            },
          0x81u8 =>  Instruction {
                opcode: 0x81,
                mode: AddressingMode::IndirectX,
                bytes: 2,
                opname: OpCode::STA,
                cycles: 6,
            },
          0x91u8 =>  Instruction {
                opcode: 0x91,
                mode: AddressingMode::IndirectY,
                bytes: 2,
                opname: OpCode::STA,
                cycles: 6,
            },
            // ADC Instructions
    0x69u8 => Instruction {
        opname: OpCode::ADC,
        opcode: 0x69,
        mode: AddressingMode::Immediate,
        bytes: 2,
        cycles: 2,
    },
    0x65u8 => Instruction {
        opname: OpCode::ADC,
        opcode: 0x65,
        mode: AddressingMode::ZeroPage,
        bytes: 2,
        cycles: 3,
    },
    0x75u8 => Instruction {
        opname: OpCode::ADC,
        opcode: 0x75,
        mode: AddressingMode::ZeroPageX,
        bytes: 2,
        cycles: 4,
    },
    0x6Du8 => Instruction {
        opname: OpCode::ADC,
        opcode: 0x6D,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    },
    0x7Du8 => Instruction {
        opname: OpCode::ADC,
        opcode: 0x7D,
        mode: AddressingMode::AbsoluteX,
        bytes: 3,
        cycles: 4, // +1 if page crossed
    },
    0x79u8 => Instruction {
        opname: OpCode::ADC,
        opcode: 0x79,
        mode: AddressingMode::AbsoluteY,
        bytes: 3,
        cycles: 4, // +1 if page crossed
    },
    0x61u8 => Instruction {
        opname: OpCode::ADC,
        opcode: 0x61,
        mode: AddressingMode::IndirectX,
        bytes: 2,
        cycles: 6,
    },
    0x71u8 => Instruction {
        opname: OpCode::ADC,
        opcode: 0x71,
        mode: AddressingMode::IndirectY,
        bytes: 2,
        cycles: 5, // +1 if page crossed
    },
    // CMP Instructions
    0xC9u8 => Instruction {
        opname: OpCode::CMP,
        opcode: 0xC9,
        mode: AddressingMode::Immediate,
        bytes: 2,
        cycles: 2,
    },
    0xC5u8 => Instruction {
        opname: OpCode::CMP,
        opcode: 0xC5,
        mode: AddressingMode::ZeroPage,
        bytes: 2,
        cycles: 3,
    },
    0xD5u8 => Instruction {
        opname: OpCode::CMP,
        opcode: 0xD5,
        mode: AddressingMode::ZeroPageX,
        bytes: 2,
        cycles: 4,
    },
    0xCDu8 => Instruction {
        opname: OpCode::CMP,
        opcode: 0xCD,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    },
    0xDDu8 => Instruction {
        opname: OpCode::CMP,
        opcode: 0xDD,
        mode: AddressingMode::AbsoluteX,
        bytes: 3,
        cycles: 4, // +1 if page crossed
    },
    0xD9u8 => Instruction {
        opname: OpCode::CMP,
        opcode: 0xD9,
        mode: AddressingMode::AbsoluteY,
        bytes: 3,
        cycles: 4, // +1 if page crossed
    },
    0xC1u8 => Instruction {
        opname: OpCode::CMP,
        opcode: 0xC1,
        mode: AddressingMode::IndirectX,
        bytes: 2,
        cycles: 6,
    },
    0xD1u8 => Instruction {
        opname: OpCode::CMP,
        opcode: 0xD1,
        mode: AddressingMode::IndirectY,
        bytes: 2,
        cycles: 5, // +1 if page crossed
    },
    // Branch Instructions
    0xF0u8 => Instruction {
        opname: OpCode::BEQ,
        opcode: 0xF0,
        mode: AddressingMode::Immediate, // Note: 6502 uses Relative, but Immediate is used here
        bytes: 2,
        cycles: 2, // +1 if branch succeeds, +2 if page crossed
    },
    0xD0u8 => Instruction {
        opname: OpCode::BNE,
        opcode: 0xD0,
        mode: AddressingMode::Immediate, // Note: 6502 uses Relative
        bytes: 2,
        cycles: 2, // +1 if branch succeeds, +2 if page crossed
    },
    0xB0u8 => Instruction {
        opname: OpCode::BCS,
        opcode: 0xB0,
        mode: AddressingMode::Immediate, // Note: 6502 uses Relative
        bytes: 2,
        cycles: 2, // +1 if branch succeeds, +2 if page crossed
    },
    0x90u8 => Instruction {
        opname: OpCode::BCC,
        opcode: 0x90,
        mode: AddressingMode::Immediate, // Note: 6502 uses Relative
        bytes: 2,
        cycles: 2, // +1 if branch succeeds, +2 if page crossed
    },
    0x30u8 => Instruction {
        opname: OpCode::BMI,
        opcode: 0x30,
        mode: AddressingMode::Immediate, // Note: 6502 uses Relative
        bytes: 2,
        cycles: 2, // +1 if branch succeeds, +2 if page crossed
    },
    0x10u8 => Instruction {
        opname: OpCode::BPL,
        opcode: 0x10,
        mode: AddressingMode::Immediate, // Note: 6502 uses Relative
        bytes: 2,
        cycles: 2, // +1 if branch succeeds, +2 if page crossed
    },
    0x50u8 => Instruction {
        opname: OpCode::BVC,
        opcode: 0x50,
        mode: AddressingMode::Immediate, // Note: 6502 uses Relative
        bytes: 2,
        cycles: 2, // +1 if branch succeeds, +2 if page crossed
    },
    0x70u8 => Instruction {
        opname: OpCode::BVS,
        opcode: 0x70,
        mode: AddressingMode::Immediate, // Note: 6502 uses Relative
        bytes: 2,
        cycles: 2, // +1 if branch succeeds, +2 if page crossed
    },
        0x60u8 => Instruction {
        opname: OpCode::RTS,
        opcode: 0x60,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 6,
    },
    0x00u8 => Instruction {
        opname: OpCode::BRK,
        opcode: 0x00,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 7,
    },
    // Register Transfers
        0xAAu8 => Instruction {
        opname: OpCode::TAX,
        opcode: 0xAA,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
    0xA8u8 => Instruction {
        opname: OpCode::TAY,
        opcode: 0xA8,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
    0x8Au8 => Instruction {
        opname: OpCode::TXA,
        opcode: 0x8A,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
    0x98u8 => Instruction {
        opname: OpCode::TYA,
        opcode: 0x98,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
        // Stack Operations
    0xBAu8 => Instruction {
        opname: OpCode::TSX,
        opcode: 0xBA,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
    0x9Au8 => Instruction {
        opname: OpCode::TXS,
        opcode: 0x9A,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
    0x48u8 => Instruction {
        opname: OpCode::PHA,
        opcode: 0x48,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 3,
    },
    0x08u8 => Instruction {
        opname: OpCode::PHP,
        opcode: 0x08,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 3,
    },
    0x68u8 => Instruction {
        opname: OpCode::PLA,
        opcode: 0x68,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 4,
    },
    0x28u8 => Instruction {
        opname: OpCode::PLP,
        opcode: 0x28,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 4,
    },
        // SBC Instructions
    0xE9u8 => Instruction {
        opname: OpCode::SBC,
        opcode: 0xE9,
        mode: AddressingMode::Immediate,
        bytes: 2,
        cycles: 2,
    },
    0xE5u8 => Instruction {
        opname: OpCode::SBC,
        opcode: 0xE5,
        mode: AddressingMode::ZeroPage,
        bytes: 2,
        cycles: 3,
    },
    0xF5u8 => Instruction {
        opname: OpCode::SBC,
        opcode: 0xF5,
        mode: AddressingMode::ZeroPageX,
        bytes: 2,
        cycles: 4,
    },
    0xEDu8 => Instruction {
        opname: OpCode::SBC,
        opcode: 0xED,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    },
    0xFDu8 => Instruction {
        opname: OpCode::SBC,
        opcode: 0xFD,
        mode: AddressingMode::AbsoluteX,
        bytes: 3,
        cycles: 4, // +1 if page crossed
    },
    0xF9u8 => Instruction {
        opname: OpCode::SBC,
        opcode: 0xF9,
        mode: AddressingMode::AbsoluteY,
        bytes: 3,
        cycles: 4, // +1 if page crossed
    },
    0xE1u8 => Instruction {
        opname: OpCode::SBC,
        opcode: 0xE1,
        mode: AddressingMode::IndirectX,
        bytes: 2,
        cycles: 6,
    },
    0xF1u8 => Instruction {
        opname: OpCode::SBC,
        opcode: 0xF1,
        mode: AddressingMode::IndirectY,
        bytes: 2,
        cycles: 5, // +1 if page crossed
    },
    // INC Instructions
    0xE6u8 => Instruction {
        opname: OpCode::INC,
        opcode: 0xE6,
        mode: AddressingMode::ZeroPage,
        bytes: 2,
        cycles: 5,
    },
    0xF6u8 => Instruction {
        opname: OpCode::INC,
        opcode: 0xF6,
        mode: AddressingMode::ZeroPageX,
        bytes: 2,
        cycles: 6,
    },
    0xEEu8 => Instruction {
        opname: OpCode::INC,
        opcode: 0xEE,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 6,
    },
    0xFEu8 => Instruction {
        opname: OpCode::INC,
        opcode: 0xFE,
        mode: AddressingMode::AbsoluteX,
        bytes: 3,
        cycles: 7,
    },
    // DEC Instructions
    0xC6u8 => Instruction {
        opname: OpCode::DEC,
        opcode: 0xC6,
        mode: AddressingMode::ZeroPage,
        bytes: 2,
        cycles: 5,
    },
    0xD6u8 => Instruction {
        opname: OpCode::DEC,
        opcode: 0xD6,
        mode: AddressingMode::ZeroPageX,
        bytes: 2,
        cycles: 6,
    },
    0xCEu8 => Instruction {
        opname: OpCode::DEC,
        opcode: 0xCE,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 6,
    },
    0xDEu8 => Instruction {
        opname: OpCode::DEC,
        opcode: 0xDE,
        mode: AddressingMode::AbsoluteX,
        bytes: 3,
        cycles: 7,
    },
    // Single byte instructions (Implied Addressing)
    0xE8u8 => Instruction {
        opname: OpCode::INX,
        opcode: 0xE8,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
    0xC8u8 => Instruction {
        opname: OpCode::INY,
        opcode: 0xC8,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
    0xCAu8 => Instruction {
        opname: OpCode::DEX,
        opcode: 0xCA,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
    0x88u8 => Instruction {
        opname: OpCode::DEY,
        opcode: 0x88,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
    // AND Instructions
    0x29u8 => Instruction {
        opname: OpCode::AND,
        opcode: 0x29,
        mode: AddressingMode::Immediate,
        bytes: 2,
        cycles: 2,
    },
    0x25u8 => Instruction {
        opname: OpCode::AND,
        opcode: 0x25,
        mode: AddressingMode::ZeroPage,
        bytes: 2,
        cycles: 3,
    },
    0x35u8 => Instruction {
        opname: OpCode::AND,
        opcode: 0x35,
        mode: AddressingMode::ZeroPageX,
        bytes: 2,
        cycles: 4,
    },
    0x2Du8 => Instruction {
        opname: OpCode::AND,
        opcode: 0x2D,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    },
    0x3Du8 => Instruction {
        opname: OpCode::AND,
        opcode: 0x3D,
        mode: AddressingMode::AbsoluteX,
        bytes: 3,
        cycles: 4, // +1 if page crossed
    },
    0x39u8 => Instruction {
        opname: OpCode::AND,
        opcode: 0x39,
        mode: AddressingMode::AbsoluteY,
        bytes: 3,
        cycles: 4, // +1 if page crossed
    },
    0x21u8 => Instruction {
        opname: OpCode::AND,
        opcode: 0x21,
        mode: AddressingMode::IndirectX,
        bytes: 2,
        cycles: 6,
    },
    0x31u8 => Instruction {
        opname: OpCode::AND,
        opcode: 0x31,
        mode: AddressingMode::IndirectY,
        bytes: 2,
        cycles: 5, // +1 if page crossed
    },
    // ORA Instructions
    0x09u8 => Instruction {
        opname: OpCode::ORA,
        opcode: 0x09,
        mode: AddressingMode::Immediate,
        bytes: 2,
        cycles: 2,
    },
    0x05u8 => Instruction {
        opname: OpCode::ORA,
        opcode: 0x05,
        mode: AddressingMode::ZeroPage,
        bytes: 2,
        cycles: 3,
    },
    0x15u8 => Instruction {
        opname: OpCode::ORA,
        opcode: 0x15,
        mode: AddressingMode::ZeroPageX,
        bytes: 2,
        cycles: 4,
    },
    0x0Du8 => Instruction {
        opname: OpCode::ORA,
        opcode: 0x0D,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    },
    0x1Du8 => Instruction {
        opname: OpCode::ORA,
        opcode: 0x1D,
        mode: AddressingMode::AbsoluteX,
        bytes: 3,
        cycles: 4, // +1 if page crossed
    },
    0x19u8 => Instruction {
        opname: OpCode::ORA,
        opcode: 0x19,
        mode: AddressingMode::AbsoluteY,
        bytes: 3,
        cycles: 4, // +1 if page crossed
    },
    0x01u8 => Instruction {
        opname: OpCode::ORA,
        opcode: 0x01,
        mode: AddressingMode::IndirectX,
        bytes: 2,
        cycles: 6,
    },
    0x11u8 => Instruction {
        opname: OpCode::ORA,
        opcode: 0x11,
        mode: AddressingMode::IndirectY,
        bytes: 2,
        cycles: 5, // +1 if page crossed
    },
        // EOR Instructions
    0x49u8 => Instruction {
        opname: OpCode::EOR,
        opcode: 0x49,
        mode: AddressingMode::Immediate,
        bytes: 2,
        cycles: 2,
    },
    0x45u8 => Instruction {
        opname: OpCode::EOR,
        opcode: 0x45,
        mode: AddressingMode::ZeroPage,
        bytes: 2,
        cycles: 3, // Corrected cycle count
    },
    0x55u8 => Instruction {
        opname: OpCode::EOR,
        opcode: 0x55,
        mode: AddressingMode::ZeroPageX,
        bytes: 2,
        cycles: 4,
    },
    0x4Du8 => Instruction {
        opname: OpCode::EOR,
        opcode: 0x4D,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    },
    0x5Du8 => Instruction {
        opname: OpCode::EOR,
        opcode: 0x5D,
        mode: AddressingMode::AbsoluteX,
        bytes: 3,
        cycles: 4, // +1 if page crossed
    },
    0x59u8 => Instruction {
        opname: OpCode::EOR,
        opcode: 0x59,
        mode: AddressingMode::AbsoluteY,
        bytes: 3,
        cycles: 4, // +1 if page crossed
    },
    0x41u8 => Instruction {
        opname: OpCode::EOR,
        opcode: 0x41,
        mode: AddressingMode::IndirectX,
        bytes: 2,
        cycles: 6,
    },
    0x51u8 => Instruction {
        opname: OpCode::EOR,
        opcode: 0x51,
        mode: AddressingMode::IndirectY,
        bytes: 2,
        cycles: 5, // +1 if page crossed
    },
    // ASL Instructions
    0x0Au8 => Instruction {
        opname: OpCode::ASL,
        opcode: 0x0A,
        mode: AddressingMode::Implied, // Accumulator
        bytes: 1,
        cycles: 2,
    },
    0x06u8 => Instruction {
        opname: OpCode::ASL,
        opcode: 0x06,
        mode: AddressingMode::ZeroPage,
        bytes: 2,
        cycles: 5,
    },
    0x16u8 => Instruction {
        opname: OpCode::ASL,
        opcode: 0x16,
        mode: AddressingMode::ZeroPageX,
        bytes: 2,
        cycles: 6,
    },
    0x0Eu8 => Instruction {
        opname: OpCode::ASL,
        opcode: 0x0E,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 6,
    },
    0x1Eu8 => Instruction {
        opname: OpCode::ASL,
        opcode: 0x1E,
        mode: AddressingMode::AbsoluteX,
        bytes: 3,
        cycles: 7,
    },
    // LSR Instructions
    0x4Au8 => Instruction {
        opname: OpCode::LSR,
        opcode: 0x4A,
        mode: AddressingMode::Implied, // Accumulator
        bytes: 1,
        cycles: 2,
    },
    0x46u8 => Instruction {
        opname: OpCode::LSR,
        opcode: 0x46,
        mode: AddressingMode::ZeroPage,
        bytes: 2,
        cycles: 5,
    },
    0x56u8 => Instruction {
        opname: OpCode::LSR,
        opcode: 0x56,
        mode: AddressingMode::ZeroPageX,
        bytes: 2,
        cycles: 6,
    },
    0x4Eu8 => Instruction {
        opname: OpCode::LSR,
        opcode: 0x4E,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 6,
    },
    0x5Eu8 => Instruction {
        opname: OpCode::LSR,
        opcode: 0x5E,
        mode: AddressingMode::AbsoluteX,
        bytes: 3,
        cycles: 7,
    },
    // ROL Instructions
    0x2Au8 => Instruction {
        opname: OpCode::ROL,
        opcode: 0x2A,
        mode: AddressingMode::Implied, // Accumulator
        bytes: 1,
        cycles: 2,
    },
    0x26u8 => Instruction {
        opname: OpCode::ROL,
        opcode: 0x26,
        mode: AddressingMode::ZeroPage,
        bytes: 2,
        cycles: 5,
    },
    0x36u8 => Instruction {
        opname: OpCode::ROL,
        opcode: 0x36,
        mode: AddressingMode::ZeroPageX,
        bytes: 2,
        cycles: 6,
    },
    0x2Eu8 => Instruction {
        opname: OpCode::ROL,
        opcode: 0x2E,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 6,
    },
    0x3Eu8 => Instruction {
        opname: OpCode::ROL,
        opcode: 0x3E,
        mode: AddressingMode::AbsoluteX,
        bytes: 3,
        cycles: 7,
    },
    // ROR Instructions
    0x6Au8 => Instruction {
        opname: OpCode::ROR,
        opcode: 0x6A,
        mode: AddressingMode::Implied, // Accumulator
        bytes: 1,
        cycles: 2,
    },
    0x66u8 => Instruction {
        opname: OpCode::ROR,
        opcode: 0x66,
        mode: AddressingMode::ZeroPage,
        bytes: 2,
        cycles: 5,
    },
    0x76u8 => Instruction {
        opname: OpCode::ROR,
        opcode: 0x76,
        mode: AddressingMode::ZeroPageX,
        bytes: 2,
        cycles: 6,
    },
    0x6Eu8 => Instruction {
        opname: OpCode::ROR,
        opcode: 0x6E,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 6,
    },
    0x7Eu8 => Instruction {
        opname: OpCode::ROR,
        opcode: 0x7E,
        mode: AddressingMode::AbsoluteX,
        bytes: 3,
        cycles: 7,
    },
        // Status Flag Changes
    0x18u8 => Instruction {
        opname: OpCode::CLC,
        opcode: 0x18,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
    0xD8u8 => Instruction {
        opname: OpCode::CLD,
        opcode: 0xD8,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
    0x58u8 => Instruction {
        opname: OpCode::CLI,
        opcode: 0x58,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
    0xB8u8 => Instruction {
        opname: OpCode::CLV,
        opcode: 0xB8,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
    0x38u8 => Instruction {
        opname: OpCode::SEC,
        opcode: 0x38,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
    0xF8u8 => Instruction {
        opname: OpCode::SED,
        opcode: 0xF8,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
    0x78u8 => Instruction {
        opname: OpCode::SEI,
        opcode: 0x78,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    },
};

pub fn create_opcode_map() -> HashMap<OpCode, Vec<Instruction>> {
    let mut map = HashMap::new();

    for instruction in INSTRUCTION_LOOKUP.values() {
        map.entry(instruction.opname)
            .or_insert_with(Vec::new)
            .push(instruction.clone());
    }

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
