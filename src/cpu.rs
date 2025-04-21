use crate::memory::Memory;

pub struct CPU {
    pub a: u8, // Accumulator
    pub x: u8, // X Register
    pub y: u8, // Y Register
    pub pc: u16, // Program Counter
    pub sp: u8, // Stack Pointer
    pub status: u8, // Status Register
    pub memory: Memory, // Memory instance
    pub halted: bool, // Flag to indicate if CPU execution should stop
}

impl CPU {
    pub fn new(memory: Memory) -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0, // 
            pc: 0x0000, // 16-bit program counter
            sp: 0xFF, // 16-bit stack pointer
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
            0xA9 => { // LDA Immediate
                let value = self.memory.read(self.pc);
                self.pc += 1;
                println!("Loading value ${:02X} into accumulator", value);
                self.a = value;
                self.set_zero_flag(self.a);
                self.set_negative_flag(self.a);
            }
            0x69 => { // ADC Immediate
                let value = self.memory.read(self.pc);
                self.pc += 1;
                let carry = self.get_carry_flag() as u16; 
                let sum = self.a as u16 + value as u16 + carry;
                self.set_carry_flag(sum > 0xFF);
                let result = sum as u8;
                // Set overflow flag if sign bit changes unexpectedly
                self.set_overflow_flag(((self.a ^ result) & (value ^ result) & 0x80) != 0);
                self.a = result;
                self.set_zero_flag(self.a);
                self.set_negative_flag(self.a);
            }
            0xC9 => { // CMP Immediate
                let value = self.memory.read(self.pc);
                self.pc += 1;
                let result = self.a.wrapping_sub(value);
                self.set_carry_flag(self.a >= value);
                self.set_zero_flag(result);
                self.set_negative_flag(result);
            }
            0xF0 => { // BEQ Relative
                let offset = self.memory.read(self.pc) as i8;
                self.pc += 1;
                if self.get_zero_flag() {
                    let jump_addr = ((self.pc as i32) + (offset as i32)) as u16;
                    self.pc = jump_addr;
                }
            }
            0x60 => { // RTS
                // For now, just halt since we haven't implemented the stack
                self.halted = true;
            }
            0x00 => { // BRK
                self.set_break_flag(true);
                self.halted = true;
            }
            0xA2 => { // LDX Immediate
                let value = self.memory.read(self.pc);
                self.pc += 1;
                self.ldx(value);
            }
            0xA0 => { // LDY Immediate
                let value = self.memory.read(self.pc);
                self.pc += 1;
                self.ldy(value);
            }
            0x86 => { // STX ZeroPage
                let addr = self.memory.read(self.pc) as u16;
                self.pc += 1;
                self.stx(addr);
            }
            0x84 => { // STY ZeroPage
                let addr = self.memory.read(self.pc) as u16;
                self.pc += 1;
                self.sty(addr);
            }
            0x85 => { // STA Zero Page
                let addr = self.memory.read(self.pc) as u16;
                self.pc += 1;
                self.sta(addr);
            }
            0xA5 => { // LDA Zero Page
                let addr = self.memory.read(self.pc) as u16;
                self.pc += 1;
                let value = self.memory.read(addr);
                self.lda(value);
            }
            0xA4 => { // LDY Zero Page
                let addr = self.memory.read(self.pc) as u16;
                self.pc += 1;
                let value = self.memory.read(addr);
                self.ldy(value);
            }
            0xA6 => { // LDX Zero Page
                let addr = self.memory.read(self.pc) as u16;
                self.pc += 1;
                let value = self.memory.read(addr);
                self.ldx(value);
            }
            0xAA => { // TAX
                self.x = self.a;
                self.set_zero_flag(self.x);
                self.set_negative_flag(self.x);
            }
            0xA8 => { // TAY
                self.y = self.a;
                self.set_zero_flag(self.y);
                self.set_negative_flag(self.y);
            }
            0x8A => { // TXA
                self.a = self.x;
                self.set_zero_flag(self.a);
                self.set_negative_flag(self.a);
            }
            0x98 => { // TYA
                self.a = self.y;
                self.set_zero_flag(self.a);
                self.set_negative_flag(self.a);
            }
            _ => {
                println!("Opcode {:02X} at address {:04X} not implemented", opcode, self.pc - 1);
                self.halted = true;
            }
        }
    }
}


impl CPU {
    pub fn lda(&mut self, value: u8) {
        self.a = value;
        self.set_zero_flag(self.a);
        self.set_negative_flag(self.a);
    }

    pub fn ldx(&mut self, value: u8) {
        self.x = value;
        self.set_zero_flag(self.x);
        self.set_negative_flag(self.x);
    }

    pub fn ldy(&mut self, value: u8) {
        self.y = value;
        self.set_zero_flag(self.y);
        self.set_negative_flag(self.y);
    }

    pub fn sta(&mut self, address: u16) {
        self.memory.write(address, self.a);
    }

    pub fn stx(&mut self, address: u16) {
        self.memory.write(address, self.x);
    }

    pub fn sty(&mut self, address: u16) {
        self.memory.write(address, self.y);
    }

    pub fn get_status(&self) -> u8 {
        self.status
    }

    pub fn set_status(&mut self, value: u8) {
        self.status = value;
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
}