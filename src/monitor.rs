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
                    Some("si" | "s") => {
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
                                for _ in 0..num {
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
                                break;
                            }
                        }
                    },
                    Some("info" | "i") => {
                        let arg = parts
                            .next()
                            .map(|text| {
                                let text = text.strip_prefix('x').unwrap_or(text);
                                text.parse::<usize>()
                            })
                            .transpose();
                        match arg {
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
                    Some("x") => {
                        let arg = parts
                            .next()
                            .map(|text| {
                                let text = text.strip_prefix("0x").unwrap_or(text);
                                u32::from_str_radix(text, 16)
                            })
                            .transpose();
                        match arg {
                            Ok(Some(addr)) => {
                                if let Ok(val) = mem.read_word(addr) {
                                    println!("0x{:08x}: 0x{:08x}", addr, val);
                                } else {
                                    println!("invalid addr!{addr}")
                                }
                            }
                            Ok(None) => {
                                println!("You must input addr!");
                            }
                            Err(error) => {
                                println!("invalid memory addr: {}", error);
                            }
                        }
                    }
                    Some("pc") => {
                        println!("0x{:08x}", cpu.get_pc());
                    }
                    Some("quit" | "q") => {
                        println!("Quit monitor!");
                        break 'monitor Ok(());
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

// fn parse_u32(text: &str) -> Result<u32, ParseIntError> {
//     let text = text.strip_prefix("0x").unwrap_or(text);
//     u32::from_str_radix(text, 16)
// }

#[derive(Debug)]
pub(crate) enum MonitorError {
    UnknownCmd,
    Interrupted,
    Eof,
}
