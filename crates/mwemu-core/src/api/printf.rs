use crate::emu::Emu;

/// Render a C printf format. `vararg(emu, n)` returns the n-th variadic
/// argument (0-based), so each platform supplies its own calling convention.
pub fn format(emu: &Emu, fmt: &str, vararg: &dyn Fn(&Emu, usize) -> u64) -> String {
    let mut out = String::new();
    let mut vararg_num: usize = 0;
    let chars: Vec<char> = fmt.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] != '%' {
            out.push(chars[i]);
            i += 1;
            continue;
        }
        i += 1;
        if i >= chars.len() {
            break;
        }
        if chars[i] == '%' {
            out.push('%');
            i += 1;
            continue;
        }

        // Parse flags
        let mut left_align = false;
        let mut zero_pad = false;
        let mut plus_sign = false;
        let mut space_sign = false;
        loop {
            if i >= chars.len() {
                break;
            }
            match chars[i] {
                '-' => left_align = true,
                '0' => zero_pad = true,
                '+' => plus_sign = true,
                ' ' => space_sign = true,
                '#' => {}
                _ => break,
            }
            i += 1;
        }

        // Parse width
        let mut width: Option<usize> = None;
        if i < chars.len() && chars[i] == '*' {
            width = Some(vararg(emu, vararg_num) as usize);
            vararg_num += 1;
            i += 1;
        } else {
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            if i > start {
                width = chars[start..i]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .ok();
            }
        }

        // Parse precision
        let mut _precision: Option<usize> = None;
        if i < chars.len() && chars[i] == '.' {
            i += 1;
            if i < chars.len() && chars[i] == '*' {
                _precision = Some(vararg(emu, vararg_num) as usize);
                vararg_num += 1;
                i += 1;
            } else {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                if i > start {
                    _precision = chars[start..i]
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .ok();
                }
            }
        }

        // Parse length modifier
        let mut _long_count = 0;
        while i < chars.len() {
            match chars[i] {
                'l' => {
                    _long_count += 1;
                    i += 1;
                }
                'h' | 'j' | 'z' | 't' | 'q' => {
                    i += 1;
                }
                _ => break,
            }
        }

        if i >= chars.len() {
            break;
        }

        // Conversion
        let w = width.unwrap_or(0);
        let val = vararg(emu, vararg_num);
        vararg_num += 1;
        let formatted = match chars[i] {
            'd' | 'i' => {
                let v = val as i64;
                let s = if plus_sign && v >= 0 {
                    format!("+{}", v)
                } else if space_sign && v >= 0 {
                    format!(" {}", v)
                } else {
                    format!("{}", v)
                };
                pad_string(&s, w, left_align, zero_pad)
            }
            'u' => pad_string(&format!("{}", val), w, left_align, false),
            'x' => pad_string(&format!("{:x}", val), w, left_align, zero_pad),
            'X' => pad_string(&format!("{:X}", val), w, left_align, zero_pad),
            'o' => pad_string(&format!("{:o}", val), w, left_align, zero_pad),
            's' => {
                let s = if val != 0 {
                    emu.maps.read_string(val)
                } else {
                    "(null)".to_string()
                };
                pad_string(&s, w, left_align, false)
            }
            'c' => {
                let ch = if val > 0 && val < 128 {
                    (val as u8) as char
                } else {
                    '?'
                };
                pad_string(&ch.to_string(), w, left_align, false)
            }
            'p' => pad_string(&format!("0x{:x}", val), w, left_align, false),
            _ => {
                format!("%{}", chars[i])
            }
        };
        out.push_str(&formatted);
        i += 1;
    }
    out
}

fn pad_string(s: &str, width: usize, left_align: bool, zero_pad: bool) -> String {
    if width == 0 || s.len() >= width {
        return s.to_string();
    }
    let pad_char = if zero_pad && !left_align { '0' } else { ' ' };
    let padding = width - s.len();
    if left_align {
        format!("{}{}", s, " ".repeat(padding))
    } else {
        format!(
            "{}{}",
            std::iter::repeat_n(pad_char, padding).collect::<String>(),
            s
        )
    }
}
