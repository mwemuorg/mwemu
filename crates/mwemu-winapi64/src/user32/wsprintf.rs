use crate::emu;

pub fn wsprintfA(emu: &mut emu::Emu) {
    let buffer = emu.regs().rcx;
    let format_ptr = emu.regs().rdx;
    let format = emu.maps.read_string(format_ptr);

    let regs = [emu.regs().r8, emu.regs().r9];
    let rsp = emu.regs().rsp;
    let mut arg_idx: usize = 0;
    let mut result = String::new();
    let mut chars = format.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '%' {
            result.push(ch);
            continue;
        }
        match chars.peek() {
            Some('%') => {
                chars.next();
                result.push('%');
            }
            Some('s') => {
                chars.next();
                let val = if arg_idx < 2 {
                    regs[arg_idx]
                } else {
                    emu.maps
                        .read_qword(rsp + 0x28 + ((arg_idx - 2) as u64) * 8)
                        .unwrap_or(0)
                };
                arg_idx += 1;
                if val != 0 {
                    result.push_str(&emu.maps.read_string(val));
                }
            }
            Some('d' | 'i') => {
                chars.next();
                let val = if arg_idx < 2 {
                    regs[arg_idx]
                } else {
                    emu.maps
                        .read_qword(rsp + 0x28 + ((arg_idx - 2) as u64) * 8)
                        .unwrap_or(0)
                };
                arg_idx += 1;
                result.push_str(&(val as i64).to_string());
            }
            Some('u') => {
                chars.next();
                let val = if arg_idx < 2 {
                    regs[arg_idx]
                } else {
                    emu.maps
                        .read_qword(rsp + 0x28 + ((arg_idx - 2) as u64) * 8)
                        .unwrap_or(0)
                };
                arg_idx += 1;
                result.push_str(&val.to_string());
            }
            Some('x' | 'X') => {
                chars.next();
                let val = if arg_idx < 2 {
                    regs[arg_idx]
                } else {
                    emu.maps
                        .read_qword(rsp + 0x28 + ((arg_idx - 2) as u64) * 8)
                        .unwrap_or(0)
                };
                arg_idx += 1;
                result.push_str(&format!("{:x}", val));
            }
            Some('c') => {
                chars.next();
                let val = if arg_idx < 2 {
                    regs[arg_idx]
                } else {
                    emu.maps
                        .read_qword(rsp + 0x28 + ((arg_idx - 2) as u64) * 8)
                        .unwrap_or(0)
                };
                arg_idx += 1;
                result.push((val & 0xFF) as u8 as char);
            }
            _ => {
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphabetic() {
                        chars.next();
                        let val = if arg_idx < 2 {
                            regs[arg_idx]
                        } else {
                            emu.maps
                                .read_qword(rsp + 0x28 + ((arg_idx - 2) as u64) * 8)
                                .unwrap_or(0)
                        };
                        arg_idx += 1;
                        result.push_str(&val.to_string());
                        break;
                    }
                    result.push(c);
                    chars.next();
                }
            }
        }
    }

    log_red!(
        emu,
        "user32!wsprintfA buffer=0x{:x} fmt='{}' => '{}'",
        buffer,
        format,
        result
    );

    emu.maps.write_string(buffer, &result);
    emu.regs_mut().rax = result.len() as u64;
}

pub fn wsprintfW(emu: &mut emu::Emu) {
    let buffer = emu.regs().rcx;
    let format_ptr = emu.regs().rdx;
    let format = emu.maps.read_wide_string(format_ptr);

    log_red!(
        emu,
        "user32!wsprintfW buffer=0x{:x} fmt='{}'",
        buffer,
        format
    );

    emu.maps.write_wide_string(buffer, &format);
    emu.regs_mut().rax = format.len() as u64;
}
