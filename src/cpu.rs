use crate::memory::{AddrError, Memory};

const FUNCT3_ADDI: u32 = 0b000;
const FUNCT3_ADD: u32 = 0b000;
const FUNCT3_SUB: u32 = 0b000;

//I-type
const OPCODE_OP_IMM: u32 = 0b001_0011;

//R-type
const OPCODE_OP: u32 = 0b011_0011;
const FUNCT7_ADD: u32 = 0b000_0000;
const FUNCT7_SUB: u32 = 0b010_0000;

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
    fn execute(&mut self, inst: Inst) -> () {
        match inst {
            Inst::Addi { rs1, imm, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let cal_value = (imm as u32).wrapping_add(rs1_value);
                self.wreg(rd as usize, cal_value);
                self.pc = self.pc.wrapping_add(4);
            }
            Inst::Add { rs1, rs2, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let rs2_value = self.rreg(rs2 as usize);
                let cal_value = rs1_value.wrapping_add(rs2_value);
                self.wreg(rd as usize, cal_value);
                self.pc = self.pc.wrapping_add(4);
            }
            Inst::SUB { rs1, rs2, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let rs2_value = self.rreg(rs2 as usize);
                let cal_value = rs1_value.wrapping_sub(rs2_value);
                self.wreg(rd as usize, cal_value);
                self.pc = self.pc.wrapping_add(4);
            }
        }
    }
    fn step(&mut self, mem: &Memory) -> Result<(), StepError> {
        let fetched_raw = match self.fetch(mem) {
            Ok(value) => value,
            Err(err) => {
                return Err(StepError::Fetch {
                    pc: self.pc,
                    addr_error: err,
                });
            }
        };
        let inst = match decode(fetched_raw) {
            Ok(value) => value,
            Err(err) => {
                return Err(StepError::Decode {
                    pc: self.pc,
                    decode_error: err,
                });
            }
        };
        self.execute(inst);
        Ok(())
    }
}

fn decode(raw: u32) -> Result<Inst, DecodeError> {
    let opcode = raw & 0x0000007f;
    //I-type
    if opcode == OPCODE_OP_IMM {
        let rd = ((raw >> 7) & 0x0000001f) as u8;
        let funct3 = (raw >> 12) & 0x00000007;
        let rs1 = ((raw >> 15) & 0x0000001f) as u8;
        let imm = raw as i32 >> 20;
        match funct3 {
            FUNCT3_ADDI => Ok(Inst::Addi {
                rs1: rs1,
                imm: imm,
                rd: rd,
            }),
            _ => Err(DecodeError::UnsupportedInst { raw }),
        }
    } else if opcode == OPCODE_OP {
        let rd = ((raw >> 7) & 0x0000001f) as u8;
        let funct3 = (raw >> 12) & 0x00000007;
        let rs1 = ((raw >> 15) & 0x0000001f) as u8;
        let rs2 = ((raw >> 20) & 0x0000001f) as u8;
        let funct7 = raw >> 25;
        match [funct7, funct3] {
            [FUNCT7_ADD, FUNCT3_ADD] => Ok(Inst::Add { rs1, rs2, rd }),
            [FUNCT7_SUB, FUNCT3_SUB] => Ok(Inst::SUB { rs1, rs2, rd }),
            _ => Err(DecodeError::UnsupportedInst { raw }),
        }
    } else {
        Err(DecodeError::UnsupportedInst { raw })
    }
}

#[derive(Debug, PartialEq)]
enum Inst {
    Addi { rs1: u8, imm: i32, rd: u8 },
    Add { rs1: u8, rs2: u8, rd: u8 },
    SUB { rs1: u8, rs2: u8, rd: u8 },
}

#[derive(Debug, PartialEq)]
enum DecodeError {
    UnsupportedInst { raw: u32 },
}

#[derive(Debug, PartialEq)]
enum StepError {
    Fetch { pc: u32, addr_error: AddrError },
    Decode { pc: u32, decode_error: DecodeError },
}

#[cfg(test)]
mod tests;
