#[derive(Debug, PartialEq)]
pub(crate) enum AddrError {
    InvalidRange { start_addr: u32, length: usize },
}

pub(crate) struct Memory {
    base_addr: u32,
    bytes: Vec<u8>,
}

impl Memory {
    pub(crate) fn new() -> Self {
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
    pub(crate) fn read_word(&self, raddr: u32) -> Result<u32, AddrError> {
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
    pub(crate) fn write_word(&mut self, waddr: u32, value: u32) -> Result<(), AddrError> {
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

#[cfg(test)]
mod tests;
