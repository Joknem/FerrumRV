use crate::memory::{AddrError, Memory};

pub struct Cpu {
    regs: [u32; 32],
    pc: u32,
    halted: bool,
}
impl Cpu {
    pub fn new(pc: u32) -> Self {
        Self {
            regs: [0; 32],
            pc,
            halted: false,
        }
    }
    pub fn rreg(&self, number: usize) -> u32 {
        self.regs[number]
    }
    pub fn wreg(&mut self, number: usize, value: u32) {
        if number == 0 {
            return;
        }
        self.regs[number] = value
    }
    pub fn fetch(&self, memory: &Memory) -> Result<u32, AddrError> {
        let pc = self.pc;
        let inst = memory.read_word(pc);
        return inst;
    }
}
#[cfg(test)]
mod tests;
