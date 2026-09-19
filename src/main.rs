use std::u32;

#[derive(Debug, PartialEq)]
enum AddrError {
    InvalidRange { start_addr: u32, length: usize },
}
mod cpu {
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
}

struct Memory {
    base_addr: u32,
    bytes: Vec<u8>,
}

impl Memory {
    fn new() -> Self {
        Self {
            base_addr: 0x80000000,
            bytes: vec![0; 16],
        }
    }
    fn rbyte(&self, addr: u32) -> Result<u8, AddrError> {
        if addr < self.base_addr {
            return Err(AddrError::InvalidRange {
                start_addr: addr,
                length: 1,
            });
        }
        let offset = (addr - self.base_addr) as usize;
        let mem_len = self.bytes.len();
        if offset > mem_len - 1 {
            return Err(AddrError::InvalidRange {
                start_addr: addr,
                length: 1,
            });
        }
        Ok(self.bytes[offset])
    }
    fn wbyte(&mut self, waddr: u32, wvalue: u8) -> Result<(), AddrError> {
        if waddr < self.base_addr {
            return Err(AddrError::InvalidRange {
                start_addr: waddr,
                length: 1,
            });
        }
        let offset = (waddr - self.base_addr) as usize;
        let mem_len = self.bytes.len();
        if offset > mem_len - 1 {
            return Err(AddrError::InvalidRange {
                start_addr: waddr,
                length: 1,
            });
        }
        self.bytes[offset] = wvalue;
        Ok(())
    }
    fn read_u16(&self, raddr: u32) -> Result<u16, AddrError> {
        let low_8 = match self.rbyte(raddr) {
            Ok(value) => value as u16,
            Err(_) => {
                return Err(AddrError::InvalidRange {
                    start_addr: raddr,
                    length: 2,
                });
            }
        };
        let high_8 = match self.rbyte(raddr + 1) {
            Ok(value) => value as u16,
            Err(_) => {
                return Err(AddrError::InvalidRange {
                    start_addr: raddr,
                    length: 2,
                });
            }
        };
        let ret = high_8 << 8 | low_8;
        Ok(ret)
    }
    fn write_u16(&mut self, waddr: u32, value: u16) -> Result<(), AddrError> {
        let high_8 = (value >> 8) as u8;
        let low_8 = (value & 0x00ff) as u8;
        let next = match waddr.checked_add(1) {
            Some(value) => value,
            None => {
                return Err(AddrError::InvalidRange {
                    start_addr: waddr,
                    length: 2,
                });
            }
        };
        if next > self.base_addr + (self.bytes.len() as u32) - 1 {
            return Err(AddrError::InvalidRange {
                start_addr: waddr,
                length: 2,
            });
        }
        match self.wbyte(waddr, low_8) {
            Ok(value) => value,
            Err(_) => {
                return Err(AddrError::InvalidRange {
                    start_addr: waddr,
                    length: 2,
                });
            }
        };
        match self.wbyte(next, high_8) {
            Ok(value) => value,
            Err(_) => {
                return Err(AddrError::InvalidRange {
                    start_addr: waddr,
                    length: 2,
                });
            }
        };
        Ok(())
    }
    fn read_word(&self, raddr: u32) -> Result<u32, AddrError> {
        let low_16 = match self.read_u16(raddr) {
            Ok(value) => value as u32,
            Err(_) => {
                return Err(AddrError::InvalidRange {
                    start_addr: raddr,
                    length: 4,
                });
            }
        };
        let read_max_addr = match raddr.checked_add(3) {
            Some(value) => value,
            None => {
                return Err(AddrError::InvalidRange {
                    start_addr: raddr,
                    length: 4,
                });
            }
        };
        if read_max_addr > self.base_addr + (self.bytes.len() as u32) - 1 {
            return Err(AddrError::InvalidRange {
                start_addr: raddr,
                length: 4,
            });
        }
        let high_16 = match self.read_u16(raddr + 2) {
            Ok(value) => value as u32,
            Err(_) => {
                return Err(AddrError::InvalidRange {
                    start_addr: raddr,
                    length: 4,
                });
            }
        };
        let ret = (high_16 << 16) | (low_16 & 0x0000ffff);
        Ok(ret)
    }
    fn write_word(&mut self, waddr: u32, value: u32) -> Result<(), AddrError> {
        let first_byte = (value & 0x00ff) as u8;
        let second_byte = ((value & 0xff00) >> 8) as u8;
        let third_byte = ((value & 0xff0000) >> 16) as u8;
        let fourth_byte = ((value & 0xff000000) >> 24) as u8;
        let next4 = match waddr.checked_add(3) {
            Some(value) => value,
            None => {
                return Err(AddrError::InvalidRange {
                    start_addr: waddr,
                    length: 4,
                });
            }
        };
        if next4 > self.base_addr + (self.bytes.len() as u32) - 1 {
            return Err(AddrError::InvalidRange {
                start_addr: waddr,
                length: 4,
            });
        }
        match self.wbyte(waddr, first_byte) {
            Ok(value) => value,
            Err(_) => {
                return Err(AddrError::InvalidRange {
                    start_addr: waddr,
                    length: 4,
                });
            }
        };
        match self.wbyte(waddr + 1, second_byte) {
            Ok(value) => value,
            Err(_) => {
                return Err(AddrError::InvalidRange {
                    start_addr: waddr,
                    length: 4,
                });
            }
        }
        match self.wbyte(waddr + 2, third_byte) {
            Ok(value) => value,
            Err(_) => {
                return Err(AddrError::InvalidRange {
                    start_addr: waddr,
                    length: 4,
                });
            }
        }
        match self.wbyte(waddr + 3, fourth_byte) {
            Ok(value) => value,
            Err(_) => {
                return Err(AddrError::InvalidRange {
                    start_addr: waddr,
                    length: 4,
                });
            }
        }
        Ok(())
    }
}

#[test]
fn mem_wr_byte_test() {
    let mut memory = Memory::new();
    assert_eq!(
        memory.rbyte(0x7fffffff),
        Err(AddrError::InvalidRange {
            start_addr: 0x7fffffff,
            length: 1
        })
    );
    assert_eq!(
        memory.rbyte(0x80000010),
        Err(AddrError::InvalidRange {
            start_addr: 0x80000010,
            length: 1
        })
    );
    assert_eq!(memory.rbyte(0x80000000), Ok(0));
    assert_eq!(memory.rbyte(0x8000000f), Ok(0));
    assert_eq!(memory.wbyte(0x80000001, 0x34), Ok(()));
    assert_eq!(memory.wbyte(0x80000002, 0x12), Ok(()));
    assert_eq!(memory.rbyte(0x80000001), Ok(0x34));
    let test_vec = {
        let mut a = vec![0; 16];
        a[1] = 0x34;
        a[2] = 0x12;
        a
    };
    assert_eq!(memory.bytes, test_vec);
    assert_eq!(
        memory.wbyte(0x7fffffff, 0x01),
        Err(AddrError::InvalidRange {
            start_addr: 0x7fffffff,
            length: 1
        })
    );
    assert_eq!(memory.bytes, test_vec);
    assert_eq!(
        memory.wbyte(0x80000010, 0x01),
        Err(AddrError::InvalidRange {
            start_addr: 0x80000010,
            length: 1
        })
    );
    assert_eq!(memory.bytes, test_vec);
}

#[test]
fn mem_wr_u16_test() {
    let mut memory = Memory::new();
    assert_eq!(memory.wbyte(0x80000001, 0x34), Ok(()));
    assert_eq!(memory.wbyte(0x80000002, 0x12), Ok(()));
    assert_eq!(memory.read_u16(0x80000001), Ok(0x1234));
    assert_eq!(memory.wbyte(0x8000000e, 0x12), Ok(()));
    assert_eq!(memory.read_u16(0x8000000e), Ok(0x12));
    assert_eq!(memory.wbyte(0x8000000e, 0), Ok(()));
    assert_eq!(memory.read_u16(0x8000000e), Ok(0));
    assert!(memory.read_u16(0x8000000f).is_err());
    assert_eq!(
        memory.read_u16(0x8000000f),
        Err(AddrError::InvalidRange {
            start_addr: 0x8000000f,
            length: 2
        })
    );
    assert!(memory.read_u16(0x7fffffff).is_err());
    assert!(memory.read_u16(u32::MAX).is_err());

    assert_eq!(memory.write_u16(0x80000003, 0x1234), Ok(()));
    assert_eq!(memory.read_u16(0x80000003), Ok(0x1234));
    let test_vec = {
        let mut a = vec![0; 16];
        a[1] = 0x34;
        a[2] = 0x12;
        a[3] = 0x34;
        a[4] = 0x12;
        a
    };
    assert_eq!(
        memory.write_u16(u32::MAX, 0x12),
        Err(AddrError::InvalidRange {
            start_addr: u32::MAX,
            length: 2
        })
    );
    assert_eq!(memory.bytes, test_vec);
    assert_eq!(
        memory.write_u16(0x8000000f, 0x1234),
        Err(AddrError::InvalidRange {
            start_addr: 0x8000000f,
            length: 2
        })
    );
    assert_eq!(memory.bytes, test_vec);
    assert_eq!(memory.rbyte(0x8000000f), Ok(0));
    assert_eq!(
        memory.write_u16(0x7fffffff, 0x5678),
        Err(AddrError::InvalidRange {
            start_addr: 0x7fffffff,
            length: 2
        })
    );
    assert_eq!(memory.bytes, test_vec);
    assert_eq!(memory.write_u16(0x8000000e, 0x1234), Ok(()));
    assert_eq!(memory.read_u16(0x8000000e), Ok(0x1234));
}

#[test]
fn mem_wr_u32_test() {
    let mut memory = Memory::new();
    assert_eq!(memory.wbyte(0x80000000, 0x78), Ok(()));
    assert_eq!(memory.wbyte(0x80000001, 0x56), Ok(()));
    assert_eq!(memory.wbyte(0x80000002, 0x34), Ok(()));
    assert_eq!(memory.wbyte(0x80000003, 0x12), Ok(()));
    assert_eq!(memory.read_word(0x80000000), Ok(0x12345678));
    assert_eq!(memory.read_word(0x8000000c), Ok(0));
    assert!(memory.read_word(0x8000000d).is_err());
    assert!(memory.read_word(0x7fffffff).is_err());
    assert!(memory.read_word(u32::MAX).is_err());
    assert_eq!(memory.write_word(0x8000000c, 0x12345678), Ok(()));
    assert_eq!(memory.read_word(0x8000000c), Ok(0x12345678));
    let test_vec = {
        let mut a = vec![0; 16];
        a[0] = 0x78;
        a[1] = 0x56;
        a[2] = 0x34;
        a[3] = 0x12;
        a[12] = 0x78;
        a[13] = 0x56;
        a[14] = 0x34;
        a[15] = 0x12;
        a
    };
    assert_eq!(
        memory.write_word(0x8000000d, 0x12345678),
        Err(AddrError::InvalidRange {
            start_addr: 0x8000000d,
            length: 4
        })
    );
    assert_eq!(memory.bytes, test_vec);
    assert_eq!(
        memory.write_word(0x7fffffff, 0x12345678),
        Err(AddrError::InvalidRange {
            start_addr: 0x7fffffff,
            length: 4
        })
    );
    assert_eq!(memory.bytes, test_vec);
    assert_eq!(
        memory.write_word(u32::MAX, 0x12345678),
        Err(AddrError::InvalidRange {
            start_addr: u32::MAX,
            length: 4
        })
    );
    assert_eq!(memory.bytes, test_vec);
    assert_eq!(
        memory.read_word(0x8000000d),
        Err(AddrError::InvalidRange {
            start_addr: 0x8000000d,
            length: 4
        })
    );
}

fn main() {
    // let mut memory = Memory::new();
    // println!("{:?}", memory.write_u16(0x80000003, 0x1234));
    // println!("{:?}", memory.read_u16(0x80000003));
    let invalid_range_error = AddrError::InvalidRange {
        start_addr: 0x8000000f,
        length: 2,
    };
    println!("{:?}", invalid_range_error);
}
