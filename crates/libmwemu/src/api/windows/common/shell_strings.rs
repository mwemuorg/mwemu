use std::cmp::Ordering;

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::read_guest_bytes;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::{errors::*, nls::*};

fn folded(unit: u16, ignore_case: bool) -> u16 {
    if !ignore_case {
        return unit;
    }
    char::from_u32(u32::from(unit))
        .and_then(|ch| {
            let mut lower = ch.to_lowercase();
            let first = lower.next()?;
            (lower.next().is_none()).then_some(first)
        })
        .and_then(|ch| u16::try_from(u32::from(ch)).ok())
        .unwrap_or(unit)
}

pub(crate) fn compare_units(
    left: &[u16],
    right: &[u16],
    ordinal: bool,
    ignore_case: bool,
) -> Option<Ordering> {
    if ordinal && !ignore_case || left == right {
        return Some(left.cmp(right));
    }
    if left
        .iter()
        .chain(right)
        .any(|&unit| unit > 0x7f || !ordinal && !(unit as u8).is_ascii_alphanumeric())
    {
        return None;
    }
    let primary = left
        .iter()
        .map(|&unit| (unit as u8).to_ascii_lowercase())
        .cmp(right.iter().map(|&unit| (unit as u8).to_ascii_lowercase()));
    let order = if ordinal {
        left.iter()
            .map(|&unit| (unit as u8).to_ascii_uppercase())
            .cmp(right.iter().map(|&unit| (unit as u8).to_ascii_uppercase()))
    } else if !ignore_case && primary.is_eq() {
        left.iter()
            .map(|&unit| (unit as u8).is_ascii_uppercase())
            .cmp(right.iter().map(|&unit| (unit as u8).is_ascii_uppercase()))
    } else {
        primary
    };
    Some(order)
}

fn bounded(emu: &Emu, address: u64, count: u32, codec: Encoding) -> Option<Vec<u16>> {
    let mut text = Vec::new();
    for index in 0..count {
        let offset = index as u64 * codec.width() as u64;
        if offset >= emu.maps.max_alloc_size {
            return None;
        }
        let at = address.checked_add(offset)?;
        let unit = if codec.width() == 1 {
            codec.decode_byte(emu.maps.read_byte(at)?)
        } else {
            emu.maps.read_word(at)?
        };
        if unit == 0 {
            break;
        }
        text.push(unit);
    }
    Some(text)
}

pub(crate) fn compare_bounded(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let [left, right, count] = api_try!(emu, call.args(emu));
    let count = count as i32;
    let codec = Encoding::from_wide(api.ends_with('W'));
    let order = u32::try_from(count).ok().and_then(|limit| {
        let first = bounded(emu, left, limit, codec)?;
        let second = bounded(emu, right, limit, codec)?;
        compare_units(
            &first,
            &second,
            false,
            api.ends_with("IA") || api.ends_with("IW"),
        )
    });
    log_red!(
        emu,
        "{library}!{api} left={left:#x} right={right:#x} count={count} order={order:?}"
    );
    let Some(order) = order else {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    };
    call.set_return(emu, order as i32 as u32 as u64);
    call.cleanup_args(emu, 3);
}

pub(crate) fn reverse_search(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let [base, end, match_value] = api_try!(emu, call.args(emu));
    let codec = Encoding::from_wide(api.ends_with('W'));
    let ignore = api.ends_with("IA") || api.ends_with("IW");
    let index = codec.read(emu, base).ok().and_then(|text| {
        let bound = if end == 0 {
            text.len()
        } else {
            usize::try_from(end.checked_sub(base)? / codec.width() as u64)
                .ok()?
                .min(text.len())
        };
        if api.starts_with("StrRChr") {
            let unit = if codec.width() == 1 {
                codec.decode_byte(match_value as u8)
            } else {
                match_value as u16
            };
            return text[..bound]
                .iter()
                .rposition(|&ch| folded(ch, ignore) == folded(unit, ignore));
        }
        let needle = codec.read(emu, match_value).ok()?;
        if needle.is_empty() {
            return None;
        }
        text.windows(needle.len()).take(bound).rposition(|part| {
            part.iter()
                .zip(&needle)
                .all(|(&a, &b)| folded(a, ignore) == folded(b, ignore))
        })
    });
    let result = index
        .and_then(|offset| base.checked_add(offset as u64 * codec.width() as u64))
        .unwrap_or(0);
    log_red!(
        emu,
        "{api} start={base:#x} end={end:#x} match_value={match_value:#x} result={result:#x}"
    );
    call.set_return(emu, result);
    call.cleanup_args(emu, 3);
}

pub(crate) fn search(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let [base, argument] = api_try!(emu, call.args(emu));
    let codec = Encoding::from_wide(api.ends_with('W'));
    let ignore = api.ends_with("IA") || api.ends_with("IW");
    let is_char = api.starts_with("StrChr");
    let index = codec.read(emu, base).ok().and_then(|text| {
        if is_char {
            let unit = if codec.width() == 1 {
                codec.decode_byte(argument as u8)
            } else {
                argument as u16
            };
            return text
                .iter()
                .position(|&c| folded(c, ignore) == folded(unit, ignore));
        }
        let needle = codec.read(emu, argument).ok()?;
        match api.trim_end_matches(['A', 'W']).trim_end_matches('I') {
            "StrStr" => {
                if needle.is_empty() {
                    None
                } else {
                    text.windows(needle.len()).position(|part| {
                        part.iter()
                            .zip(&needle)
                            .all(|(&a, &b)| folded(a, ignore) == folded(b, ignore))
                    })
                }
            }
            "StrPBrk" => text.iter().position(|c| needle.contains(c)),
            "StrSpn" => Some(text.iter().take_while(|c| needle.contains(c)).count()),
            "StrCSpn" => Some(
                text.iter()
                    .take_while(|&&c| {
                        !needle
                            .iter()
                            .any(|&v| folded(v, ignore) == folded(c, ignore))
                    })
                    .count(),
            ),
            _ => None,
        }
    });
    let result = if api.starts_with("StrSpn") || api.starts_with("StrCSpn") {
        index.unwrap_or(0) as u64
    } else {
        index
            .and_then(|i| base.checked_add((i * codec.width()) as u64))
            .unwrap_or(0)
    };
    log_red!(emu, "{api} => {result:#x}");
    call.set_return(emu, result);
    call.cleanup_args(emu, 2);
}

pub(crate) fn trim(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let [base, chars] = api_try!(emu, call.args(emu));
    let codec = Encoding::from_wide(api.ends_with('W'));
    let changed = codec
        .read(emu, base)
        .ok()
        .zip(codec.read(emu, chars).ok())
        .is_some_and(|(text, set)| {
            let start = text
                .iter()
                .position(|c| !set.contains(c))
                .unwrap_or(text.len());
            let end = text
                .iter()
                .rposition(|c| !set.contains(c))
                .map_or(start, |i| i + 1);
            if start == 0 && end == text.len() {
                return false;
            }
            let bytes = codec.terminated(&text[start..end]);
            emu.maps.validate_write_range(base, bytes.len() as u64)
                && emu.maps.write_bytes(base, &bytes)
        });
    log_red!(emu, "{api} changed={changed}");
    call.set_return(emu, u64::from(changed));
    call.cleanup_args(emu, 2);
}

fn convert(unit: u16, upper: bool) -> u16 {
    let Some(ch) = char::from_u32(u32::from(unit)) else {
        return unit;
    };
    let mapped = if upper {
        let mut iter = ch.to_uppercase();
        iter.next().filter(|_| iter.next().is_none())
    } else {
        let mut iter = ch.to_lowercase();
        iter.next().filter(|_| iter.next().is_none())
    };
    mapped
        .and_then(|v| u16::try_from(u32::from(v)).ok())
        .unwrap_or(unit)
}

pub(crate) fn char_case(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));
    let wide = api.ends_with('W');
    let codec = Encoding::from_wide(wide);
    let upper = api.starts_with("CharUpper");
    let counted = api.contains("Buff");
    let capacity = if counted {
        Some(api_try!(emu, call.arg_u32(emu, 1)))
    } else {
        None
    };
    let result = (|| {
        if !counted && ptr <= u64::from(u16::MAX) {
            let unit = if wide {
                ptr as u16
            } else {
                codec.decode_byte(ptr as u8)
            };
            let mapped = convert(unit, upper);
            return Some(if wide {
                u64::from(mapped)
            } else {
                u64::from(*codec.encode(&[mapped]).first()?)
            });
        }
        let units = if let Some(count) = capacity {
            let len = (count as usize).checked_mul(codec.width())?;
            if len as u64 > emu.maps.max_alloc_size || (count != 0 && ptr == 0) {
                return None;
            }
            let bytes = read_guest_bytes(emu, ptr, len)?;
            if wide {
                bytes
                    .chunks_exact(2)
                    .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                    .collect()
            } else {
                bytes.into_iter().map(|b| codec.decode_byte(b)).collect()
            }
        } else {
            codec.read(emu, ptr).ok()?
        };
        let mut bytes = Vec::with_capacity(units.len() * codec.width());
        for u in &units {
            let value = convert(*u, upper);
            // One input unit occupies one output unit, including unpaired surrogates.
            if wide {
                bytes.extend(value.to_le_bytes());
            } else {
                bytes.extend(codec.encode(&[value]));
            }
        }
        if !bytes.is_empty()
            && (!emu.maps.validate_write_range(ptr, bytes.len() as u64)
                || !emu.maps.write_bytes(ptr, &bytes))
        {
            return None;
        }
        Some(if counted { units.len() as u64 } else { ptr })
    })();
    log_red!(emu, "{api} buffer={ptr:#x}");
    call.set_return(emu, result.unwrap_or(0));
    call.cleanup_args(emu, 1 + usize::from(counted));
}

fn wide_slice(emu: &Emu, ptr: u64, length: i32) -> Result<Vec<u16>, u64> {
    if ptr == 0 || length < -1 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    if length == -1 {
        return Encoding::Wide.read(emu, ptr);
    }
    let bytes = (length as usize)
        .checked_mul(size_of::<u16>())
        .ok_or(ERROR_INVALID_PARAMETER)?;
    if bytes as u64 > emu.maps.max_alloc_size {
        return Err(ERROR_NOT_ENOUGH_MEMORY);
    }
    let raw = read_guest_bytes(emu, ptr, bytes).ok_or(ERROR_NOACCESS)?;
    Ok(raw
        .chunks_exact(2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .collect())
}
pub(crate) fn find_ordinal(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let flags = api_try!(emu, call.arg_u32(emu, 0));
    let ptr = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg_i32(emu, 2));
    let needle = api_try!(emu, call.arg(emu, 3));
    let len = api_try!(emu, call.arg_i32(emu, 4));
    let ignore = api_try!(emu, call.arg_u32(emu, 5)) != 0;
    let result = (|| {
        if !matches!(
            flags,
            0 | FIND_FROMSTART | FIND_FROMEND | FIND_STARTSWITH | FIND_ENDSWITH
        ) {
            return Err(ERROR_INVALID_FLAGS);
        }
        let text = wide_slice(emu, ptr, count)?;
        let needle = wide_slice(emu, needle, len)?;
        let matches = |window: &[u16]| {
            window.iter().zip(&needle).all(|(&a, &b)| {
                if ignore {
                    convert(a, true) == convert(b, true)
                } else {
                    a == b
                }
            })
        };
        if needle.len() > text.len() {
            return Ok(None);
        }
        let last = text.len() - needle.len();
        if needle.is_empty() {
            return Ok(Some(if matches!(flags, FIND_FROMEND | FIND_ENDSWITH) {
                last
            } else {
                0
            }));
        }
        let mut windows = text.windows(needle.len());
        Ok(match flags {
            FIND_STARTSWITH => matches(&text[..needle.len()]).then_some(0),
            FIND_ENDSWITH => matches(&text[last..]).then_some(last),
            FIND_FROMEND => windows.rposition(matches),
            _ => windows.position(matches),
        })
    })();
    let value = match result {
        Ok(found) => {
            emu.set_last_error(ERROR_SUCCESS);
            found.map_or(u32::MAX, |i| i as u32)
        }
        Err(error) => {
            emu.set_last_error(error);
            u32::MAX
        }
    };
    log_red!(emu, "FindStringOrdinal => {value:#x}");
    call.set_return(emu, u64::from(value));
    call.cleanup_args(emu, 6);
}

pub(crate) fn map_locale_case(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let [locale, flags, src, count, dest, capacity] = api_try!(emu, call.args(emu));
    let flags = flags as u32;
    let count = count as i32;
    let capacity = capacity as i32;
    let ex = api == "LCMapStringEx";
    let wide = ex || api.ends_with('W');
    let supported = if ex {
        let version = api_try!(emu, call.arg(emu, 6));
        let reserved = api_try!(emu, call.arg(emu, 7));
        let sort = api_try!(emu, call.arg(emu, 8));
        version == 0
            && reserved == 0
            && sort == 0
            && (locale == 0
                || Encoding::Wide.read(emu, locale).is_ok_and(|v| {
                    v.is_empty() || String::from_utf16_lossy(&v).eq_ignore_ascii_case("en-US")
                }))
    } else {
        [
            0,
            LOCALE_USER_DEFAULT,
            LOCALE_SYSTEM_DEFAULT,
            u64::from(EN_US_LOCALE),
        ]
        .contains(&locale)
    };
    if !supported || !matches!(flags, LCMAP_UPPERCASE | LCMAP_LOWERCASE) {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    }
    let result = (|| {
        if count == 0 || capacity < 0 {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let mut text = source_units(emu, src, count.max(-1), wide)?;
        if count < 0 {
            text.push(0);
        }
        // The guest has no linguistic casing tables. Route other inputs through API policy.
        if text.iter().any(|&c| c > 0x7f) {
            return Ok(None);
        }
        let required = text.len() as u64;
        if capacity == 0 {
            return Ok(Some(required));
        }
        if (capacity as u64) < required {
            return Err(ERROR_INSUFFICIENT_BUFFER);
        }
        let encoding = if wide { Encoding::Wide } else { Encoding::Ansi };
        let mapped = text
            .into_iter()
            .map(|c| convert(c, flags == LCMAP_UPPERCASE))
            .collect::<Vec<_>>();
        let bytes = encoding.encode(&mapped);
        if dest == 0
            || !emu.maps.validate_write_range(dest, bytes.len() as u64)
            || !emu.maps.write_bytes(dest, &bytes)
        {
            return Err(ERROR_NOACCESS);
        }
        Ok(Some(required))
    })();
    let value = match result {
        Ok(Some(n)) => n,
        Ok(None) => {
            policy::unsupported(emu, library, api, emu.pc());
            return;
        }
        Err(error) => {
            emu.set_last_error(error);
            0
        }
    };
    log_red!(
        emu,
        "{library}!{api} flags={flags:#x} source={src:#x} length={count} output={dest:#x} capacity={capacity}"
    );
    call.set_return(emu, value);
    call.cleanup_args(emu, if ex { 9 } else { 6 });
}

pub(crate) fn source_units(emu: &Emu, ptr: u64, count: i32, wide: bool) -> Result<Vec<u16>, u64> {
    if ptr == 0 || count < -1 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    if wide {
        return wide_slice(emu, ptr, count);
    }
    if count < 0 {
        return Encoding::Ansi.read(emu, ptr);
    }
    if count as u64 > emu.maps.max_alloc_size {
        return Err(ERROR_NOT_ENOUGH_MEMORY);
    }
    let bytes = read_guest_bytes(emu, ptr, count as usize).ok_or(ERROR_NOACCESS)?;
    Ok(bytes
        .iter()
        .map(|&b| Encoding::Ansi.decode_byte(b))
        .collect())
}
