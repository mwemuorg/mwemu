use crate::emu::Emu;
use crate::winapi::common::memory::Call;
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
