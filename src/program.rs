use crate::Memory;

pub(crate) fn load_program(mem: &mut Memory, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut instruction_index = 0;
    let path = format!("programs/{}.hex", name);
    let text = std::fs::read_to_string(path)?;
    for line in text.lines() {
        let item = line
            .split_once('#')
            .map_or(line, |(item, _comment)| item)
            .trim();
        let code = item
            .split_once(':')
            .map_or(item, |(_addr, code)| code)
            .trim();
        if code.is_empty() {
            continue;
        }
        let token = match code.split_whitespace().next() {
            Some(value) => value,
            None => continue,
        };
        let hex = token.strip_prefix("0x").unwrap_or(token);
        let instruction = u32::from_str_radix(hex, 16)?;
        let addr = 0x80000000 + instruction_index * 4;
        mem.write_word(addr, instruction)
            .map_err(|error| std::io::Error::other(format!("{error:?}")))?;
        instruction_index += 1;
    }
    Ok(())
}
