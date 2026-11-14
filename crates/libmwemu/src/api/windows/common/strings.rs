use crate::emu::Emu;
use crate::winapi::common::memory::Call;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;
use crate::windows::constants::nls::*;

fn is_lead_byte(code_page: u32, byte: u8) -> bool {
    match code_page {
        CP_ACP | CP_OEMCP | CP_MACCP | CP_THREAD_ACP | CP_WINDOWS_1252 => false,
        CP_SHIFT_JIS => matches!(byte, 0x81..=0x9f | 0xe0..=0xfc),
        CP_GBK | CP_KOREAN | CP_BIG5 => {
            matches!(byte, 0x81..=0xfe)
        }
        CP_JOHAB => matches!(byte, 0x84..=0xd3 | 0xd8..=0xde | 0xe0..=0xf9),
        _ => false,
    }
}

fn next_ansi(emu: &Emu, current: u64, code_page: u32) -> u64 {
    let Some(byte) = emu.maps.read_byte(current).filter(|byte| *byte != 0) else {
        return current;
    };
    let step = if is_lead_byte(code_page, byte)
        && emu
            .maps
            .read_byte(current.saturating_add(1))
            .is_some_and(|b| b != 0)
    {
        2
    } else {
        1
    };
    current.saturating_add(step)
}

pub(crate) fn char_next(emu: &mut Emu, api: &str, wide: bool, extended: bool) {
    let call = Call::Api;
    let code_page = if extended {
        call.arg(emu, 0) as u16 as u32
    } else {
        CP_WINDOWS_1252
    };
    let current = call.arg(emu, usize::from(extended));
    let next = if wide {
        current.saturating_add(if emu.maps.read_word(current).is_some_and(|c| c != 0) {
            size_of::<u16>() as u64
        } else {
            0
        })
    } else {
        next_ansi(emu, current, code_page)
    };
    log_red!(emu, "{} current:0x{:x} => 0x{:x}", api, current, next);
    call.finish(emu, next, if extended { 3 } else { 1 });
}

pub(crate) fn char_prev(emu: &mut Emu, api: &str, wide: bool, extended: bool) {
    let call = Call::Api;
    let code_page = if extended {
        call.arg(emu, 0) as u16 as u32
    } else {
        CP_WINDOWS_1252
    };
    let start = call.arg(emu, usize::from(extended));
    let current = call.arg(emu, usize::from(extended) + 1);
    let previous = if current <= start {
        current
    } else if wide {
        current.saturating_sub(size_of::<u16>() as u64).max(start)
    } else {
        // Walk from the start: a trail byte may also lie in a lead-byte range.
        let mut previous = start;
        loop {
            let next = next_ansi(emu, previous, code_page);
            if next == previous || next >= current {
                break previous;
            }
            previous = next;
        }
    };
    log_red!(
        emu,
        "{} start:0x{:x} current:0x{:x} => 0x{:x}",
        api,
        start,
        current,
        previous
    );
    call.finish(emu, previous, if extended { 4 } else { 2 });
}

pub(crate) fn dbcs_lead_byte(emu: &mut Emu, api: &str, extended: bool) {
    let call = Call::Api;
    let code_page = if extended {
        call.arg(emu, 0) as u32
    } else {
        CP_WINDOWS_1252
    };
    let byte = call.arg(emu, usize::from(extended)) as u8;
    let result = is_lead_byte(code_page, byte);
    log_red!(
        emu,
        "{} cp:{} byte:0x{:02x} => {}",
        api,
        code_page,
        byte,
        result
    );
    call.finish(emu, result as u64, if extended { 2 } else { 1 });
}

/// The en-US answer for a `GetLocaleInfo*` LCTYPE, shared by the 32- and
/// 64-bit stubs; unknown types fall back to "." and are logged by the caller.
pub(crate) fn locale_info_value(lctype: u64) -> Option<&'static str> {
    Some(match lctype {
        LOCALE_SLANGUAGE => "English",
        LOCALE_SCOUNTRY => "United States",
        LOCALE_SLIST => ",",
        LOCALE_SDECIMAL => ".",
        LOCALE_STHOUSAND => ",",
        LOCALE_SCURRENCY => "$",
        LOCALE_SDATE => "/",
        LOCALE_STIME => ":",
        LOCALE_ICURRDIGITS => "2",
        LOCALE_IINTLCURRDIGITS => "2",
        LOCALE_ICURRENCY => "0",
        LOCALE_INEGCURR => "0",
        LOCALE_SSHORTDATE => "M/d/yyyy",
        LOCALE_SLONGDATE => "dddd, MMMM d, yyyy",
        LOCALE_IDATE => "0",
        LOCALE_ILDATE => "0",
        LOCALE_ITIME => "0",
        LOCALE_ICENTURY => "1",
        LOCALE_ITLZERO => "0",
        LOCALE_IDAYLZERO => "0",
        LOCALE_IMONLZERO => "0",
        LOCALE_S1159 => "AM",
        LOCALE_S2359 => "PM",
        LOCALE_ICALENDARTYPE => "1",
        LOCALE_IOPTIONALCALENDAR => "1",
        LOCALE_IFIRSTDAYOFWEEK => "6",
        LOCALE_IFIRSTWEEKOFYEAR => "0",
        LOCALE_ICOUNTRY => "1",
        LOCALE_SDAYNAME1 => "Monday",
        LOCALE_SDAYNAME2 => "Tuesday",
        LOCALE_SDAYNAME3 => "Wednesday",
        LOCALE_SDAYNAME4 => "Thursday",
        LOCALE_SDAYNAME5 => "Friday",
        LOCALE_SDAYNAME6 => "Saturday",
        LOCALE_SDAYNAME7 => "Sunday",
        LOCALE_SMONTHNAME1 => "January",
        LOCALE_SMONTHNAME2 => "February",
        LOCALE_SMONTHNAME3 => "March",
        LOCALE_SMONTHNAME4 => "April",
        LOCALE_SMONTHNAME5 => "May",
        LOCALE_SMONTHNAME6 => "June",
        LOCALE_SMONTHNAME7 => "July",
        LOCALE_SMONTHNAME8 => "August",
        LOCALE_SMONTHNAME9 => "September",
        LOCALE_SMONTHNAME10 => "October",
        LOCALE_SMONTHNAME11 => "November",
        LOCALE_SMONTHNAME12 => "December",
        LOCALE_SABBREVMONTHNAME1 => "Jan",
        LOCALE_SABBREVMONTHNAME2 => "Feb",
        LOCALE_SABBREVMONTHNAME3 => "Mar",
        LOCALE_SABBREVMONTHNAME4 => "Apr",
        LOCALE_SABBREVMONTHNAME5 => "May",
        LOCALE_SABBREVMONTHNAME6 => "Jun",
        LOCALE_SABBREVMONTHNAME7 => "Jul",
        LOCALE_SABBREVMONTHNAME8 => "Aug",
        LOCALE_SABBREVMONTHNAME9 => "Sep",
        LOCALE_SABBREVMONTHNAME10 => "Oct",
        LOCALE_SABBREVMONTHNAME11 => "Nov",
        LOCALE_SABBREVMONTHNAME12 => "Dec",
        LOCALE_SABBREVDAYNAME1 => "Mon",
        LOCALE_SABBREVDAYNAME2 => "Tue",
        LOCALE_SABBREVDAYNAME3 => "Wed",
        LOCALE_SABBREVDAYNAME4 => "Thu",
        LOCALE_SABBREVDAYNAME5 => "Fri",
        LOCALE_SABBREVDAYNAME6 => "Sat",
        LOCALE_SABBREVDAYNAME7 => "Sun",
        LOCALE_ILANGUAGE => "0409",
        LOCALE_IDEFAULTLANGUAGE => "0409",
        LOCALE_IDEFAULTCOUNTRY => "1",
        LOCALE_IDEFAULTCODEPAGE => "1252",
        LOCALE_IMEASURE => "1",
        LOCALE_SGROUPING => "3;0",
        LOCALE_IDIGITS => "2",
        LOCALE_ILZERO => "1",
        LOCALE_SINTLSYMBOL => "USD",
        LOCALE_SMONDECIMALSEP => ".",
        LOCALE_SMONTHOUSANDSEP => ",",
        LOCALE_SMONGROUPING => "3;0",
        LOCALE_SPOSITIVESIGN => "",
        LOCALE_SNEGATIVESIGN => "-",
        LOCALE_STIMEFORMAT => "h:mm:ss tt",
        LOCALE_IDEFAULTANSICODEPAGE => "1252",
        LOCALE_SYEARMONTH => "MMMM yyyy",
        LOCALE_SENGCURRNAME => "US Dollar",
        LOCALE_SNATIVECURRNAME => "US Dollar",
        LOCALE_INEGNUMBER => "1",
        LOCALE_IDEFAULTMACCODEPAGE => "10000",
        LOCALE_SSORTNAME => "Default",
        LOCALE_IDIGITSUBSTITUTION => "1",
        _ => return None,
    })
}

#[derive(Clone, Copy)]
pub(crate) enum CharacterClass {
    Alphabetic,
    Alphanumeric,
    Lowercase,
    Uppercase,
}

fn ansi_character(byte: u8) -> char {
    // The emulated ANSI code page is Windows-1252.
    const EXTENDED_CHARACTERS: [char; 32] = [
        '€', '\u{0081}', '‚', 'ƒ', '„', '…', '†', '‡', 'ˆ', '‰', 'Š', '‹', 'Œ', '\u{008d}', 'Ž',
        '\u{008f}', '\u{0090}', '‘', '’', '“', '”', '•', '–', '—', '˜', '™', 'š', '›', 'œ',
        '\u{009d}', 'ž', 'Ÿ',
    ];
    if (0x80..=0x9f).contains(&byte) {
        EXTENDED_CHARACTERS[(byte - 0x80) as usize]
    } else {
        byte as char
    }
}

pub(crate) fn classify_character(emu: &mut Emu, api: &str, wide: bool, class: CharacterClass) {
    let call = Call::Api;
    let value = call.arg(emu, 0);
    let character = if wide {
        char::from_u32(value as u16 as u32)
    } else {
        Some(ansi_character(value as u8))
    };
    // Use Unicode properties without consulting the host locale. Version-specific
    // Windows NLS tables and configurable ANSI code pages are not modelled here.
    let result = character.is_some_and(|ch| match class {
        CharacterClass::Alphabetic => ch.is_alphabetic(),
        CharacterClass::Alphanumeric => ch.is_alphanumeric(),
        CharacterClass::Lowercase => ch.is_lowercase(),
        CharacterClass::Uppercase => ch.is_uppercase(),
    });
    log_red!(emu, "{} character:0x{:x} => {}", api, value, result);
    call.finish(emu, result as u64, 1);
}

fn write_computer_name(emu: &mut Emu, buffer: u64, size: u64, wide: bool) -> Result<(), u64> {
    let capacity = emu.maps.read_dword(size).ok_or(ERROR_NOACCESS)?;
    if size == 0 || !emu.maps.validate_write_range(size, size_of::<u32>() as u64) {
        return Err(ERROR_NOACCESS);
    }
    let name = &emu.cfg.host_name;
    let mut bytes: Vec<u8> = if wide {
        name.encode_utf16().flat_map(u16::to_le_bytes).collect()
    } else {
        name.as_bytes().to_vec()
    };
    let unit_size = if wide {
        size_of::<u16>()
    } else {
        size_of::<u8>()
    };
    let length = u32::try_from(bytes.len() / unit_size).map_err(|_| ERROR_NOT_ENOUGH_MEMORY)?;
    let required = length.checked_add(1).ok_or(ERROR_NOT_ENOUGH_MEMORY)?;
    if capacity < required {
        emu.maps.write_dword(size, required);
        return Err(ERROR_BUFFER_OVERFLOW);
    }
    bytes.resize(bytes.len() + unit_size, 0);
    if buffer == 0 || !emu.maps.validate_write_range(buffer, bytes.len() as u64) {
        return Err(ERROR_NOACCESS);
    }
    if !emu.maps.write_bytes(buffer, &bytes) {
        return Err(ERROR_NOACCESS);
    }
    emu.maps.write_dword(size, length);
    Ok(())
}

pub(crate) fn computer_name(emu: &mut Emu, api: &str, wide: bool) {
    let call = Call::Api;
    let buffer = call.arg(emu, 0);
    let size = call.arg(emu, 1);
    let result = write_computer_name(emu, buffer, size, wide);
    log_red!(
        emu,
        "{} buffer:0x{:x} size:0x{:x} => {:?}",
        api,
        buffer,
        size,
        result
    );
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.finish(emu, result.is_ok() as u64, 2);
}

pub(crate) fn dispatch(api: &str, library: &str, emu: &mut Emu) -> bool {
    let handler: fn(&mut Emu, &str) = match api {
        "GetComputerNameA" => |emu, name| computer_name(emu, name, false),
        "GetComputerNameW" => |emu, name| computer_name(emu, name, true),
        "CharNextA" => |emu, name| char_next(emu, name, false, false),
        "CharNextW" => |emu, name| char_next(emu, name, true, false),
        "CharNextExA" => |emu, name| char_next(emu, name, false, true),
        "CharPrevA" => |emu, name| char_prev(emu, name, false, false),
        "CharPrevW" => |emu, name| char_prev(emu, name, true, false),
        "CharPrevExA" => |emu, name| char_prev(emu, name, false, true),
        "IsDBCSLeadByte" => |emu, name| dbcs_lead_byte(emu, name, false),
        "IsDBCSLeadByteEx" => |emu, name| dbcs_lead_byte(emu, name, true),
        "IsCharAlphaA" => {
            |emu, name| classify_character(emu, name, false, CharacterClass::Alphabetic)
        }
        "IsCharAlphaW" => {
            |emu, name| classify_character(emu, name, true, CharacterClass::Alphabetic)
        }
        "IsCharAlphaNumericA" => {
            |emu, name| classify_character(emu, name, false, CharacterClass::Alphanumeric)
        }
        "IsCharAlphaNumericW" => {
            |emu, name| classify_character(emu, name, true, CharacterClass::Alphanumeric)
        }
        "IsCharLowerA" => {
            |emu, name| classify_character(emu, name, false, CharacterClass::Lowercase)
        }
        "IsCharLowerW" => {
            |emu, name| classify_character(emu, name, true, CharacterClass::Lowercase)
        }
        "IsCharUpperA" => {
            |emu, name| classify_character(emu, name, false, CharacterClass::Uppercase)
        }
        "IsCharUpperW" => {
            |emu, name| classify_character(emu, name, true, CharacterClass::Uppercase)
        }
        _ => return false,
    };
    handler(emu, &format!("{library}!{api}"));
    true
}

/// Read an API string using the emulator's existing ANSI or UTF-16 decoder.
pub(crate) fn read_text(emu: &Emu, address: u64, wide: bool) -> String {
    if wide {
        emu.maps.read_wide_string(address)
    } else {
        emu.maps.read_string(address)
    }
}

pub(crate) fn encode_text(text: &str, wide: bool) -> Vec<u8> {
    if wide {
        text.encode_utf16().flat_map(u16::to_le_bytes).collect()
    } else {
        text.as_bytes().to_vec()
    }
}

/// Size-query and bounded output for a null-terminated API string.
pub(crate) fn write_text(
    emu: &mut Emu,
    address: u64,
    capacity: i32,
    text: &str,
    wide: bool,
) -> u64 {
    let bytes = encode_text(&format!("{text}\0"), wide);
    let count = bytes.len()
        / if wide {
            size_of::<u16>()
        } else {
            size_of::<u8>()
        };
    if capacity == 0 {
        return count as u64;
    }
    let error = if capacity < 0 || address == 0 {
        ERROR_INVALID_PARAMETER
    } else if (capacity as usize) < count {
        ERROR_INSUFFICIENT_BUFFER
    } else if !emu.maps.validate_write_range(address, bytes.len() as u64)
        || !emu.maps.write_bytes(address, &bytes)
    {
        ERROR_NOACCESS
    } else {
        return count as u64;
    };
    emu.set_last_error(error);
    0
}

fn calendar_text(kind: u64) -> Option<&'static str> {
    let locale_kind = match kind {
        CAL_SCALNAME => return Some("Gregorian Calendar"),
        CAL_SERASTRING => return Some("A.D."),
        CAL_SABBREVERASTRING => return Some("AD"),
        CAL_SSHORTDATE => LOCALE_SSHORTDATE,
        CAL_SLONGDATE => LOCALE_SLONGDATE,
        CAL_SYEARMONTH => LOCALE_SYEARMONTH,
        CAL_SDAYNAME1..=CAL_SDAYNAME7 => LOCALE_SDAYNAME1 + kind - CAL_SDAYNAME1,
        CAL_SABBREVDAYNAME1..=CAL_SABBREVDAYNAME7 => {
            LOCALE_SABBREVDAYNAME1 + kind - CAL_SABBREVDAYNAME1
        }
        CAL_SMONTHNAME1..=CAL_SMONTHNAME12 => LOCALE_SMONTHNAME1 + kind - CAL_SMONTHNAME1,
        CAL_SABBREVMONTHNAME1..=CAL_SABBREVMONTHNAME12 => {
            LOCALE_SABBREVMONTHNAME1 + kind - CAL_SABBREVMONTHNAME1
        }
        CAL_SMONTHNAME13 | CAL_SABBREVMONTHNAME13 => return Some(""),
        _ => return None,
    };
    locale_info_value(locale_kind)
}

pub(crate) fn calendar_info(emu: &mut Emu, api: &str, wide: bool) {
    // Like the existing locale handlers, model a fixed en-US profile.
    const TWO_DIGIT_YEAR_MAX: u32 = 2029;
    let locale = Call::Api.arg(emu, 0) as u32;
    let calendar = Call::Api.arg(emu, 1) as u32;
    let kind = Call::Api.arg(emu, 2) as u32 as u64;
    let data = Call::Api.arg(emu, 3);
    let capacity = Call::Api.arg(emu, 4) as i32;
    let value = Call::Api.arg(emu, 5);
    let selector = kind
        & !(CAL_NOUSEROVERRIDE | CAL_RETURN_NUMBER | CAL_RETURN_GENITIVE_NAMES | CAL_USE_CP_ACP);
    let numeric = match selector {
        CAL_ICALINTVALUE => Some(calendar),
        CAL_IYEAROFFSETRANGE => Some(1),
        CAL_ITWODIGITYEARMAX => Some(TWO_DIGIT_YEAR_MAX),
        _ => None,
    };
    let result = if !matches!(locale as u64, LOCALE_USER_DEFAULT | LOCALE_SYSTEM_DEFAULT)
        && locale != EN_US_LOCALE
        || !matches!(calendar, CAL_GREGORIAN | CAL_GREGORIAN_US)
        || capacity < 0
    {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        0
    } else if kind & CAL_RETURN_NUMBER != 0 {
        if data != 0 || capacity != 0 || value == 0 {
            emu.set_last_error(ERROR_INVALID_PARAMETER);
            0
        } else if let Some(number) = numeric {
            if !emu
                .maps
                .validate_write_range(value, size_of::<u32>() as u64)
                || !emu.maps.write_dword(value, number)
            {
                emu.set_last_error(ERROR_NOACCESS);
                0
            } else {
                (size_of::<u32>() / code_unit_size(wide)) as u64
            }
        } else {
            emu.set_last_error(ERROR_INVALID_FLAGS);
            0
        }
    } else if value != 0 {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        0
    } else if let Some(number) = numeric {
        write_text(emu, data, capacity, &number.to_string(), wide)
    } else if let Some(text) = calendar_text(selector) {
        write_text(emu, data, capacity, text, wide)
    } else {
        emu.set_last_error(ERROR_INVALID_FLAGS);
        0
    };
    log_red!(
        emu,
        "{} locale:0x{:x} calendar:{} kind:0x{:x} capacity:{} => {}",
        api,
        locale,
        calendar,
        kind,
        capacity,
        result
    );
    Call::Api.finish(emu, result, 6);
}

fn code_unit_size(wide: bool) -> usize {
    if wide {
        size_of::<u16>()
    } else {
        size_of::<u8>()
    }
}

fn string_bytes(emu: &Emu, address: u64, wide: bool, limit: Option<usize>) -> Option<Vec<u8>> {
    // Match the existing map string readers' scan limit.
    const MAX_STRING_BYTES: usize = 1_000_000;
    if address == 0 {
        return None;
    }
    let unit_size = code_unit_size(wide);
    let scan_limit = MAX_STRING_BYTES / unit_size;
    let mut cursor = address;
    let mut bytes = Vec::new();
    for _ in 0..limit.unwrap_or(scan_limit).min(scan_limit) {
        let value = if wide {
            emu.maps.read_word(cursor)?
        } else {
            emu.maps.read_byte(cursor)? as u16
        };
        if value == 0 {
            return Some(bytes);
        }
        if wide {
            bytes.extend_from_slice(&value.to_le_bytes());
        } else {
            bytes.push(value as u8);
        }
        cursor = cursor.checked_add(unit_size as u64)?;
    }
    limit.filter(|count| *count <= scan_limit).map(|_| bytes)
}

fn write_string_bytes(emu: &mut Emu, destination: u64, mut bytes: Vec<u8>, wide: bool) -> u64 {
    bytes.resize(bytes.len() + code_unit_size(wide), 0);
    if destination != 0
        && emu
            .maps
            .validate_write_range(destination, bytes.len() as u64)
        && emu.maps.write_bytes(destination, &bytes)
    {
        destination
    } else {
        0
    }
}

pub(crate) fn concatenate(emu: &mut Emu, api: &str, wide: bool) {
    let destination = Call::Api.arg(emu, 0);
    let source = Call::Api.arg(emu, 1);
    let result = match (
        string_bytes(emu, destination, wide, None),
        string_bytes(emu, source, wide, None),
    ) {
        (Some(mut prefix), Some(suffix)) => {
            prefix.extend_from_slice(&suffix);
            write_string_bytes(emu, destination, prefix, wide)
        }
        _ => 0,
    };
    log_red!(
        emu,
        "{} destination:0x{:x} source:0x{:x} => 0x{:x}",
        api,
        destination,
        source,
        result
    );
    Call::Api.finish(emu, result, 2);
}

pub(crate) fn copy_string(emu: &mut Emu, api: &str, wide: bool, bounded: bool) {
    let destination = Call::Api.arg(emu, 0);
    let source = Call::Api.arg(emu, 1);
    let capacity = bounded.then(|| Call::Api.arg(emu, 2) as u32 as usize);
    let result = if capacity == Some(0) {
        destination
    } else {
        string_bytes(emu, source, wide, capacity.map(|count| count - 1))
            .map(|bytes| write_string_bytes(emu, destination, bytes, wide))
            .unwrap_or(0)
    };
    log_red!(
        emu,
        "{} destination:0x{:x} source:0x{:x} capacity:{:?} => 0x{:x}",
        api,
        destination,
        source,
        capacity,
        result
    );
    Call::Api.finish(emu, result, if bounded { 3 } else { 2 });
}

pub(crate) fn string_length(emu: &mut Emu, api: &str, wide: bool) {
    let source = Call::Api.arg(emu, 0);
    let length = string_bytes(emu, source, wide, None)
        .map(|bytes| bytes.len() / code_unit_size(wide))
        .unwrap_or(0);
    log_red!(emu, "{} source:0x{:x} => {}", api, source, length);
    Call::Api.finish(emu, length as u64, 1);
}

pub(crate) fn compare(emu: &mut Emu, api: &str, wide: bool, ignore_case: bool) {
    let mut left = read_text(emu, Call::Api.arg(emu, 0), wide);
    let mut right = read_text(emu, Call::Api.arg(emu, 1), wide);
    if ignore_case {
        left = left.to_lowercase();
        right = right.to_lowercase();
    }
    let result = match left.cmp(&right) {
        std::cmp::Ordering::Less => -1i32,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    };
    log_red!(emu, "{} '{}' '{}' => {}", api, left, right, result);
    Call::Api.finish(emu, result as u32 as u64, 2);
}

/// Unbounded API string output. Callers with a capacity must check it first or
/// use write_text; this adapter preserves the existing maps writer semantics.
pub(crate) fn write_string(emu: &mut Emu, address: u64, text: &str, wide: bool) {
    if wide {
        emu.maps.write_wide_string(address, text);
    } else {
        emu.maps.write_string(address, text);
    }
}

/// The integer/string subset of wsprintf. Unlike wvsprintf, its trailing
/// arguments are passed directly by the caller, not through a va_list pointer.
pub(crate) fn wsprintf(emu: &mut Emu, api: &str, wide: bool) {
    const BUFFER_CHARACTERS: i32 = 1024;
    let destination = WinApiAbi::arg(emu, 0);
    let format = read_text(emu, WinApiAbi::arg(emu, 1), wide);
    let mut position = 2;
    let mut characters = format.chars().peekable();
    let mut result = String::new();
    while let Some(character) = characters.next() {
        if character != '%' {
            result.push(character);
            continue;
        }
        if characters.peek() == Some(&'%') {
            characters.next();
            result.push('%');
            continue;
        }
        let left_justify = characters.next_if_eq(&'-').is_some();
        let zero_pad = characters.next_if_eq(&'0').is_some();
        let mut width = 0usize;
        while let Some(digit) = characters.peek().and_then(|c| c.to_digit(10)) {
            width = (width * 10 + digit as usize).min(BUFFER_CHARACTERS as usize);
            characters.next();
        }
        let mut precision = None;
        if characters.next_if_eq(&'.').is_some() {
            let mut length = 0usize;
            while let Some(digit) = characters.peek().and_then(|c| c.to_digit(10)) {
                length = (length * 10 + digit as usize).min(BUFFER_CHARACTERS as usize);
                characters.next();
            }
            precision = Some(length);
        }
        let modifier = characters.next_if(|c| matches!(c, 'h' | 'l' | 'w'));
        let Some(specifier) = characters.next() else {
            break;
        };
        let argument = WinApiAbi::arg(emu, position);
        position += 1;
        let text_wide = match modifier {
            Some('h') => false,
            Some('l' | 'w') => true,
            _ => {
                if matches!(specifier, 'S' | 'C') {
                    !wide
                } else {
                    wide
                }
            }
        };
        let mut field = match specifier {
            's' | 'S' => read_text(emu, argument, text_wide),
            'c' | 'C' => char::from_u32(if text_wide {
                argument as u16 as u32
            } else {
                argument as u8 as u32
            })
            .unwrap_or(char::REPLACEMENT_CHARACTER)
            .to_string(),
            'd' | 'i' => (argument as i32).to_string(),
            'u' => (argument as u32).to_string(),
            'x' => format!("{:x}", argument as u32),
            'X' => format!("{:X}", argument as u32),
            _ => {
                result.push('%');
                result.push(specifier);
                continue;
            }
        };
        if matches!(specifier, 's' | 'S')
            && let Some(limit) = precision
        {
            field = field.chars().take(limit).collect();
        }
        let padding = width.saturating_sub(field.chars().count());
        let fill = if zero_pad && !left_justify && !matches!(specifier, 's' | 'S' | 'c' | 'C') {
            '0'
        } else {
            ' '
        };
        if !left_justify {
            if fill == '0' && field.starts_with('-') {
                result.push('-');
                field.remove(0);
            }
            result.extend(std::iter::repeat_n(fill, padding));
        }
        result.push_str(&field);
        if left_justify {
            result.extend(std::iter::repeat_n(' ', padding));
        }
        if result.len() > BUFFER_CHARACTERS as usize * size_of::<u16>() {
            break;
        }
    }
    let written = write_text(emu, destination, BUFFER_CHARACTERS, &result, wide);
    log_red!(emu, "user32!{} format:'{}' => '{}'", api, format, result);
    // wsprintf is cdecl even when exported by user32.
    WinApiAbi::set_ret(emu, written.saturating_sub(1));
}
