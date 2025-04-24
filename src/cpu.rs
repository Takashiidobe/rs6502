use crate::assembler::AddressingMode;
use crate::memory::Memory;

pub struct CPU {
    pub a: u8,          // Accumulator
    pub x: u8,          // X Register
    pub y: u8,          // Y Register
    pub pc: u16,        // Program Counter
    pub sp: u8,         // Stack Pointer
    pub status: u8,     // Status Register
    pub memory: Memory, // Memory instance
    pub halted: bool,   // Flag to indicate if CPU execution should stop
}

impl CPU {
    pub fn new(memory: Memory) -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,       //
            pc: 0x0000, // 16-bit program counter
            sp: 0xFF,   // 16-bit stack pointer
            status: 0,
            memory,
            halted: false, // Initialize halted to false
        }
    }

    pub fn reset(&mut self) {
        self.pc = self.memory.read_u16(0xFFFC); // Read reset vector
        self.sp = 0xFD; // Reset stack pointer
        self.status = 0x00; // Clear status register
    }

    pub fn execute_instruction(&mut self) {
        if self.halted {
            return;
        }
        let opcode = self.memory.read(self.pc);
        self.pc += 1;

        match opcode {
            // LDA (Load Accumulator)
            0xA9 => self.lda(&AddressingMode::Immediate),
            0xA5 => self.lda(&AddressingMode::ZeroPage),
            0xB5 => self.lda(&AddressingMode::ZeroPageX),
            0xAD => self.lda(&AddressingMode::Absolute),
            0xBD => self.lda(&AddressingMode::AbsoluteX),
            0xB9 => self.lda(&AddressingMode::AbsoluteY),
            0xA1 => self.lda(&AddressingMode::IndirectX),
            0xB1 => self.lda(&AddressingMode::IndirectY),

            // LDX (Load X Register)
            0xA2 => self.ldx(&AddressingMode::Immediate),
            0xA6 => self.ldx(&AddressingMode::ZeroPage),
            0xB6 => self.ldx(&AddressingMode::ZeroPageY),
            0xAE => self.ldx(&AddressingMode::Absolute),
            0xBE => self.ldx(&AddressingMode::AbsoluteY),

            // LDY (Load Y Register)
            0xA0 => self.ldy(&AddressingMode::Immediate),
            0xA4 => self.ldy(&AddressingMode::ZeroPage),
            0xB4 => self.ldy(&AddressingMode::ZeroPageX),
            0xAC => self.ldy(&AddressingMode::Absolute),
            0xBC => self.ldy(&AddressingMode::AbsoluteX),

            // STA (Store Accumulator)
            0x85 => self.sta(&AddressingMode::ZeroPage),
            0x95 => self.sta(&AddressingMode::ZeroPageX),
            0x8D => self.sta(&AddressingMode::Absolute),
            0x9D => self.sta(&AddressingMode::AbsoluteX),
            0x99 => self.sta(&AddressingMode::AbsoluteY),
            0x81 => self.sta(&AddressingMode::IndirectX),
            0x91 => self.sta(&AddressingMode::IndirectY),

            // STX (Store X Register)
            0x86 => self.stx(&AddressingMode::ZeroPage),
            0x96 => self.stx(&AddressingMode::ZeroPageY),
            0x8E => self.stx(&AddressingMode::Absolute),

            // STY (Store Y Register)
            0x84 => self.sty(&AddressingMode::ZeroPage),
            0x94 => self.sty(&AddressingMode::ZeroPageX),
            0x8C => self.sty(&AddressingMode::Absolute),

            0x69 => self.adc(&AddressingMode::Immediate),
            0x65 => self.adc(&AddressingMode::ZeroPage),
            0x75 => self.adc(&AddressingMode::ZeroPageX),
            0x6D => self.adc(&AddressingMode::Absolute),
            0x7D => self.adc(&AddressingMode::AbsoluteX),
            0x79 => self.adc(&AddressingMode::AbsoluteY),
            0x61 => self.adc(&AddressingMode::IndirectX),
            0x71 => self.adc(&AddressingMode::IndirectY),
            0xC9 => {
                // CMP Immediate
                let value = self.memory.read(self.pc);
                self.pc += 1;
                let result = self.a.wrapping_sub(value);
                self.set_carry_flag(self.a >= value);
                self.set_zero_flag(result);
                self.set_negative_flag(result);
            }
            0xF0 => {
                // BEQ Relative
                let offset = self.memory.read(self.pc) as i8;
                self.pc += 1;
                if self.get_zero_flag() {
                    let jump_addr = ((self.pc as i32) + (offset as i32)) as u16;
                    self.pc = jump_addr;
                }
            }
            0x60 => {
                // RTS
                // For now, just halt since we haven't implemented the stack
                self.halted = true;
            }
            0x00 => {
                // BRK
                self.set_break_flag(true);
                self.halted = true;
            }
            0xAA => {
                // TAX
                self.x = self.a;
                self.set_zero_flag(self.x);
                self.set_negative_flag(self.x);
            }
            0xA8 => {
                // TAY
                self.y = self.a;
                self.set_zero_flag(self.y);
                self.set_negative_flag(self.y);
            }
            0x8A => {
                // TXA
                self.a = self.x;
                self.set_zero_flag(self.a);
                self.set_negative_flag(self.a);
            }
            0x98 => {
                // TYA
                self.a = self.y;
                self.set_zero_flag(self.a);
                self.set_negative_flag(self.a);
            }
            0xBA => {
                // TSX
                self.x = self.sp;
                self.set_zero_flag(self.x);
                self.set_negative_flag(self.x);
            }
            0x9A => self.sp = self.x, // TXS
            0x48 => {
                // PHA
                self.push(self.a);
            }
            0x08 => {
                // PHP
                self.push(self.status | 0b0011_0000); // Set bits 4 and 5 when pushing
            }
            0x68 => {
                // PLA
                self.a = self.pull();
                self.set_zero_flag(self.a);
                self.set_negative_flag(self.a);
            }
            0x28 => {
                // PLP
                self.status = (self.pull() & 0b1110_1111) | 0b0010_0000; // Keep bit 5 set, ignore bit 4
            }
            0xE8 => self.inx(),
            0xC8 => self.iny(),
            0xCA => self.dex(),
            0x88 => self.dey(),
            0xE6 => self.inc(&AddressingMode::ZeroPage),
            0xF6 => self.inc(&AddressingMode::ZeroPageX),
            0xEE => self.inc(&AddressingMode::Absolute),
            0xFE => self.inc(&AddressingMode::AbsoluteX),
            0xC6 => self.dec(&AddressingMode::ZeroPage),
            0xD6 => self.dec(&AddressingMode::ZeroPageX),
            0xCE => self.dec(&AddressingMode::Absolute),
            0xDE => self.dec(&AddressingMode::AbsoluteX),
            0xE9 => self.sbc(&AddressingMode::Immediate),
            0xE5 => self.sbc(&AddressingMode::ZeroPage),
            0xF5 => self.sbc(&AddressingMode::ZeroPageX),
            0xED => self.sbc(&AddressingMode::Absolute),
            0xFD => self.sbc(&AddressingMode::AbsoluteX),
            0xF9 => self.sbc(&AddressingMode::AbsoluteY),
            0xE1 => self.sbc(&AddressingMode::IndirectX),
            0xF1 => self.sbc(&AddressingMode::IndirectY),
            0x29 => self.and(&AddressingMode::Immediate),
            0x25 => self.and(&AddressingMode::ZeroPage),
            0x35 => self.and(&AddressingMode::ZeroPageX),
            0x2D => self.and(&AddressingMode::Absolute),
            0x3D => self.and(&AddressingMode::AbsoluteX),
            0x39 => self.and(&AddressingMode::AbsoluteY),
            0x21 => self.and(&AddressingMode::IndirectX),
            0x31 => self.and(&AddressingMode::IndirectY),
            0x09 => self.ora(&AddressingMode::Immediate),
            0x05 => self.ora(&AddressingMode::ZeroPage),
            0x15 => self.ora(&AddressingMode::ZeroPageX),
            0x0D => self.ora(&AddressingMode::Absolute),
            0x1D => self.ora(&AddressingMode::AbsoluteX),
            0x19 => self.ora(&AddressingMode::AbsoluteY),
            0x01 => self.ora(&AddressingMode::IndirectX),
            0x11 => self.ora(&AddressingMode::IndirectY),
            0x49 => self.eor(&AddressingMode::Immediate),
            0x45 => self.eor(&AddressingMode::ZeroPage),
            0x55 => self.eor(&AddressingMode::ZeroPageX),
            0x4D => self.eor(&AddressingMode::Absolute),
            0x5D => self.eor(&AddressingMode::AbsoluteX),
            0x59 => self.eor(&AddressingMode::AbsoluteY),
            0x41 => self.eor(&AddressingMode::IndirectX),
            0x51 => self.eor(&AddressingMode::IndirectY),
            0x0A => self.asl_accumulator(),
            0x06 => self.asl(&AddressingMode::ZeroPage),
            0x16 => self.asl(&AddressingMode::ZeroPageX),
            0x0E => self.asl(&AddressingMode::Absolute),
            0x1E => self.asl(&AddressingMode::AbsoluteX),
            0x4A => self.lsr_accumulator(),
            0x46 => self.lsr(&AddressingMode::ZeroPage),
            0x56 => self.lsr(&AddressingMode::ZeroPageX),
            0x4E => self.lsr(&AddressingMode::Absolute),
            0x5E => self.lsr(&AddressingMode::AbsoluteX),
            0x2A => self.rol_accumulator(),
            0x26 => self.rol(&AddressingMode::ZeroPage),
            0x36 => self.rol(&AddressingMode::ZeroPageX),
            0x2E => self.rol(&AddressingMode::Absolute),
            0x3E => self.rol(&AddressingMode::AbsoluteX),
            0x6A => self.ror_accumulator(),
            0x66 => self.ror(&AddressingMode::ZeroPage),
            0x76 => self.ror(&AddressingMode::ZeroPageX),
            0x6E => self.ror(&AddressingMode::Absolute),
            0x7E => self.ror(&AddressingMode::AbsoluteX),
            // Branch Instructions
            0x80 => self.branch(true), // BRA (Branch always)
            0xB0 => self.branch(self.get_carry_flag()), // BCS (Branch if Carry Set)
            0x90 => self.branch(!self.get_carry_flag()), // BCC (Branch if Carry Clear)
            0x30 => self.branch(self.get_negative_flag()), // BMI (Branch if Minus)
            0x10 => self.branch(!self.get_negative_flag()), // BPL (Branch if Plus)
            0x50 => self.branch(!self.get_overflow_flag()), // BVC (Branch if Overflow Clear)
            0x70 => self.branch(self.get_overflow_flag()), // BVS (Branch if Overflow Set)
            // Status Flag Changes
            0x18 => self.clear_carry(),     // CLC
            0xD8 => self.clear_decimal(),   // CLD
            0x58 => self.clear_interrupt(), // CLI
            0xB8 => self.clear_overflow(),  // CLV
            0x38 => self.set_carry(),       // SEC
            0xF8 => self.set_decimal(),     // SED
            0x78 => self.set_interrupt(),   // SEI
            _ => {
                println!(
                    "Opcode {:02X} at address {:04X} not implemented",
                    opcode,
                    self.pc - 1
                );
                self.halted = true;
            }
        }
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let value = self.get_operand(mode);
        let carry = self.get_carry_flag() as u16;
        let sum = self.a as u16 + value as u16 + carry;

        self.set_carry_flag(sum > 0xFF);
        let result = sum as u8;
        self.set_overflow_flag(((self.a ^ result) & (value ^ result) & 0x80) != 0);

        self.a = result;
        self.update_zero_and_negative_flags(self.a);
    }

    fn and(&mut self, mode: &AddressingMode) {
        let value = self.get_operand(mode);
        self.a &= value;
        self.update_zero_and_negative_flags(self.a);
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let value = self.get_operand(mode);
        self.a |= value;
        self.update_zero_and_negative_flags(self.a);
    }

    fn eor(&mut self, mode: &AddressingMode) {
        let value = self.get_operand(mode);
        self.a ^= value;
        self.update_zero_and_negative_flags(self.a);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.set_carry_flag(value & 0x80 != 0);
        let result = value << 1;
        self.mem_write(addr, result);
        self.update_zero_and_negative_flags(result);
    }

    fn asl_accumulator(&mut self) {
        self.set_carry_flag(self.a & 0x80 != 0);
        self.a <<= 1;
        self.update_zero_and_negative_flags(self.a);
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.set_carry_flag(value & 0x01 != 0);
        let result = value >> 1;
        self.mem_write(addr, result);
        self.update_zero_and_negative_flags(result);
    }

    fn lsr_accumulator(&mut self) {
        self.set_carry_flag(self.a & 0x01 != 0);
        self.a >>= 1;
        self.update_zero_and_negative_flags(self.a);
    }

    fn rol(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let carry = self.get_carry_flag() as u8;
        self.set_carry_flag(value & 0x80 != 0);
        let result = (value << 1) | carry;
        self.mem_write(addr, result);
        self.update_zero_and_negative_flags(result);
    }

    fn rol_accumulator(&mut self) {
        let carry = self.get_carry_flag() as u8;
        self.set_carry_flag(self.a & 0x80 != 0);
        self.a = (self.a << 1) | carry;
        self.update_zero_and_negative_flags(self.a);
    }

    fn ror(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let carry = self.get_carry_flag() as u8;
        self.set_carry_flag(value & 0x01 != 0);
        let result = (value >> 1) | (carry << 7);
        self.mem_write(addr, result);
        self.update_zero_and_negative_flags(result);
    }

    fn ror_accumulator(&mut self) {
        let carry = self.get_carry_flag() as u8;
        self.set_carry_flag(self.a & 0x01 != 0);
        self.a = (self.a >> 1) | (carry << 7);
        self.update_zero_and_negative_flags(self.a);
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            let offset = self.memory.read(self.pc) as i8; // Read signed offset
            self.pc += 1; // Increment program counter
            self.pc = ((self.pc as i32) + (offset as i32)) as u16; // Calculate new address
        } else {
            self.pc += 1; // Skip the offset byte
        }
    }

    fn clear_carry(&mut self) {
        self.set_carry_flag(false);
    }

    fn clear_decimal(&mut self) {
        self.set_decimal_flag(false);
    }

    fn clear_interrupt(&mut self) {
        self.set_interrupt_disable_flag(false);
    }

    fn clear_overflow(&mut self) {
        self.set_overflow_flag(false);
    }

    fn set_carry(&mut self) {
        self.set_carry_flag(true);
    }

    fn set_decimal(&mut self) {
        self.set_decimal_flag(true);
    }

    fn set_interrupt(&mut self) {
        self.set_interrupt_disable_flag(true);
    }
}

impl CPU {
    pub fn lda(&mut self, mode: &AddressingMode) {
        let value = self.get_operand(mode);
        self.a = value;
        self.set_zero_flag(self.a);
        self.set_negative_flag(self.a);
    }

    pub fn ldx(&mut self, mode: &AddressingMode) {
        let value = self.get_operand(mode);
        self.x = value;
        self.set_zero_flag(self.x);
        self.set_negative_flag(self.x);
    }

    pub fn ldy(&mut self, mode: &AddressingMode) {
        let value = self.get_operand(mode);
        self.y = value;
        self.set_zero_flag(self.y);
        self.set_negative_flag(self.y);
    }

    pub fn sta(&mut self, mode: &AddressingMode) {
        self.memory.write(self.get_operand_address(mode), self.a);
    }

    pub fn stx(&mut self, mode: &AddressingMode) {
        self.memory.write(self.get_operand_address(mode), self.x);
    }

    pub fn sty(&mut self, mode: &AddressingMode) {
        self.memory.write(self.get_operand_address(mode), self.y);
    }

    fn set_flag(&mut self, mask: u8) {
        self.status |= mask;
    }

    fn clear_flag(&mut self, mask: u8) {
        self.status &= !mask;
    }

    pub fn set_negative_flag(&mut self, value: u8) {
        if value & 0x80 != 0 {
            self.set_flag(0b1000_0000); // Set N flag
        } else {
            self.clear_flag(0b1000_0000); // Clear N flag
        }
    }

    pub fn set_overflow_flag(&mut self, overflow: bool) {
        if overflow {
            self.set_flag(0b0100_0000); // Set V flag
        } else {
            self.clear_flag(0b0100_0000); // Clear V flag
        }
    }

    pub fn set_break_flag(&mut self, break_flag: bool) {
        if break_flag {
            self.set_flag(0b0001_0000); // Set B flag
        } else {
            self.clear_flag(0b0001_0000); // Clear B flag
        }
    }

    pub fn set_decimal_flag(&mut self, decimal: bool) {
        if decimal {
            self.set_flag(0b0000_1000); // Set D flag
        } else {
            self.clear_flag(0b0000_1000); // Clear D flag
        }
    }

    pub fn set_interrupt_disable_flag(&mut self, interrupt_disable: bool) {
        if interrupt_disable {
            self.set_flag(0b0000_0100); // Set I flag
        } else {
            self.clear_flag(0b0000_0100); // Clear I flag
        }
    }

    pub fn set_zero_flag(&mut self, value: u8) {
        if value == 0 {
            self.set_flag(0b0000_0010); // Set Z flag
        } else {
            self.clear_flag(0b0000_0010); // Clear Z flag
        }
    }

    pub fn set_carry_flag(&mut self, carry: bool) {
        if carry {
            self.set_flag(0b0000_0001); // Set C flag
        } else {
            self.clear_flag(0b0000_0001); // Clear C flag
        }
    }

    pub fn get_negative_flag(&self) -> bool {
        self.status & 0b1000_0000 != 0
    }

    pub fn get_overflow_flag(&self) -> bool {
        self.status & 0b0100_0000 != 0
    }

    pub fn get_break_flag(&self) -> bool {
        self.status & 0b0001_0000 != 0
    }

    pub fn get_decimal_flag(&self) -> bool {
        self.status & 0b0000_1000 != 0
    }

    pub fn get_interrupt_disable_flag(&self) -> bool {
        self.status & 0b0000_0100 != 0
    }

    pub fn get_zero_flag(&self) -> bool {
        self.status & 0b0000_0010 != 0
    }

    pub fn get_carry_flag(&self) -> bool {
        self.status & 0b0000_0001 != 0
    }

    fn push(&mut self, value: u8) {
        self.memory.write(0x0100 + self.sp as u16, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pull(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.memory.read(0x0100 + self.sp as u16)
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let value = self.get_operand(mode);
        let carry = self.get_carry_flag() as u8;
        let result = self.a.wrapping_sub(value).wrapping_sub(carry);

        self.set_carry_flag(self.a >= value);
        self.set_overflow_flag((self.a ^ value) & (self.a ^ result) & 0x80 != 0);

        self.a = result;
        self.update_zero_and_negative_flags(self.a);
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let result = value.wrapping_add(1);
        self.mem_write(addr, result);
        self.update_zero_and_negative_flags(result);
    }

    fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.x);
    }

    fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.y);
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let result = value.wrapping_sub(1);
        self.mem_write(addr, result);
        self.update_zero_and_negative_flags(result);
    }

    fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.x);
    }

    fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.y);
    }

    fn get_operand(&mut self, mode: &AddressingMode) -> u8 {
        match mode {
            AddressingMode::Immediate => {
                let val = self.memory.read(self.pc);
                self.pc += 1;
                val
            }
            _ => {
                let addr = self.get_operand_address(mode);
                self.memory.read(addr)
            }
        }
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::ZeroPage => self.memory.read(self.pc) as u16,
            AddressingMode::ZeroPageX => {
                let pos = self.memory.read(self.pc);
                pos.wrapping_add(self.x) as u16
            }
            AddressingMode::ZeroPageY => {
                let pos = self.memory.read(self.pc);
                pos.wrapping_add(self.y) as u16
            }
            AddressingMode::Absolute => self.memory.read_u16(self.pc),
            AddressingMode::AbsoluteX => {
                let base = self.memory.read_u16(self.pc);
                base.wrapping_add(self.x as u16)
            }
            AddressingMode::AbsoluteY => {
                let base = self.memory.read_u16(self.pc);
                base.wrapping_add(self.y as u16)
            }
            AddressingMode::IndirectX => {
                let base = self.memory.read(self.pc);
                let ptr = base.wrapping_add(self.x);
                let lo = self.memory.read(ptr as u16);
                let hi = self.memory.read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::IndirectY => {
                let base = self.memory.read(self.pc);
                let lo = self.memory.read(base as u16);
                let hi = self.memory.read(base.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                deref_base.wrapping_add(self.y as u16)
            }
            _ => panic!("mode {:?} not supported", mode),
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory.read(addr)
    }

    fn mem_write(&mut self, addr: u16, value: u8) {
        self.memory.write(addr, value)
    }

    fn update_zero_and_negative_flags(&mut self, value: u8) {
        self.set_zero_flag(value);
        self.set_negative_flag(value);
    }
}
