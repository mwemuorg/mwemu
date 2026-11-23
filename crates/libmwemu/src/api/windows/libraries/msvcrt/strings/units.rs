use super::bounded::{read, write};
use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::api::unix::{MemoryError, memory};
use crate::emu::Emu;
use crate::exception::types::{ExceptionType, MemoryAccess};
use crate::winapi::common::crt::Runtime;
use crate::winapi::policy::api_try;

type Fault = (u64, MemoryAccess);

fn advance(address: u64, width: usize, access: MemoryAccess) -> Result<u64, Fault> {
    address.checked_add(width as u64).ok_or((address, access))
}

fn length(emu: &Emu, mut source: u64, width: usize) -> Result<u64, Fault> {
    let start = source;
    while read(emu, source, width)? != 0 {
        source = advance(source, width, MemoryAccess::Read)?;
    }
    Ok((source - start) / width as u64)
}

fn compare(
    emu: &Emu,
    mut left: u64,
    mut right: u64,
    count: u64,
    width: usize,
    fold: bool,
) -> Result<u64, Fault> {
    let fold_unit = |unit: u16| {
        if fold && unit <= 0x7f {
            u16::from((unit as u8).to_ascii_lowercase())
        } else {
            unit
        }
    };
    for _ in 0..count {
        let a = fold_unit(read(emu, left, width)?);
        let b = fold_unit(read(emu, right, width)?);
        if a != b || a == 0 {
            return Ok((i32::from(a) - i32::from(b)) as u32 as u64);
        }
        left = advance(left, width, MemoryAccess::Read)?;
        right = advance(right, width, MemoryAccess::Read)?;
    }
    Ok(0)
}

fn copy_units(
    emu: &mut Emu,
    destination: u64,
    mut source: u64,
    width: usize,
    count: Option<u64>,
    append: bool,
) -> Result<u64, Fault> {
    let mut out = destination;
    if append {
        while read(emu, out, width)? != 0 {
            out = advance(out, width, MemoryAccess::Read)?;
        }
    }
    let mut ended = false;
    for _ in 0..count.unwrap_or(u64::MAX) {
        let unit = if ended { 0 } else { read(emu, source, width)? };
        write(emu, out, unit, width)?;
        ended = unit == 0;
        if ended && (append || count.is_none()) {
            return Ok(destination);
        }
        out = advance(out, width, MemoryAccess::Write)?;
        if !ended {
            source = advance(source, width, MemoryAccess::Read)?;
        }
    }
    if append {
        write(emu, out, 0, width)?;
    }
    Ok(destination)
}

fn scan(emu: &Emu, mut source: u64, value: u8, count: u64) -> Result<u64, Fault> {
    for _ in 0..count {
        if read(emu, source, 1)? == u16::from(value) {
            return Ok(source);
        }
        source = advance(source, 1, MemoryAccess::Read)?;
    }
    Ok(0)
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu, runtime: Runtime) -> bool {
    let arguments = match api {
        "strlen" | "wcslen" => 1,
        "strcpy" | "wcscpy" | "strcat" | "wcscat" | "strcmp" | "wcscmp" | "_stricmp"
        | "_strcmpi" | "_wcsicmp" | "strchr" | "strrchr" | "strstr" => 2,
        "strncpy" | "wcsncpy" | "strncat" | "wcsncat" | "strncmp" | "_strnicmp" | "_wcsnicmp"
        | "memcmp" | "memchr" => 3,
        _ => return false,
    };
    log_red!(emu, "{}!{api}", runtime.library());
    string_operation(emu, api, arguments);
    true
}

pub(crate) fn string_operation(emu: &mut Emu, api: &str, arguments: usize) {
    let call = WinApi(Cdecl);
    let a = api_try!(emu, call.arg(emu, 0));
    let b = if arguments >= 2 {
        api_try!(emu, call.arg(emu, 1))
    } else {
        0
    };
    let count = if arguments == 3 {
        Some(api_try!(emu, call.arg(emu, 2)))
    } else {
        None
    };
    let width = if api.trim_start_matches('_').starts_with("wcs") {
        size_of::<u16>()
    } else {
        1
    };
    let byte_fault = |error: MemoryError| (error.address, MemoryAccess::Read);
    let result = match api {
        "strlen" | "wcslen" => length(emu, a, width),
        "strcpy" | "wcscpy" | "strncpy" | "wcsncpy" => copy_units(emu, a, b, width, count, false),
        "strcat" | "wcscat" | "strncat" | "wcsncat" => copy_units(emu, a, b, width, count, true),
        "strchr" | "strrchr" => {
            memory::find_character(emu, a, b as u8, api == "strrchr").map_err(byte_fault)
        }
        "strstr" => memory::find_string(emu, a, b).map_err(byte_fault),
        "memcmp" => memory::compare_bytes(emu, a, b, count.unwrap_or_default())
            .map(|n| u64::from(n as u32))
            .map_err(byte_fault),
        "memchr" => scan(emu, a, b as u8, count.unwrap_or_default()),
        _ => compare(
            emu,
            a,
            b,
            count.unwrap_or(u64::MAX),
            width,
            api.starts_with('_'),
        ),
    };
    match result {
        Ok(value) => call.set_return(emu, value),
        Err((address, access)) => {
            emu.memory_exception(
                ExceptionType::BadAddressDereferencing,
                address,
                width,
                access,
            );
        }
    }
}

msvcrt_exports!(
    _strcmpi, _stricmp, _strnicmp, _wcsicmp, _wcsnicmp, memchr, memcmp, strcat, strchr, strcmp,
    strcpy, strlen, strncat, strncmp, strncpy, strrchr, strstr, wcscat, wcscmp, wcscpy, wcslen,
    wcsncat, wcsncpy
);
