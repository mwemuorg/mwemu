use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::automation::{self, Decimal, Number};
use crate::winapi::common::{bstr, memory, strings};
use crate::winapi::policy::{api_try, unsupported};
use crate::windows::constants::errors::{
    DISP_E_OVERFLOW, DISP_E_TYPEMISMATCH, E_OUTOFMEMORY, E_POINTER, S_OK,
};
use crate::windows::constants::nls::{
    LOCALE_INVARIANT, LOCALE_SYSTEM_DEFAULT, LOCALE_USER_DEFAULT,
};

use super::fixed_point::write_fixed;

const EN_US: u64 = 0x0409;

fn english_number_format(locale: u64, flags: u64) -> bool {
    matches!(
        locale as u32 as u64,
        0 | EN_US | LOCALE_INVARIANT | LOCALE_USER_DEFAULT | LOCALE_SYSTEM_DEFAULT
    ) && flags as u32 == 0
}

fn parse_fixed(text: &str) -> Result<Decimal, u64> {
    let text = text.trim();
    let negative = text.starts_with('-');
    let text = text.strip_prefix(['+', '-']).unwrap_or(text);
    let (whole, fraction) = text.split_once('.').unwrap_or((text, ""));
    if (whole.is_empty() && fraction.is_empty())
        || !whole
            .bytes()
            .chain(fraction.bytes())
            .all(|c| c.is_ascii_digit())
    {
        return Err(DISP_E_TYPEMISMATCH);
    }
    let fraction = fraction.trim_end_matches('0');
    let magnitude = whole
        .bytes()
        .chain(fraction.bytes())
        .try_fold(0u128, |acc, digit| {
            acc.checked_mul(10)?.checked_add(u128::from(digit - b'0'))
        })
        .ok_or(DISP_E_OVERFLOW)?;
    if fraction.len() > 28 || magnitude >= (1u128 << 96) {
        return Err(DISP_E_OVERFLOW);
    }
    Ok(Decimal {
        magnitude,
        scale: fraction.len() as u8,
        negative,
    })
}

fn from_string(emu: &mut Emu, api: &str, kind: Option<Number>) {
    let call = WinApi(Stdcall);
    let [input, locale, flags, output] = api_try!(emu, call.args(emu));
    if !english_number_format(locale, flags) {
        unsupported(emu, "oleaut32", api, emu.pc());
        return;
    }
    let text = strings::read_text_checked(emu, input, true);
    log_red!(
        emu,
        "oleaut32!{api} text={text:?} locale={locale:#x} output={output:#x}"
    );
    let result = text.as_deref().ok_or(DISP_E_TYPEMISMATCH).and_then(|s| {
        let s = s.trim();
        if matches!(kind, Some(Number::Bool))
            && (s.eq_ignore_ascii_case("True") || s.eq_ignore_ascii_case("False"))
        {
            let value = if s.eq_ignore_ascii_case("True") {
                automation::VARIANT_TRUE
            } else {
                automation::VARIANT_FALSE
            };
            return automation::output(emu, output, &value.to_le_bytes());
        }
        if let Some(number @ (Number::Single | Number::Double)) = kind {
            let value = s.parse::<f64>().map_err(|_| DISP_E_TYPEMISMATCH)?;
            if !value.is_finite() {
                return Err(if s.bytes().any(|c| c.is_ascii_digit()) {
                    DISP_E_OVERFLOW
                } else {
                    DISP_E_TYPEMISMATCH
                });
            }
            let bytes = automation::numeric_bytes(value, number)?;
            return automation::output(emu, output, &bytes);
        }
        write_fixed(emu, output, parse_fixed(s)?, kind)
    });
    call.set_return(emu, result.err().unwrap_or(S_OK));
    call.cleanup_args(emu, 4);
}

macro_rules! from_strings {
    ($($name:ident => $kind:expr),+ $(,)?) => {
        $(pub fn $name(emu: &mut Emu) { from_string(emu, stringify!($name), $kind); })+
    };
}
from_strings!(
    VarBoolFromStr => Some(Number::Bool), VarI1FromStr => Some(Number::Signed(i8::BITS)),
    VarUI1FromStr => Some(Number::Unsigned(u8::BITS)), VarI2FromStr => Some(Number::Signed(i16::BITS)),
    VarUI2FromStr => Some(Number::Unsigned(u16::BITS)), VarI4FromStr => Some(Number::Signed(i32::BITS)),
    VarUI4FromStr => Some(Number::Unsigned(u32::BITS)), VarI8FromStr => Some(Number::Signed(i64::BITS)),
    VarUI8FromStr => Some(Number::Unsigned(u64::BITS)), VarR4FromStr => Some(Number::Single),
    VarR8FromStr => Some(Number::Double), VarDecFromStr => None,
);

fn format_fixed(value: Decimal) -> String {
    let digits = value.magnitude.to_string();
    let scale = usize::from(value.scale);
    let mut text = if value.negative && value.magnitude != 0 {
        "-".to_owned()
    } else {
        String::new()
    };
    if scale == 0 {
        text.push_str(&digits);
    } else {
        if digits.len() > scale {
            let (whole, fraction) = digits.split_at(digits.len() - scale);
            text.push_str(whole);
            text.push('.');
            text.push_str(fraction);
        } else {
            text.push_str("0.");
            text.extend(std::iter::repeat_n('0', scale - digits.len()));
            text.push_str(&digits);
        }
        text.truncate(text.trim_end_matches('0').trim_end_matches('.').len());
    }
    text
}

fn to_bstr(emu: &mut Emu, api: &str, words: usize, text: Result<String, u64>) {
    let call = WinApi(Stdcall);
    let locale = api_try!(emu, automation::argument_at(emu, 1, words));
    let flags = api_try!(emu, automation::argument_at(emu, 2, words + 1));
    let output = api_try!(emu, automation::argument_at(emu, 3, words + 2));
    if !english_number_format(locale, flags) {
        unsupported(emu, "oleaut32", api, emu.pc());
        return;
    }
    log_red!(
        emu,
        "oleaut32!{api} text={text:?} locale={locale:#x} output={output:#x}"
    );
    let result = text.and_then(|text| {
        if !memory::writable_pointer(emu, output) {
            return Err(E_POINTER);
        }
        let bytes = strings::encode_text(&text, true);
        let address = bstr::allocate(emu, 0, bytes.len() as u64).ok_or(E_OUTOFMEMORY)?;
        emu.maps.write_bytes(address, &bytes);
        memory::write_pointer(emu, output, address);
        Ok(())
    });
    call.set_return(emu, result.err().unwrap_or(S_OK));
    call.cleanup_args(emu, words + 3);
}

macro_rules! integer_strings {
    ($($name:ident => $arg:ident, $words:literal),+ $(,)?) => {
        $(pub fn $name(emu: &mut Emu) {
            let call = WinApi(Stdcall);
            let value = api_try!(emu, call.$arg(emu, 0));
            to_bstr(emu, stringify!($name), $words, Ok(value.to_string()));
        })+
    };
}
integer_strings!(
    VarBstrFromI1 => arg_i8, 1, VarBstrFromUI1 => arg_u8, 1,
    VarBstrFromI2 => arg_i16, 1, VarBstrFromUI2 => arg_u16, 1,
    VarBstrFromI4 => arg_i32, 1, VarBstrFromUI4 => arg_u32, 1,
    VarBstrFromI8 => arg_i64, 2, VarBstrFromUI8 => arg_u64, 2,
);

pub fn VarBstrFromBool(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let value = api_try!(emu, call.arg_i16(emu, 0));
    to_bstr(
        emu,
        "VarBstrFromBool",
        1,
        Ok(if value == 0 { "False" } else { "True" }.to_owned()),
    );
}

pub fn VarBstrFromCy(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let value = api_try!(emu, call.arg_i64(emu, 0));
    let text = format_fixed(Decimal {
        magnitude: u128::from(value.unsigned_abs()),
        scale: 4,
        negative: value < 0,
    });
    to_bstr(emu, "VarBstrFromCy", 2, Ok(text));
}

pub fn VarBstrFromDec(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let decimal = api_try!(emu, call.arg(emu, 0));
    let text = Decimal::read(emu, decimal).map(format_fixed);
    to_bstr(emu, "VarBstrFromDec", 1, text);
}
