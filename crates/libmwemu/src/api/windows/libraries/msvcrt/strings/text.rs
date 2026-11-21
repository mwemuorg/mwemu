use super::bounded::{read, write};
use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::api::strconv::parse_integer;
use crate::emu::Emu;
use crate::exception::types::{ExceptionType, MemoryAccess};
use crate::maps::{buffers::BufferKind, mem64::Permission};
use crate::winapi::common::buffers;
use crate::winapi::common::crt::{Runtime, record_errno};
use crate::winapi::common::memory::read_guest_bytes;
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;

fn length(emu: &Emu, address: u64, width: usize, capacity: u64) -> Result<Option<u64>, u64> {
    for i in 0..capacity {
        let at = i
            .checked_mul(width as u64)
            .and_then(|n| address.checked_add(n))
            .ok_or(address)?;
        if read(emu, at, width).map_err(|(at, _)| at)? == 0 {
            return Ok(Some(i));
        }
    }
    Ok(None)
}

fn change_case(emu: &mut Emu, runtime: Runtime, api: &str, wide: bool, upper: bool) {
    let call = WinApi(Cdecl);
    let address = api_try!(emu, call.arg(emu, 0));
    let secure = api.ends_with("_s");
    let capacity = if secure {
        api_try!(emu, call.arg(emu, 1))
    } else {
        u64::MAX
    };
    let width = if wide { 2 } else { 1 };
    log_red!(emu, "{}!{api} buffer={address:#x}", runtime.library());
    let result = (|| {
        if address == 0 || capacity == 0 {
            return Ok(EINVAL);
        }
        let Some(n) = length(emu, address, width, capacity).map_err(|a| (a, MemoryAccess::Read))?
        else {
            write(emu, address, 0, width)?;
            return Ok(EINVAL);
        };
        for i in 0..n {
            let at = address + i * width as u64;
            let value = read(emu, at, width)?;
            if let Ok(byte) = u8::try_from(value) {
                let next = if upper {
                    byte.to_ascii_uppercase()
                } else {
                    byte.to_ascii_lowercase()
                };
                if byte != next {
                    write(emu, at, u16::from(next), width)?;
                }
            }
        }
        Ok(0)
    })();
    match result {
        Ok(errno) => {
            if errno != 0 && !record_errno(emu, runtime, errno) {
                return;
            }
            call.set_return(emu, if secure { errno as u64 } else { address });
        }
        Err((at, access)) => {
            emu.memory_exception(ExceptionType::BadAddressDereferencing, at, width, access)
        }
    }
}

fn duplicate(emu: &mut Emu, runtime: Runtime, api: &str, wide: bool) {
    let call = WinApi(Cdecl);
    let address = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "{}!{api} source={address:#x}", runtime.library());
    if address == 0 {
        call.set_return(emu, 0);
        return;
    }
    let width = if wide { 2 } else { 1 };
    let n = match length(emu, address, width, u64::MAX) {
        Ok(Some(n)) => n,
        Ok(None) => {
            call.set_return(emu, 0);
            return;
        }
        Err(at) => {
            emu.memory_exception(
                ExceptionType::BadAddressDereferencing,
                at,
                width,
                MemoryAccess::Read,
            );
            return;
        }
    };
    let size = (n + 1) * width as u64;
    let copy = buffers::allocate(emu, size, 16, 0, BufferKind::Crt);
    if let Some(out) = copy {
        if let Some(bytes) = read_guest_bytes(emu, address, size as usize)
            && emu.maps.write_bytes(out, &bytes)
        {
            call.set_return(emu, out);
            return;
        }
        buffers::free(emu, out, BufferKind::Crt);
    }
    if record_errno(emu, runtime, ENOMEM) {
        call.set_return(emu, 0);
    }
}

fn decimal(emu: &mut Emu, runtime: Runtime, api: &str, wide: bool) {
    let call = WinApi(Cdecl);
    let address = api_try!(emu, call.arg(emu, 0));
    let width = if wide { 2 } else { 1 };
    log_red!(emu, "{}!{api} source={address:#x}", runtime.library());
    if address == 0 {
        if record_errno(emu, runtime, EINVAL) {
            call.set_return_u64(emu, 0);
        }
        return;
    }
    let result = parse_integer(
        |i| {
            let at = i
                .checked_mul(width as u64)
                .and_then(|n| address.checked_add(n))
                .ok_or(address)?;
            read(emu, at, width)
                .map(|n| u8::try_from(n).unwrap_or(0))
                .map_err(|(a, _)| a)
        },
        10,
        false,
    );
    match result {
        Ok(number) => {
            if !number.overflow || record_errno(emu, runtime, ERANGE) {
                call.set_return_u64(emu, number.bits);
            }
        }
        Err(at) => emu.memory_exception(
            ExceptionType::BadAddressDereferencing,
            at,
            width,
            MemoryAccess::Read,
        ),
    }
}

fn strerror(emu: &mut Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let error = api_try!(emu, call.arg_i32(emu, 0));
    let message = match error {
        0 => "No error",
        ENOENT => "No such file or directory",
        EIO => "Input/output error",
        EBADF => "Bad file descriptor",
        ENOMEM => "Not enough space",
        EACCES => "Permission denied",
        EEXIST => "File exists",
        EINVAL => "Invalid argument",
        EMFILE => "Too many open files",
        ERANGE => "Result too large",
        _ => "Unknown error",
    };
    const MESSAGE_CAPACITY: u64 = 128;
    let name = format!("{}_strerror_{}", runtime.library(), emu.current_thread_id);
    let buffer = emu
        .maps
        .get_mem2(&name)
        .map(|map| map.get_base())
        .or_else(|| {
            let at = emu.maps.alloc(MESSAGE_CAPACITY)?;
            emu.maps
                .create_map(&name, at, MESSAGE_CAPACITY, Permission::READ_WRITE)
                .ok()?;
            Some(at)
        });
    log_red!(emu, "{}!strerror errno={error}", runtime.library());
    let address = buffer.filter(|&at| {
        emu.maps.write_bytes(at, message.as_bytes())
            && emu.maps.write_byte(at + message.len() as u64, 0)
    });
    call.set_return(emu, address.unwrap_or(0));
}

pub(crate) fn dispatch_text(api: &str, emu: &mut Emu, runtime: Runtime) -> bool {
    match api {
        "_strlwr" | "_strlwr_s" => change_case(emu, runtime, api, false, false),
        "_strupr" | "_strupr_s" => change_case(emu, runtime, api, false, true),
        "_wcslwr" | "_wcslwr_s" => change_case(emu, runtime, api, true, false),
        "_wcsupr" | "_wcsupr_s" => change_case(emu, runtime, api, true, true),
        "strdup" | "_strdup" => duplicate(emu, runtime, api, false),
        "_wcsdup" => duplicate(emu, runtime, api, true),
        "_atoi64" => decimal(emu, runtime, api, false),
        "_wtoi64" => decimal(emu, runtime, api, true),
        "strerror" => strerror(emu, runtime),
        _ => return false,
    }
    true
}
