//! C-library numeric parsing shared by the Linux and macOS API stubs.

/// strtol-style parse: skips whitespace, honours sign and base 0/16 prefixes.
/// Returns (value, bytes consumed); consumed is 0 when no digits were found.
pub fn parse_c_integer(s: &[u8], base: u32) -> (i64, usize) {
    let mut i = 0;
    while i < s.len() && s[i].is_ascii_whitespace() {
        i += 1;
    }
    let mut neg = false;
    if i < s.len() && (s[i] == b'-' || s[i] == b'+') {
        neg = s[i] == b'-';
        i += 1;
    }
    let has_hex_prefix = i + 1 < s.len() && s[i] == b'0' && (s[i + 1] == b'x' || s[i + 1] == b'X');
    let base = match base {
        0 if has_hex_prefix => 16,
        0 if i < s.len() && s[i] == b'0' => 8,
        0 => 10,
        b => b,
    };
    if base == 16 && has_hex_prefix {
        i += 2;
    }
    let start = i;
    let mut val: i64 = 0;
    while i < s.len() {
        let Some(d) = (s[i] as char).to_digit(base) else {
            break;
        };
        val = val.wrapping_mul(base as i64).wrapping_add(d as i64);
        i += 1;
    }
    if i == start {
        return (0, 0);
    }
    (if neg { val.wrapping_neg() } else { val }, i)
}

/// strtod-style parse of a decimal float. Returns (value, bytes consumed);
/// consumed is 0 when no number was found.
pub fn parse_c_float(s: &[u8]) -> (f64, usize) {
    let start = s.iter().take_while(|b| b.is_ascii_whitespace()).count();
    let mut end = start;
    let mut best = (0.0, 0);
    while end < s.len() && b"+-.0123456789eE".contains(&s[end]) {
        end += 1;
        if let Ok(v) = std::str::from_utf8(&s[start..end])
            .unwrap_or("")
            .parse::<f64>()
        {
            best = (v, end);
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_c_integer_bases() {
        assert_eq!(parse_c_integer(b"  -42xyz", 10), (-42, 5));
        assert_eq!(parse_c_integer(b"0x1F", 0), (31, 4));
        assert_eq!(parse_c_integer(b"0x1F", 16), (31, 4));
        assert_eq!(parse_c_integer(b"017", 0), (15, 3));
        assert_eq!(parse_c_integer(b"abc", 10), (0, 0));
    }

    #[test]
    fn parse_c_float_prefix() {
        assert_eq!(parse_c_float(b" 3.5abc"), (3.5, 4));
        assert_eq!(parse_c_float(b"-1e3"), (-1000.0, 4));
        assert_eq!(parse_c_float(b"x"), (0.0, 0));
    }
}
