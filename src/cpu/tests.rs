use super::Cpu;
use super::Inst;
use super::decode;
use super::{DecodeError, StepError};
use crate::memory::{AddrError, Memory};

#[test]
fn cpu_initial_example() {
    let cpu1 = Cpu::new(0x80000000);
    let cpu2 = Cpu::new(0x00008000);
    assert_eq!(cpu1.pc, 0x80000000);
    assert_eq!(cpu2.pc, 0x00008000);
    assert_eq!(cpu1.regs, [0; 32]);
    assert_eq!(cpu2.regs, [0; 32]);
    assert_eq!(cpu1.halted, false);
    assert_eq!(cpu2.halted, false);
}

#[test]
fn reg_write_example() {
    let mut cpu1 = Cpu::new(0x80000000);
    cpu1.wreg(2, 0xffffffff);
    cpu1.wreg(0, 0x00001234);
    cpu1.wreg(31, 0xffffffff);
    let test_regs = {
        let mut a = [0; 32];
        a[2] = 0xffffffff;
        a[31] = 0xffffffff;
        a
    };
    assert_eq!(cpu1.rreg(2), 0xffffffff);
    assert_eq!(cpu1.rreg(31), 0xffffffff);
    assert_eq!(cpu1.regs, test_regs);
    assert_eq!(cpu1.rreg(0), 0);
    assert_eq!(cpu1.halted, false);
    assert_eq!(cpu1.pc, 0x80000000);
}

#[test]
fn fetch_test() {
    let mut mem = Memory::new(16);
    assert_eq!(mem.write_word(0x80000000, 0x12345678), Ok(()));
    assert_eq!(mem.write_word(0x80000004, 0xaabbccdd), Ok(()));
    let cpu1 = Cpu::new(0x80000000);
    let cpu2 = Cpu::new(0x80000004);
    assert_eq!(cpu1.fetch(&mem), Ok(0x12345678));
    assert_eq!(cpu1.pc, 0x80000000);
    assert_eq!(cpu1.regs, [0; 32]);
    assert_eq!(cpu2.fetch(&mem), Ok(0xaabbccdd));
    assert_eq!(mem.write_word(0x8000000c, 0x87654321), Ok(()));
    let cpu3 = Cpu::new(0x8000000c);
    assert_eq!(cpu3.fetch(&mem), Ok(0x87654321));
    let cpu4 = Cpu::new(0x80000010);
    assert_eq!(
        cpu4.fetch(&mem),
        Err(AddrError::InvalidRange {
            start_addr: 0x80000010,
            length: 4
        })
    );
}

#[test]
fn inst_test() {
    let addi = Inst::Addi {
        rs1: 6,
        imm: -3,
        rd: 5,
    };
    match addi {
        Inst::Addi { rs1, imm, rd } => {
            assert_eq!(rs1, 6);
            assert_eq!(imm, -3);
            assert_eq!(rd, 5);
        }
        _ => todo!(),
    }
    assert_eq!(
        decode(0x00330293),
        Ok(Inst::Addi {
            rs1: 6,
            imm: 3,
            rd: 5
        })
    );
    assert_eq!(
        decode(0x00330280),
        Err(DecodeError::UnsupportedInst { raw: 0x00330280 })
    );
    assert_eq!(
        decode(0x00331293),
        Err(DecodeError::UnsupportedInst { raw: 0x00331293 })
    );
    assert_eq!(
        decode(0x000f8013),
        Ok(Inst::Addi {
            rs1: 31,
            imm: 0,
            rd: 0
        })
    );
    assert_eq!(
        decode(0x7ff00f93),
        Ok(Inst::Addi {
            rs1: 0,
            imm: 2047,
            rd: 31
        })
    );
    println!("{:?}", decode(0x7ff00f93));
    assert_eq!(
        decode(0xffd30293),
        Ok(Inst::Addi {
            rs1: 6,
            imm: -3,
            rd: 5
        })
    );
    assert_eq!(
        decode(0xfff30293),
        Ok(Inst::Addi {
            rs1: 6,
            imm: -1,
            rd: 5
        })
    );
    assert_eq!(
        decode(0x80030293),
        Ok(Inst::Addi {
            rs1: 6,
            imm: -2048,
            rd: 5
        })
    );
}
#[test]
fn execute_test() {
    let mut cpu = Cpu::new(0x80000000);
    cpu.wreg(6, 10);
    let inst = Inst::Addi {
        rs1: 6,
        imm: -3,
        rd: 5,
    };
    cpu.execute(inst);
    assert_eq!(cpu.rreg(5), 7);
    assert_eq!(cpu.rreg(6), 10);
    assert_eq!(cpu.halted, false);
    assert_eq!(cpu.pc, 0x80000004);

    let mut cpu1 = Cpu::new(0x80000000);
    cpu1.wreg(6, 0xffffffff);
    let inst = Inst::Addi {
        rs1: 6,
        imm: 1,
        rd: 6,
    };
    cpu1.execute(inst);
    assert_eq!(cpu1.rreg(6), 0);
    assert_eq!(cpu1.pc, 0x80000004);

    let mut cpu2 = Cpu::new(0x80000000);
    cpu2.wreg(6, 0);
    let inst = Inst::Addi {
        rs1: 6,
        imm: -1,
        rd: 5,
    };
    cpu2.execute(inst);
    assert_eq!(cpu2.rreg(5), 0xffffffff);
    assert_eq!(cpu2.pc, 0x80000004);

    let mut cpu3 = Cpu::new(0xfffffffc);
    cpu3.wreg(6, 10);
    let inst = Inst::Addi {
        rs1: 6,
        imm: 1,
        rd: 0,
    };
    cpu3.execute(inst);
    assert_eq!(cpu3.rreg(0), 0);
    assert_eq!(cpu3.rreg(6), 10);
    assert_eq!(cpu3.pc, 0);
}

#[test]
fn step_test() {
    let mut cpu = Cpu::new(0x80000000);
    let mut mem = Memory::new(16);
    cpu.wreg(6, 10);
    assert_eq!(mem.write_word(0x80000000, 0xffd30293), Ok(()));
    assert_eq!(cpu.step(&mem), Ok(()));
    assert_eq!(cpu.rreg(5), 7);
    assert_eq!(cpu.rreg(6), 10);
    assert_eq!(cpu.pc, 0x80000004);
    assert_eq!(cpu.halted, false);
    let cpu_saved_regs = cpu.regs;

    let mut cpu1 = Cpu::new(0x80000010);
    cpu1.wreg(10, 0x12340000);
    let cpu1_saved_regs = cpu1.regs;
    assert_eq!(
        cpu1.step(&mem),
        Err(StepError::Fetch {
            pc: 0x80000010,
            addr_error: AddrError::InvalidRange {
                start_addr: 0x80000010,
                length: 4
            }
        })
    );
    assert_eq!(cpu1.halted, false);
    assert_eq!(cpu1.pc, 0x80000010);
    assert_eq!(cpu1.regs, cpu1_saved_regs);

    assert_eq!(mem.write_word(0x80000004, 0x11223344), Ok(()));
    assert_eq!(
        cpu.step(&mem),
        Err(StepError::Decode {
            pc: 0x80000004,
            decode_error: DecodeError::UnsupportedInst { raw: 0x11223344 }
        })
    );
    assert_eq!(cpu.regs, cpu_saved_regs);
    assert_eq!(cpu.pc, 0x80000004);
    assert_eq!(cpu.halted, false);
}

#[test]
fn step_3_test() {
    let mut cpu = Cpu::new(0x80000000);
    let mut mem = Memory::new(16);
    assert_eq!(mem.write_word(0x80000000, 0x00a00093), Ok(()));
    assert_eq!(mem.write_word(0x80000004, 0x01400113), Ok(()));
    assert_eq!(mem.write_word(0x80000008, 0xffd08113), Ok(()));
    assert_eq!(cpu.step(&mem), Ok(()));
    assert_eq!(cpu.pc, 0x80000004);
    assert_eq!(cpu.rreg(1), 10);
    assert_eq!(cpu.rreg(2), 0);
    assert_eq!(cpu.step(&mem), Ok(()));
    assert_eq!(cpu.pc, 0x80000008);
    assert_eq!(cpu.rreg(1), 10);
    assert_eq!(cpu.rreg(2), 20);
    assert_eq!(cpu.step(&mem), Ok(()));
    assert_eq!(cpu.pc, 0x8000000c);
    assert_eq!(cpu.rreg(1), 10);
    assert_eq!(cpu.rreg(2), 7);
    assert_eq!(cpu.regs[0], 0);
    assert_eq!(cpu.halted, false);
}

#[test]
fn add_sub_test() {
    let init_pc = 0x80000000;
    let mut cpu = Cpu::new(init_pc);
    let mut mem = Memory::new(256);
    cpu.wreg(6, 10);
    cpu.wreg(7, 20);
    assert_eq!(mem.write_word(init_pc, 0x007302b3), Ok(()));
    assert_eq!(cpu.step(&mem), Ok(()));
    assert_eq!(cpu.rreg(5), 30);

    assert_eq!(mem.write_word(init_pc + 4, 0x407302b3), Ok(()));
    assert_eq!(cpu.step(&mem), Ok(()));
    assert_eq!((cpu.rreg(5)) as i32, -10);

    cpu.wreg(31, 1000);
    cpu.wreg(1, 20);
    assert_eq!(mem.write_word(init_pc + 8, 0x001f8fb3), Ok(()));
    assert_eq!(cpu.step(&mem), Ok(()));
    assert_eq!(cpu.rreg(31), 1020);

    assert_eq!(mem.write_word(init_pc + 12, 0x41f08fb3), Ok(()));
    assert_eq!(cpu.step(&mem), Ok(()));
    assert_eq!(cpu.rreg(31) as i32, -1000);

    assert_eq!(mem.write_word(init_pc + 16, 0x00730033), Ok(()));
    assert_eq!(cpu.step(&mem), Ok(()));
    assert_eq!(cpu.rreg(6), 10);
    assert_eq!(cpu.rreg(7), 20);
    assert_eq!(cpu.rreg(0), 0);
    assert_eq!(cpu.pc, init_pc + 20);
    assert_eq!(cpu.halted, false);

    assert_eq!(mem.write_word(init_pc + 20, 0x40730033), Ok(()));
    assert_eq!(cpu.step(&mem), Ok(()));
    assert_eq!(cpu.rreg(6), 10);
    assert_eq!(cpu.rreg(7), 20);
    assert_eq!(cpu.rreg(0), 0);
    assert_eq!(cpu.pc, init_pc + 24);
    assert_eq!(cpu.halted, false);

    cpu.wreg(6, 0xffffffff);
    cpu.wreg(7, 1);
    assert_eq!(mem.write_word(init_pc + 24, 0x007302b3), Ok(()));
    assert_eq!(cpu.step(&mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0);

    cpu.wreg(6, 0);
    cpu.wreg(7, 1);
    assert_eq!(mem.write_word(init_pc + 28, 0x407302b3), Ok(()));
    assert_eq!(cpu.step(&mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0xffffffff);

    assert_eq!(mem.write_word(init_pc + 32, 0x007312b3), Ok(()));
    assert_eq!(
        cpu.step(&mem),
        Err(StepError::Decode {
            pc: init_pc + 32,
            decode_error: DecodeError::UnsupportedInst { raw: 0x007312b3 }
        })
    );

    assert_eq!(mem.write_word(init_pc + 32, 0x027302b3), Ok(()));
    assert_eq!(
        cpu.step(&mem),
        Err(StepError::Decode {
            pc: init_pc + 32,
            decode_error: DecodeError::UnsupportedInst { raw: 0x027302b3 }
        })
    );

    let mut cpu1 = Cpu::new(init_pc + 24);
    assert_eq!(mem.write_word(init_pc + 24, 0x00a00093), Ok(()));
    assert_eq!(mem.write_word(init_pc + 28, 0x00300113), Ok(()));
    assert_eq!(mem.write_word(init_pc + 32, 0x002081b3), Ok(()));
    assert_eq!(mem.write_word(init_pc + 36, 0x402181b3), Ok(()));
    let expected_x3 = [0, 0, 13, 10];
    for i in 0..4 {
        assert_eq!(cpu1.step(&mem), Ok(()));
        assert_eq!(cpu1.halted, false);
        assert_eq!(cpu1.pc, 0x8000001c + 4 * i);
        assert_eq!(cpu1.rreg(3), expected_x3[i as usize]);
    }
    assert_eq!(cpu1.rreg(1), 10);
    assert_eq!(cpu1.rreg(2), 3);
    assert_eq!(cpu1.rreg(3), 10);
    assert_eq!(cpu1.rreg(0), 0);
}

#[test]
fn ebreak_test() {
    let init_pc = 0x80000000;
    let mut cpu = Cpu::new(init_pc);
    let mut mem = Memory::new(256);
    assert_eq!(mem.write_word(init_pc, 0x00a00093), Ok(()));
    assert_eq!(mem.write_word(init_pc + 4, 0x00300113), Ok(()));
    assert_eq!(mem.write_word(init_pc + 8, 0x002081b3), Ok(()));
    assert_eq!(mem.write_word(init_pc + 12, 0x402181b3), Ok(()));
    assert_eq!(mem.write_word(init_pc + 16, 0x00100073), Ok(()));
    assert_eq!(mem.write_word(init_pc + 20, 0x402181b3), Ok(()));
    for _ in 0..10 {
        assert_eq!(cpu.step(&mem), Ok(()));
        if cpu.halted == true {
            break;
        };
    }
    assert_eq!(mem.write_word(init_pc + 16, 0x402181b3), Ok(()));
    let saved_regs = cpu.regs;
    let saved_pc = cpu.pc;
    let saved_halted = cpu.halted;
    assert_eq!(cpu.step(&mem), Ok(()));
    assert_eq!(saved_halted, cpu.halted);
    assert_eq!(saved_pc, cpu.pc);
    assert_eq!(saved_regs, cpu.regs);

    assert_eq!(cpu.rreg(3), 10);
    assert_eq!(cpu.rreg(2), 3);
    assert_eq!(cpu.rreg(1), 10);
    assert_eq!(cpu.pc, init_pc + 16);
    assert_eq!(cpu.halted, true);

    let mut cpu1 = Cpu::new(init_pc + 40);
    assert_eq!(mem.write_word(init_pc + 40, 0x00000073), Ok(()));
    assert_eq!(
        cpu1.step(&mem),
        Err(StepError::Decode {
            pc: init_pc + 40,
            decode_error: DecodeError::UnsupportedInst { raw: 0x00000073 }
        })
    );
    assert_eq!(mem.write_word(init_pc + 40, 0x001000f3), Ok(()));
    assert_eq!(
        cpu1.step(&mem),
        Err(StepError::Decode {
            pc: init_pc + 40,
            decode_error: DecodeError::UnsupportedInst { raw: 0x001000f3 }
        })
    );
    assert_eq!(mem.write_word(init_pc + 40, 0x00108073), Ok(()));
    assert_eq!(
        cpu1.step(&mem),
        Err(StepError::Decode {
            pc: init_pc + 40,
            decode_error: DecodeError::UnsupportedInst { raw: 0x00108073 }
        })
    );
    assert_eq!(mem.write_word(init_pc + 40, 0x00101073), Ok(()));
    assert_eq!(
        cpu1.step(&mem),
        Err(StepError::Decode {
            pc: init_pc + 40,
            decode_error: DecodeError::UnsupportedInst { raw: 0x00101073 }
        })
    );
}
