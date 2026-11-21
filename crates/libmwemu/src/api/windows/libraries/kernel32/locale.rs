use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::read_guest_bytes;
use crate::winapi::common::{
    shell_strings::{compare_units, source_units},
    strings,
};
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use crate::windows::constants::nls::*;
use crate::windows::structures::CpInfo;
use std::{cmp::Ordering, str::from_utf8};

pub(crate) fn locale(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let name = "en-US";
    let (result, slots) = match api {
        "GetUserDefaultLCID"
        | "GetSystemDefaultLCID"
        | "GetSystemDefaultUILanguage"
        | "GetUserDefaultLangID"
        | "GetUserDefaultUILanguage"
        | "GetSystemDefaultLangID" => (Ok(u64::from(EN_US_LOCALE)), 0),
        "GetUserDefaultLocaleName" | "GetSystemDefaultLocaleName" | "LCIDToLocaleName" => {
            let convert = api == "LCIDToLocaleName";
            if convert {
                let id = api_try!(emu, call.arg_u32(emu, 0));
                let flags = api_try!(emu, call.arg_u32(emu, 3));
                if ![
                    u64::from(EN_US_LOCALE),
                    LOCALE_USER_DEFAULT,
                    LOCALE_SYSTEM_DEFAULT,
                ]
                .contains(&u64::from(id))
                    || flags != 0
                {
                    policy::unsupported(emu, library, api, emu.pc());
                    return;
                }
            }
            let index = usize::from(convert);
            let dest = api_try!(emu, call.arg(emu, index));
            let capacity = api_try!(emu, call.arg_i32(emu, index + 1));
            let bytes = Encoding::Wide.terminated(&name.encode_utf16().collect::<Vec<_>>());
            let required = (bytes.len() / 2) as i32;
            let result = if convert && capacity == 0 {
                Ok(required as u64)
            } else if capacity < 0 || dest == 0 {
                Err(ERROR_INVALID_PARAMETER)
            } else if capacity < required {
                Err(ERROR_INSUFFICIENT_BUFFER)
            } else if !emu.maps.validate_write_range(dest, bytes.len() as u64)
                || !emu.maps.write_bytes(dest, &bytes)
            {
                Err(ERROR_NOACCESS)
            } else {
                Ok(required as u64)
            };
            (result, if convert { 4 } else { 2 })
        }
        "LocaleNameToLCID" | "IsValidLocaleName" => {
            let address = api_try!(emu, call.arg(emu, 0));
            let convert = api == "LocaleNameToLCID";
            let flags = if convert {
                api_try!(emu, call.arg_u32(emu, 1))
            } else {
                0
            };
            let supported = address == 0 && convert
                || Encoding::Wide
                    .read(emu, address)
                    .ok()
                    .is_some_and(|v| String::from_utf16_lossy(&v).eq_ignore_ascii_case(name));
            if flags != 0 || !supported {
                policy::unsupported(emu, library, api, emu.pc());
                return;
            }
            (
                Ok(if convert { u64::from(EN_US_LOCALE) } else { 1 }),
                1 + usize::from(convert),
            )
        }
        _ => return,
    };
    let value = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    });
    log_red!(emu, "{api} => {value:#x}");
    call.set_return(emu, value);
    call.cleanup_args(emu, slots);
}

pub(crate) fn preferred_languages(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let flags = api_try!(emu, call.arg_u32(emu, 0));
    let count = api_try!(emu, call.arg(emu, 1));
    let out = api_try!(emu, call.arg(emu, 2));
    let size = api_try!(emu, call.arg(emu, 3));
    let text = match flags {
        0 | MUI_LANGUAGE_NAME => "en-US",
        MUI_LANGUAGE_ID => "0409",
        _ => {
            policy::unsupported(emu, library, api, emu.pc());
            return;
        }
    };
    let mut bytes = Encoding::Wide.terminated(&text.encode_utf16().collect::<Vec<_>>());
    bytes.extend([0, 0]);
    let required = (bytes.len() / 2) as u32;
    let result = (|| {
        let capacity = emu.maps.read_dword(size).ok_or(ERROR_NOACCESS)?;
        if count == 0
            || size == 0
            || !emu.maps.validate_write_range(count, 4)
            || !emu.maps.validate_write_range(size, 4)
        {
            return Err(ERROR_NOACCESS);
        }
        if out != 0
            && capacity >= required
            && !emu.maps.validate_write_range(out, bytes.len() as u64)
        {
            return Err(ERROR_NOACCESS);
        }
        emu.maps.write_dword(size, required);
        emu.maps.write_dword(count, 1);
        if out == 0 && capacity == 0 {
            return Ok(());
        }
        if capacity < required {
            return Err(ERROR_INSUFFICIENT_BUFFER);
        }
        if out == 0 || !emu.maps.write_bytes(out, &bytes) {
            return Err(ERROR_NOACCESS);
        }
        Ok(())
    })();
    log_red!(emu, "{api} flags={flags:#x}");
    if let Err(e) = result {
        emu.set_last_error(e);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 4);
}

fn ascii_ctype1(byte: u8) -> u16 {
    let category = match byte {
        b'A'..=b'Z' => C1_ALPHA | C1_UPPER,
        b'a'..=b'z' => C1_ALPHA | C1_LOWER,
        b'0'..=b'9' => C1_DIGIT,
        b' ' => C1_BLANK | C1_SPACE,
        b'\t' => C1_CNTRL | C1_SPACE | C1_BLANK,
        b'\n'..=b'\r' => C1_CNTRL | C1_SPACE,
        0..=31 | 127 => C1_CNTRL,
        _ => C1_PUNCT,
    };
    C1_DEFINED
        | category
        | if byte.is_ascii_hexdigit() {
            C1_XDIGIT
        } else {
            0
        }
}

fn ascii_ctype2(byte: u8) -> u16 {
    match byte {
        b'A'..=b'Z' | b'a'..=b'z' => C2_LEFTTORIGHT,
        b'0'..=b'9' => C2_EUROPENUMBER,
        b'+' | b'-' => C2_EUROPESEPARATOR,
        b'#' | b'$' | b'%' => C2_EUROPETERMINATOR,
        b',' | b'.' | b'/' | b':' => C2_COMMONSEPARATOR,
        b'\n' | b'\r' | 28..=30 => C2_BLOCKSEPARATOR,
        b'\t' | 11 | 31 => C2_SEGMENTSEPARATOR,
        b' ' | 12 => C2_WHITESPACE,
        0..=31 | 127 => C2_NOTAPPLICABLE,
        _ => C2_OTHERNEUTRAL,
    }
}

fn ascii_ctype3(byte: u8) -> u16 {
    match byte {
        b'A'..=b'Z' | b'a'..=b'z' => C3_HALFWIDTH | C3_ALPHA,
        b'0'..=b'9' => C3_HALFWIDTH,
        b'\t'..=b'\r' => C3_SYMBOL,
        0..=31 | 127 => 0,
        b'\'' | b'-' => C3_HALFWIDTH | C3_LEXICAL,
        b'^' | b'`' => C3_HALFWIDTH | C3_LEXICAL | C3_SYMBOL | C3_DIACRITIC,
        b'"' | b'$' | b'/' | b'=' | b'@' | b'\\' | b'_' | b'~' => {
            C3_HALFWIDTH | C3_LEXICAL | C3_SYMBOL
        }
        _ => C3_HALFWIDTH | C3_SYMBOL,
    }
}

pub(crate) fn character_types(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let wide = api.ends_with('W');
    let first = usize::from(api != "GetStringTypeW");
    let locale = if first != 0 {
        api_try!(emu, call.arg_u32(emu, 0))
    } else {
        0
    };
    let info_type = api_try!(emu, call.arg_u32(emu, first));
    let source = api_try!(emu, call.arg(emu, first + 1));
    let count = api_try!(emu, call.arg_i32(emu, first + 2));
    let types = api_try!(emu, call.arg(emu, first + 3));
    log_red!(
        emu,
        "{library}!{api} locale={locale:#x} info_type={info_type} source={source:#x} count={count} character_types={types:#x}"
    );
    let result = (|| {
        if count == 0 || types == 0 || types == source {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let classify = match info_type {
            CT_CTYPE1 => ascii_ctype1,
            CT_CTYPE2 => ascii_ctype2,
            CT_CTYPE3 => ascii_ctype3,
            _ => return Err(ERROR_INVALID_FLAGS),
        };
        let mut units = source_units(emu, source, count.max(-1), wide)?;
        if count < 0 {
            units.push(0);
        }
        if units.iter().any(|&c| c > 0x7f) {
            return Ok(false);
        }
        let len = units.len() as u64 * size_of::<u16>() as u64;
        if !emu.maps.validate_write_range(types, len) {
            return Err(ERROR_NOACCESS);
        }
        let bytes: Vec<_> = units
            .iter()
            .flat_map(|&c| classify(c as u8).to_le_bytes())
            .collect();
        emu.maps
            .write_bytes(types, &bytes)
            .then_some(true)
            .ok_or(ERROR_NOACCESS)
    })();
    match result {
        Ok(false) => {
            policy::unsupported(emu, library, api, emu.pc());
            return;
        }
        Err(error) => emu.set_last_error(error),
        Ok(true) => {}
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, first + 4);
}

fn collation_result(order: Ordering) -> u64 {
    match order {
        Ordering::Less => CSTR_LESS_THAN,
        Ordering::Equal => CSTR_EQUAL,
        Ordering::Greater => CSTR_GREATER_THAN,
    }
}

pub(crate) fn compare_text(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let ordinal = api == "CompareStringOrdinal";
    let first = if ordinal { 0 } else { 2 };
    let locale = if ordinal {
        0
    } else {
        api_try!(emu, call.arg_u32(emu, 0))
    };
    let flags = api_try!(emu, call.arg_u32(emu, if ordinal { 4 } else { 1 }));
    let left = api_try!(emu, call.arg(emu, first));
    let left_count = api_try!(emu, call.arg_i32(emu, first + 1));
    let right = api_try!(emu, call.arg(emu, first + 2));
    let right_count = api_try!(emu, call.arg_i32(emu, first + 3));
    let wide = ordinal || api.ends_with('W');
    log_red!(
        emu,
        "{library}!{api} locale={locale:#x} flags={flags:#x} left={left:#x} left_count={left_count} right={right:#x} right_count={right_count}"
    );
    let result = (|| {
        if ordinal && flags > 1 {
            return Err(ERROR_INVALID_PARAMETER);
        }
        if !ordinal
            && (![
                0,
                LOCALE_USER_DEFAULT,
                LOCALE_SYSTEM_DEFAULT,
                u64::from(EN_US_LOCALE),
                LOCALE_INVARIANT,
            ]
            .contains(&u64::from(locale))
                || flags & !NORM_IGNORECASE != 0)
        {
            return Ok(None);
        }
        let length = |count: i32| if ordinal { count } else { count.max(-1) };
        let left = source_units(emu, left, length(left_count), wide)?;
        let right = source_units(emu, right, length(right_count), wide)?;
        let Some(order) = compare_units(&left, &right, ordinal, flags != 0) else {
            return Ok(None);
        };
        Ok(Some(collation_result(order)))
    })();
    let value = match result {
        Ok(Some(value)) => value,
        Ok(None) => {
            policy::unsupported(emu, library, api, emu.pc());
            return;
        }
        Err(error) => {
            emu.set_last_error(error);
            0
        }
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, if ordinal { 5 } else { 6 });
}

pub(crate) fn locale_info(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let locale = api_try!(emu, call.arg(emu, 0));
    let kind = u64::from(api_try!(emu, call.arg_u32(emu, 1)));
    let output = api_try!(emu, call.arg(emu, 2));
    let capacity = api_try!(emu, call.arg_i32(emu, 3));
    let ex = api.ends_with("Ex");
    let wide = ex || api.ends_with('W');
    let supported = if ex {
        locale == 0
            || Encoding::Wide
                .read(emu, locale)
                .is_ok_and(|n| String::from_utf16_lossy(&n).eq_ignore_ascii_case("en-US"))
    } else {
        [
            LOCALE_USER_DEFAULT,
            LOCALE_SYSTEM_DEFAULT,
            u64::from(EN_US_LOCALE),
            0,
        ]
        .contains(&locale)
    };
    log_red!(
        emu,
        "{library}!{api} locale={locale:#x} type={kind:#x} output={output:#x} capacity={capacity}"
    );
    if !supported {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    }
    let value = if kind == LOCALE_SNAME {
        Some("en-US")
    } else {
        strings::locale_info_value(kind)
    };
    let result = if let Some(text) = value {
        strings::write_text(emu, output, capacity, text, wide)
    } else if kind & LOCALE_RETURN_NUMBER != 0 && kind as u16 != 0 {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    } else {
        emu.set_last_error(ERROR_INVALID_FLAGS);
        0
    };
    call.set_return(emu, result);
    call.cleanup_args(emu, 4);
}

pub(crate) fn multi_byte_to_wide(emu: &mut Emu, library: &str) {
    let call = WinApi(Stdcall);
    let code_page = api_try!(emu, call.arg_u32(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let source = api_try!(emu, call.arg(emu, 2));
    let length = api_try!(emu, call.arg_i32(emu, 3));
    let output = api_try!(emu, call.arg(emu, 4));
    let capacity = api_try!(emu, call.arg_i32(emu, 5));
    log_red!(
        emu,
        "{library}!MultiByteToWideChar code_page={code_page} flags={flags:#x} source={source:#x} length={length} output={output:#x} capacity={capacity}"
    );
    let result = (|| {
        if source == 0
            || length == 0
            || length < -1
            || capacity < 0
            || capacity > 0 && (output == 0 || output == source)
        {
            return Err(ERROR_INVALID_PARAMETER);
        }
        if !matches!(
            code_page,
            CP_ACP | CP_THREAD_ACP | CP_WINDOWS_1252 | CP_OEMCP | CP_IBM437
        ) && u64::from(code_page) != CP_UTF8
        {
            return Ok(None);
        }
        if u64::from(code_page) == CP_UTF8 {
            if flags & !MB_ERR_INVALID_CHARS != 0 {
                return Err(ERROR_INVALID_FLAGS);
            }
        } else if flags & !(MB_PRECOMPOSED | MB_ERR_INVALID_CHARS) != 0 {
            return Ok(None);
        }
        let bytes = if length == -1 {
            let mut bytes =
                strings::string_bytes(emu, source, false, None).ok_or(ERROR_NOACCESS)?;
            bytes.push(0);
            bytes
        } else {
            if length as u64 > emu.maps.max_alloc_size {
                return Err(ERROR_NOT_ENOUGH_MEMORY);
            }
            read_guest_bytes(emu, source, length as usize).ok_or(ERROR_NOACCESS)?
        };
        let units: Vec<u16> = if u64::from(code_page) == CP_UTF8 {
            if flags == MB_ERR_INVALID_CHARS {
                from_utf8(&bytes)
                    .map_err(|_| ERROR_NO_UNICODE_TRANSLATION)?
                    .encode_utf16()
                    .collect()
            } else {
                String::from_utf8_lossy(&bytes).encode_utf16().collect()
            }
        } else {
            let encoding = if matches!(code_page, CP_OEMCP | CP_IBM437) {
                Encoding::Oem
            } else {
                Encoding::Ansi
            };
            bytes.into_iter().map(|b| encoding.decode_byte(b)).collect()
        };
        let n = units.len() as u64;
        if capacity == 0 {
            return Ok(Some(n));
        }
        if (capacity as u64) < n {
            return Err(ERROR_INSUFFICIENT_BUFFER);
        }
        let bytes = Encoding::Wide.encode(&units);
        if !emu.maps.validate_write_range(output, bytes.len() as u64)
            || !emu.maps.write_bytes(output, &bytes)
        {
            return Err(ERROR_NOACCESS);
        }
        Ok(Some(n))
    })();
    let value = match result {
        Ok(Some(n)) => n,
        Ok(None) => {
            policy::unsupported(emu, library, "MultiByteToWideChar", emu.pc());
            return;
        }
        Err(error) => {
            emu.set_last_error(error);
            0
        }
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, 6);
}

pub(crate) fn code_page_info(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let page = api_try!(emu, call.arg_u32(emu, 0));
    let query = api == "GetCPInfo";
    let output = if query {
        api_try!(emu, call.arg(emu, 1))
    } else {
        0
    };
    let mut info = CpInfo::default();
    let supported = match page {
        CP_ACP | CP_THREAD_ACP | CP_MACCP | CP_OEMCP => query,
        CP_IBM437 | CP_WINDOWS_1252 => true,
        CP_SHIFT_JIS | CP_GBK | CP_KOREAN | CP_BIG5 | CP_JOHAB => {
            let ranges: &[u8] = match page {
                CP_SHIFT_JIS => &[0x81, 0x9f, 0xe0, 0xfc],
                CP_JOHAB => &[0x84, 0xd3, 0xd8, 0xde, 0xe0, 0xf9],
                _ => &[0x81, 0xfe],
            };
            info.max_char_size = 2;
            info.lead_byte[..ranges.len()].copy_from_slice(ranges);
            true
        }
        n if u64::from(n) == CP_UTF8 => {
            info.max_char_size = 4;
            true
        }
        n if u64::from(n) == CP_UTF7 => {
            info.max_char_size = 5;
            true
        }
        _ => false,
    };
    let mut success = supported;
    if query {
        if !supported || output == 0 {
            emu.set_last_error(ERROR_INVALID_PARAMETER);
            success = false;
        } else if !info.save_fields(output, &mut emu.maps) {
            emu.set_last_error(ERROR_NOACCESS);
            success = false;
        }
    }
    log_red!(
        emu,
        "{library}!{api} code_page={page} supported={supported}"
    );
    call.set_return(emu, u64::from(success));
    call.cleanup_args(emu, 1 + usize::from(query));
}

pub(crate) fn thread_locale(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    if api == "GetThreadLocale" {
        log_red!(emu, "{library}!GetThreadLocale locale={EN_US_LOCALE:#x}");
        call.set_return(emu, u64::from(EN_US_LOCALE));
        return;
    }
    let locale = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "{library}!SetThreadLocale locale={locale:#x}");
    if locale != EN_US_LOCALE
        && u64::from(locale) != LOCALE_USER_DEFAULT
        && u64::from(locale) != LOCALE_SYSTEM_DEFAULT
    {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    }
    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}
