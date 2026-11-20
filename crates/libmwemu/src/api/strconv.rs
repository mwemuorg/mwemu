//! Numeric parsing for emulated C runtimes.

use crate::api::abi::unix::UnixApi;
use crate::api::unix::{api_try, errno_address, memory};
use crate::arch::OperatingSystem;
use crate::emu::Emu;

const EINVAL: u32 = 22;
const ERANGE: u32 = 34;

#[derive(Default)]
pub(crate) struct Integer {
    pub(crate) bits: u64,
    pub(crate) consumed: u64,
    pub(crate) overflow: bool,
}

pub(crate) fn parse_integer<E>(
    read: impl FnMut(u64) -> Result<u8, E>,
    base: u32,
    unsigned: bool,
) -> Result<Integer, E> {
    parse_integer_with_limit(read, base, unsigned, u64::MAX)
}

pub(crate) fn parse_integer_with_limit<E>(
    mut read: impl FnMut(u64) -> Result<u8, E>,
    mut base: u32,
    unsigned: bool,
    unsigned_max: u64,
) -> Result<Integer, E> {
    let mut i = 0;
    let mut byte = read(i)?;
    while byte.is_ascii_whitespace() || byte == 0x0b {
        i += 1;
        byte = read(i)?;
    }
    let negative = byte == b'-';
    if negative || byte == b'+' {
        i += 1;
        byte = read(i)?;
    }
    if byte == b'0'
        && matches!(base, 0 | 16)
        && matches!(read(i + 1)?, b'x' | b'X')
        && read(i + 2)?.is_ascii_hexdigit()
    {
        base = 16;
        i += 2;
        byte = read(i)?;
    }
    if base == 0 {
        base = if byte == b'0' { 8 } else { 10 };
    }
    let limit = if unsigned {
        unsigned_max
    } else {
        (unsigned_max >> 1) + u64::from(negative)
    };
    let mut number = Integer::default();
    while let Some(digit) = char::from(byte).to_digit(base) {
        let value = number
            .bits
            .checked_mul(base.into())
            .and_then(|n| n.checked_add(digit.into()));
        if let Some(value) = value.filter(|n| *n <= limit) {
            number.bits = value;
        } else {
            number.bits = limit;
            number.overflow = true;
        }
        i += 1;
        number.consumed = i;
        byte = read(i)?;
    }
    if negative && (!unsigned || !number.overflow) {
        number.bits = number.bits.wrapping_neg();
    }
    Ok(number)
}

pub(crate) fn integer(emu: &mut Emu, name: &str) {
    let call = UnixApi;
    let source = api_try!(emu, call.arg(emu, 0));
    let simple = matches!(name, "atoi" | "atol" | "atoll");
    let (end, base) = if simple {
        (0, 10)
    } else {
        (
            api_try!(emu, call.arg(emu, 1)),
            api_try!(emu, call.arg_u32(emu, 2)),
        )
    };
    let valid_base = base == 0 || (2..=36).contains(&base);
    let unsigned = matches!(name, "strtoul" | "strtoull" | "strtouq" | "strtoumax");
    let number = if valid_base {
        api_try!(
            emu,
            parse_integer(|index| memory::byte_at(emu, source, index), base, unsigned)
        )
    } else {
        Integer::default()
    };
    let darwin = emu.os == OperatingSystem::MacOS;
    if end != 0 && (valid_base || darwin) {
        api_try!(
            emu,
            memory::write(emu, end, &(source + number.consumed).to_le_bytes())
        );
    }
    let errno = if !valid_base || (darwin && !simple && number.consumed == 0) {
        Some(EINVAL)
    } else if number.overflow {
        Some(ERANGE)
    } else {
        None
    };
    if let Some(error) = errno {
        if let Some(address) = errno_address(emu) {
            api_try!(emu, memory::write(emu, address, &error.to_le_bytes()));
        } else {
            log::error!("{name}: cannot allocate thread-local errno");
        }
    }
    let result = if name == "atoi" {
        number.bits as u32 as u64
    } else {
        number.bits
    };
    log::info!("{} Unix API {name}({source:#x}) => {result:#x}", emu.pos);
    call.set_return(emu, result);
}

/// strtod-style parse of a decimal float. Returns (value, bytes consumed);
/// consumed is 0 when no number was found.
pub fn parse_c_float(s: &[u8]) -> (f64, usize) {
    let start = s
        .iter()
        .take_while(|b| b.is_ascii_whitespace() || **b == 0x0b)
        .count();
    let mut end = start + usize::from(matches!(s.get(start), Some(b'+' | b'-')));
    let digits = |at: usize| s[at..].iter().take_while(|b| b.is_ascii_digit()).count();
    let whole = digits(end);
    end += whole;
    let mut fraction = 0;
    if s.get(end) == Some(&b'.') {
        end += 1;
        fraction = digits(end);
        end += fraction;
    }
    if whole + fraction == 0 {
        return (0.0, 0);
    }
    if matches!(s.get(end), Some(b'e' | b'E')) {
        let exponent = end + 1 + usize::from(matches!(s.get(end + 1), Some(b'+' | b'-')));
        let length = digits(exponent);
        if length != 0 {
            end = exponent + length;
        }
    }
    let value = std::str::from_utf8(&s[start..end])
        .ok()
        .and_then(|text| text.parse().ok());
    value.map_or((0.0, 0), |number| (number, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_c_float_prefix() {
        assert_eq!(parse_c_float(b" 3.5abc"), (3.5, 4));
        assert_eq!(parse_c_float(b"-1e3"), (-1000.0, 4));
        assert_eq!(parse_c_float(b"x"), (0.0, 0));
    }
}
