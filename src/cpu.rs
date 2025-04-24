use crate::assembler::AddressingMode;
use crate::assembler::INSTRUCTION_LOOKUP;
use crate::assembler::Instruction;
use crate::assembler::OpCode;
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

        let Instruction {
            opname: op, mode, ..
        } = INSTRUCTION_LOOKUP.get(&opcode).unwrap();

        match (op, mode) {
            (OpCode::LDA, _) => self.lda(mode),
            (OpCode::LDX, _) => self.ldx(mode),
            (OpCode::LDY, _) => self.ldy(mode),
            (OpCode::STA, _) => self.sta(mode),
            (OpCode::STX, _) => self.stx(mode),
            (OpCode::STY, _) => self.sty(mode),
            (OpCode::ADC, _) => self.adc(mode),
            (OpCode::INX, _) => self.inx(),
            (OpCode::INY, _) => self.iny(),
            (OpCode::DEX, _) => self.dex(),
            (OpCode::DEY, _) => self.dey(),
            (OpCode::INC, _) => self.inc(mode),
            (OpCode::DEC, _) => self.dec(mode),
            (OpCode::SBC, _) => self.sbc(mode),
            (OpCode::AND, _) => self.and(mode),
            (OpCode::ORA, _) => self.ora(mode),
            (OpCode::EOR, _) => self.eor(mode),
            (OpCode::ASL, mode) if *mode == AddressingMode::Immediate => self.asl_accumulator(),
            (OpCode::ASL, _) => self.asl(mode),
            (OpCode::LSR, mode) if *mode == AddressingMode::Immediate => self.lsr_accumulator(),
            (OpCode::LSR, _) => self.lsr(mode),
            (OpCode::ROL, mode) if *mode == AddressingMode::Immediate => self.rol_accumulator(),
            (OpCode::ROL, _) => self.rol(mode),
            (OpCode::ROR, mode) if *mode == AddressingMode::Immediate => self.ror_accumulator(),
            (OpCode::ROR, _) => self.ror(mode),
            (OpCode::BRA, _) => self.branch(true),
            (OpCode::BCS, _) => self.branch(self.get_carry_flag()),
            (OpCode::BCC, _) => self.branch(!self.get_carry_flag()), // BCC (Branch if Carry Clear)
            (OpCode::BMI, _) => self.branch(self.get_negative_flag()), // BMI (Branch if Minus)
            (OpCode::BPL, _) => self.branch(!self.get_negative_flag()), // BPL (Branch if Plus)
            (OpCode::BVC, _) => self.branch(!self.get_overflow_flag()), // BVC (Branch if Overflow Clear)
            (OpCode::BVS, _) => self.branch(self.get_overflow_flag()), // BVS (Branch if Overflow Set)
            (OpCode::CLC, _) => self.clear_carry(),                    // CLC
            (OpCode::CLD, _) => self.clear_decimal(),                  // CLD
            (OpCode::CLI, _) => self.clear_interrupt(),                // CLI
            (OpCode::CLV, _) => self.clear_overflow(),                 // CLV
            (OpCode::SEC, _) => self.set_carry(),                      // SEC
            (OpCode::SED, _) => self.set_decimal(),                    // SED
            (OpCode::SEI, _) => self.set_interrupt(),                  // SEI
            (OpCode::BRK, _) => self.brk(),
            (OpCode::PLP, _) => self.plp(),
            (OpCode::PLA, _) => self.pla(),
            (OpCode::TXS, _) => self.txs(),
            (OpCode::TYA, _) => self.tya(),
            (OpCode::TSX, _) => self.tsx(),
            (OpCode::TAY, _) => self.tay(),
            (OpCode::TXA, _) => self.txa(),
            (OpCode::TAX, _) => self.tax(),
            (OpCode::RTS, _) => self.rts(),
            (OpCode::PHP, _) => self.php(),
            (OpCode::PHA, _) => self.pha(),
            (OpCode::CMP, _) => self.cmp(),
            (OpCode::BEQ, _) => self.beq(),
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

    fn beq(&mut self) {
        let offset = self.memory.read(self.pc) as i8;
        self.pc += 1;
        if self.get_zero_flag() {
            let jump_addr = ((self.pc as i32) + (offset as i32)) as u16;
            self.pc = jump_addr;
        }
    }

    fn cmp(&mut self) {
        let value = self.memory.read(self.pc);
        self.pc += 1;
        let result = self.a.wrapping_sub(value);
        self.set_carry_flag(self.a >= value);
        self.set_zero_flag(result);
        self.set_negative_flag(result);
    }

    fn tax(&mut self) {
        self.x = self.a;
        self.set_zero_flag(self.x);
        self.set_negative_flag(self.x);
    }
    fn tay(&mut self) {
        self.y = self.a;
        self.set_zero_flag(self.y);
        self.set_negative_flag(self.y);
    }
    fn txa(&mut self) {
        self.a = self.x;
        self.set_zero_flag(self.a);
        self.set_negative_flag(self.a);
    }
    fn tya(&mut self) {
        self.a = self.y;
        self.set_zero_flag(self.a);
        self.set_negative_flag(self.a);
    }
    fn tsx(&mut self) {
        self.x = self.sp;
        self.set_zero_flag(self.x);
        self.set_negative_flag(self.x);
    }

    fn rts(&mut self) {
        self.halted = true;
    }

    fn pla(&mut self) {
        self.a = self.pull();
        self.set_zero_flag(self.a);
        self.set_negative_flag(self.a);
    }

    fn txs(&mut self) {
        self.sp = self.x;
    }

    fn pha(&mut self) {
        self.push(self.a);
    }

    fn php(&mut self) {
        self.push(self.status | 0b0011_0000);
    }

    fn plp(&mut self) {
        self.status = (self.pull() & 0b1110_1111) | 0b0010_0000;
    }

    fn brk(&mut self) {
        self.set_break_flag(true);
        self.halted = true;
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
