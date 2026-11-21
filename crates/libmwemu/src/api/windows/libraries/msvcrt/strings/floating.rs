use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::api::strconv::parse_c_float;
use crate::emu::Emu;
use crate::exception::types::{ExceptionType, MemoryAccess};
use crate::winapi::common::crt::{Runtime, record_errno};
use crate::winapi::common::memory::{pointer_size, read_array, read_pointer};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::EINVAL;

fn decimal_point(emu: &Emu, runtime: Runtime, locale: u64) -> Result<u8, u64> {
    if locale == 0 {
        return Ok(b'.');
    }
    // _locale_t -> locinfo -> lconv -> decimal_point. The legacy and universal
    // CRT use different threadlocinfo layouts before the lconv pointer.
    const MSVCRT_LCONV32: u64 = 188;
    const MSVCRT_LCONV64: u64 = 296;
    const UCRT_LCONV32: u64 = 136;
    const UCRT_LCONV64: u64 = 248;
    let width = pointer_size(emu);
    let offset = match (runtime, width) {
        (Runtime::Msvcrt, 4) => MSVCRT_LCONV32,
        (Runtime::Msvcrt, _) => MSVCRT_LCONV64,
        (Runtime::Ucrt, 4) => UCRT_LCONV32,
        (Runtime::Ucrt, _) => UCRT_LCONV64,
    };
    let info = read_pointer(emu, locale).ok_or(locale)?;
    let field = info.checked_add(offset).ok_or(info)?;
    let convention = read_pointer(emu, field).ok_or(field)?;
    let decimal = read_pointer(emu, convention).ok_or(convention)?;
    emu.maps.read_byte(decimal).ok_or(decimal)
}

struct Input<'a> {
    emu: &'a Emu,
    address: u64,
    wide: bool,
    decimal: u8,
}

impl Input<'_> {
    fn peek(&self) -> Result<u8, u64> {
        let unit = if self.wide {
            read_array(self.emu, self.address).map(u16::from_le_bytes)
        } else {
            self.emu.maps.read_byte(self.address).map(u16::from)
        }
        .ok_or(self.address)?;
        let byte = u8::try_from(unit).unwrap_or(0);
        Ok(if byte == self.decimal {
            b'.'
        } else if byte == b'.' {
            0
        } else {
            byte.to_ascii_lowercase()
        })
    }

    fn advance(&mut self) -> Result<(), u64> {
        self.address = self
            .address
            .checked_add(if self.wide { 2 } else { 1 })
            .ok_or(self.address)?;
        Ok(())
    }

    fn take(&mut self, allowed: &[u8], out: &mut Vec<u8>) -> Result<bool, u64> {
        let byte = self.peek()?;
        if allowed.contains(&byte) {
            out.push(byte);
            self.advance()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn token(mut self, legacy: bool) -> Result<Vec<u8>, u64> {
        let mut out = Vec::new();
        while matches!(self.peek()?, b' ' | b'\t'..=b'\r') {
            self.advance()?;
        }
        self.take(b"+-", &mut out)?;
        if !legacy {
            let word: &[u8] = match self.peek()? {
                b'i' => b"inf",
                b'n' => b"nan",
                _ => b"",
            };
            if !word.is_empty() {
                for letter in word {
                    if !self.take(std::slice::from_ref(letter), &mut out)? {
                        break;
                    }
                }
                return Ok(out);
            }
        }
        let zero = self.take(b"0", &mut out)?;
        let hex = zero && !legacy && self.take(b"x", &mut out)?;
        let digits: &[u8] = if hex {
            b"0123456789abcdef"
        } else {
            b"0123456789"
        };
        let start = out.len();
        while self.take(digits, &mut out)? {}
        let whole = out.len() > start || (zero && !hex);
        let decimal = self.take(b".", &mut out)?;
        let start = out.len();
        if decimal {
            while self.take(digits, &mut out)? {}
        }
        if whole || out.len() > start {
            let markers: &[u8] = if hex {
                b"p"
            } else if legacy {
                b"de"
            } else {
                b"e"
            };
            if self.take(markers, &mut out)? {
                if !hex && let Some(marker) = out.last_mut() {
                    *marker = b'e';
                }
                self.take(b"+-", &mut out)?;
                while self.take(b"0123456789", &mut out)? {}
            }
        }
        Ok(out)
    }
}

fn hexadecimal(input: &[u8]) -> Option<f64> {
    let mut i = 0;
    let mut mantissa = 0u64;
    let mut exponent = 0i64;
    let mut fraction = false;
    let mut has_digit = false;
    let mut sticky = false;
    while let Some(&byte) = input.get(i) {
        if byte == b'.' && !fraction {
            fraction = true;
        } else if let Some(digit) = char::from(byte).to_digit(16) {
            has_digit = true;
            if fraction {
                exponent -= 4;
            }
            // Keep at least one guard nibble beyond the binary64 significand.
            if mantissa < 1 << f64::MANTISSA_DIGITS {
                mantissa = (mantissa << 4) | u64::from(digit);
            } else {
                sticky |= digit != 0;
                exponent += 4;
            }
        } else {
            break;
        }
        i += 1;
    }
    if !has_digit {
        return None;
    }
    if input.get(i) == Some(&b'p') {
        i += 1;
        let negative = input.get(i) == Some(&b'-');
        i += usize::from(matches!(input.get(i), Some(b'+' | b'-')));
        let mut power = 0i64;
        for &digit in input[i..].iter().take_while(|b| b.is_ascii_digit()) {
            power = power
                .saturating_mul(10)
                .saturating_add(i64::from(digit - b'0'));
        }
        exponent = exponent.saturating_add(if negative { -power } else { power });
    }
    if mantissa == 0 {
        return Some(0.0);
    }
    let top = i64::from(mantissa.ilog2());
    let mut power = exponent.saturating_add(top);
    const MIN_NORMAL: i64 = f64::MIN_EXP as i64 - 1;
    const FRACTION_BITS: i64 = f64::MANTISSA_DIGITS as i64 - 1;
    if power >= f64::MAX_EXP as i64 {
        return Some(f64::INFINITY);
    }
    if power < MIN_NORMAL - FRACTION_BITS - 1 {
        return Some(0.0);
    }
    let shift = power.max(MIN_NORMAL) - FRACTION_BITS - exponent;
    let mut rounded = if shift > 0 {
        let half = 1u64 << (shift - 1);
        let rest = mantissa & ((half << 1) - 1);
        let high = mantissa >> shift;
        high + u64::from(rest > half || (rest == half && (sticky || high & 1 != 0)))
    } else {
        mantissa << -shift
    };
    if rounded == 1 << f64::MANTISSA_DIGITS {
        rounded >>= 1;
        power += 1;
    }
    let base = (power - MIN_NORMAL).max(0) as u64;
    Some(f64::from_bits((base << FRACTION_BITS) + rounded))
}

fn number(bytes: &[u8], runtime: Runtime) -> f64 {
    if matches!(runtime, Runtime::Ucrt) {
        let unsigned = bytes
            .strip_prefix(b"-")
            .or_else(|| bytes.strip_prefix(b"+"))
            .unwrap_or(bytes);
        let special = if unsigned.starts_with(b"inf") {
            Some(f64::INFINITY)
        } else if unsigned.starts_with(b"nan") {
            Some(f64::NAN)
        } else {
            unsigned.strip_prefix(b"0x").and_then(hexadecimal)
        };
        if let Some(value) = special {
            return if bytes.starts_with(b"-") {
                -value
            } else {
                value
            };
        }
    }
    parse_c_float(bytes).0
}

pub(crate) fn dispatch_float(api: &str, emu: &mut Emu, runtime: Runtime) -> bool {
    let (wide, localized) = match api {
        "atof" => (false, false),
        "_atof_l" => (false, true),
        "_wtof" => (true, false),
        "_wtof_l" => (true, true),
        _ => return false,
    };
    log_red!(emu, "{}!{api}", runtime.library());
    atof(emu, runtime, wide, localized);
    true
}

fn atof(emu: &mut Emu, runtime: Runtime, wide: bool, localized: bool) {
    let call = WinApi(Cdecl);
    let source = api_try!(emu, call.arg(emu, 0));
    let locale = if localized {
        api_try!(emu, call.arg(emu, 1))
    } else {
        0
    };
    if source == 0 {
        if record_errno(emu, runtime, EINVAL) {
            call.set_return_f64(emu, 0.0);
        }
        return;
    }
    let result = decimal_point(emu, runtime, locale).and_then(|decimal| {
        Input {
            emu,
            address: source,
            wide,
            decimal,
        }
        .token(matches!(runtime, Runtime::Msvcrt))
    });
    match result {
        Ok(bytes) => call.set_return_f64(emu, number(&bytes, runtime)),
        Err(address) => {
            emu.memory_exception(
                ExceptionType::BadAddressDereferencing,
                address,
                1,
                MemoryAccess::Read,
            );
        }
    }
}
