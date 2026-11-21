use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::{ExceptionType, MemoryAccess};
use crate::winapi::common::memory::read_array;
use crate::winapi::policy::api_try;

const WCHAR_BYTES: u64 = size_of::<u16>() as u64;

fn word(emu: &Emu, address: u64) -> Result<u16, u64> {
    read_array(emu, address)
        .map(u16::from_le_bytes)
        .ok_or(address)
}

fn next(address: u64) -> Result<u64, u64> {
    address.checked_add(WCHAR_BYTES).ok_or(address)
}

fn character(emu: &Emu, mut address: u64, wanted: u16, last: bool) -> Result<u64, u64> {
    let mut found = 0;
    loop {
        let unit = word(emu, address)?;
        if unit == wanted {
            found = address;
            if !last {
                return Ok(found);
            }
        }
        if unit == 0 {
            return Ok(found);
        }
        address = next(address)?;
    }
}

// Each prefix remembers the longest proper prefix that is also its suffix.
fn extend_prefix(pattern: &[u16], prefixes: &[usize], mut length: usize, unit: u16) -> usize {
    while length != 0 && pattern[length] != unit {
        length = prefixes[length - 1];
    }
    length + usize::from(pattern[length] == unit)
}

fn substring(emu: &Emu, mut address: u64, mut needle: u64) -> Result<u64, u64> {
    let mut pattern = Vec::new();
    let mut prefixes = Vec::new();
    loop {
        let unit = word(emu, needle)?;
        if unit == 0 {
            break;
        }
        let length = match prefixes.last() {
            Some(&previous) => extend_prefix(&pattern, &prefixes, previous, unit),
            None => 0,
        };
        pattern.push(unit);
        prefixes.push(length);
        needle = next(needle)?;
    }
    if pattern.is_empty() {
        return Ok(address);
    }
    let mut matched = 0;
    loop {
        let unit = word(emu, address)?;
        if unit == 0 {
            return Ok(0);
        }
        matched = extend_prefix(&pattern, &prefixes, matched, unit);
        if matched == pattern.len() {
            return Ok(address - (matched as u64 - 1) * WCHAR_BYTES);
        }
        address = next(address)?;
    }
}

fn compare(emu: &Emu, mut left: u64, mut right: u64, count: u64) -> Result<u64, u64> {
    for index in 0..count {
        let a = word(emu, left)?;
        let b = word(emu, right)?;
        if a != b {
            return Ok(u64::from((i32::from(a) - i32::from(b)) as u32));
        }
        if a == 0 || index + 1 == count {
            break;
        }
        left = next(left)?;
        right = next(right)?;
    }
    Ok(0)
}

pub(crate) fn dispatch_wide_search(api: &str, library: &str, emu: &mut Emu) -> bool {
    let handler = match api {
        "wcschr" => wcschr,
        "wcsrchr" => wcsrchr,
        "wcsstr" => wcsstr,
        "wcsncmp" => wcsncmp,
        _ => return false,
    };
    log_red!(emu, "{library}!{api}");
    handler(emu);
    true
}

fn return_search(emu: &mut Emu, result: Result<u64, u64>) {
    let call = WinApi(Cdecl);
    match result {
        Ok(value) => call.set_return(emu, value),
        Err(address) => {
            emu.memory_exception(
                ExceptionType::BadAddressDereferencing,
                address,
                size_of::<u16>(),
                MemoryAccess::Read,
            );
        }
    }
}

fn find_character(emu: &mut Emu, last: bool) {
    let call = WinApi(Cdecl);
    let text = api_try!(emu, call.arg(emu, 0));
    let wanted = api_try!(emu, call.arg_u16(emu, 1));
    let result = character(emu, text, wanted, last);
    return_search(emu, result);
}

fn wcschr(emu: &mut Emu) {
    find_character(emu, false);
}
fn wcsrchr(emu: &mut Emu) {
    find_character(emu, true);
}

fn wcsstr(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let text = api_try!(emu, call.arg(emu, 0));
    let needle = api_try!(emu, call.arg(emu, 1));
    let result = substring(emu, text, needle);
    return_search(emu, result);
}

fn wcsncmp(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let left = api_try!(emu, call.arg(emu, 0));
    let right = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    let result = compare(emu, left, right, count);
    return_search(emu, result);
}
