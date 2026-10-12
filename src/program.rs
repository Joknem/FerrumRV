use crate::Memory;

pub(crate) fn load_program(mem: &mut Memory, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("programs/{}.hex", name);
    let text = std::fs::read_to_string(path)?;
    for line in text.lines() {
        let item = line
            .split_once('#')
            .map_or(line, |(item, _comment)| item)
            .trim();
        let (addr_text, code) = match item.split_once(':') {
            Some((addr, code)) => (addr.trim(), code.trim()),
            None => {
                return Err(format!("missing ':' in line: {line}").into());
            }
        };
        if code.is_empty() {
            continue;
        }
        let token = match code.split_whitespace().next() {
            Some(value) => value,
            None => continue,
        };
        let hex = token.strip_prefix("0x").unwrap_or(token);
        let instruction = u32::from_str_radix(hex, 16)?;
        let addr_text = u32::from_str_radix(addr_text.strip_prefix("0x").unwrap_or(addr_text), 16)?;
        mem.write_word(addr_text, instruction)
            .map_err(|error| std::io::Error::other(format!("{error:?}")))?;
    }
    Ok(())
}
