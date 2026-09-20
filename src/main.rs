mod cpu;
mod memory;
fn main() {
    // let mut memory = Memory::new();
    // println!("{:?}", memory.write_u16(0x80000003, 0x1234));
    // println!("{:?}", memory.read_u16(0x80000003));
    let invalid_range_error = memory::AddrError::InvalidRange {
        start_addr: 0x8000000f,
        length: 2,
    };
    println!("{:?}", invalid_range_error);
}
