mod cpu;
mod memory;
fn main() {
    let mut sum = 0;
    for i in [3, 7, 9] {
        sum += i;
    }
    println!("{sum}");
}
