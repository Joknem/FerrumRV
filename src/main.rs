struct Cpu {
    regs: [u32; 32],
    pc: u32,
    halted: bool,
}

impl Cpu {
    fn new(pc: u32) -> Self {
        Self {
            regs: [0; 32],
            pc,
            halted: false,
        }
    }
    fn rreg(&self, number: usize) -> u32 {
        self.regs[number]
    }
    fn wreg(&mut self, number: usize, value: u32) {
        if number == 0 {
            return;
        }
        self.regs[number] = value
    }
}

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

fn main() {
    let mut cpu = Cpu::new(0x80000000);
    cpu.wreg(14, 0x12345678);
    cpu.wreg(0, 0x12345678);
    println!(
        "pc:0x{:08x}, halted:{}, x0:0x{:08x}",
        cpu.pc,
        cpu.halted,
        cpu.rreg(0)
    );
    println!(
        "reg[13] is 0x{:08x}, reg[14] is 0x{:08x}, reg[0] is 0x{:08x}",
        cpu.rreg(13),
        cpu.rreg(14),
        cpu.rreg(0)
    );
}
