#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Memory {
    data: [u8; 65536], // 64KB of memory
}

impl Default for Memory {
    fn default() -> Self {
        Memory::new()
    } 
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; 65536], // Initialize memory to zero
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let low = self.read(address) as u16;
        let high = self.read(address + 1) as u16;
        (high << 8) | low
    } 

    pub fn write_u16(&mut self, address: u16, value: u16) {
        self.write(address, (value & 0x00FF) as u8);
        self.write(address + 1, (value >> 8) as u8);
    }

    pub fn load_program(&mut self, program: Vec<u8>, start_address: u16) {
        let start = start_address as usize;
        let end = start + program.len();
        self.data[start..end].copy_from_slice(&program);
    }
}