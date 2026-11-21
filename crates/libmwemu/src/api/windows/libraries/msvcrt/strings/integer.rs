use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::api::strconv::parse_integer_with_limit;
use crate::emu::Emu;
use crate::exception::types::{ExceptionType, MemoryAccess};
use crate::winapi::common::{
    crt::{Runtime, record_errno},
    memory::write_pointer,
};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::{EINVAL, ERANGE};

use super::bounded::{read, write};

pub(crate) fn dispatch(api: &str, emu: &mut Emu, runtime: Runtime) -> bool {
    match api {
        "atoi" | "atol" | "_wtoi" | "_wtol" | "strtol" | "strtoul" | "wcstol" | "wcstoul" => {
            number(emu, api, runtime)
        }
        "_itoa" | "_itow" => digits(emu, api, runtime),
        _ => return false,
    }
    true
}

fn number(emu: &mut Emu, api: &str, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let start = api_try!(emu, call.arg(emu, 0));
    let explicit_base = api.starts_with("str") || api.starts_with("wcs");
    let (end, base) = if explicit_base {
        (
            api_try!(emu, call.arg(emu, 1)),
            api_try!(emu, call.arg_u32(emu, 2)),
        )
    } else {
        (0, 10)
    };
    log_red!(
        emu,
        "{}!{api} source={start:#x} base={base}",
        runtime.library()
    );
    let width = if api.contains('w') {
        size_of::<u16>()
    } else {
        1
    };
    let value = if start == 0 || (base != 0 && !(2..=36).contains(&base)) {
        if !record_errno(emu, runtime, EINVAL) {
            return;
        }
        Default::default()
    } else {
        let read_unit = |index: u64| {
            let address = index
                .checked_mul(width as u64)
                .and_then(|offset| start.checked_add(offset))
                .ok_or(start)?;
            read(emu, address, width)
                .map(|v| u8::try_from(v).unwrap_or(0))
                .map_err(|fault| fault.0)
        };
        match parse_integer_with_limit(read_unit, base, api.ends_with("toul"), u32::MAX as u64) {
            Ok(integer) => integer,
            Err(address) => {
                emu.memory_exception(
                    ExceptionType::BadAddressDereferencing,
                    address,
                    width,
                    MemoryAccess::Read,
                );
                return;
            }
        }
    };
    if end != 0 && !write_pointer(emu, end, start + value.consumed * width as u64) {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    if !value.overflow || record_errno(emu, runtime, ERANGE) {
        call.set_return(emu, u64::from(value.bits as u32));
    }
}

fn digits(emu: &mut Emu, api: &str, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let input = api_try!(emu, call.arg_i32(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let radix = api_try!(emu, call.arg_u32(emu, 2));
    let width = if api == "_itow" { 2 } else { 1 };
    log_red!(
        emu,
        "{}!{api} value={input} radix={radix}",
        runtime.library()
    );
    if output == 0 || !(2..=36).contains(&radix) {
        if record_errno(emu, runtime, EINVAL) {
            call.set_return(emu, 0);
        }
        return;
    }
    let signed = input < 0 && radix == 10;
    let mut n = if signed {
        input.unsigned_abs()
    } else {
        input as u32
    };
    let mut buffer = Vec::new();
    loop {
        buffer.push(b"0123456789abcdefghijklmnopqrstuvwxyz"[(n % radix) as usize]);
        n /= radix;
        if n == 0 {
            break;
        }
    }
    if signed {
        buffer.push(b'-');
    }
    for (index, b) in buffer.into_iter().rev().chain([0]).enumerate() {
        let Some(address) = output.checked_add((index * width) as u64) else {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        };
        if write(emu, address, u16::from(b), width).is_err() {
            emu.memory_exception(
                ExceptionType::BadAddressDereferencing,
                address,
                width,
                MemoryAccess::Write,
            );
            return;
        }
    }
    call.set_return(emu, output);
}

pub fn _itoa(emu: &mut Emu, api: &str) {
    digits(emu, api, Runtime::Msvcrt);
}

msvcrt_exports!(_wtoi, _wtol, atoi, atol, strtol, strtoul);
