use super::{AddrError, Memory};

#[test]
fn mem_wr_byte_test() {
    let mut memory = Memory::new(16);
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
    let mut memory = Memory::new(16);
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
    let mut memory = Memory::new(16);
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
