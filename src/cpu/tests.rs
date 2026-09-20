use super::Cpu;
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
    let mut mem = Memory::new();
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
