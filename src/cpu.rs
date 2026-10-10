use crate::{
    cpu::Inst::EBREAK,
    memory::{AddrError, Memory},
};

const FUNCT7_ADD: u32 = 0b000_0000;
const FUNCT7_SUB: u32 = 0b010_0000;
const FUNCT7_AND: u32 = 0b000_0000;
const FUNCT7_OR: u32 = 0b000_0000;
const FUNCT7_XOR: u32 = 0b000_0000;
const FUNCT7_SLT: u32 = 0b000_0000;
const FUNCT7_SLTU: u32 = 0b000_0000;
const FUNCT7_SLL: u32 = 0b000_0000;
const FUNCT7_SRL: u32 = 0b000_0000;
const FUNCT7_SRA: u32 = 0b010_0000;

const FUNCT3_ADD: u32 = 0b000;
const FUNCT3_SUB: u32 = 0b000;
const FUNCT3_AND: u32 = 0b111;
const FUNCT3_OR: u32 = 0b110;
const FUNCT3_XOR: u32 = 0b100;
const FUNCT3_SLT: u32 = 0b010;
const FUNCT3_SLTU: u32 = 0b011;
const FUNCT3_SLL: u32 = 0b001;
const FUNCT3_SRL: u32 = 0b101;
const FUNCT3_SRA: u32 = 0b101;

const FUNCT3_BEQ: u32 = 0b000;
const FUNCT3_BNE: u32 = 0b001;
const FUNCT3_BLT: u32 = 0b100;
const FUNCT3_BGE: u32 = 0b101;
const FUNCT3_BLTU: u32 = 0b110;
const FUNCT3_BGEU: u32 = 0b111;

const FUNCT3_JALR: u32 = 0b000;

const FUNCT3_SW: u32 = 0b010;
const FUNCT3_SH: u32 = 0b001;
const FUNCT3_SB: u32 = 0b000;

const FUNCT3_LW: u32 = 0b010;
const FUNCT3_LB: u32 = 0b000;
const FUNCT3_LH: u32 = 0b001;
const FUNCT3_LBU: u32 = 0b100;
const FUNCT3_LHU: u32 = 0b101;

const FUNCT3_ADDI: u32 = 0b000;
const FUNCT3_ANDI: u32 = 0b111;
const FUNCT3_ORI: u32 = 0b110;
const FUNCT3_XORI: u32 = 0b100;
const FUNCT3_SLTI: u32 = 0b010;
const FUNCT3_SLTIU: u32 = 0b011;
const FUNCT3_SLLI: u32 = 0b001;
const FUNCT3_SRI: u32 = 0b101;

//I-type
const OPCODE_OP_IMM: u32 = 0b001_0011;
const OPCODE_SYSTEM: u32 = 0b111_0011;
const OPCODE_LOAD: u32 = 0b000_0011;

//R-type
const OPCODE_OP: u32 = 0b011_0011;

//B-type
const OPCODE_BR: u32 = 0b110_0011;

//J-type
const OPCODE_JAL: u32 = 0b110_1111;
const OPCODE_JALR: u32 = 0b110_0111;

//S-type
const OPCODE_STORE: u32 = 0b010_0011;

//LUI
const OPCODE_LUI: u32 = 0b011_0111;

//AUIPC
const OPCODE_AUIPC: u32 = 0b001_0111;

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
    fn execute(&mut self, mem: &mut Memory, inst: Inst) -> Result<(), AddrError> {
        match inst {
            Inst::ADDI { rs1, imm, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let cal_value = (imm as u32).wrapping_add(rs1_value);
                self.wreg(rd as usize, cal_value);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::ANDI { rs1, imm, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let cal_value = (imm as u32) & rs1_value;
                self.wreg(rd as usize, cal_value);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::ORI { rs1, imm, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let cal_value = (imm as u32) | rs1_value;
                self.wreg(rd as usize, cal_value);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::XORI { rs1, imm, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let cal_value = (imm as u32) ^ rs1_value;
                self.wreg(rd as usize, cal_value);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::SLTI { rs1, imm, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                if (rs1_value as i32) < imm {
                    self.wreg(rd as usize, 1);
                } else {
                    self.wreg(rd as usize, 0);
                }
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::SLTIU { rs1, imm, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                if rs1_value < imm as u32 {
                    self.wreg(rd as usize, 1);
                } else {
                    self.wreg(rd as usize, 0);
                }
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::SLLI { rs1, imm, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let shift = (imm & 0x0000001f) as u32;
                let cal_value = rs1_value << shift;
                self.wreg(rd as usize, cal_value as u32);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::SRLI { rs1, imm, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let shift = (imm & 0x0000001f) as u32;
                let cal_value = rs1_value >> shift;
                self.wreg(rd as usize, cal_value as u32);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::SRAI { rs1, imm, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let shift = (imm & 0x0000001f) as u32;
                let cal_value = (rs1_value as i32) >> shift;
                self.wreg(rd as usize, cal_value as u32);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::ADD { rs1, rs2, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let rs2_value = self.rreg(rs2 as usize);
                let cal_value = rs1_value.wrapping_add(rs2_value);
                self.wreg(rd as usize, cal_value);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::SUB { rs1, rs2, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let rs2_value = self.rreg(rs2 as usize);
                let cal_value = rs1_value.wrapping_sub(rs2_value);
                self.wreg(rd as usize, cal_value);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::AND { rs1, rs2, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let rs2_value = self.rreg(rs2 as usize);
                let cal_value = rs1_value & rs2_value;
                self.wreg(rd as usize, cal_value);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::OR { rs1, rs2, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let rs2_value = self.rreg(rs2 as usize);
                let cal_value = rs1_value | rs2_value;
                self.wreg(rd as usize, cal_value);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::XOR { rs1, rs2, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let rs2_value = self.rreg(rs2 as usize);
                let cal_value = rs1_value ^ rs2_value;
                self.wreg(rd as usize, cal_value);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::SLL { rs1, rs2, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let rs2_value = self.rreg(rs2 as usize);
                let shift = rs2_value & 0x0000001f;
                let cal_value = rs1_value << shift;
                self.wreg(rd as usize, cal_value);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::SRL { rs1, rs2, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let rs2_value = self.rreg(rs2 as usize);
                let shift = rs2_value & 0x0000001f;
                let cal_value = rs1_value >> shift;
                self.wreg(rd as usize, cal_value);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::SRA { rs1, rs2, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let rs2_value = self.rreg(rs2 as usize);
                let shift = rs2_value & 0x0000001f;
                let cal_value = (rs1_value as i32) >> shift;
                self.wreg(rd as usize, cal_value as u32);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::SLT { rs1, rs2, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let rs2_value = self.rreg(rs2 as usize);
                if (rs1_value as i32) < (rs2_value as i32) {
                    self.wreg(rd as usize, 1);
                } else {
                    self.wreg(rd as usize, 0);
                }
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::SLTU { rs1, rs2, rd } => {
                let rs1_value = self.rreg(rs1 as usize);
                let rs2_value = self.rreg(rs2 as usize);
                if rs1_value < rs2_value {
                    self.wreg(rd as usize, 1);
                } else {
                    self.wreg(rd as usize, 0);
                }
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::BEQ { rs1, rs2, imm } => {
                if self.rreg(rs1 as usize) == self.rreg(rs2 as usize) {
                    self.pc = self.pc.wrapping_add(imm as u32);
                } else {
                    self.pc = self.pc.wrapping_add(4);
                }
                Ok(())
            }
            Inst::BNE { rs1, rs2, imm } => {
                if self.rreg(rs1 as usize) != self.rreg(rs2 as usize) {
                    self.pc = self.pc.wrapping_add(imm as u32);
                } else {
                    self.pc = self.pc.wrapping_add(4);
                }
                Ok(())
            }
            Inst::BLT { rs1, rs2, imm } => {
                if (self.rreg(rs1 as usize) as i32) < (self.rreg(rs2 as usize) as i32) {
                    self.pc = self.pc.wrapping_add(imm as u32);
                } else {
                    self.pc = self.pc.wrapping_add(4);
                }
                Ok(())
            }
            Inst::BGE { rs1, rs2, imm } => {
                if (self.rreg(rs1 as usize) as i32) >= (self.rreg(rs2 as usize) as i32) {
                    self.pc = self.pc.wrapping_add(imm as u32);
                } else {
                    self.pc = self.pc.wrapping_add(4);
                }
                Ok(())
            }
            Inst::BLTU { rs1, rs2, imm } => {
                if self.rreg(rs1 as usize) < self.rreg(rs2 as usize) {
                    self.pc = self.pc.wrapping_add(imm as u32);
                } else {
                    self.pc = self.pc.wrapping_add(4);
                }
                Ok(())
            }
            Inst::BGEU { rs1, rs2, imm } => {
                if self.rreg(rs1 as usize) >= self.rreg(rs2 as usize) {
                    self.pc = self.pc.wrapping_add(imm as u32);
                } else {
                    self.pc = self.pc.wrapping_add(4);
                }
                Ok(())
            }
            Inst::JAL { rd, imm } => {
                self.wreg(rd as usize, self.pc.wrapping_add(4));
                self.pc = self.pc.wrapping_add(imm as u32);
                Ok(())
            }
            Inst::JALR { rs1, rd, imm } => {
                let rs1_value = self.rreg(rs1 as usize);
                self.wreg(rd as usize, self.pc.wrapping_add(4));
                self.pc = (rs1_value.wrapping_add(imm as u32)) & !1;
                Ok(())
            }
            Inst::LW { rs1, rd, imm } => {
                let rs1_value = self.rreg(rs1 as usize);
                let raddr = rs1_value.wrapping_add(imm as u32);
                let rvalue = mem.read_word(raddr)?;
                self.wreg(rd as usize, rvalue);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::LB { rs1, rd, imm } => {
                let rs1_value = self.rreg(rs1 as usize);
                let raddr = rs1_value.wrapping_add(imm as u32);
                let rvalue = match mem.rbyte(raddr) {
                    Ok(value) => value as i8,
                    Err(_) => {
                        return Err(AddrError::InvalidRange {
                            start_addr: raddr,
                            length: 1,
                        });
                    }
                };
                self.wreg(rd as usize, (rvalue as i32) as u32);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::LBU { rs1, rd, imm } => {
                let rs1_value = self.rreg(rs1 as usize);
                let raddr = rs1_value.wrapping_add(imm as u32);
                let rvalue = match mem.rbyte(raddr) {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(AddrError::InvalidRange {
                            start_addr: raddr,
                            length: 1,
                        });
                    }
                };
                self.wreg(rd as usize, rvalue as u32);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::LH { rs1, rd, imm } => {
                let rs1_value = self.rreg(rs1 as usize);
                let raddr = rs1_value.wrapping_add(imm as u32);
                let rvalue = match mem.read_u16(raddr) {
                    Ok(value) => value as i16,
                    Err(_) => {
                        return Err(AddrError::InvalidRange {
                            start_addr: raddr,
                            length: 2,
                        });
                    }
                };
                self.wreg(rd as usize, (rvalue as i32) as u32);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::LHU { rs1, rd, imm } => {
                let rs1_value = self.rreg(rs1 as usize);
                let raddr = rs1_value.wrapping_add(imm as u32);
                let rvalue = match mem.read_u16(raddr) {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(AddrError::InvalidRange {
                            start_addr: raddr,
                            length: 2,
                        });
                    }
                };
                self.wreg(rd as usize, (rvalue as i32) as u32);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::SW { rs1, rs2, imm } => {
                let rs1_value = self.rreg(rs1 as usize);
                let rs2_value = self.rreg(rs2 as usize);
                let waddr = rs1_value.wrapping_add(imm as u32);
                mem.write_word(waddr, rs2_value)?;
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::SH { rs1, rs2, imm } => {
                let rs1_value = self.rreg(rs1 as usize);
                let rs2_value = self.rreg(rs2 as usize);
                let waddr = rs1_value.wrapping_add(imm as u32);
                let wvalue = (rs2_value & 0x0000ffff) as u16;
                mem.write_u16(waddr, wvalue)?;
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::SB { rs1, rs2, imm } => {
                let rs1_value = self.rreg(rs1 as usize);
                let rs2_value = self.rreg(rs2 as usize);
                let waddr = rs1_value.wrapping_add(imm as u32);
                let wvalue = (rs2_value & 0x000000ff) as u8;
                mem.wbyte(waddr, wvalue)?;
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::LUI { rd, imm } => {
                self.wreg(rd as usize, imm as u32);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            Inst::AUIPC { rd, imm } => {
                let cal_value = self.pc.wrapping_add(imm as u32);
                self.wreg(rd as usize, cal_value);
                self.pc = self.pc.wrapping_add(4);
                Ok(())
            }
            EBREAK => {
                self.halted = true;
                Ok(())
            }
        }
    }
    fn step(&mut self, mem: &mut Memory) -> Result<(), StepError> {
        if self.halted {
            Ok(())
        } else {
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
            match self.execute(mem, inst) {
                Ok(()) => (),
                Err(err) => {
                    return Err(StepError::Execute {
                        pc: self.pc,
                        addr_error: err,
                    });
                }
            };
            Ok(())
        }
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
            FUNCT3_ADDI => Ok(Inst::ADDI {
                rs1: rs1,
                imm: imm,
                rd: rd,
            }),
            FUNCT3_ANDI => Ok(Inst::ANDI { rs1, imm, rd }),
            FUNCT3_ORI => Ok(Inst::ORI { rs1, imm, rd }),
            FUNCT3_XORI => Ok(Inst::XORI { rs1, imm, rd }),
            FUNCT3_SLTI => Ok(Inst::SLTI { rs1, imm, rd }),
            FUNCT3_SLTIU => Ok(Inst::SLTIU { rs1, imm, rd }),
            FUNCT3_SLLI => Ok(Inst::SLLI { rs1, imm, rd }),
            FUNCT3_SRI => {
                if imm as u32 & 0x0000fe0 == 0 {
                    Ok(Inst::SRLI { rs1, imm, rd })
                } else if imm as u32 & 0x00000fe0 == 0x00000400 {
                    Ok(Inst::SRAI { rs1, imm, rd })
                } else {
                    Err(DecodeError::UnsupportedInst { raw })
                }
            }
            _ => Err(DecodeError::UnsupportedInst { raw }),
        }
    }
    //Load
    else if opcode == OPCODE_LOAD {
        let rd = ((raw >> 7) & 0x0000001f) as u8;
        let funct3 = (raw >> 12) & 0x00000007;
        let rs1 = ((raw >> 15) & 0x0000001f) as u8;
        let imm = raw as i32 >> 20;
        match funct3 {
            FUNCT3_LW => Ok(Inst::LW { rs1, rd, imm }),
            FUNCT3_LB => Ok(Inst::LB { rs1, rd, imm }),
            FUNCT3_LBU => Ok(Inst::LBU { rs1, rd, imm }),
            FUNCT3_LH => Ok(Inst::LH { rs1, rd, imm }),
            FUNCT3_LHU => Ok(Inst::LHU { rs1, rd, imm }),
            _ => Err(DecodeError::UnsupportedInst { raw }),
        }
    } else if opcode == OPCODE_OP {
        let rd = ((raw >> 7) & 0x0000001f) as u8;
        let funct3 = (raw >> 12) & 0x00000007;
        let rs1 = ((raw >> 15) & 0x0000001f) as u8;
        let rs2 = ((raw >> 20) & 0x0000001f) as u8;
        let funct7 = raw >> 25;
        match [funct7, funct3] {
            [FUNCT7_ADD, FUNCT3_ADD] => Ok(Inst::ADD { rs1, rs2, rd }),
            [FUNCT7_SUB, FUNCT3_SUB] => Ok(Inst::SUB { rs1, rs2, rd }),
            [FUNCT7_AND, FUNCT3_AND] => Ok(Inst::AND { rs1, rs2, rd }),
            [FUNCT7_OR, FUNCT3_OR] => Ok(Inst::OR { rs1, rs2, rd }),
            [FUNCT7_XOR, FUNCT3_XOR] => Ok(Inst::XOR { rs1, rs2, rd }),
            [FUNCT7_SLT, FUNCT3_SLT] => Ok(Inst::SLT { rs1, rs2, rd }),
            [FUNCT7_SLTU, FUNCT3_SLTU] => Ok(Inst::SLTU { rs1, rs2, rd }),
            [FUNCT7_SLL, FUNCT3_SLL] => Ok(Inst::SLL { rs1, rs2, rd }),
            [FUNCT7_SRL, FUNCT3_SRL] => Ok(Inst::SRL { rs1, rs2, rd }),
            [FUNCT7_SRA, FUNCT3_SRA] => Ok(Inst::SRA { rs1, rs2, rd }),
            _ => Err(DecodeError::UnsupportedInst { raw }),
        }
    } else if opcode == OPCODE_STORE {
        let rs1 = ((raw >> 15) & 0x0000001f) as u8;
        let rs2 = ((raw >> 20) & 0x0000001f) as u8;
        let funct3 = (raw >> 12) & 0x00000007;
        let imm = get_stype_imm(raw);
        match funct3 {
            FUNCT3_SW => Ok(Inst::SW { rs1, rs2, imm }),
            FUNCT3_SB => Ok(Inst::SB { rs1, rs2, imm }),
            FUNCT3_SH => Ok(Inst::SH { rs1, rs2, imm }),
            _ => Err(DecodeError::UnsupportedInst { raw }),
        }
    } else if opcode == OPCODE_SYSTEM {
        //ECALL and EBREAK
        match raw >> 7 {
            0x00002000 => Ok(Inst::EBREAK),
            _ => Err(DecodeError::UnsupportedInst { raw }),
        }
    } else if opcode == OPCODE_BR {
        let funct3 = (raw >> 12) & 0x00000007;
        let rs1 = ((raw >> 15) & 0x0000001f) as u8;
        let rs2 = ((raw >> 20) & 0x0000001f) as u8;
        let imm = get_btype_imm(raw);
        match funct3 {
            FUNCT3_BEQ => Ok(Inst::BEQ { rs1, rs2, imm }),
            FUNCT3_BNE => Ok(Inst::BNE { rs1, rs2, imm }),
            FUNCT3_BLT => Ok(Inst::BLT { rs1, rs2, imm }),
            FUNCT3_BGE => Ok(Inst::BGE { rs1, rs2, imm }),
            FUNCT3_BLTU => Ok(Inst::BLTU { rs1, rs2, imm }),
            FUNCT3_BGEU => Ok(Inst::BGEU { rs1, rs2, imm }),
            _ => Err(DecodeError::UnsupportedInst { raw }),
        }
    } else if opcode == OPCODE_JAL {
        let rd = ((raw >> 7) & 0x0000001f) as u8;
        let imm = get_jal_imm(raw);
        Ok(Inst::JAL { rd, imm })
    } else if opcode == OPCODE_JALR {
        let funct3 = (raw >> 12) & 0x00000007;
        match funct3 {
            FUNCT3_JALR => {
                let rs1 = ((raw >> 15) & 0x0000001f) as u8;
                let rd = ((raw >> 7) & 0x0000001f) as u8;
                let imm = raw as i32 >> 20;
                Ok(Inst::JALR { rs1: rs1, rd, imm })
            }
            _ => Err(DecodeError::UnsupportedInst { raw }),
        }
    } else if opcode == OPCODE_LUI {
        let imm = raw & 0xfffff000;
        let rd = ((raw >> 7) & 0x0000001f) as u8;
        Ok(Inst::LUI {
            rd: rd,
            imm: imm as i32,
        })
    } else if opcode == OPCODE_AUIPC {
        let imm = raw & 0xfffff000;
        let rd = ((raw >> 7) & 0x0000001f) as u8;
        Ok(Inst::AUIPC {
            rd: rd,
            imm: imm as i32,
        })
    } else {
        Err(DecodeError::UnsupportedInst { raw })
    }
}

fn get_btype_imm(raw: u32) -> i32 {
    let imm = (((raw & 0x80000000)
        | ((raw & 0x00000080) << 23)
        | ((raw & 0x7e000000) >> 1)
        | ((raw & 0x00000f00) << 12)) as i32)
        >> 19;
    return imm;
}

fn get_jal_imm(raw: u32) -> i32 {
    let imm = (((raw & 0x80000000)
        | ((raw & 0x000ff000) << 11)
        | ((raw & 0x00100000) << 2)
        | ((raw & 0x7fe00000) >> 9)) as i32)
        >> 11;
    return imm;
}

fn get_stype_imm(raw: u32) -> i32 {
    let imm = ((raw & 0xfe000000) | (raw << 13) & 0x01f00000) as i32 >> 20;
    return imm;
}

#[derive(Debug, PartialEq)]
enum Inst {
    ADDI { rs1: u8, imm: i32, rd: u8 },
    ANDI { rs1: u8, imm: i32, rd: u8 },
    ORI { rs1: u8, imm: i32, rd: u8 },
    XORI { rs1: u8, imm: i32, rd: u8 },
    SLTI { rs1: u8, imm: i32, rd: u8 },
    SLTIU { rs1: u8, imm: i32, rd: u8 },
    SLLI { rs1: u8, imm: i32, rd: u8 },
    SRLI { rs1: u8, imm: i32, rd: u8 },
    SRAI { rs1: u8, imm: i32, rd: u8 },

    ADD { rs1: u8, rs2: u8, rd: u8 },
    SUB { rs1: u8, rs2: u8, rd: u8 },
    AND { rs1: u8, rs2: u8, rd: u8 },
    OR { rs1: u8, rs2: u8, rd: u8 },
    XOR { rs1: u8, rs2: u8, rd: u8 },
    SLL { rs1: u8, rs2: u8, rd: u8 },
    SRL { rs1: u8, rs2: u8, rd: u8 },
    SRA { rs1: u8, rs2: u8, rd: u8 },
    SLT { rs1: u8, rs2: u8, rd: u8 },
    SLTU { rs1: u8, rs2: u8, rd: u8 },

    BEQ { rs1: u8, rs2: u8, imm: i32 },
    BNE { rs1: u8, rs2: u8, imm: i32 },
    BLT { rs1: u8, rs2: u8, imm: i32 },
    BGE { rs1: u8, rs2: u8, imm: i32 },
    BLTU { rs1: u8, rs2: u8, imm: i32 },
    BGEU { rs1: u8, rs2: u8, imm: i32 },

    LUI { rd: u8, imm: i32 },
    AUIPC { rd: u8, imm: i32 },

    JAL { rd: u8, imm: i32 },
    JALR { rs1: u8, rd: u8, imm: i32 },

    LW { rs1: u8, rd: u8, imm: i32 },
    LB { rs1: u8, rd: u8, imm: i32 },
    LBU { rs1: u8, rd: u8, imm: i32 },
    LH { rs1: u8, rd: u8, imm: i32 },
    LHU { rs1: u8, rd: u8, imm: i32 },
    SW { rs1: u8, rs2: u8, imm: i32 },
    SH { rs1: u8, rs2: u8, imm: i32 },
    SB { rs1: u8, rs2: u8, imm: i32 },
    EBREAK,
}

#[derive(Debug, PartialEq)]
enum DecodeError {
    UnsupportedInst { raw: u32 },
}

#[derive(Debug, PartialEq)]
enum StepError {
    Fetch { pc: u32, addr_error: AddrError },
    Decode { pc: u32, decode_error: DecodeError },
    Execute { pc: u32, addr_error: AddrError },
}

#[cfg(test)]
mod tests;
