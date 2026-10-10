use super::Cpu;
use super::Inst;
use super::decode;
use super::{DecodeError, StepError};
use crate::cpu::get_btype_imm;
use crate::cpu::get_jal_imm;
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
    let addi = Inst::ADDI {
        rs1: 6,
        imm: -3,
        rd: 5,
    };
    match addi {
        Inst::ADDI { rs1, imm, rd } => {
            assert_eq!(rs1, 6);
            assert_eq!(imm, -3);
            assert_eq!(rd, 5);
        }
        _ => todo!(),
    }
    assert_eq!(
        decode(0x00330293),
        Ok(Inst::ADDI {
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
        decode(0x20335293),
        Err(DecodeError::UnsupportedInst { raw: 0x20335293 })
    );
    assert_eq!(
        decode(0x000f8013),
        Ok(Inst::ADDI {
            rs1: 31,
            imm: 0,
            rd: 0
        })
    );
    assert_eq!(
        decode(0x7ff00f93),
        Ok(Inst::ADDI {
            rs1: 0,
            imm: 2047,
            rd: 31
        })
    );
    println!("{:?}", decode(0x7ff00f93));
    assert_eq!(
        decode(0xffd30293),
        Ok(Inst::ADDI {
            rs1: 6,
            imm: -3,
            rd: 5
        })
    );
    assert_eq!(
        decode(0xfff30293),
        Ok(Inst::ADDI {
            rs1: 6,
            imm: -1,
            rd: 5
        })
    );
    assert_eq!(
        decode(0x80030293),
        Ok(Inst::ADDI {
            rs1: 6,
            imm: -2048,
            rd: 5
        })
    );
}
#[test]
fn execute_test() {
    let mut mem = Memory::new(16);
    let mut cpu = Cpu::new(0x80000000);
    cpu.wreg(6, 10);
    let inst = Inst::ADDI {
        rs1: 6,
        imm: -3,
        rd: 5,
    };
    assert_eq!(cpu.execute(&mut mem, inst), Ok(()));
    assert_eq!(cpu.rreg(5), 7);
    assert_eq!(cpu.rreg(6), 10);
    assert_eq!(cpu.halted, false);
    assert_eq!(cpu.pc, 0x80000004);

    let mut cpu1 = Cpu::new(0x80000000);
    cpu1.wreg(6, 0xffffffff);
    let inst = Inst::ADDI {
        rs1: 6,
        imm: 1,
        rd: 6,
    };
    assert_eq!(cpu1.execute(&mut mem, inst), Ok(()));
    assert_eq!(cpu1.rreg(6), 0);
    assert_eq!(cpu1.pc, 0x80000004);

    let mut cpu2 = Cpu::new(0x80000000);
    cpu2.wreg(6, 0);
    let inst = Inst::ADDI {
        rs1: 6,
        imm: -1,
        rd: 5,
    };
    assert_eq!(cpu2.execute(&mut mem, inst), Ok(()));
    assert_eq!(cpu2.rreg(5), 0xffffffff);
    assert_eq!(cpu2.pc, 0x80000004);

    let mut cpu3 = Cpu::new(0xfffffffc);
    cpu3.wreg(6, 10);
    let inst = Inst::ADDI {
        rs1: 6,
        imm: 1,
        rd: 0,
    };
    assert_eq!(cpu3.execute(&mut mem, inst), Ok(()));
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
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 7);
    assert_eq!(cpu.rreg(6), 10);
    assert_eq!(cpu.pc, 0x80000004);
    assert_eq!(cpu.halted, false);
    let cpu_saved_regs = cpu.regs;

    let mut cpu1 = Cpu::new(0x80000010);
    cpu1.wreg(10, 0x12340000);
    let cpu1_saved_regs = cpu1.regs;
    assert_eq!(
        cpu1.step(&mut mem),
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
        cpu.step(&mut mem),
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
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.pc, 0x80000004);
    assert_eq!(cpu.rreg(1), 10);
    assert_eq!(cpu.rreg(2), 0);
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.pc, 0x80000008);
    assert_eq!(cpu.rreg(1), 10);
    assert_eq!(cpu.rreg(2), 20);
    assert_eq!(cpu.step(&mut mem), Ok(()));
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
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 30);

    assert_eq!(mem.write_word(init_pc + 4, 0x407302b3), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!((cpu.rreg(5)) as i32, -10);

    cpu.wreg(31, 1000);
    cpu.wreg(1, 20);
    assert_eq!(mem.write_word(init_pc + 8, 0x001f8fb3), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(31), 1020);

    assert_eq!(mem.write_word(init_pc + 12, 0x41f08fb3), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(31) as i32, -1000);

    assert_eq!(mem.write_word(init_pc + 16, 0x00730033), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(6), 10);
    assert_eq!(cpu.rreg(7), 20);
    assert_eq!(cpu.rreg(0), 0);
    assert_eq!(cpu.pc, init_pc + 20);
    assert_eq!(cpu.halted, false);

    assert_eq!(mem.write_word(init_pc + 20, 0x40730033), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(6), 10);
    assert_eq!(cpu.rreg(7), 20);
    assert_eq!(cpu.rreg(0), 0);
    assert_eq!(cpu.pc, init_pc + 24);
    assert_eq!(cpu.halted, false);

    cpu.wreg(6, 0xffffffff);
    cpu.wreg(7, 1);
    assert_eq!(mem.write_word(init_pc + 24, 0x007302b3), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0);

    cpu.wreg(6, 0);
    cpu.wreg(7, 1);
    assert_eq!(mem.write_word(init_pc + 28, 0x407302b3), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0xffffffff);

    assert_eq!(mem.write_word(init_pc + 32, 0x027312b3), Ok(()));
    assert_eq!(
        cpu.step(&mut mem),
        Err(StepError::Decode {
            pc: init_pc + 32,
            decode_error: DecodeError::UnsupportedInst { raw: 0x027312b3 }
        })
    );

    assert_eq!(mem.write_word(init_pc + 32, 0x027302b3), Ok(()));
    assert_eq!(
        cpu.step(&mut mem),
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
        assert_eq!(cpu1.step(&mut mem), Ok(()));
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
        assert_eq!(cpu.step(&mut mem), Ok(()));
        if cpu.halted == true {
            break;
        };
    }
    assert_eq!(mem.write_word(init_pc + 16, 0x402181b3), Ok(()));
    let saved_regs = cpu.regs;
    let saved_pc = cpu.pc;
    let saved_halted = cpu.halted;
    assert_eq!(cpu.step(&mut mem), Ok(()));
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
        cpu1.step(&mut mem),
        Err(StepError::Decode {
            pc: init_pc + 40,
            decode_error: DecodeError::UnsupportedInst { raw: 0x00000073 }
        })
    );
    assert_eq!(mem.write_word(init_pc + 40, 0x001000f3), Ok(()));
    assert_eq!(
        cpu1.step(&mut mem),
        Err(StepError::Decode {
            pc: init_pc + 40,
            decode_error: DecodeError::UnsupportedInst { raw: 0x001000f3 }
        })
    );
    assert_eq!(mem.write_word(init_pc + 40, 0x00108073), Ok(()));
    assert_eq!(
        cpu1.step(&mut mem),
        Err(StepError::Decode {
            pc: init_pc + 40,
            decode_error: DecodeError::UnsupportedInst { raw: 0x00108073 }
        })
    );
    assert_eq!(mem.write_word(init_pc + 40, 0x00101073), Ok(()));
    assert_eq!(
        cpu1.step(&mut mem),
        Err(StepError::Decode {
            pc: init_pc + 40,
            decode_error: DecodeError::UnsupportedInst { raw: 0x00101073 }
        })
    );
}

#[test]
fn get_imm_test() {
    assert_eq!(get_btype_imm(0x00628863), 16);
    assert_eq!(get_btype_imm(0xFE418CE3), -8);
    assert_eq!(get_jal_imm(0xFC1FF0EF), -64);
    assert_eq!(get_jal_imm(0x010000EF), 16);
}

#[test]
fn beq_test() {
    let init_pc = 0x80000000;
    let mut cpu = Cpu::new(init_pc);
    let mut mem = Memory::new(256);
    assert_eq!(mem.write_word(init_pc, 0x0020a463), Ok(()));
    assert_eq!(
        cpu.step(&mut mem),
        Err(StepError::Decode {
            pc: init_pc,
            decode_error: DecodeError::UnsupportedInst { raw: 0x0020a463 }
        })
    );
    let test_prog: [u32; 13] = [
        0x00500093, 0x00500113, 0x00208463, 0x00100193, 0x00200193, 0x00700213, 0x00408463,
        0x00300293, 0x00400313, 0x00100393, 0x00140413, 0xFE838EE3, 0x00100073,
    ];
    for i in 0..13 {
        assert_eq!(
            mem.write_word(init_pc + 4 * i, test_prog[i as usize]),
            Ok(())
        );
    }
    for _ in 0..100 {
        assert_eq!(cpu.step(&mut mem), Ok(()));
        if cpu.halted {
            break;
        }
    }
    assert_eq!(cpu.halted, true);
    assert_eq!(cpu.rreg(1), 5);
    assert_eq!(cpu.rreg(2), 5);
    assert_eq!(cpu.rreg(3), 2);
    assert_eq!(cpu.rreg(4), 7);
    assert_eq!(cpu.rreg(5), 3);
    assert_eq!(cpu.rreg(6), 4);
    assert_eq!(cpu.rreg(7), 1);
    assert_eq!(cpu.rreg(8), 2);
    assert_eq!(cpu.pc, init_pc + 48);
}

#[test]
fn jal_test() {
    let init_pc = 0x80000000;
    let mut cpu = Cpu::new(init_pc);
    let mut mem = Memory::new(256);
    let test_prog: [u32; 13] = [
        0x00100093, 0x008002EF, 0x00A08093, 0x00108093, 0x00000113, 0x00110113, 0x00110463,
        0xFF9FF36F, 0x00900193, 0x0080006F, 0x06300193, 0x00700213, 0x00100073,
    ];
    for i in 0..13 {
        assert_eq!(
            mem.write_word(init_pc + 4 * i, test_prog[i as usize]),
            Ok(())
        );
    }
    for _ in 0..100 {
        assert_eq!(cpu.step(&mut mem), Ok(()));
        if cpu.halted {
            break;
        }
    }
    assert_eq!(cpu.halted, true);
    assert_eq!(cpu.rreg(1), 2);
    assert_eq!(cpu.rreg(2), 2);
    assert_eq!(cpu.rreg(3), 9);
    assert_eq!(cpu.rreg(4), 7);
    assert_eq!(cpu.rreg(5), 0x80000008);
    assert_eq!(cpu.rreg(6), 0x80000020);
    assert_eq!(cpu.pc, 0x80000030);
}

#[test]
fn jalr_test() {
    let init_pc = 0x80000000;
    let mut cpu = Cpu::new(init_pc);
    let mut mem = Memory::new(256);
    let test_prog: [u32; 15] = [
        0x008002EF, 0x06300093, 0x00D28293, 0x00828367, 0x06300113, 0x05800113, 0x00700193,
        0x00100493, 0x00200513, 0x00C003EF, 0x00148493, 0x00A48663, 0x01038393, 0xFF038467,
        0x00100073,
    ];
    for i in 0..15 {
        assert_eq!(
            mem.write_word(init_pc + 4 * i, test_prog[i as usize]),
            Ok(())
        );
    }
    for _ in 0..100 {
        assert_eq!(cpu.step(&mut mem), Ok(()));
        if cpu.halted {
            break;
        }
    }
    assert_eq!(cpu.halted, true);
    assert_eq!(cpu.pc, 0x80000038);
    assert_eq!(cpu.rreg(3), 0x00000007);
    assert_eq!(cpu.rreg(5), 0x80000011);
    assert_eq!(cpu.rreg(6), 0x80000010);
    assert_eq!(cpu.rreg(7), 0x80000038);
    assert_eq!(cpu.rreg(8), 0x80000038);
    assert_eq!(cpu.rreg(9), 0x00000002);
    assert_eq!(cpu.rreg(10), 0x00000002);

    let mut cpu1 = Cpu::new(0xfffffffc);
    cpu1.wreg(5, 0x80000021);
    assert_eq!(
        cpu1.execute(
            &mut mem,
            Inst::JALR {
                rs1: 5,
                rd: 5,
                imm: 0,
            }
        ),
        Ok(())
    );
    assert_eq!(cpu1.rreg(5), 0);
    assert_eq!(cpu1.pc, 0x80000020);

    let mut cpu2 = Cpu::new(0x80000040);
    cpu2.wreg(1, 0x80000010);
    assert_eq!(mem.write_word(0x80000040, 0x00008067), Ok(()));
    assert_eq!(mem.write_word(0x80000010, 0x00001067), Ok(()));
    assert_eq!(cpu2.step(&mut mem), Ok(()));
    assert_eq!(cpu2.rreg(0), 0);
    assert_eq!(cpu2.rreg(1), 0x80000010);
    assert_eq!(cpu2.halted, false);
    assert_eq!(cpu2.pc, 0x80000010);
    assert_eq!(
        cpu2.step(&mut mem),
        Err(StepError::Decode {
            pc: 0x80000010,
            decode_error: DecodeError::UnsupportedInst { raw: 0x00001067 }
        })
    );
}

#[test]
fn lw_test() {
    let mut cpu = Cpu::new(0x80000000);
    let mut mem = Memory::new(256);
    assert_eq!(mem.write_word(0x80000000, 0x02032283), Ok(()));
    assert_eq!(mem.write_word(0x80000020, 0x12345678), Ok(()));
    cpu.wreg(6, 0x80000000);
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x12345678);
    assert_eq!(cpu.pc, 0x80000004);

    assert_eq!(mem.write_word(0x80000004, 0xffc32303), Ok(()));
    assert_eq!(mem.write_word(0x80000008, 0x00032283), Ok(()));

    cpu.wreg(6, 0x80000024);
    assert_eq!(mem.write_word(0x80000020, 0x89abcdef), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(6), 0x89abcdef);
    assert_eq!(cpu.pc, 0x80000008);

    cpu.wreg(6, 0x80000100);
    cpu.wreg(5, 0x11111111);
    let test_regs = cpu.regs;
    assert_eq!(
        cpu.step(&mut mem),
        Err(StepError::Execute {
            pc: 0x80000008,
            addr_error: AddrError::InvalidRange {
                start_addr: 0x80000100,
                length: 4
            }
        })
    );
    assert_eq!(cpu.halted, false);
    assert_eq!(cpu.regs, test_regs);
    assert_eq!(cpu.pc, 0x80000008);

    assert_eq!(mem.write_word(0x80000040, 0x12349876), Ok(()));
    cpu.wreg(6, 0x80000040);
    assert_eq!(mem.write_word(0x80000008, 0x00032003), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(0), 0);
    assert_eq!(cpu.pc, 0x8000000c);
    cpu.wreg(6, 0x80000100);
    assert_eq!(mem.write_word(0x8000000c, 0x00032003), Ok(()));
    assert_eq!(
        cpu.step(&mut mem),
        Err(StepError::Execute {
            pc: 0x8000000c,
            addr_error: AddrError::InvalidRange {
                start_addr: 0x80000100,
                length: 4
            }
        })
    );

    assert_eq!(mem.write_word(0x8000000c, 0x00037283), Ok(()));
    assert_eq!(
        cpu.step(&mut mem),
        Err(StepError::Decode {
            pc: 0x8000000c,
            decode_error: DecodeError::UnsupportedInst { raw: 0x00037283 }
        })
    );
}

#[test]
fn sw_test() {
    let mut cpu = Cpu::new(0x80000000);
    let mut mem = Memory::new(256);
    cpu.wreg(6, 0x80000040);
    cpu.wreg(5, 0x12345678);
    assert_eq!(mem.write_word(0x80000000, 0x00532023), Ok(()));
    assert_eq!(mem.write_word(0x80000004, 0x00532223), Ok(()));
    assert_eq!(mem.write_word(0x80000008, 0x00532023), Ok(()));

    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(mem.read_word(0x80000040), Ok(0x12345678));
    assert_eq!(cpu.pc, 0x80000004);

    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(mem.read_word(0x80000044), Ok(0x12345678));
    assert_eq!(cpu.pc, 0x80000008);

    cpu.wreg(6, 0x80000100);
    //保存内存快照
    let mut test_mem: Vec<u8> = Vec::new();
    for offset in 0..256u32 {
        let addr = 0x80000000 + offset;
        match mem.rbyte(addr) {
            Ok(value) => test_mem.push(value),
            Err(error) => panic!("snapshot failed:{:?}", error),
        }
    }
    let test_regs = cpu.regs;
    assert_eq!(
        cpu.step(&mut mem),
        Err(StepError::Execute {
            pc: 0x80000008,
            addr_error: AddrError::InvalidRange {
                start_addr: 0x80000100,
                length: 4
            }
        })
    );
    assert_eq!(cpu.pc, 0x80000008);
    assert_eq!(cpu.halted, false);
    assert_eq!(cpu.regs, test_regs);
    //检查越界后内存
    let mut after: Vec<u8> = Vec::new();
    for offset in 0..256u32 {
        let addr = 0x80000000 + offset;
        match mem.rbyte(addr) {
            Ok(value) => after.push(value),
            Err(error) => panic!("snapshot failed:{:?}", error),
        }
    }
    assert_eq!(test_mem, after);

    cpu.wreg(10, 0x33333333);
    cpu.wreg(6, 0x80000040);
    assert_eq!(mem.write_word(0x80000008, 0xfea32e23), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(mem.read_word(0x80000040 - 4), Ok(0x33333333));

    assert_eq!(mem.write_word(0x8000000c, 0x00534023), Ok(()));
    assert_eq!(
        cpu.step(&mut mem),
        Err(StepError::Decode {
            pc: 0x8000000c,
            decode_error: DecodeError::UnsupportedInst { raw: 0x00534023 }
        })
    );
}

#[test]
fn lb_test() {
    let mut cpu = Cpu::new(0x80000000);
    let mut mem = Memory::new(256);
    assert_eq!(mem.write_word(0x80000020, 0x1122337f), Ok(()));
    cpu.wreg(6, 0x80000020);
    assert_eq!(mem.write_word(0x80000000, 0x00030283), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x0000007f);
    assert_eq!(cpu.pc, 0x80000004);
    assert_eq!(cpu.halted, false);

    cpu.wreg(6, 0x80000021);
    assert_eq!(mem.write_word(0x80000020, 0x11223380), Ok(()));
    //lb x5, -1(x6)
    assert_eq!(mem.write_word(0x80000004, 0xfff30283), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0xffffff80);
    assert_eq!(cpu.pc, 0x80000008);
    assert_eq!(cpu.halted, false);

    cpu.wreg(6, 0x80000020);
    assert_eq!(mem.write_word(0x80000020, 0x112233ff), Ok(()));
    assert_eq!(mem.write_word(0x80000008, 0x00030003), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(0), 0);
    assert_eq!(cpu.pc, 0x8000000c);
    assert_eq!(cpu.halted, false);

    assert_eq!(mem.write_word(0x80000020, 0x112233ff), Ok(()));
    assert_eq!(mem.write_word(0x8000000c, 0x00030283), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0xffffffff);
    assert_eq!(cpu.pc, 0x80000010);
    assert_eq!(cpu.halted, false);

    cpu.wreg(6, 0x80000100);
    let test_regs = cpu.regs;
    // lb x5, 0(x6)
    assert_eq!(mem.write_word(0x80000010, 0x00030283), Ok(()));
    assert_eq!(
        cpu.step(&mut mem),
        Err(StepError::Execute {
            pc: 0x80000010,
            addr_error: AddrError::InvalidRange {
                start_addr: 0x80000100,
                length: 1
            }
        })
    );
    assert_eq!(cpu.pc, 0x80000010);
    assert_eq!(cpu.halted, false);
    assert_eq!(cpu.regs, test_regs);
}

#[test]
fn load_test() {
    let start_pc = 0x80000000;
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);
    cpu.wreg(6, start_pc + 40);
    assert_eq!(mem.write_word(start_pc + 40, 0x112280ff), Ok(()));
    //lbu x5, 0(x6)
    assert_eq!(mem.write_word(start_pc, 0x00034283), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x000000ff);
    assert_eq!(cpu.pc, start_pc + 4);
    //lh x5, 0(x6)
    assert_eq!(mem.write_word(start_pc + 4, 0x00031283), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0xffff80ff);
    //lhu x5, 0(x6)
    assert_eq!(mem.write_word(start_pc + 8, 0x00035283), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x000080ff);
    //lbu x5, -1(x6)
    assert_eq!(mem.write_word(0x80000040, 0x112280ff), Ok(()));
    cpu.wreg(6, 0x80000041);
    assert_eq!(mem.write_word(start_pc + 12, 0xfff34283), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x000000ff);
    assert_eq!(cpu.pc, start_pc + 16);
    //lh x5, -2(x6)
    cpu.wreg(6, 0x80000042);
    assert_eq!(mem.write_word(start_pc + 16, 0xffe31283), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0xffff80ff);
    assert_eq!(cpu.pc, start_pc + 20);
    //lhu x5, -2(x6)
    cpu.wreg(6, 0x80000042);
    assert_eq!(mem.write_word(start_pc + 20, 0xffe35283), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x000080ff);
    assert_eq!(cpu.pc, start_pc + 24);

    //lbu x5, 0(x6)
    cpu.wreg(6, 0x80000100);
    assert_eq!(mem.write_word(start_pc + 24, 0x00034283), Ok(()));
    assert_eq!(
        cpu.step(&mut mem),
        Err(StepError::Execute {
            pc: start_pc + 24,
            addr_error: AddrError::InvalidRange {
                start_addr: 0x80000100,
                length: 1
            }
        })
    );
    assert_eq!(cpu.pc, start_pc + 24);
    assert_eq!(cpu.halted, false);

    //lh x5, 0(x6)
    assert_eq!(mem.write_word(start_pc + 24, 0x00031283), Ok(()));
    assert_eq!(
        cpu.step(&mut mem),
        Err(StepError::Execute {
            pc: start_pc + 24,
            addr_error: AddrError::InvalidRange {
                start_addr: 0x80000100,
                length: 2
            }
        })
    );
    assert_eq!(cpu.pc, start_pc + 24);
    assert_eq!(cpu.halted, false);

    //lhu x5, 0(x6)
    assert_eq!(mem.write_word(start_pc + 24, 0x00035283), Ok(()));
    assert_eq!(
        cpu.step(&mut mem),
        Err(StepError::Execute {
            pc: start_pc + 24,
            addr_error: AddrError::InvalidRange {
                start_addr: 0x80000100,
                length: 2
            }
        })
    );
    assert_eq!(cpu.pc, start_pc + 24);
    assert_eq!(cpu.halted, false);
}

#[test]
fn store_test() {
    let start_pc = 0x80000000;
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);
    cpu.wreg(5, 0x12345678);
    cpu.wreg(6, start_pc + 40);
    //sb x5, 0(x6)
    assert_eq!(mem.write_word(start_pc, 0x00530023), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(mem.rbyte(start_pc + 40), Ok(0x78));
    assert_eq!(mem.read_word(start_pc + 40), Ok(0x00000078));

    //sh x5, 0(x6)
    assert_eq!(mem.write_word(start_pc + 4, 0x00531023), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(mem.read_u16(start_pc + 40), Ok(0x5678));
    assert_eq!(mem.read_word(start_pc + 40), Ok(0x00005678));
}

#[test]
fn teacher_load_widths_and_extensions() {
    let start_pc = 0x80000000;
    let data_addr = start_pc + 40;
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);

    // The low halfword is 0x80ff in little-endian memory.
    assert_eq!(mem.write_word(data_addr, 0x112280ff), Ok(()));
    cpu.wreg(6, data_addr);

    // LBU zero-extends one byte.
    assert_eq!(mem.write_word(start_pc, 0x00034283), Ok(())); // lbu x5, 0(x6)
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x000000ff);

    // LH sign-extends two bytes.
    assert_eq!(mem.write_word(start_pc + 4, 0x00031283), Ok(())); // lh x5, 0(x6)
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0xffff80ff);

    // LHU zero-extends the same two bytes.
    assert_eq!(mem.write_word(start_pc + 8, 0x00035283), Ok(())); // lhu x5, 0(x6)
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x000080ff);
}

#[test]
fn teacher_store_widths_and_truncation() {
    let start_pc = 0x80000000;
    let data_addr = start_pc + 40;
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);
    cpu.wreg(5, 0x12345678);
    cpu.wreg(6, data_addr);

    // SB stores only the low byte and leaves the following bytes unchanged.
    assert_eq!(mem.write_word(start_pc, 0x00530023), Ok(())); // sb x5, 0(x6)
    assert_eq!(mem.write_word(data_addr, 0xaabbccdd), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(mem.read_word(data_addr), Ok(0xaabbcc78));

    // SH stores only the low halfword in little-endian order.
    assert_eq!(mem.write_word(start_pc + 4, 0x00531023), Ok(())); // sh x5, 0(x6)
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(mem.read_word(data_addr), Ok(0xaabb5678));
}

#[test]
fn teacher_halfword_load_error_preserves_state() {
    let start_pc = 0x80000000;
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);
    cpu.wreg(5, 0x12345678);
    cpu.wreg(6, 0x80000100);
    let saved_regs = cpu.regs;

    // A halfword load must report length 2 and must not update CPU state.
    assert_eq!(mem.write_word(start_pc, 0x00031283), Ok(())); // lh x5, 0(x6)
    assert_eq!(
        cpu.step(&mut mem),
        Err(StepError::Execute {
            pc: start_pc,
            addr_error: AddrError::InvalidRange {
                start_addr: 0x80000100,
                length: 2,
            },
        })
    );
    assert_eq!(cpu.regs, saved_regs);
    assert_eq!(cpu.pc, start_pc);
    assert!(!cpu.halted);
}

#[test]
fn teacher_halfword_store_error_preserves_memory() {
    let start_pc = 0x80000000;
    let data_addr = start_pc + 40;
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);
    cpu.wreg(5, 0x12345678);
    cpu.wreg(6, 0x80000100);
    let saved_regs = cpu.regs;
    assert_eq!(mem.write_word(data_addr, 0xaabbccdd), Ok(()));

    // A failed SH must report length 2 and leave valid memory untouched.
    assert_eq!(mem.write_word(start_pc, 0x00531023), Ok(())); // sh x5, 0(x6)
    assert_eq!(
        cpu.step(&mut mem),
        Err(StepError::Execute {
            pc: start_pc,
            addr_error: AddrError::InvalidRange {
                start_addr: 0x80000100,
                length: 2,
            },
        })
    );
    assert_eq!(mem.read_word(data_addr), Ok(0xaabbccdd));
    assert_eq!(cpu.regs, saved_regs);
    assert_eq!(cpu.pc, start_pc);
    assert!(!cpu.halted);
}

#[test]
fn integer_rr_test() {
    let start_pc = 0x80000000;
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);
    cpu.wreg(6, 0x11010101);
    cpu.wreg(7, 0x10101010);
    //and x5, x6, x7
    assert_eq!(mem.write_word(start_pc, 0x007372b3), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x10000000);
    //or x5, x6, x7
    assert_eq!(mem.write_word(start_pc + 4, 0x007362b3), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x11111111);
    //xor x5, x6, x7
    assert_eq!(mem.write_word(start_pc + 8, 0x007342b3), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x01111111);
    //slt x5, x6, x7
    assert_eq!(mem.write_word(start_pc + 12, 0x007322b3), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0);
    //slt x5, x6, x7
    cpu.wreg(6, 0x10010101);
    cpu.wreg(7, 0x10101010);
    assert_eq!(mem.write_word(start_pc + 16, 0x007322b3), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 1);
    //slt x5, x6, x7
    cpu.wreg(6, 0xffffffff);
    cpu.wreg(7, 1);
    assert_eq!(mem.write_word(start_pc + 20, 0x007322b3), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 1);
    //sltu x5, x6, x7
    cpu.wreg(6, 0xffffffff);
    cpu.wreg(7, 1);
    assert_eq!(mem.write_word(start_pc + 24, 0x007332b3), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0);
}

#[test]
fn integer_ir_test() {
    let start_pc = 0x80000000;
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);
    // let data_addr = start_pc + 80;
    cpu.wreg(6, 0x000001af);
    //andi x5, x6, 0x0f0
    assert_eq!(mem.write_word(start_pc, 0x0f037293), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x000000a0);
    //ori x5, x6, 0x0f0
    assert_eq!(mem.write_word(start_pc + 4, 0x0f036293), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x000001ff);
    //xori x5, x6, 0x0f0
    assert_eq!(mem.write_word(start_pc + 8, 0x0f034293), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x0000015f);
    //slti x5, x6, -1
    assert_eq!(mem.write_word(start_pc + 12, 0xfff32293), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0);
    //sltiu x5, x6, -1
    assert_eq!(mem.write_word(start_pc + 16, 0xfff33293), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 1);
}

#[test]
fn teacher_logic_and_signed_unsigned_compare() {
    let start_pc = 0x80000000;
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);

    cpu.wreg(6, 0xffffffff);
    cpu.wreg(7, 1);

    // Signed comparison sees -1 as smaller than 1.
    assert_eq!(mem.write_word(start_pc, 0x007322b3), Ok(())); // slt x5, x6, x7
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 1);

    // Unsigned comparison sees 0xffffffff as larger than 1.
    assert_eq!(mem.write_word(start_pc + 4, 0x007332b3), Ok(())); // sltu x5, x6, x7
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0);

    // Logical operations use the complete 32-bit register value.
    cpu.wreg(6, 0xf0f00ff0);
    cpu.wreg(7, 0x0ff0f00f);
    assert_eq!(mem.write_word(start_pc + 8, 0x007372b3), Ok(())); // and x5, x6, x7
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x00f00000);
    assert_eq!(mem.write_word(start_pc + 12, 0x007362b3), Ok(())); // or x5, x6, x7
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0xfff0ffff);
    assert_eq!(mem.write_word(start_pc + 16, 0x007342b3), Ok(())); // xor x5, x6, x7
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0xff00ffff);
}

#[test]
fn teacher_immediate_logic_compare_and_x0() {
    let start_pc = 0x80000000;
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);
    cpu.wreg(6, 0);

    // The logical immediate instructions use the sign-extended immediate.
    assert_eq!(mem.write_word(start_pc, 0x0f037293), Ok(())); // andi x5, x6, 0x0f0
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0);
    assert_eq!(mem.write_word(start_pc + 4, 0x0f036293), Ok(())); // ori x5, x6, 0x0f0
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0xf0);
    assert_eq!(mem.write_word(start_pc + 8, 0x0f034293), Ok(())); // xori x5, x6, 0x0f0
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0xf0);

    // With x6 = 0 and immediate -1, signed and unsigned comparisons differ.
    assert_eq!(mem.write_word(start_pc + 12, 0xfff32293), Ok(())); // slti x5, x6, -1
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0);
    assert_eq!(mem.write_word(start_pc + 16, 0xfff33293), Ok(())); // sltiu x5, x6, -1
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 1);

    // The destination x0 remains zero while the instruction still advances PC.
    assert_eq!(mem.write_word(start_pc + 20, 0x0f037013), Ok(())); // andi x0, x6, 0x0f0
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(0), 0);
    assert_eq!(cpu.pc, start_pc + 24);
}

#[test]
fn shift_test() {
    let start_pc = 0x80000000;
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);
    cpu.wreg(6, 1);
    cpu.wreg(7, 28);
    //sll x5, x6, x7
    assert_eq!(mem.write_word(start_pc, 0x007312b3), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x10000000);

    cpu.wreg(6, 0x80000000);
    cpu.wreg(7, 31);
    //srl x5, x6, x7
    assert_eq!(mem.write_word(start_pc + 4, 0x007352b3), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x00000001);
    //sra x5, x6, x7
    assert_eq!(mem.write_word(start_pc + 8, 0x407352b3), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0xffffffff);
    //slli x5, x6, 7
    cpu.wreg(6, 1);
    assert_eq!(mem.write_word(start_pc + 12, 0x00731293), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x00000080);
    //srli x5, x6, 7
    cpu.wreg(6, 0x00000080);
    assert_eq!(mem.write_word(start_pc + 16, 0x00735293), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x00000001);
    //srai x5, x6, 7
    cpu.wreg(6, 0x80000000);
    assert_eq!(mem.write_word(start_pc + 20, 0x40735293), Ok(()));
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0xff000000);
}

#[test]
fn teacher_shift_signedness_and_amount_mask() {
    let start_pc = 0x80000000;
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);

    // The same negative bit pattern has different logical and arithmetic right shifts.
    cpu.wreg(6, 0x80000000);
    cpu.wreg(7, 1);
    assert_eq!(mem.write_word(start_pc, 0x007352b3), Ok(())); // srl x5, x6, x7
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x40000000);
    assert_eq!(mem.write_word(start_pc + 4, 0x407352b3), Ok(())); // sra x5, x6, x7
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0xc0000000);

    // Register shifts use only the low five bits of rs2.
    cpu.wreg(6, 1);
    cpu.wreg(7, 32);
    assert_eq!(mem.write_word(start_pc + 8, 0x007312b3), Ok(())); // sll x5, x6, x7
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 1);
}

#[test]
fn teacher_immediate_shifts_and_decode_guard() {
    let start_pc = 0x80000000;
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);
    cpu.wreg(6, 0x80000001);

    assert_eq!(mem.write_word(start_pc, 0x00731293), Ok(())); // slli x5, x6, 7
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x00000080);
    assert_eq!(mem.write_word(start_pc + 4, 0x00735293), Ok(())); // srli x5, x6, 7
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x01000000);
    assert_eq!(mem.write_word(start_pc + 8, 0x40735293), Ok(())); // srai x5, x6, 7
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0xff000000);

    // funct3=101 with an unsupported upper immediate is not SRLI or SRAI.
    assert_eq!(
        decode(0x20735293),
        Err(DecodeError::UnsupportedInst { raw: 0x20735293 })
    );
}

#[test]
fn teacher_branch_conditions_and_offsets() {
    let start_pc = 0x80000000;

    // BNE is taken when the operands differ and falls through when equal.
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);
    cpu.wreg(6, 1);
    cpu.wreg(7, 2);
    assert_eq!(mem.write_word(start_pc, 0x00731463), Ok(())); // bne x6, x7, +8
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.pc, start_pc + 8);

    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);
    cpu.wreg(6, 1);
    cpu.wreg(7, 1);
    assert_eq!(mem.write_word(start_pc, 0x00731463), Ok(())); // bne x6, x7, +8
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.pc, start_pc + 4);

    // BLT/BGE use signed comparison: -1 is smaller than 1.
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);
    cpu.wreg(6, 0xffffffff);
    cpu.wreg(7, 1);
    assert_eq!(mem.write_word(start_pc, 0x00734463), Ok(())); // blt x6, x7, +8
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.pc, start_pc + 8);

    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);
    cpu.wreg(6, 0xffffffff);
    cpu.wreg(7, 1);
    assert_eq!(mem.write_word(start_pc, 0x00735463), Ok(())); // bge x6, x7, +8
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.pc, start_pc + 4);

    // BLTU/BGEU use unsigned comparison: 0xffffffff is larger than 1.
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);
    cpu.wreg(6, 0xffffffff);
    cpu.wreg(7, 1);
    assert_eq!(mem.write_word(start_pc, 0x00736463), Ok(())); // bltu x6, x7, +8
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.pc, start_pc + 4);

    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);
    cpu.wreg(6, 0xffffffff);
    cpu.wreg(7, 1);
    assert_eq!(mem.write_word(start_pc, 0x00737463), Ok(())); // bgeu x6, x7, +8
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.pc, start_pc + 8);

    // A negative branch offset returns from start_pc + 8 to start_pc.
    let mut cpu = Cpu::new(start_pc + 8);
    let mut mem = Memory::new(256);
    cpu.wreg(6, 1);
    cpu.wreg(7, 2);
    assert_eq!(mem.write_word(start_pc + 8, 0xfe731ce3), Ok(())); // bne x6, x7, -8
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.pc, start_pc);
}

#[test]
fn teacher_upper_immediates() {
    let start_pc = 0x80000000;
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(256);

    // LUI places the immediate in bits 31:12 and clears the low 12 bits.
    assert_eq!(mem.write_word(start_pc, 0x123452b7), Ok(())); // lui x5, 0x12345
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), 0x12345000);
    assert_eq!(cpu.pc, start_pc + 4);

    // AUIPC adds the shifted immediate to the old PC, then advances PC.
    assert_eq!(mem.write_word(start_pc + 4, 0x12345297), Ok(())); // auipc x5, 0x12345
    assert_eq!(cpu.step(&mut mem), Ok(()));
    assert_eq!(cpu.rreg(5), (start_pc + 4).wrapping_add(0x12345000));
    assert_eq!(cpu.pc, start_pc + 8);
}
