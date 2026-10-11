use crate::{cpu::Cpu, memory::Memory, monitor::start_monitor, program::load_program};

mod cpu;
mod memory;
mod monitor;
mod program;

fn main() {
    let start_pc = 0x80000000;
    let mut cpu = Cpu::new(start_pc);
    let mut mem = Memory::new(4096);
    if let Err(error) = load_program(&mut mem, "00_add_sub") {
        eprintln!("Load program failed: {}", error);
        return;
    };
    if let Err(error) = start_monitor(&mut cpu, &mut mem) {
        eprintln!("monitor stopped: {:?}", error);
    };
}
