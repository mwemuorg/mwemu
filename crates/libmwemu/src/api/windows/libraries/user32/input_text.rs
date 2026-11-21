use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::{
    memory::{read_array, read_guest_bytes},
    strings,
};
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::ERROR_NOACCESS;

const MAX_CASE_UNITS: usize = 1 << 20;
const VK_SHIFT: usize = 0x10;
const VK_CONTROL: usize = 0x11;
const VK_CAPITAL: usize = 0x14;

fn case_unit(unit: u16, upper: bool) -> u16 {
    let Some(ch) = char::from_u32(unit.into()) else {
        return unit;
    };
    let result = if upper {
        let mut it = ch.to_uppercase();
        it.next().filter(|_| it.next().is_none())
    } else {
        let mut it = ch.to_lowercase();
        it.next().filter(|_| it.next().is_none())
    };
    result
        .and_then(|v| u16::try_from(u32::from(v)).ok())
        .unwrap_or(unit)
}

fn change_case(emu: &mut Emu, api: &str, wide: bool, upper: bool, counted: bool) {
    let call = WinApi(Stdcall);
    let input = api_try!(emu, call.arg(emu, 0));
    let count = if counted {
        Some(api_try!(emu, call.arg_u32(emu, 1)) as usize)
    } else {
        None
    };
    let encoding = if wide { Encoding::Wide } else { Encoding::Ansi };
    log_red!(emu, "user32!{api} input={input:#x} count={count:?}");
    if count.is_some_and(|len| len > MAX_CASE_UNITS) {
        policy::unsupported(emu, "user32", api, emu.pc());
        return;
    }
    let result = if !counted && input <= u64::from(u16::MAX) {
        let unit = if wide {
            input as u16
        } else {
            encoding.decode_byte(input as u8)
        };
        let out = case_unit(unit, upper);
        if wide {
            u64::from(out)
        } else {
            encoding
                .encode(&[out])
                .first()
                .copied()
                .map_or(0, u64::from)
        }
    } else {
        let bytes = if let Some(count) = count {
            read_guest_bytes(emu, input, count * encoding.width())
        } else {
            strings::string_bytes(emu, input, wide, None)
        };
        let converted = bytes.and_then(|bytes| {
            let units: Vec<u16> = if wide {
                bytes
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .copied()
                    .map(|p| case_unit(u16::from_le_bytes(p), upper))
                    .collect()
            } else {
                bytes
                    .iter()
                    .map(|c| case_unit(encoding.decode_byte(*c), upper))
                    .collect()
            };
            let out = encoding.encode(&units);
            (out.is_empty()
                || (emu.maps.validate_write_range(input, out.len() as u64)
                    && emu.maps.write_bytes(input, &out)))
            .then_some(units.len())
        });
        if let Some(n) = converted {
            if counted { n as u64 } else { input }
        } else {
            emu.set_last_error(ERROR_NOACCESS);
            0
        }
    };
    call.set_return(emu, result);
    call.cleanup_args(emu, 1 + usize::from(counted));
}

fn keyboard_text(emu: &mut Emu, api: &str, wide: bool, extended: bool) {
    let call = WinApi(Stdcall);
    let [key, scan, state, output] = api_try!(emu, call.args(emu));
    let capacity = if wide {
        api_try!(emu, call.arg_i32(emu, 4))
    } else {
        1
    };
    let flags = api_try!(emu, call.arg_u32(emu, if wide { 5 } else { 4 }));
    let layout = if extended {
        api_try!(emu, call.arg(emu, if wide { 6 } else { 5 }))
    } else {
        0
    };
    log_red!(
        emu,
        "user32!{api} virtual_key={key:#x} scan_code={scan:#x} keyboard_state={state:#x} output={output:#x} capacity={capacity} flags={flags:#x} layout={layout:#x}"
    );
    if layout != 0 && layout as u16 != 0x0409 {
        policy::unsupported(emu, "user32", api, emu.pc());
        return;
    }
    let amount = if let Some(keys) = read_array::<256>(emu, state).filter(|_| state != 0) {
        let shift = keys[VK_SHIFT] & 0x80 != 0;
        let caps = keys[VK_CAPITAL] & 1 != 0;
        let control = keys[VK_CONTROL] & 0x80 != 0;
        let value = u8::try_from(key).ok().and_then(|k| match k {
            b'A'..=b'Z' => Some(if control {
                k - b'A' + 1
            } else if shift ^ caps {
                k
            } else {
                k.to_ascii_lowercase()
            }),
            b'0'..=b'9' => Some(if shift {
                b")!@#$%^&*("[(k - b'0') as usize]
            } else {
                k
            }),
            8 | 9 | 13 | 27 | 32 => Some(k),
            _ => None,
        });
        if let Some(v) = value {
            if capacity > 0 && output != 0 && emu.maps.write_word(output, u16::from(v)) {
                1
            } else {
                0
            }
        } else {
            0
        }
    } else {
        emu.set_last_error(ERROR_NOACCESS);
        0
    };
    call.set_return(emu, amount);
    call.cleanup_args(emu, 5 + usize::from(wide) + usize::from(extended));
}
pub fn CharLowerA(emu: &mut Emu) {
    change_case(emu, "CharLowerA", false, false, false);
}

pub fn CharLowerW(emu: &mut Emu) {
    change_case(emu, "CharLowerW", true, false, false);
}

pub fn CharUpperA(emu: &mut Emu) {
    change_case(emu, "CharUpperA", false, true, false);
}

pub fn CharUpperW(emu: &mut Emu) {
    change_case(emu, "CharUpperW", true, true, false);
}

pub fn CharLowerBuffA(emu: &mut Emu) {
    change_case(emu, "CharLowerBuffA", false, false, true);
}

pub fn CharLowerBuffW(emu: &mut Emu) {
    change_case(emu, "CharLowerBuffW", true, false, true);
}

pub fn CharUpperBuffA(emu: &mut Emu) {
    change_case(emu, "CharUpperBuffA", false, true, true);
}

pub fn CharUpperBuffW(emu: &mut Emu) {
    change_case(emu, "CharUpperBuffW", true, true, true);
}

pub fn ToAscii(emu: &mut Emu) {
    keyboard_text(emu, "ToAscii", false, false);
}

pub fn ToAsciiEx(emu: &mut Emu) {
    keyboard_text(emu, "ToAsciiEx", false, true);
}

pub fn ToUnicode(emu: &mut Emu) {
    keyboard_text(emu, "ToUnicode", true, false);
}

pub fn ToUnicodeEx(emu: &mut Emu) {
    keyboard_text(emu, "ToUnicodeEx", true, true);
}
