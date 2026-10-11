use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use crate::cpu::Cpu;
use crate::memory::Memory;

pub(crate) fn start_monitor(cpu: &mut Cpu, mem: &mut Memory) -> Result<(), MonitorError> {
    let mut rl = match DefaultEditor::new() {
        Ok(value) => value,
        Err(_) => return Err(MonitorError::UnknownCmd),
    };
    'monitor: loop {
        match rl.readline("Ferrum Monitor> ") {
            Ok(line) => {
                let mut parts = line.split_whitespace();
                let command = parts.next();
                match command {
                    Some("step" | "s") => {
                        if cpu.get_halted() {
                            println!("Program ends!");
                            break Ok(());
                        }
                        let result = parts.next().map(|text| text.parse::<u32>()).transpose();
                        match result {
                            Ok(None) => {
                                if let Err(error) = cpu.step(mem) {
                                    println!("step error:{:?}", error);
                                } else {
                                    println!("step ok");
                                }
                            }
                            Ok(Some(num)) => {
                                for _ in 1..num {
                                    if let Err(error) = cpu.step(mem) {
                                        println!("step error:{:?}", error);
                                    } else {
                                        println!("step ok");
                                    }
                                }
                            }
                            Err(error) => {
                                println!("invalid number of steps: {:?}", error);
                            }
                        }
                        match cpu.step(mem) {
                            Ok(()) => {
                                println!("step ok");
                            }
                            Err(error) => {
                                println!("step error:{:?}", error);
                            }
                        }
                    }
                    Some("continue" | "c") => loop {
                        if cpu.get_halted() {
                            println!("Program ends!");
                            break 'monitor Ok(());
                        }
                        match cpu.step(mem) {
                            Ok(()) => {
                                println!("step ok");
                            }
                            Err(error) => {
                                println!("step error:{:?}", error);
                            }
                        }
                    },
                    Some("regs" | "r") => {
                        let result = parts
                            .next()
                            .map(|text| {
                                let text = text.strip_prefix('x').unwrap_or(text);
                                text.parse::<usize>()
                            })
                            .transpose();
                        match result {
                            Ok(None) => {
                                for i in 0..32usize {
                                    println!("x{}:0x{:08x}", i, cpu.rreg(i));
                                }
                            }
                            Ok(Some(index)) if index < 32 => {
                                println!("x{} = {:#010x}", index, cpu.rreg(index));
                            }
                            Ok(Some(_)) => {
                                println!("register index must be between 0 and 31");
                            }
                            Err(error) => {
                                println!("invalid register index: {}", error);
                            }
                        }
                    }
                    Some("mem") => {}
                    Some("pc") => {
                        println!("0x{:08x}", cpu.get_pc());
                    }
                    _ => println!("Unknown command!"),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl+C!");
                break Err(MonitorError::Interrupted);
            }
            Err(ReadlineError::Eof) => {
                println!("Ctrl+D!");
                break Err(MonitorError::Eof);
            }
            _ => {
                println!("ReadLine Unknown Error!");
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum MonitorError {
    UnknownCmd,
    Interrupted,
    Eof,
}
